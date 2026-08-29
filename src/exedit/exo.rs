use std::fmt::Write as _;
use std::io::Write;

use super::{
    Effect, EffectKind, ExEditProject, TimelineObject, TimelineObjectFlag, TrackbarScript,
};
use crate::codec::{SliceReader, decode_sjis, encode_sjis};
use crate::{AupError, EditHandle, Result};

/// 拡張編集のオブジェクトファイル (`.exo`) です。
#[derive(Clone, Debug, Default)]
pub struct ExeditObjectFile {
    pub width: i32,
    pub height: i32,
    pub rate: i32,
    pub scale: i32,
    pub length: i32,
    pub audio_rate: i32,
    pub audio_ch: i32,
    pub alpha: bool,
    pub scene_name: String,
    pub objects: Vec<TimelineObject>,
    pub all_objects: Vec<TimelineObject>,
    pub trackbar_scripts: Vec<TrackbarScript>,
}

impl ExeditObjectFile {
    pub(crate) fn from_project(
        project: &ExEditProject,
        scene_index: u32,
        edit_handle: &EditHandle,
    ) -> Result<Self> {
        let scene = project
            .scenes
            .iter()
            .find(|scene| scene.scene_index == scene_index);
        let (mut width, mut height, mut length, alpha, scene_name) = match scene {
            Some(scene) => (
                i32::try_from(scene.width).map_err(|_| AupError::Overflow("EXO scene width"))?,
                i32::try_from(scene.height).map_err(|_| AupError::Overflow("EXO scene height"))?,
                i32::try_from(scene.max_frame)
                    .map_err(|_| AupError::Overflow("EXO scene frame count"))?,
                scene.flag.contains(super::SceneFlag::ALPHA),
                scene.name.clone(),
            ),
            None => (0, 0, 0, false, String::new()),
        };
        if width == 0 {
            width = edit_handle.width;
            height = edit_handle.height;
        }
        if scene_index == project.editing_scene {
            length = i32::try_from(edit_handle.frames.len())
                .map_err(|_| AupError::Overflow("EXO frame count"))?;
        }
        Ok(Self {
            width,
            height,
            rate: edit_handle.video_rate,
            scale: edit_handle.video_scale,
            length,
            audio_rate: edit_handle.audio_rate,
            audio_ch: i32::from(edit_handle.audio_ch),
            alpha,
            scene_name,
            objects: project
                .objects
                .iter()
                .filter(|object| object.scene_index == scene_index)
                .cloned()
                .collect(),
            all_objects: project.objects.clone(),
            trackbar_scripts: project.trackbar_scripts.clone(),
        })
    }

    /// CRLF 改行の `.exo` テキストを生成します。
    pub fn to_text(&self) -> Result<String> {
        let mut output = String::new();
        line(&mut output, "[exedit]");
        property(&mut output, "width", self.width);
        property(&mut output, "height", self.height);
        property(&mut output, "rate", self.rate);
        property(&mut output, "scale", self.scale);
        property(&mut output, "length", self.length);
        property(&mut output, "audio_rate", self.audio_rate);
        property(&mut output, "audio_ch", self.audio_ch);
        if self.alpha {
            property(&mut output, "alpha", 1);
        }
        if !self.scene_name.is_empty() {
            property(&mut output, "name", &self.scene_name);
        }

        for (index, object) in self.objects.iter().enumerate() {
            let chain_parent = if object.chain {
                self.all_objects.iter().find(|candidate| {
                    !candidate.chain && candidate.chain_group == object.chain_group
                })
            } else {
                None
            };
            export_object(
                &mut output,
                object,
                index,
                &self.trackbar_scripts,
                chain_parent,
            )?;
        }
        Ok(output)
    }

    /// `.exo` を CP932 で書き込みます。
    pub fn write<W: Write>(&self, mut writer: W) -> Result<()> {
        let text = self.to_text()?;
        writer.write_all(&encode_sjis(&text, "ExeditObjectFile")?)?;
        Ok(())
    }
}

fn export_object(
    output: &mut String,
    object: &TimelineObject,
    index: usize,
    scripts: &[TrackbarScript],
    chain_parent: Option<&TimelineObject>,
) -> Result<()> {
    line(output, &format!("[{index}]"));
    property(output, "start", object.start_frame + 1);
    property(output, "end", object.end_frame + 1);
    property(output, "layer", object.layer_index + 1);
    if object.group != TimelineObject::NO_GROUP {
        property(output, "group", object.group);
    }
    let media = object.flag.contains(TimelineObjectFlag::MEDIA);
    let audio = object.flag.contains(TimelineObjectFlag::AUDIO);
    let media_filter = object.flag.contains(TimelineObjectFlag::MEDIA_FILTER);
    let control = object.flag.contains(TimelineObjectFlag::CONTROL);
    if media || media_filter {
        property(
            output,
            "overlay",
            i32::from(!object.flag.contains(TimelineObjectFlag::CLIPPING)),
        );
    }
    if (media && !audio) || (media_filter && control) {
        property(
            output,
            "camera",
            i32::from(object.flag.contains(TimelineObjectFlag::CAMERA)),
        );
    }
    if audio {
        property(output, "audio", 1);
    }
    if object.chain {
        property(output, "chain", 1);
    }

    for (effect_index, effect) in object.effects.iter().enumerate() {
        line(output, &format!("[{index}.{effect_index}]"));
        let parent_effect = chain_parent.and_then(|parent| parent.effects.get(effect_index));
        export_effect(output, effect, scripts, object.chain, parent_effect)?;
    }
    Ok(())
}

