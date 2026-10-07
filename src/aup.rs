use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use bitflags::bitflags;
use nom::bytes::complete::{take, take_until};
use nom::multi::count;
use nom::number::complete::{le_i32, le_u32};

use crate::codec::{
    SliceReader, checked_len, compress, decode_sjis, decompress_into, encode_sjis,
    encode_sjis_fixed, header, parse, put_bytes, put_i16, put_i32, put_u32, write_i32, write_u32,
};
use crate::exedit::ExEditProject;
use crate::{AupError, Result};

const AUP_HEADER: &[u8] = b"AviUtl ProjectFile version 0.18\0";
const FILTER_HEADER: &[u8] = b"FilterProject 0.1\0";
const MAX_COLLECTION_ITEMS: usize = 10_000_000;

/// AviUtl プロジェクトファイルを表します。
#[derive(Clone, Debug, Default)]
pub struct AviUtlProject {
    pub edit_handle: EditHandle,
    pub data_before_footer: Vec<u8>,
    pub filter_projects: Vec<FilterProject>,
}

impl AviUtlProject {
    /// リーダーからプロジェクトを読み込みます。
    pub fn read<R: Read>(mut reader: R) -> Result<Self> {
        let mut input = Vec::new();
        reader.read_to_end(&mut input)?;
        Self::from_bytes(&input)
    }

    /// バイト列からプロジェクトを読み込みます。
    pub fn from_bytes(mut input: &[u8]) -> Result<Self> {
        header(&mut input, AUP_HEADER, "AviUtl project")?;
        let edit_handle = EditHandle::read(&mut input)?;
        let data_before_footer = parse(&mut input, take_until(AUP_HEADER))
            .map_err(|_| AupError::InvalidHeader {
                kind: "AviUtl project footer",
            })?
            .to_vec();
        header(&mut input, AUP_HEADER, "AviUtl project footer")?;
        let mut filter_projects = Vec::new();
        while !input.is_empty() {
            filter_projects.push(FilterProject::Raw(RawFilterProject::read(&mut input)?));
        }

        Ok(Self {
            edit_handle,
            data_before_footer,
            filter_projects,
        })
    }

    /// ファイルからプロジェクトを読み込みます。
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Self::read(File::open(path)?)
    }

    /// ライターへプロジェクトを書き込みます。
    pub fn write<W: Write>(&self, mut writer: W) -> Result<()> {
        writer.write_all(AUP_HEADER)?;
        self.edit_handle.write(&mut writer)?;
        writer.write_all(&self.data_before_footer)?;
        writer.write_all(AUP_HEADER)?;
        for filter in &self.filter_projects {
            filter.write(&mut writer)?;
        }
        Ok(())
    }

    /// ファイルへプロジェクトを書き込みます。
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        self.write(File::create(path)?)
    }

    /// 最初の拡張編集フィルターを型付きデータへ変換して返します。
    pub fn decode_exedit(&mut self) -> Result<Option<&mut ExEditProject>> {
        let index = self.filter_projects.iter().position(|filter| {
            matches!(filter, FilterProject::ExEdit(_))
                || filter.name() == ExEditProject::FILTER_NAME
        });
        let Some(index) = index else {
            return Ok(None);
        };

        if let FilterProject::Raw(raw) = &self.filter_projects[index] {
            let exedit = ExEditProject::try_from(raw.clone())?;
            self.filter_projects[index] = FilterProject::ExEdit(Box::new(exedit));
        }
        match &mut self.filter_projects[index] {
            FilterProject::ExEdit(exedit) => Ok(Some(exedit)),
            FilterProject::Raw(_) => unreachable!("raw filter was converted above"),
        }
    }
}

/// AviUtl の EditHandle を表します。
#[derive(Clone, Debug)]
pub struct EditHandle {
    pub data: Vec<u8>,
    pub flag: u32,
    pub edit_filename: String,
    pub output_filename: String,
    pub project_filename: String,
    pub width: i32,
    pub height: i32,
    pub selected_frame_start: i32,
    pub selected_frame_end: i32,
    pub resized_width: i32,
    pub resized_height: i32,
    pub current_frame: i32,
    pub video_decode_bit: i16,
    pub video_decode_format: u32,
    pub audio_ch: i16,
    pub audio_rate: i32,
    pub video_scale: i32,
    pub video_rate: i32,
    pub frames: Vec<FrameStatus>,
    pub filter_configs: Vec<FilterConfig>,
    pub clipped_images: Vec<Option<ClippedImage>>,
}

