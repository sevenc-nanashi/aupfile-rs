use std::cmp::Ordering;

use bitflags::bitflags;

use super::{Effect, EffectFlag, EffectType};
use crate::codec::{
    SliceReader, decode_sjis, encode_sjis, encode_sjis_fixed, put_bytes, put_i32, put_u16, put_u32,
};
use crate::{AupError, EditHandle, RawFilterProject, Result};

bitflags! {
    /// レイヤー情報のフラグです。
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct LayerFlag: u32 {
        const HIDE = 1;
        const LOCK = 2;
        const LINK = 0x10;
        const CLIPPING = 0x20;
    }
}

/// 拡張編集のレイヤー情報です。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Layer {
    pub scene_index: u32,
    pub layer_index: u32,
    pub flag: LayerFlag,
    pub name: String,
}

impl Layer {
    pub const SIZE: usize = 76;
    pub const MAX_NAME_LENGTH: usize = 64;

    fn read(data: &[u8]) -> Result<Self> {
        let view = SliceReader::new(data, "Layer");
        Ok(Self {
            scene_index: view.u32(0)?,
            layer_index: view.u32(4)?,
            flag: LayerFlag::from_bits_retain(view.u32(8)?),
            name: decode_sjis(view.bytes(12, Self::MAX_NAME_LENGTH)?, "Layer.name")?,
        })
    }

    fn write(&self, output: &mut [u8]) -> Result<()> {
        put_u32(output, 0, self.scene_index, "Layer")?;
        put_u32(output, 4, self.layer_index, "Layer")?;
        put_u32(output, 8, self.flag.bits(), "Layer")?;
        put_bytes(
            output,
            12,
            &encode_sjis_fixed(&self.name, Self::MAX_NAME_LENGTH, "Layer.name")?,
            "Layer",
        )
    }
}

bitflags! {
    /// シーン情報のフラグです。
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct SceneFlag: u32 {
        const BASE = 1;
        const ALPHA = 2;
    }
}

/// 拡張編集のシーン情報です。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scene {
    pub scene_index: u32,
    pub flag: SceneFlag,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub max_frame: u32,
    pub cursor: u32,
    pub zoom: u32,
    pub time_scroll: u32,
    pub editing_object: u32,
    pub selected_frame_start: u32,
    pub selected_frame_end: u32,
    pub enable_bpm_grid: bool,
    pub bpm_grid_tempo: u32,
    pub bpm_grid_offset: u32,
    pub enable_xy_grid: bool,
    pub xy_grid_width: u32,
    pub xy_grid_height: u32,
    pub enable_camera_grid: bool,
    pub camera_grid_size: u32,
    pub camera_grid_num: u32,
    pub show_outside_frame: bool,
    pub outside_frame_scale: u32,
    pub bpm_grid_beat: u32,
    pub layer_scroll: u32,
    pub unknown_0xa0_0xdc: [u8; 60],
}

impl Scene {
    pub const SIZE: usize = 220;
    pub const MAX_NAME_LENGTH: usize = 64;

    fn read(data: &[u8]) -> Result<Self> {
        let view = SliceReader::new(data, "Scene");
        Ok(Self {
            scene_index: view.u32(0)?,
            flag: SceneFlag::from_bits_retain(view.u32(4)?),
            name: decode_sjis(view.bytes(8, Self::MAX_NAME_LENGTH)?, "Scene.name")?,
            width: view.u32(0x48)?,
            height: view.u32(0x4c)?,
            max_frame: view.u32(0x50)?,
            cursor: view.u32(0x54)?,
            zoom: view.u32(0x58)?,
            time_scroll: view.u32(0x5c)?,
            editing_object: view.u32(0x60)?,
            selected_frame_start: view.u32(0x64)?,
            selected_frame_end: view.u32(0x68)?,
            enable_bpm_grid: view.i32(0x6c)? != 0,
            bpm_grid_tempo: view.u32(0x70)?,
            bpm_grid_offset: view.u32(0x74)?,
            enable_xy_grid: view.i32(0x78)? != 0,
            xy_grid_width: view.u32(0x7c)?,
            xy_grid_height: view.u32(0x80)?,
            enable_camera_grid: view.i32(0x84)? != 0,
            camera_grid_size: view.u32(0x88)?,
            camera_grid_num: view.u32(0x8c)?,
            show_outside_frame: view.i32(0x90)? != 0,
            outside_frame_scale: view.u32(0x94)?,
            bpm_grid_beat: view.u32(0x98)?,
            layer_scroll: view.u32(0x9c)?,
            unknown_0xa0_0xdc: view.bytes(0xa0, 60)?.try_into().expect("length checked"),
        })
    }