fn export_effect(
    output: &mut String,
    effect: &Effect,
    scripts: &[TrackbarScript],
    chain: bool,
    chain_parent: Option<&Effect>,
) -> Result<()> {
    property(output, "_name", &effect.effect_type.name);
    let flag = match chain_parent {
        Some(parent) => parent.flag,
        None => effect.flag,
    };
    if !flag.contains(super::EffectFlag::ENABLE) {
        property(output, "_disable", 1);
    }
    for (index, trackbar) in effect.trackbars.iter().enumerate() {
        let Some(definition) = effect
            .effect_type
            .trackbars
            .get(index)
            .and_then(Option::as_ref)
        else {
            continue;
        };
        if definition.name.is_empty() {
            continue;
        }
        property(
            output,
            &definition.name,
            trackbar.to_exo(definition.scale, scripts)?,
        );
    }
    for (index, value) in effect.checkboxes.iter().enumerate() {
        let Some(definition) = effect
            .effect_type
            .checkboxes
            .get(index)
            .and_then(Option::as_ref)
        else {
            continue;
        };
        if definition.is_checkbox {
            property(output, &definition.name, *value);
        }
    }
    if !chain {
        export_ext_data(output, effect)?;
    }
    Ok(())
}

fn export_ext_data(output: &mut String, effect: &Effect) -> Result<()> {
    if effect.ext_data.len() != effect.effect_type.ext_size as usize {
        return Err(AupError::InvalidModel(
            "effect extension size does not match definition",
        ));
    }
    let Some(kind) = effect.kind else {
        return Ok(());
    };
    let data = SliceReader::new(&effect.ext_data, "effect extension data");
    match kind {
        EffectKind::VideoFile | EffectKind::AudioFile => {
            sjis_property(output, "file", data.bytes(0, 260)?)?;
        }
        EffectKind::ImageFile => {
            sjis_property(output, "file", data.bytes(4, 256)?)?;
        }
        EffectKind::Text => {
            property(output, "type", data.u8(0)?);
            bool_property(output, "autoadjust", data.u8(1)? != 0);
            bool_property(output, "soft", data.u8(2)? != 0);
            bool_property(output, "monospace", data.u8(3)? != 0);
            property(output, "align", data.u8(4)?);
            property(output, "spacing_x", data.u8(5)?);
            property(output, "spacing_y", data.u8(6)?);
            bool_property(output, "precision", data.u8(7)? != 0);
            color_property(output, "color", data.bytes(8, 4)?)?;
            color_property(output, "color2", data.bytes(0x0c, 4)?)?;
            sjis_property(output, "font", data.bytes(0x10, 32)?)?;
            hex_property(output, "text", data.bytes(0x30, 2048)?);
        }
        EffectKind::Figure => {
            property(output, "type", data.i32(0)?);
            color_property(output, "color", data.bytes(4, 4)?)?;
            sjis_property(output, "name", data.bytes(8, 256)?)?;
        }
        EffectKind::Waveform => {
            sjis_property(output, "file", data.bytes(0, 260)?)?;
            property(output, "type", data.i16(0x118)?);
            property(output, "mode", data.i16(0x11a)?);
            property(output, "res_w", data.i16(0x11c)?);
            property(output, "res_h", data.i16(0x11e)?);
            property(output, "pad_w", data.i16(0x120)?);
            property(output, "pad_h", data.i16(0x122)?);
            color_property(output, "color", data.bytes(0x124, 4)?)?;
            property(output, "sample_n", data.i32(0x128)?);
            bool_property(output, "mirror", data.u8(0x12c)? != 0);
        }
        EffectKind::Scene | EffectKind::SceneAudio => {
            property(output, "scene", data.i32(0)?);
        }
        EffectKind::StandardDraw | EffectKind::ExtendedDraw | EffectKind::Particle => {
            property(output, "blend", data.i32(0)?);
        }
        EffectKind::SceneChange
        | EffectKind::AnimationEffect
        | EffectKind::CustomObject
        | EffectKind::CameraEffect => {
            property(output, "type", data.i16(0)?);
            property(output, "filter", data.i16(2)?);
            sjis_property(output, "name", data.bytes(4, 256)?)?;
            sjis_property(output, "param", data.bytes(0x104, 256)?)?;
        }
        EffectKind::Emission | EffectKind::EmissionFilter => {
            color_property(output, "color", data.bytes(0, 4)?)?;
            bool_property(output, "no_color", data.u8(3)? != 0);
        }
        EffectKind::Flash => {
            color_property(output, "color", data.bytes(0, 4)?)?;
            bool_property(output, "no_color", data.u8(3)? != 0);
            property(output, "mode", data.i32(4)?);
        }
        EffectKind::Glow | EffectKind::GlowFilter => {
            color_property(output, "color", data.bytes(0, 4)?)?;
            bool_property(output, "no_color", data.u8(3)? != 0);
            property(output, "type", data.i32(4)?);
        }
        EffectKind::ChromaKey | EffectKind::ColorKey => {
            hex_property(output, "color_yc", data.bytes(0, 6)?);
            property(output, "status", data.i32(8)?);
        }
        EffectKind::LuminanceKey => property(output, "type", data.i32(0)?),
        EffectKind::Light
        | EffectKind::EdgeExtraction
        | EffectKind::Monochromatic
        | EffectKind::MonochromaticFilter => {
            color_property(output, "color", data.bytes(0, 4)?)?;
        }
        EffectKind::Shadow | EffectKind::Border => {
            color_property(output, "color", data.bytes(0, 4)?)?;
            sjis_property(output, "file", data.bytes(4, 256)?)?;
        }
        EffectKind::Wipe => {
            property(output, "type", data.i32(0)?);
            sjis_property(output, "name", data.bytes(4, 256)?)?;
        }
        EffectKind::Mask => {
            property(output, "type", data.i32(0)?);
            sjis_property(output, "name", data.bytes(4, 256)?)?;
            property(output, "mode", data.i32(0x104)?);
        }
        EffectKind::Mirror
        | EffectKind::ColorShift
        | EffectKind::ColorShiftFilter
        | EffectKind::Deinterlacing => property(output, "type", data.i32(0)?),
        EffectKind::Ripple => {
            property(output, "num", data.i32(0)?);
            property(output, "interval", data.i32(4)?);
            property(output, "add", data.i32(8)?);
        }
        EffectKind::Displacement => {
            property(output, "type", data.i32(0)?);
            sjis_property(output, "name", data.bytes(4, 256)?)?;
            property(output, "mode", data.i32(0x104)?);
            property(output, "calc", data.i32(0x108)?);
        }
        EffectKind::Noise => {
            property(output, "type", data.i32(0)?);
            property(output, "mode", data.i32(4)?);
            property(output, "seed", data.i32(8)?);
        }
        EffectKind::Gradation => {
            property(output, "blend", data.i32(0)?);
            color_property(output, "color", data.bytes(4, 4)?)?;
            bool_property(output, "no_color", data.u8(7)? != 0);
            color_property(output, "color2", data.bytes(8, 4)?)?;
            bool_property(output, "no_color2", data.u8(11)? != 0);
            property(output, "type", data.i32(12)?);
        }
        EffectKind::GamutConversion => {
            hex_property(output, "color_yc", data.bytes(0, 6)?);
            bool_property(output, "status", data.u16(6)? != 0);
            hex_property(output, "color_yc2", data.bytes(8, 6)?);
            bool_property(output, "status2", data.u16(14)? != 0);
        }
        EffectKind::Script | EffectKind::CameraScript => {
            hex_property(output, "text", data.bytes(0, 2048)?);
        }
        EffectKind::VideoComposition => {
            sjis_property(output, "file", data.bytes(0, 260)?)?;
            property(output, "mode", data.i32(0x118)?);
        }
        EffectKind::ImageComposition => {
            property(output, "mode", data.i32(0)?);
            sjis_property(output, "file", data.bytes(4, 256)?)?;
        }
        EffectKind::PartialFilter => {
            property(output, "type", data.i32(0)?);
            sjis_property(output, "name", data.bytes(4, 256)?)?;
        }
        EffectKind::TimeControl | EffectKind::GroupControl | EffectKind::CameraControl => {
            property(output, "range", data.i32(0)?);
        }
        EffectKind::ClipResizeFilter => hex_property(output, "_exdata", &effect.ext_data),
        _ => {}
    }
    Ok(())
}

fn sjis_property(output: &mut String, name: &str, bytes: &[u8]) -> Result<()> {
    property(output, name, decode_sjis(bytes, "effect extension string")?);
    Ok(())
}

fn bool_property(output: &mut String, name: &str, value: bool) {
    property(output, name, u8::from(value));
}

fn color_property(output: &mut String, name: &str, bytes: &[u8]) -> Result<()> {
    let color = bytes.get(..3).ok_or(AupError::UnexpectedEnd {
        context: "effect color",
        offset: 0,
        end: 3,
        actual: bytes.len(),
    })?;
    hex_property(output, name, color);
    Ok(())
}

fn hex_property(output: &mut String, name: &str, bytes: &[u8]) {
    let mut value = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let _ = write!(value, "{byte:02x}");
    }
    property(output, name, value);
}

fn line(output: &mut String, value: &str) {
    output.push_str(value);
    output.push_str("\r\n");
}

fn property(output: &mut String, name: &str, value: impl std::fmt::Display) {
    let _ = write!(output, "{name}={value}\r\n");
}