impl EditHandle {
    pub const SIZE: usize = 0x4c09e8;
    pub const UNCOMPRESSED_SIZE: usize = 0x20c;
    pub const MAX_FILENAME: usize = 260;
    pub const MAX_CONFIG_FILES: usize = 96;
    pub const MAX_IMAGES: usize = 256;

    const DATA_SIZE: usize = Self::SIZE - Self::UNCOMPRESSED_SIZE;

    /// 入力の先頭から EditHandle を読み込み、入力を未消費部分へ進めます。
    pub fn read(reader: &mut &[u8]) -> Result<Self> {
        let size = checked_len(parse(reader, le_i32)?, "EditHandle size")?;
        if size != Self::SIZE {
            return Err(AupError::InvalidValue {
                field: "EditHandle size",
                value: size as i128,
            });
        }
        let flag = parse(reader, le_u32)?;
        let edit_filename = decode_sjis(
            parse(reader, take(Self::MAX_FILENAME))?,
            "EditHandle.edit_filename",
        )?;
        let output_filename = decode_sjis(
            parse(reader, take(Self::MAX_FILENAME))?,
            "EditHandle.output_filename",
        )?;
        let mut data = vec![0; Self::DATA_SIZE];
        decompress_into(reader, &mut data)?;
        let view = SliceReader::new(&data, "EditHandle data");

        let project_filename = decode_sjis(
            view.bytes(0, Self::MAX_FILENAME)?,
            "EditHandle.project_filename",
        )?;
        let width = view.i32(0x310 - Self::UNCOMPRESSED_SIZE)?;
        let height = view.i32(0x314 - Self::UNCOMPRESSED_SIZE)?;
        let frame_count = checked_len(parse(reader, le_i32)?, "frame count")?;
        if frame_count > MAX_COLLECTION_ITEMS {
            return Err(AupError::InvalidValue {
                field: "frame count",
                value: frame_count as i128,
            });
        }

        let videos = read_compressed_u32_array(reader, frame_count)?;
        let audios = read_compressed_u32_array(reader, frame_count)?;
        let fields2 = read_compressed_u32_array(reader, frame_count)?;
        let fields3 = read_compressed_u32_array(reader, frame_count)?;
        let inters = read_compressed_u8_array(reader, frame_count)?;
        let index_24fps = read_compressed_u8_array(reader, frame_count)?;
        let edit_flags = read_compressed_u8_array(reader, frame_count)?;
        let configs = read_compressed_u8_array(reader, frame_count)?;
        let vcms = read_compressed_u8_array(reader, frame_count)?;
        let clips = read_compressed_u8_array(reader, frame_count)?;
        let frames = (0..frame_count)
            .map(|index| FrameStatus {
                video: videos[index],
                audio: audios[index],
                field2: fields2[index],
                field3: fields3[index],
                inter: FrameStatusInter(inters[index]),
                index_24fps: index_24fps[index],
                edit_flag: EditFrameEditFlag::from_bits_retain(edit_flags[index]),
                config: configs[index],
                vcm: vcms[index],
                clip: clips[index],
            })
            .collect();

        let mut filter_configs = Vec::new();
        for index in 0..Self::MAX_CONFIG_FILES {
            let offset = 0x20d18 - Self::UNCOMPRESSED_SIZE + index * Self::MAX_FILENAME;
            let name = decode_sjis(view.bytes(offset, Self::MAX_FILENAME)?, "FilterConfig.name")?;
            if name.is_empty() {
                break;
            }
            let data_len = checked_len(parse(reader, le_i32)?, "FilterConfig data size")?;
            filter_configs.push(FilterConfig {
                name,
                data: parse(reader, take(data_len))?.to_vec(),
            });
        }

        let mut clipped_images = Vec::with_capacity(Self::MAX_IMAGES);
        for index in 0..Self::MAX_IMAGES {
            let offset = 0x4bbd98 - Self::UNCOMPRESSED_SIZE + index * 4;
            let handle = view.u32(offset)?;
            if handle == ClippedImage::NO_DATA_HANDLE {
                clipped_images.push(None);
            } else {
                let data_len = checked_len(parse(reader, le_i32)?, "clipped image data size")?;
                clipped_images.push(Some(ClippedImage {
                    handle,
                    data: parse(reader, take(data_len))?.to_vec(),
                }));
            }
        }

        let selected_frame_start = view.i32(0x31c - Self::UNCOMPRESSED_SIZE)?;
        let selected_frame_end = view.i32(0x320 - Self::UNCOMPRESSED_SIZE)?;
        let resized_width = view.i32(0x328 - Self::UNCOMPRESSED_SIZE)?;
        let resized_height = view.i32(0x32c - Self::UNCOMPRESSED_SIZE)?;
        let current_frame = view.i32(0x330 - Self::UNCOMPRESSED_SIZE)?;
        let video_decode_bit = view.i16(0x3de - Self::UNCOMPRESSED_SIZE)?;
        let video_decode_format = view.u32(0x3e0 - Self::UNCOMPRESSED_SIZE)?;
        let audio_ch = view.i16(0x3fa - Self::UNCOMPRESSED_SIZE)?;
        let audio_rate = view.i32(0x3fc - Self::UNCOMPRESSED_SIZE)?;
        let video_scale = view.i32(0x468 - Self::UNCOMPRESSED_SIZE)?;
        let video_rate = view.i32(0x46c - Self::UNCOMPRESSED_SIZE)?;

        Ok(Self {
            data,
            flag,
            edit_filename,
            output_filename,
            project_filename,
            width,
            height,
            selected_frame_start,
            selected_frame_end,
            resized_width,
            resized_height,
            current_frame,
            video_decode_bit,
            video_decode_format,
            audio_ch,
            audio_rate,
            video_scale,
            video_rate,
            frames,
            filter_configs,
            clipped_images,
        })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        if self.data.len() != Self::DATA_SIZE {
            return Err(AupError::InvalidModel(
                "EditHandle.data has an invalid length",
            ));
        }
        if self.filter_configs.len() > Self::MAX_CONFIG_FILES {
            return Err(AupError::InvalidModel("too many filter configs"));
        }
        if self.clipped_images.len() != Self::MAX_IMAGES {
            return Err(AupError::InvalidModel(
                "clipped_images must contain 256 slots",
            ));
        }
        let frame_count =
            i32::try_from(self.frames.len()).map_err(|_| AupError::Overflow("frame count"))?;

        write_i32(writer, Self::SIZE as i32)?;
        write_u32(writer, self.flag)?;
        writer.write_all(&encode_sjis_fixed(
            &self.edit_filename,
            Self::MAX_FILENAME,
            "EditHandle.edit_filename",
        )?)?;
        writer.write_all(&encode_sjis_fixed(
            &self.output_filename,
            Self::MAX_FILENAME,
            "EditHandle.output_filename",
        )?)?;

        let mut data = self.data.clone();
        put_bytes(
            &mut data,
            0,
            &encode_sjis_fixed(
                &self.project_filename,
                Self::MAX_FILENAME,
                "EditHandle.project_filename",
            )?,
            "EditHandle data",
        )?;
        put_i32(
            &mut data,
            0x310 - Self::UNCOMPRESSED_SIZE,
            self.width,
            "EditHandle data",
        )?;
        put_i32(
            &mut data,
            0x314 - Self::UNCOMPRESSED_SIZE,
            self.height,
            "EditHandle data",
        )?;
        put_i32(
            &mut data,
            0x318 - Self::UNCOMPRESSED_SIZE,
            frame_count,
            "EditHandle data",
        )?;
        put_i32(
            &mut data,
            0x31c - Self::UNCOMPRESSED_SIZE,
            self.selected_frame_start,
            "EditHandle data",
        )?;
        put_i32(
            &mut data,
            0x320 - Self::UNCOMPRESSED_SIZE,
            self.selected_frame_end,
            "EditHandle data",
        )?;
        put_i32(
            &mut data,
            0x328 - Self::UNCOMPRESSED_SIZE,
            self.resized_width,
            "EditHandle data",
        )?;
        put_i32(
            &mut data,
            0x32c - Self::UNCOMPRESSED_SIZE,
            self.resized_height,
            "EditHandle data",
        )?;
        put_i32(
            &mut data,
            0x330 - Self::UNCOMPRESSED_SIZE,
            self.current_frame,
            "EditHandle data",
        )?;
        put_i16(
            &mut data,
            0x3de - Self::UNCOMPRESSED_SIZE,
            self.video_decode_bit,
            "EditHandle data",
        )?;
        put_u32(
            &mut data,
            0x3e0 - Self::UNCOMPRESSED_SIZE,
            self.video_decode_format,
            "EditHandle data",
        )?;
        put_i16(
            &mut data,
            0x3fa - Self::UNCOMPRESSED_SIZE,
            self.audio_ch,
            "EditHandle data",
        )?;
        put_i32(
            &mut data,
            0x3fc - Self::UNCOMPRESSED_SIZE,
            self.audio_rate,
            "EditHandle data",
        )?;
        put_i32(
            &mut data,
            0x468 - Self::UNCOMPRESSED_SIZE,
            self.video_scale,
            "EditHandle data",
        )?;
        put_i32(
            &mut data,
            0x46c - Self::UNCOMPRESSED_SIZE,
            self.video_rate,
            "EditHandle data",
        )?;

        for index in 0..Self::MAX_CONFIG_FILES {
            let name = match self.filter_configs.get(index) {
                Some(config) => config.name.as_str(),
                None => "",
            };
            let offset = 0x20d18 - Self::UNCOMPRESSED_SIZE + index * Self::MAX_FILENAME;
            put_bytes(
                &mut data,
                offset,
                &encode_sjis_fixed(name, Self::MAX_FILENAME, "FilterConfig.name")?,
                "EditHandle data",
            )?;
        }
        for (index, image) in self.clipped_images.iter().enumerate() {
            let handle = match image {
                Some(image) => image.handle,
                None => ClippedImage::NO_DATA_HANDLE,
            };
            let offset = 0x4bbd98 - Self::UNCOMPRESSED_SIZE + index * 4;
            put_u32(&mut data, offset, handle, "EditHandle data")?;
        }

        compress(writer, &data)?;
        write_i32(writer, frame_count)?;
        write_compressed_u32_array(writer, self.frames.iter().map(|frame| frame.video))?;
        write_compressed_u32_array(writer, self.frames.iter().map(|frame| frame.audio))?;
        write_compressed_u32_array(writer, self.frames.iter().map(|frame| frame.field2))?;
        write_compressed_u32_array(writer, self.frames.iter().map(|frame| frame.field3))?;
        compress(
            writer,
            &self
                .frames
                .iter()
                .map(|frame| frame.inter.0)
                .collect::<Vec<_>>(),
        )?;
        compress(
            writer,
            &self
                .frames
                .iter()
                .map(|frame| frame.index_24fps)
                .collect::<Vec<_>>(),
        )?;
        compress(
            writer,
            &self
                .frames
                .iter()
                .map(|frame| frame.edit_flag.bits())
                .collect::<Vec<_>>(),
        )?;
        compress(
            writer,
            &self
                .frames
                .iter()
                .map(|frame| frame.config)
                .collect::<Vec<_>>(),
        )?;
        compress(
            writer,
            &self
                .frames
                .iter()
                .map(|frame| frame.vcm)
                .collect::<Vec<_>>(),
        )?;
        compress(
            writer,
            &self
                .frames
                .iter()
                .map(|frame| frame.clip)
                .collect::<Vec<_>>(),
        )?;

        for config in &self.filter_configs {
            write_i32(
                writer,
                i32::try_from(config.data.len())
                    .map_err(|_| AupError::Overflow("filter config size"))?,
            )?;
            writer.write_all(&config.data)?;
        }
        for image in self.clipped_images.iter().flatten() {
            if image.handle == ClippedImage::NO_DATA_HANDLE {
                continue;
            }
            write_i32(
                writer,
                i32::try_from(image.data.len())
                    .map_err(|_| AupError::Overflow("clipped image size"))?,
            )?;
            writer.write_all(&image.data)?;
        }
        Ok(())
    }
}