    fn write(&self, output: &mut [u8]) -> Result<()> {
        put_u32(output, 0, self.scene_index, "Scene")?;
        put_u32(output, 4, self.flag.bits(), "Scene")?;
        put_bytes(
            output,
            8,
            &encode_sjis_fixed(&self.name, Self::MAX_NAME_LENGTH, "Scene.name")?,
            "Scene",
        )?;
        let values = [
            (0x48, self.width),
            (0x4c, self.height),
            (0x50, self.max_frame),
            (0x54, self.cursor),
            (0x58, self.zoom),
            (0x5c, self.time_scroll),
            (0x60, self.editing_object),
            (0x64, self.selected_frame_start),
            (0x68, self.selected_frame_end),
            (0x6c, u32::from(self.enable_bpm_grid)),
            (0x70, self.bpm_grid_tempo),
            (0x74, self.bpm_grid_offset),
            (0x78, u32::from(self.enable_xy_grid)),
            (0x7c, self.xy_grid_width),
            (0x80, self.xy_grid_height),
            (0x84, u32::from(self.enable_camera_grid)),
            (0x88, self.camera_grid_size),
            (0x8c, self.camera_grid_num),
            (0x90, u32::from(self.show_outside_frame)),
            (0x94, self.outside_frame_scale),
            (0x98, self.bpm_grid_beat),
            (0x9c, self.layer_scroll),
        ];
        for (offset, value) in values {
            put_u32(output, offset, value, "Scene")?;
        }
        put_bytes(output, 0xa0, &self.unknown_0xa0_0xdc, "Scene")
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            scene_index: 0,
            flag: SceneFlag::empty(),
            name: String::new(),
            width: 0,
            height: 0,
            max_frame: 0,
            cursor: 0,
            zoom: 0,
            time_scroll: 0,
            editing_object: 0,
            selected_frame_start: 0,
            selected_frame_end: 0,
            enable_bpm_grid: false,
            bpm_grid_tempo: 0,
            bpm_grid_offset: 0,
            enable_xy_grid: false,
            xy_grid_width: 0,
            xy_grid_height: 0,
            enable_camera_grid: false,
            camera_grid_size: 0,
            camera_grid_num: 0,
            show_outside_frame: false,
            outside_frame_scale: 0,
            bpm_grid_beat: 0,
            layer_scroll: 0,
            unknown_0xa0_0xdc: [0; 60],
        }
    }
}

/// トラックバーの変化方法です。未知値も保持します。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TrackbarType(pub u8);

impl TrackbarType {
    pub const STOP: Self = Self(0);
    pub const LINEAR: Self = Self(1);
    pub const CURVE: Self = Self(2);
    pub const STEP: Self = Self(3);
    pub const IGNORE_KEYFRAME: Self = Self(4);
    pub const MOVEMENT: Self = Self(5);
    pub const RANDOM: Self = Self(6);
    pub const ACCEL_DECEL: Self = Self(7);
    pub const REPEAT: Self = Self(8);
    pub const SCRIPT: Self = Self(0xf);
}

bitflags! {
    /// トラックバー変化方法のフラグです。
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct TrackbarFlag: u8 {
        const DECELERATION = 0x20;
        const ACCELERATION = 0x40;
    }
}

/// 拡張編集のトラックバー値です。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Trackbar {
    pub current: i32,
    pub next: i32,
    pub transition_type: TrackbarType,
    pub flag: TrackbarFlag,
    pub script_index: i32,
    pub parameter: i32,
}

impl Trackbar {
    pub fn from_transition(current: i32, next: i32, transition: u32, parameter: i32) -> Self {
        Self {
            current,
            next,
            transition_type: TrackbarType((transition & 0xf) as u8),
            flag: TrackbarFlag::from_bits_retain((transition & 0xf0) as u8),
            script_index: (transition >> 16) as i32,
            parameter,
        }
    }

    pub fn transition(&self) -> Result<u32> {
        let script_index =
            u32::try_from(self.script_index).map_err(|_| AupError::InvalidValue {
                field: "trackbar script index",
                value: i128::from(self.script_index),
            })?;
        Ok(u32::from(self.transition_type.0) | u32::from(self.flag.bits()) | (script_index << 16))
    }

    pub(crate) fn to_exo(&self, scale: i32, scripts: &[TrackbarScript]) -> Result<String> {
        let (current, next) = match scale {
            100 => (
                format!("{:.2}", f64::from(self.current) / 100.0),
                format!(",{:.2}", f64::from(self.next) / 100.0),
            ),
            10 => (
                format!("{:.1}", f64::from(self.current) / 10.0),
                format!(",{:.1}", f64::from(self.next) / 10.0),
            ),
            _ => (self.current.to_string(), format!(",{}", self.next)),
        };
        if self.transition_type == TrackbarType::STOP {
            return Ok(current);
        }
        let script = if self.transition_type == TrackbarType::SCRIPT {
            let index = usize::try_from(self.script_index).map_err(|_| AupError::InvalidValue {
                field: "trackbar script index",
                value: i128::from(self.script_index),
            })?;
            scripts.get(index).ok_or(AupError::InvalidIndex {
                field: "trackbar script",
                index,
                len: scripts.len(),
            })?
        } else {
            TrackbarScript::built_in(self.transition_type).ok_or(AupError::InvalidValue {
                field: "trackbar transition type",
                value: i128::from(self.transition_type.0),
            })?
        };
        let transition = if script.enable_speed {
            self.transition_type.0 | self.flag.bits()
        } else {
            self.transition_type.0
        };
        let mut output = format!("{current}{next},{transition}");
        if self.transition_type == TrackbarType::SCRIPT {
            output.push('@');
            output.push_str(&script.name);
        }
        if script.enable_param && self.parameter != 0 {
            output.push(',');
            output.push_str(&self.parameter.to_string());
        }
        Ok(output)
    }
}

/// トラックバー変化方法スクリプトです。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackbarScript {
    pub name: String,
    pub enable_two_point: bool,
    pub enable_param: bool,
    pub enable_speed: bool,
}

impl TrackbarScript {
    pub const SIZE: usize = 128;

    fn read(data: &[u8]) -> Result<Self> {
        Ok(Self {
            name: decode_sjis(data, "TrackbarScript.name")?,
            enable_two_point: false,
            enable_param: true,
            enable_speed: true,
        })
    }

    fn write(&self, output: &mut [u8]) -> Result<()> {
        output.copy_from_slice(&encode_sjis_fixed(
            &self.name,
            Self::SIZE,
            "TrackbarScript.name",
        )?);
        Ok(())
    }

    pub fn parse_script(&mut self, script: &str) {
        self.enable_two_point = false;
        self.enable_param = false;
        self.enable_speed = false;
        for line in script.lines() {
            if line.starts_with("--twopoint") {
                self.enable_two_point = true;
            } else if line.starts_with("--param:") {
                self.enable_param = true;
            } else if line.starts_with("--speed:") {
                self.enable_speed = true;
            }
        }
    }

    pub fn defaults() -> Vec<Self> {
        vec![
            Self {
                name: "補間移動".to_owned(),
                enable_two_point: false,
                enable_param: false,
                enable_speed: true,
            },
            Self {
                name: "回転".to_owned(),
                enable_two_point: true,
                enable_param: true,
                enable_speed: false,
            },
        ]
    }

    fn built_in(transition_type: TrackbarType) -> Option<&'static Self> {
        static BUILT_IN: std::sync::LazyLock<Vec<TrackbarScript>> =
            std::sync::LazyLock::new(|| {
                vec![
                    TrackbarScript::built_in_value("移動無し", false, false, false),
                    TrackbarScript::built_in_value("直線移動", false, false, true),
                    TrackbarScript::built_in_value("曲線移動", false, false, true),
                    TrackbarScript::built_in_value("瞬間移動", false, false, false),
                    TrackbarScript::built_in_value("中間点無視", true, false, true),
                    TrackbarScript::built_in_value("移動量指定", false, false, false),
                    TrackbarScript::built_in_value("ランダム移動", false, true, false),
                    TrackbarScript::built_in_value("加減速移動", false, false, true),
                    TrackbarScript::built_in_value("反復移動", false, true, true),
                ]
            });
        BUILT_IN.get(usize::from(transition_type.0))
    }

    fn built_in_value(
        name: &str,
        enable_two_point: bool,
        enable_param: bool,
        enable_speed: bool,
    ) -> Self {
        Self {
            name: name.to_owned(),
            enable_two_point,
            enable_param,
            enable_speed,
        }
    }
}

bitflags! {
    /// タイムラインオブジェクトのフラグです。
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct TimelineObjectFlag: u32 {
        const ENABLE = 1;
        const CLIPPING = 0x0000_0100;
        const CAMERA = 0x0000_0200;
        const MEDIA = 0x0001_0000;
        const AUDIO = 0x0002_0000;
        const MEDIA_FILTER = 0x0004_0000;
        const CONTROL = 0x0008_0000;
        const RANGE = 0x0010_0000;
    }
}

/// 拡張編集のタイムラインオブジェクトです。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimelineObject {
    pub flag: TimelineObjectFlag,
    pub start_frame: u32,
    pub end_frame: u32,
    pub preview: String,
    pub chain_group: u32,
    pub chain: bool,
    pub unknown_0x4b8: u32,
    pub group: u32,
    pub layer_index: u32,
    pub scene_index: u32,
    pub effects: Vec<Effect>,
    /// AupDotNet が解釈していない固定領域を含む、元のオブジェクトデータです。
    ///
    /// 既知のフィールドは書き込み時に現在の値で上書きされます。
    pub raw_base: Vec<u8>,
}

impl TimelineObject {
    pub const BASE_SIZE: usize = 0x5c8;
    pub const MAX_EFFECTS: usize = 12;
    pub const MAX_PREVIEW_LENGTH: usize = 64;
    pub const NO_CHAIN_GROUP: u32 = u32::MAX;
    pub const NO_GROUP: u32 = 0;

    pub fn ext_size(&self) -> Result<usize> {
        if self.chain {
            return Ok(0);
        }
        self.effects.iter().try_fold(0usize, |size, effect| {
            if effect.ext_data.len() != effect.effect_type.ext_size as usize {
                return Err(AupError::InvalidModel(
                    "effect extension size does not match definition",
                ));
            }
            size.checked_add(effect.effect_type.ext_size as usize)
                .ok_or(AupError::Overflow("timeline object extension size"))
        })
    }

    pub fn size(&self) -> Result<usize> {
        Self::BASE_SIZE
            .checked_add(self.ext_size()?)
            .ok_or(AupError::Overflow("timeline object size"))
    }

    fn read_preview(bytes: &[u8]) -> Result<String> {
        let end = bytes
            .iter()
            .position(|byte| *byte == 0)
            .unwrap_or(bytes.len());
        let mut output = String::with_capacity(end * 3);
        // ExEdit cuts this display-only label at 63 bytes, even inside a CP932 character.
        let (result, _) = encoding_rs::SHIFT_JIS
            .new_decoder_without_bom_handling()
            .decode_to_string_without_replacement(
                &bytes[..end],
                &mut output,
                end < Self::MAX_PREVIEW_LENGTH - 1,
            );
        if result != encoding_rs::DecoderResult::InputEmpty {
            return Err(AupError::InvalidString {
                field: "TimelineObject.preview",
                encoding: "CP932",
            });
        }
        Ok(output)
    }