impl Default for EditHandle {
    fn default() -> Self {
        Self {
            data: vec![0; Self::DATA_SIZE],
            flag: 0,
            edit_filename: String::new(),
            output_filename: String::new(),
            project_filename: String::new(),
            width: 0,
            height: 0,
            selected_frame_start: 0,
            selected_frame_end: 0,
            resized_width: 0,
            resized_height: 0,
            current_frame: 0,
            video_decode_bit: 0,
            video_decode_format: 0,
            audio_ch: 0,
            audio_rate: 0,
            video_scale: 0,
            video_rate: 0,
            frames: Vec::new(),
            filter_configs: Vec::new(),
            clipped_images: vec![None; Self::MAX_IMAGES],
        }
    }
}

fn read_compressed_u8_array(reader: &mut &[u8], len: usize) -> Result<Vec<u8>> {
    let mut output = vec![0; len];
    decompress_into(reader, &mut output)?;
    Ok(output)
}

fn read_compressed_u32_array(reader: &mut &[u8], len: usize) -> Result<Vec<u32>> {
    let byte_len = len
        .checked_mul(4)
        .ok_or(AupError::Overflow("compressed u32 array size"))?;
    let mut bytes = vec![0; byte_len];
    decompress_into(reader, &mut bytes)?;
    parse(&mut bytes.as_slice(), count(le_u32, len))
}