    // exedit.auf 0.92: 0x1007fac0 expands older fixed records into the 0x5c8 layout.
    fn read_legacy(
        data: &[u8],
        extension: &[u8],
        last_chain_group: u32,
        effect_types: &[EffectType],
        counts: [usize; 3],
        version: u32,
    ) -> Result<Self> {
        let [tracks, checks, filters] = counts;
        let view = SliceReader::new(data, "legacy TimelineObject");
        let mut base = vec![0; Self::BASE_SIZE];
        for index in 0..Self::MAX_EFFECTS {
            put_u32(&mut base, 0x54 + index * 12, u32::MAX, "TimelineObject")?;
        }
        let mut cursor = 0;
        let transition_width = if version < 9100 { 1 } else { 4 };
        let fields = [
            (0, 0x54),
            (0x54, filters * 12),
            (0xe4, filters),
            (0xf0, 4),
            (0xf4, 4),
            (0xf8, tracks * 4),
            (0x1f8, tracks * 4),
            (0x2f8, tracks * transition_width),
            (0x3f8, checks * 4),
            (0x4b8, 8),
            (0x4c0, tracks * 4),
            (0x5c0, 8),
        ];
        for (offset, len) in fields {
            // Early versions omit the metadata, parameters and scene at the end.
            if offset >= 0x4b8 && cursor + len > data.len() {
                break;
            }
            let bytes = view.bytes(cursor, len)?;
            if offset == 0x2f8 && transition_width == 1 {
                for (index, value) in bytes.iter().enumerate() {
                    put_u32(
                        &mut base,
                        offset + index * 4,
                        u32::from(*value),
                        "TimelineObject",
                    )?;
                }
            } else {
                put_bytes(&mut base, offset, bytes, "TimelineObject")?;
            }
            cursor += len;
        }
        if version < 9000 {
            let layer = view.u32(4)?;
            put_u32(&mut base, 0x5c0, layer, "TimelineObject")?;
            put_u32(&mut base, 0x5c4, 0, "TimelineObject")?;
        }
        base.extend_from_slice(extension);
        let object = Self::read(&base, last_chain_group, effect_types)?;
        if object
            .effects
            .iter()
            .map(|effect| effect.trackbars.len())
            .sum::<usize>()
            > tracks
            || object
                .effects
                .iter()
                .map(|effect| effect.checkboxes.len())
                .sum::<usize>()
                > checks
        {
            return Err(AupError::InvalidModel(
                "legacy effect values exceed object capacity",
            ));
        }
        Ok(object)
    }

    fn read(data: &[u8], last_chain_group: u32, effect_types: &[EffectType]) -> Result<Self> {
        let view = SliceReader::new(data, "TimelineObject");
        view.bytes(0, Self::BASE_SIZE)?;
        let chain_group = view.u32(0x50)?;
        let declared_ext_size = view.u32(0xf4)? as usize;
        let chain = chain_group != Self::NO_CHAIN_GROUP
            && chain_group == last_chain_group
            && declared_ext_size == 0;
        let mut effects = Vec::new();
        let mut trackbar_count = 0usize;
        let mut checkbox_count = 0usize;
        for index in 0..Self::MAX_EFFECTS {
            let type_index = view.i32(0x54 + index * 12)?;
            if type_index == -1 {
                break;
            }
            let type_index = usize::try_from(type_index).map_err(|_| AupError::InvalidValue {
                field: "effect type index",
                value: i128::from(type_index),
            })?;
            let effect_type = effect_types
                .get(type_index)
                .ok_or(AupError::InvalidIndex {
                    field: "effect type",
                    index: type_index,
                    len: effect_types.len(),
                })?
                .clone();
            let raw_ext_offset = view.i32(0x54 + index * 12 + 8)?;
            let ext_offset =
                usize::try_from(raw_ext_offset).map_err(|_| AupError::InvalidValue {
                    field: "effect extension offset",
                    value: i128::from(raw_ext_offset),
                })?;
            let ext_data = if chain {
                Vec::new()
            } else {
                view.bytes(
                    Self::BASE_SIZE
                        .checked_add(ext_offset)
                        .ok_or(AupError::Overflow("effect extension offset"))?,
                    effect_type.ext_size as usize,
                )?
                .to_vec()
            };
            let flag = EffectFlag::from_bits_retain(view.u8(0xe4 + index)?);

            let mut trackbars = Vec::with_capacity(effect_type.trackbar_count as usize);
            for item in 0..effect_type.trackbar_count as usize {
                let value_index = trackbar_count + item;
                if value_index >= 64 {
                    return Err(AupError::InvalidValue {
                        field: "timeline trackbar count",
                        value: value_index as i128,
                    });
                }
                trackbars.push(Trackbar::from_transition(
                    view.i32(0xf8 + value_index * 4)?,
                    view.i32(0x1f8 + value_index * 4)?,
                    view.u32(0x2f8 + value_index * 4)?,
                    view.i32(0x4c0 + value_index * 4)?,
                ));
            }
            let mut checkboxes = Vec::with_capacity(effect_type.checkbox_count as usize);
            for item in 0..effect_type.checkbox_count as usize {
                let value_index = checkbox_count + item;
                if value_index >= 48 {
                    return Err(AupError::InvalidValue {
                        field: "timeline checkbox count",
                        value: value_index as i128,
                    });
                }
                checkboxes.push(view.i32(0x3f8 + value_index * 4)?);
            }
            trackbar_count += trackbars.len();
            checkbox_count += checkboxes.len();
            effects.push(Effect::try_new(
                effect_type,
                flag,
                trackbars,
                checkboxes,
                ext_data,
            )?);
        }
        let computed_ext_size = effects.iter().try_fold(0usize, |size, effect| {
            size.checked_add(effect.effect_type.ext_size as usize)
                .ok_or(AupError::Overflow("timeline extension size"))
        })?;
        if !chain && declared_ext_size != computed_ext_size {
            return Err(AupError::InvalidValue {
                field: "timeline extension size",
                value: declared_ext_size as i128,
            });
        }

        Ok(Self {
            flag: TimelineObjectFlag::from_bits_retain(view.u32(0)?),
            start_frame: view.u32(8)?,
            end_frame: view.u32(12)?,
            preview: Self::read_preview(view.bytes(0x10, Self::MAX_PREVIEW_LENGTH)?)?,
            chain_group,
            chain,
            unknown_0x4b8: view.u32(0x4b8)?,
            group: view.u32(0x4bc)?,
            layer_index: view.u32(0x5c0)?,
            scene_index: view.u32(0x5c4)?,
            effects,
            raw_base: view.bytes(0, Self::BASE_SIZE)?.to_vec(),
        })
    }

    fn write(&self, output: &mut [u8], editing_scene: u32) -> Result<()> {
        let size = self.size()?;
        if output.len() != size {
            return Err(AupError::InvalidModel(
                "timeline output size does not match",
            ));
        }
        if self.effects.len() > Self::MAX_EFFECTS {
            return Err(AupError::InvalidModel(
                "timeline object has too many effects",
            ));
        }
        for effect in &self.effects {
            if effect.trackbars.len() != effect.effect_type.trackbar_count as usize
                || effect.checkboxes.len() != effect.effect_type.checkbox_count as usize
            {
                return Err(AupError::InvalidModel(
                    "effect value count does not match definition",
                ));
            }
        }
        if self.raw_base.len() != Self::BASE_SIZE {
            return Err(AupError::InvalidModel(
                "timeline object raw base size does not match",
            ));
        }
        output[..Self::BASE_SIZE].copy_from_slice(&self.raw_base);
        put_u32(output, 0, self.flag.bits(), "TimelineObject")?;
        put_u32(
            output,
            4,
            if self.scene_index == editing_scene {
                self.layer_index
            } else {
                u32::MAX
            },
            "TimelineObject",
        )?;
        put_u32(output, 8, self.start_frame, "TimelineObject")?;
        put_u32(output, 12, self.end_frame, "TimelineObject")?;
        if self.preview != Self::read_preview(&self.raw_base[0x10..0x50])? {
            let preview = encode_sjis(&self.preview, "TimelineObject.preview")?;
            if preview.len() >= Self::MAX_PREVIEW_LENGTH {
                return Err(AupError::StringTooLong {
                    field: "TimelineObject.preview",
                    max: Self::MAX_PREVIEW_LENGTH - 1,
                    actual: preview.len(),
                });
            }
            put_bytes(output, 0x10, &preview, "TimelineObject")?;
            output[0x10 + preview.len()] = 0;
        }
        put_u32(output, 0x50, self.chain_group, "TimelineObject")?;
        put_u32(
            output,
            0xf4,
            u32::try_from(self.ext_size()?)
                .map_err(|_| AupError::Overflow("timeline extension size"))?,
            "TimelineObject",
        )?;
        put_u32(output, 0x4b8, self.unknown_0x4b8, "TimelineObject")?;
        put_u32(output, 0x4bc, self.group, "TimelineObject")?;
        put_u32(output, 0x5c0, self.layer_index, "TimelineObject")?;
        put_u32(output, 0x5c4, self.scene_index, "TimelineObject")?;

        let mut ext_cursor = 0usize;
        let mut trackbar_count = 0usize;
        let mut checkbox_count = 0usize;
        for (index, effect) in self.effects.iter().enumerate() {
            if effect.effect_type.id < 0 {
                return Err(AupError::InvalidValue {
                    field: "effect type ID",
                    value: i128::from(effect.effect_type.id),
                });
            }
            put_i32(
                output,
                0x54 + index * 12,
                effect.effect_type.id,
                "TimelineObject",
            )?;
            put_u16(
                output,
                0x54 + index * 12 + 4,
                u16::try_from(trackbar_count)
                    .map_err(|_| AupError::Overflow("timeline trackbar count"))?,
                "TimelineObject",
            )?;
            put_u16(
                output,
                0x54 + index * 12 + 6,
                u16::try_from(checkbox_count)
                    .map_err(|_| AupError::Overflow("timeline checkbox count"))?,
                "TimelineObject",
            )?;
            put_i32(
                output,
                0x54 + index * 12 + 8,
                i32::try_from(ext_cursor)
                    .map_err(|_| AupError::Overflow("effect extension offset"))?,
                "TimelineObject",
            )?;
            output[0xe4 + index] = effect.flag.bits();
            for trackbar in &effect.trackbars {
                if trackbar_count >= 64 {
                    return Err(AupError::InvalidModel(
                        "timeline object has too many trackbars",
                    ));
                }
                put_i32(
                    output,
                    0xf8 + trackbar_count * 4,
                    trackbar.current,
                    "TimelineObject",
                )?;
                put_i32(
                    output,
                    0x1f8 + trackbar_count * 4,
                    trackbar.next,
                    "TimelineObject",
                )?;
                put_u32(
                    output,
                    0x2f8 + trackbar_count * 4,
                    trackbar.transition()?,
                    "TimelineObject",
                )?;
                put_i32(
                    output,
                    0x4c0 + trackbar_count * 4,
                    trackbar.parameter,
                    "TimelineObject",
                )?;
                trackbar_count += 1;
            }
            for checkbox in &effect.checkboxes {
                if checkbox_count >= 48 {
                    return Err(AupError::InvalidModel(
                        "timeline object has too many checkboxes",
                    ));
                }
                put_i32(
                    output,
                    0x3f8 + checkbox_count * 4,
                    *checkbox,
                    "TimelineObject",
                )?;
                checkbox_count += 1;
            }
            if !self.chain {
                put_bytes(
                    output,
                    Self::BASE_SIZE + ext_cursor,
                    &effect.ext_data,
                    "TimelineObject extension data",
                )?;
                ext_cursor += effect.ext_data.len();
            }
        }
        put_u16(
            output,
            0xf0,
            u16::try_from(trackbar_count)
                .map_err(|_| AupError::Overflow("timeline trackbar count"))?,
            "TimelineObject",
        )?;
        put_u16(
            output,
            0xf2,
            u16::try_from(checkbox_count)
                .map_err(|_| AupError::Overflow("timeline checkbox count"))?,
            "TimelineObject",
        )?;
        for index in self.effects.len()..Self::MAX_EFFECTS {
            put_u32(output, 0x54 + index * 12, u32::MAX, "TimelineObject")?;
        }
        Ok(())
    }
}