fn write_compressed_u32_array<W: Write>(
    writer: &mut W,
    values: impl Iterator<Item = u32>,
) -> Result<()> {
    let bytes = values.flat_map(u32::to_le_bytes).collect::<Vec<_>>();
    compress(writer, &bytes)
}

/// フレームのインターレース設定です。未知値も保持します。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct FrameStatusInter(pub u8);

impl FrameStatusInter {
    pub const NORMAL: Self = Self(0);
    pub const REVERSE: Self = Self(1);
    pub const ODD: Self = Self(2);
    pub const EVEN: Self = Self(3);
    pub const MIX: Self = Self(4);
    pub const AUTO: Self = Self(5);
}

bitflags! {
    /// フレームの編集フラグです。
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct EditFrameEditFlag: u8 {
        const KEYFRAME = 1;
        const MARK_FRAME = 2;
        const DEL_FRAME = 4;
        const NULL_FRAME = 8;
    }
}

/// フレーム情報です。
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FrameStatus {
    pub video: u32,
    pub audio: u32,
    pub field2: u32,
    pub field3: u32,
    pub inter: FrameStatusInter,
    pub index_24fps: u8,
    pub edit_flag: EditFrameEditFlag,
    pub config: u8,
    pub vcm: u8,
    pub clip: u8,
}