impl Default for TimelineObject {
    fn default() -> Self {
        Self {
            flag: TimelineObjectFlag::default(),
            start_frame: 0,
            end_frame: 0,
            preview: String::new(),
            chain_group: Self::NO_CHAIN_GROUP,
            chain: false,
            unknown_0x4b8: 0,
            group: Self::NO_GROUP,
            layer_index: 0,
            scene_index: 0,
            effects: Vec::new(),
            raw_base: vec![0; Self::BASE_SIZE],
        }
    }
}

/// YCbCr 色です。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct YCbCr {
    pub y: i16,
    pub cb: i16,
    pub cr: i16,
}

impl YCbCr {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let view = SliceReader::new(data, "YCbCr");
        Ok(Self {
            y: view.i16(0)?,
            cb: view.i16(2)?,
            cr: view.i16(4)?,
        })
    }

    pub fn to_bytes(self) -> [u8; 6] {
        let mut output = [0; 6];
        output[0..2].copy_from_slice(&self.y.to_le_bytes());
        output[2..4].copy_from_slice(&self.cb.to_le_bytes());
        output[4..6].copy_from_slice(&self.cr.to_le_bytes());
        output
    }
}

/// 拡張編集フィルターのプロジェクトデータです。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExEditProject {
    pub unknown_0x0c: u32,
    pub zoom: u32,
    pub unknown_0x14: u32,
    pub editing_object: u32,
    pub unknown_0x1c: u32,
    pub unknown_0x20: u32,
    pub unknown_0x24: u32,
    pub unknown_0x28: u32,
    pub version: u32,
    pub enable_bpm_grid: bool,
    pub bpm_grid_tempo: u32,
    pub bpm_grid_offset: u32,
    pub enable_xy_grid: bool,
    pub xy_grid_width: u32,
    pub xy_grid_height: u32,
    pub enable_camera_grid: bool,
    pub camera_grid_width: u32,
    pub camera_grid_height: u32,
    pub show_outside_frame: bool,
    pub outside_frame_scale: u32,
    pub bpm_grid_beat: u32,
    pub unknown_0x60: u32,
    pub editing_scene: u32,
    pub unknown_0x78: u32,
    pub unknown_0x80_0xff: [u8; 128],
    pub layers: Vec<Layer>,
    pub scenes: Vec<Scene>,
    pub trackbar_scripts: Vec<TrackbarScript>,
    pub effect_types: Vec<EffectType>,
    pub objects: Vec<TimelineObject>,
}

impl ExEditProject {
    pub const FILTER_NAME: &'static str = "拡張編集";
    pub const NO_EDITING_OBJECT: u32 = u32::MAX;
    pub const BPM_GRID_TEMPO_SCALE: i32 = 10_000;

    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let view = SliceReader::new(data, "ExEditProject");
        if view.bytes(0, 4)? != b"80EE" {
            return Err(AupError::InvalidHeader {
                kind: "ExEdit project",
            });
        }
        view.bytes(0, 0x100)?;
        let effect_type_count = view.u32(4)? as usize;
        let object_count = view.u32(8)? as usize;
        let legacy_layer_count = view.u32(0x0c)? as usize;
        let version = view.u32(0x2c)?;
        // Zero layout fields mean 0x248 bytes / 32 tracks / 16 checks / 8 filters
        // in exedit.auf 0.92's project loader (0x100319b4..0x100319ec).
        let mut layout = [0usize; 4];
        for (index, default) in [0x248, 32, 16, 8].into_iter().enumerate() {
            let value = view.u32(0x1c + index * 4)? as usize;
            layout[index] = if value == 0 { default } else { value };
        }
        let [object_size, tracks, checks, filters] = layout;
        for (value, max, field) in [
            (
                object_size,
                TimelineObject::BASE_SIZE,
                "timeline object size",
            ),
            (tracks, 64, "timeline trackbar capacity"),
            (checks, 48, "timeline checkbox capacity"),
            (
                filters,
                TimelineObject::MAX_EFFECTS,
                "timeline effect capacity",
            ),
        ] {
            if value > max {
                return Err(AupError::InvalidValue {
                    field,
                    value: value as i128,
                });
            }
        }
        let scene_count = view.u32(0x68)? as usize;
        let layer_count = view.u32(0x6c)? as usize;
        let trackbar_script_count = view.u32(0x7c)? as usize;
        let counts = [
            (effect_type_count, EffectType::SIZE, "effect type count"),
            (object_count, object_size, "timeline object count"),
            (scene_count, Scene::SIZE, "scene count"),
            (layer_count, Layer::SIZE, "layer count"),
            (legacy_layer_count, 68, "legacy layer count"),
            (
                trackbar_script_count,
                TrackbarScript::SIZE,
                "trackbar script count",
            ),
        ];
        for (count, minimum_size, field) in counts {
            if count > data.len() / minimum_size {
                return Err(AupError::InvalidValue {
                    field,
                    value: count as i128,
                });
            }
        }
        let mut cursor = 0x100usize;