/// AviUtl のプロファイル情報です。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilterConfig {
    pub name: String,
    pub data: Vec<u8>,
}

/// クリップボードから貼り付けられた画像です。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClippedImage {
    pub handle: u32,
    pub data: Vec<u8>,
}

impl ClippedImage {
    pub const NO_DATA_HANDLE: u32 = 0;
}

/// フィルタープラグインが保存したプロジェクト情報です。
#[derive(Clone, Debug)]
pub enum FilterProject {
    Raw(RawFilterProject),
    ExEdit(Box<ExEditProject>),
}

impl FilterProject {
    pub fn name(&self) -> &str {
        match self {
            Self::Raw(raw) => &raw.name,
            Self::ExEdit(_) => ExEditProject::FILTER_NAME,
        }
    }

    pub fn dump_data(&self) -> Result<Vec<u8>> {
        match self {
            Self::Raw(raw) => Ok(raw.data.clone()),
            Self::ExEdit(exedit) => exedit.to_bytes(),
        }
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(FILTER_HEADER)?;
        let name = encode_sjis(self.name(), "FilterProject.name")?;
        let name_len = name
            .len()
            .checked_add(1)
            .ok_or(AupError::Overflow("filter name size"))?;
        write_i32(
            writer,
            i32::try_from(name_len).map_err(|_| AupError::Overflow("filter name size"))?,
        )?;
        writer.write_all(&name)?;
        writer.write_all(&[0])?;
        let data = self.dump_data()?;
        write_i32(
            writer,
            i32::try_from(data.len()).map_err(|_| AupError::Overflow("filter data size"))?,
        )?;
        compress(writer, &data)
    }
}