        let mut layers = Vec::with_capacity(layer_count);
        for index in 0..legacy_layer_count {
            let layer = SliceReader::new(view.bytes(cursor, 68)?, "legacy Layer");
            layers.push(Layer {
                scene_index: 0,
                layer_index: index as u32,
                flag: LayerFlag::from_bits_retain(layer.u32(0)?),
                name: decode_sjis(layer.bytes(4, Layer::MAX_NAME_LENGTH)?, "Layer.name")?,
            });
            cursor += 68;
        }
        for _ in 0..layer_count {
            layers.push(Layer::read(view.bytes(cursor, Layer::SIZE)?)?);
            cursor = cursor
                .checked_add(Layer::SIZE)
                .ok_or(AupError::Overflow("layer position"))?;
        }
        let mut scenes = Vec::with_capacity(scene_count);
        for _ in 0..scene_count {
            scenes.push(Scene::read(view.bytes(cursor, Scene::SIZE)?)?);
            cursor = cursor
                .checked_add(Scene::SIZE)
                .ok_or(AupError::Overflow("scene position"))?;
        }
        let mut trackbar_scripts = Vec::with_capacity(trackbar_script_count);
        for _ in 0..trackbar_script_count {
            trackbar_scripts.push(TrackbarScript::read(
                view.bytes(cursor, TrackbarScript::SIZE)?,
            )?);
            cursor = cursor
                .checked_add(TrackbarScript::SIZE)
                .ok_or(AupError::Overflow("trackbar script position"))?;
        }
        let mut effect_types = Vec::with_capacity(effect_type_count);
        for id in 0..effect_type_count {
            let mut effect_type = EffectType::read(
                view.bytes(cursor, EffectType::SIZE)?,
                i32::try_from(id).map_err(|_| AupError::Overflow("effect type ID"))?,
            )?;
            if let Some(default) = EffectType::defaults().get(id)
                && default.flag == effect_type.flag
                && default.trackbar_count == effect_type.trackbar_count
                && default.checkbox_count == effect_type.checkbox_count
                && default.ext_size == effect_type.ext_size
                && default.name == effect_type.name
            {
                effect_type.trackbars.clone_from(&default.trackbars);
                effect_type.checkboxes.clone_from(&default.checkboxes);
            }
            effect_types.push(effect_type);
            cursor = cursor
                .checked_add(EffectType::SIZE)
                .ok_or(AupError::Overflow("effect type position"))?;
        }
        let mut objects = Vec::with_capacity(object_count);
        let mut last_chain_group = TimelineObject::NO_CHAIN_GROUP;
        for _ in 0..object_count {
            let object = if object_size == TimelineObject::BASE_SIZE {
                TimelineObject::read(
                    view.bytes(cursor, data.len().saturating_sub(cursor))?,
                    last_chain_group,
                    &effect_types,
                )?
            } else {
                let base = view.bytes(cursor, object_size)?;
                let base_view = SliceReader::new(base, "legacy TimelineObject");
                let ext_size = base_view.u32(0x54 + filters * 13 + 4)? as usize;
                TimelineObject::read_legacy(
                    base,
                    view.bytes(cursor + object_size, ext_size)?,
                    last_chain_group,
                    &effect_types,
                    [tracks, checks, filters],
                    version,
                )?
            };
            cursor = cursor
                .checked_add(object_size + object.ext_size()?)
                .ok_or(AupError::Overflow("timeline object position"))?;
            last_chain_group = object.chain_group;
            objects.push(object);
        }
        if cursor != data.len() {
            return Err(AupError::InvalidValue {
                field: "ExEdit trailing data size",
                value: (data.len() - cursor) as i128,
            });
        }

        Ok(Self {
            unknown_0x0c: 0,
            zoom: view.u32(0x10)?,
            unknown_0x14: view.u32(0x14)?,
            editing_object: view.u32(0x18)?,
            unknown_0x1c: TimelineObject::BASE_SIZE as u32,
            unknown_0x20: 64,
            unknown_0x24: 48,
            unknown_0x28: TimelineObject::MAX_EFFECTS as u32,
            version,
            enable_bpm_grid: view.i32(0x30)? != 0,
            bpm_grid_tempo: view.u32(0x34)?,
            bpm_grid_offset: view.u32(0x38)?,
            enable_xy_grid: view.i32(0x3c)? != 0,
            xy_grid_width: view.u32(0x40)?,
            xy_grid_height: view.u32(0x44)?,
            enable_camera_grid: view.i32(0x48)? != 0,
            camera_grid_width: view.u32(0x4c)?,
            camera_grid_height: view.u32(0x50)?,
            show_outside_frame: view.i32(0x54)? != 0,
            outside_frame_scale: view.u32(0x58)?,
            bpm_grid_beat: view.u32(0x5c)?,
            unknown_0x60: view.u32(0x60)?,
            editing_scene: view.u32(0x64)?,
            unknown_0x78: view.u32(0x78)?,
            unknown_0x80_0xff: view.bytes(0x80, 128)?.try_into().expect("length checked"),
            layers,
            scenes,
            trackbar_scripts,
            effect_types,
            objects,
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let scene_count =
            u32::try_from(self.scenes.len()).map_err(|_| AupError::Overflow("scene count"))?;
        let layer_count =
            u32::try_from(self.layers.len()).map_err(|_| AupError::Overflow("layer count"))?;
        let trackbar_script_count = u32::try_from(self.trackbar_scripts.len())
            .map_err(|_| AupError::Overflow("trackbar script count"))?;
        let layer_size = self
            .layers
            .len()
            .checked_mul(Layer::SIZE)
            .ok_or(AupError::Overflow("layer data size"))?;
        let scene_size = self
            .scenes
            .len()
            .checked_mul(Scene::SIZE)
            .ok_or(AupError::Overflow("scene data size"))?;
        let script_size = self
            .trackbar_scripts
            .len()
            .checked_mul(TrackbarScript::SIZE)
            .ok_or(AupError::Overflow("trackbar script data size"))?;
        let effect_type_size = self
            .effect_types
            .len()
            .checked_mul(EffectType::SIZE)
            .ok_or(AupError::Overflow("effect type data size"))?;
        let size = self.objects.iter().try_fold(
            0x100usize
                .checked_add(layer_size)
                .and_then(|size| size.checked_add(scene_size))
                .and_then(|size| size.checked_add(script_size))
                .and_then(|size| size.checked_add(effect_type_size))
                .ok_or(AupError::Overflow("ExEdit project size"))?,
            |size, object| {
                size.checked_add(object.size()?)
                    .ok_or(AupError::Overflow("ExEdit project size"))
            },
        )?;
        let mut output = vec![0; size];
        output[..4].copy_from_slice(b"80EE");
        put_u32(
            &mut output,
            4,
            u32::try_from(self.effect_types.len())
                .map_err(|_| AupError::Overflow("effect type count"))?,
            "ExEditProject",
        )?;
        put_u32(
            &mut output,
            8,
            u32::try_from(self.objects.len())
                .map_err(|_| AupError::Overflow("timeline object count"))?,
            "ExEditProject",
        )?;
        let values = [
            (0x0c, self.unknown_0x0c),
            (0x10, self.zoom),
            (0x14, self.unknown_0x14),
            (0x18, self.editing_object),
            (0x1c, TimelineObject::BASE_SIZE as u32),
            (0x20, 64),
            (0x24, 48),
            (0x28, TimelineObject::MAX_EFFECTS as u32),
            (0x2c, self.version),
            (0x30, u32::from(self.enable_bpm_grid)),
            (0x34, self.bpm_grid_tempo),
            (0x38, self.bpm_grid_offset),
            (0x3c, u32::from(self.enable_xy_grid)),
            (0x40, self.xy_grid_width),
            (0x44, self.xy_grid_height),
            (0x48, u32::from(self.enable_camera_grid)),
            (0x4c, self.camera_grid_width),
            (0x50, self.camera_grid_height),
            (0x54, u32::from(self.show_outside_frame)),
            (0x58, self.outside_frame_scale),
            (0x5c, self.bpm_grid_beat),
            (0x60, self.unknown_0x60),
            (0x64, self.editing_scene),
            (0x68, scene_count),
            (0x6c, layer_count),
            (0x70, Scene::SIZE as u32),
            (0x74, Layer::SIZE as u32),
            (0x78, self.unknown_0x78),
            (0x7c, trackbar_script_count),
        ];
        for (offset, value) in values {
            put_u32(&mut output, offset, value, "ExEditProject")?;
        }
        put_bytes(&mut output, 0x80, &self.unknown_0x80_0xff, "ExEditProject")?;

        let mut cursor = 0x100usize;
        for layer in &self.layers {
            layer.write(&mut output[cursor..cursor + Layer::SIZE])?;
            cursor += Layer::SIZE;
        }
        for scene in &self.scenes {
            scene.write(&mut output[cursor..cursor + Scene::SIZE])?;
            cursor += Scene::SIZE;
        }
        for script in &self.trackbar_scripts {
            script.write(&mut output[cursor..cursor + TrackbarScript::SIZE])?;
            cursor += TrackbarScript::SIZE;
        }
        for effect_type in &self.effect_types {
            effect_type.write(&mut output[cursor..cursor + EffectType::SIZE])?;
            cursor += EffectType::SIZE;
        }
        for object in &self.objects {
            let object_size = object.size()?;
            object.write(
                &mut output[cursor..cursor + object_size],
                self.editing_scene,
            )?;
            cursor += object_size;
        }
        Ok(output)
    }

    pub fn sort_objects(&mut self) {
        let editing_scene = self.editing_scene;
        self.objects.sort_by(|left, right| {
            if left.scene_index != right.scene_index {
                if left.scene_index == editing_scene {
                    return Ordering::Less;
                }
                if right.scene_index == editing_scene {
                    return Ordering::Greater;
                }
                return left.scene_index.cmp(&right.scene_index);
            }
            left.layer_index
                .cmp(&right.layer_index)
                .then_with(|| left.start_frame.cmp(&right.start_frame))
        });
    }

    pub fn export_object(
        &self,
        scene_index: u32,
        edit_handle: &EditHandle,
    ) -> Result<super::ExeditObjectFile> {
        super::ExeditObjectFile::from_project(self, scene_index, edit_handle)
    }
}

impl Default for ExEditProject {
    fn default() -> Self {
        Self {
            unknown_0x0c: 0,
            zoom: 0,
            unknown_0x14: 0,
            editing_object: Self::NO_EDITING_OBJECT,
            unknown_0x1c: 0,
            unknown_0x20: 0,
            unknown_0x24: 0,
            unknown_0x28: 0,
            version: 0,
            enable_bpm_grid: false,
            bpm_grid_tempo: 0,
            bpm_grid_offset: 0,
            enable_xy_grid: false,
            xy_grid_width: 0,
            xy_grid_height: 0,
            enable_camera_grid: false,
            camera_grid_width: 0,
            camera_grid_height: 0,
            show_outside_frame: false,
            outside_frame_scale: 0,
            bpm_grid_beat: 0,
            unknown_0x60: 0,
            editing_scene: 0,
            unknown_0x78: 0,
            unknown_0x80_0xff: [0; 128],
            layers: Vec::new(),
            scenes: Vec::new(),
            trackbar_scripts: TrackbarScript::defaults(),
            effect_types: EffectType::defaults().to_vec(),
            objects: Vec::new(),
        }
    }
}

impl TryFrom<RawFilterProject> for ExEditProject {
    type Error = AupError;

    fn try_from(raw: RawFilterProject) -> Result<Self> {
        if raw.name != Self::FILTER_NAME {
            return Err(AupError::InvalidModel("filter is not ExEdit"));
        }
        Self::from_bytes(&raw.data)
    }
}