/// 未解釈のフィルタープロジェクト情報です。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawFilterProject {
    pub name: String,
    pub data: Vec<u8>,
}

impl RawFilterProject {
    fn read(reader: &mut &[u8]) -> Result<Self> {
        header(reader, FILTER_HEADER, "FilterProject")?;
        let name_len = checked_len(parse(reader, le_i32)?, "filter name size")?;
        let name = decode_sjis(parse(reader, take(name_len))?, "FilterProject.name")?;
        let data_len = checked_len(parse(reader, le_i32)?, "filter data size")?;
        let mut data = vec![0; data_len];
        decompress_into(reader, &mut data)?;
        Ok(Self { name, data })
    }
}

#[cfg(test)]
mod tests {
    use super::{AUP_HEADER, FILTER_HEADER};
    use crate::{
        AupError, AviUtlProject, ClippedImage, EditHandle, FilterConfig, FilterProject,
        RawFilterProject,
    };

    #[test]
    fn rejects_invalid_header() {
        let error = AviUtlProject::read(&b"not an aup"[..]).unwrap_err();
        assert!(matches!(error, AupError::Io(_)));

        let mut input = AUP_HEADER.to_vec();
        input[0] = b'X';
        let error = AviUtlProject::read(input.as_slice()).unwrap_err();
        assert!(matches!(error, AupError::InvalidHeader { .. }));
    }

    #[test]
    fn slice_parsing_preserves_sections_and_rejects_truncation() {
        let mut project = AviUtlProject::default();
        project.edit_handle.edit_filename = "入力.avi".to_owned();
        project.edit_handle.filter_configs.push(FilterConfig {
            name: "設定".to_owned(),
            data: vec![1, 2, 3],
        });
        project.edit_handle.clipped_images[0] = Some(ClippedImage {
            handle: 1,
            data: vec![4, 5, 6],
        });
        project.data_before_footer = b"AviUtl ProjectFile version 0.17\0".to_vec();
        let filter = RawFilterProject {
            name: "テスト".to_owned(),
            data: b"abcdddd".to_vec(),
        };
        project.filter_projects.push(FilterProject::Raw(filter));
        let mut bytes = Vec::new();
        project.write(&mut bytes).unwrap();
        let parsed = AviUtlProject::from_bytes(&bytes).unwrap();
        let mut output = Vec::new();
        parsed.write(&mut output).unwrap();
        assert_eq!(bytes, output);

        let mut input = &bytes[AUP_HEADER.len()..];
        let handle = EditHandle::read(&mut input).unwrap();
        assert_eq!(handle.edit_filename, "入力.avi");
        assert_eq!(handle.filter_configs, project.edit_handle.filter_configs);
        assert_eq!(handle.clipped_images, project.edit_handle.clipped_images);
        assert!(input.starts_with(&project.data_before_footer));
        let handle_end = bytes.len() - input.len();
        for end in [
            0,
            AUP_HEADER.len() - 1,
            AUP_HEADER.len(),
            handle_end - 1,
            handle_end,
            bytes.len() - 1,
        ] {
            assert!(
                AviUtlProject::from_bytes(&bytes[..end]).is_err(),
                "end={end}"
            );
        }
        bytes.push(0);
        assert!(AviUtlProject::from_bytes(&bytes).is_err());

        let mut negative_length = FILTER_HEADER.to_vec();
        negative_length.extend_from_slice(&(-1i32).to_le_bytes());
        assert!(matches!(
            RawFilterProject::read(&mut negative_length.as_slice()),
            Err(AupError::InvalidValue {
                field: "filter name size",
                value: -1
            })
        ));
    }
}
