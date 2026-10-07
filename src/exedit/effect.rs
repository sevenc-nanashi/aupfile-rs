use std::sync::LazyLock;

use bitflags::bitflags;
use nom::bytes::complete::take;
use nom::number::complete::le_u32;

use crate::codec::{decode_sjis, encode_sjis_fixed, parse, put_u32};
use crate::{AupError, Result};

/// トラックバーの定義です。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackbarDefinition {
    pub name: String,
    pub scale: i32,
    pub min: i32,
    pub max: i32,
    pub default: i32,
}

/// チェックボックス、ボタンまたはコンボボックスの定義です。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckboxDefinition {
    pub name: String,
    pub is_checkbox: bool,
    pub default: i32,
}

/// 拡張編集に登録されたエフェクト定義です。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EffectType {
    pub id: i32,
    pub flag: u32,
    pub trackbar_count: u32,
    pub checkbox_count: u32,
    pub ext_size: u32,
    pub name: String,
    pub trackbars: Vec<Option<TrackbarDefinition>>,
    pub checkboxes: Vec<Option<CheckboxDefinition>>,
}

impl EffectType {
    pub const SIZE: usize = 112;
    pub const MAX_NAME_LENGTH: usize = 96;

    pub(crate) fn read(mut data: &[u8], id: i32) -> Result<Self> {
        let (flag, trackbar_count, checkbox_count, ext_size, name) = parse(
            &mut data,
            (le_u32, le_u32, le_u32, le_u32, take(Self::MAX_NAME_LENGTH)),
        )?;
        if trackbar_count > 64 {
            return Err(AupError::InvalidValue {
                field: "effect trackbar count",
                value: i128::from(trackbar_count),
            });
        }
        if checkbox_count > 48 {
            return Err(AupError::InvalidValue {
                field: "effect checkbox count",
                value: i128::from(checkbox_count),
            });
        }
        Ok(Self {
            id,
            flag,
            trackbar_count,
            checkbox_count,
            ext_size,
            name: decode_sjis(name, "EffectType.name")?,
            trackbars: vec![
                None;
                usize::try_from(trackbar_count)
                    .map_err(|_| { AupError::Overflow("effect trackbar count") })?
            ],
            checkboxes: vec![
                None;
                usize::try_from(checkbox_count)
                    .map_err(|_| { AupError::Overflow("effect checkbox count") })?
            ],
        })
    }

    pub(crate) fn write(&self, output: &mut [u8]) -> Result<()> {
        if self.trackbars.len() != self.trackbar_count as usize
            || self.checkboxes.len() != self.checkbox_count as usize
        {
            return Err(AupError::InvalidModel(
                "effect definition descriptor count does not match",
            ));
        }
        put_u32(output, 0, self.flag, "EffectType")?;
        put_u32(output, 4, self.trackbar_count, "EffectType")?;
        put_u32(output, 8, self.checkbox_count, "EffectType")?;
        put_u32(output, 12, self.ext_size, "EffectType")?;
        output[16..].copy_from_slice(&encode_sjis_fixed(
            &self.name,
            Self::MAX_NAME_LENGTH,
            "EffectType.name",
        )?);
        Ok(())
    }

    /// AupDotNet v0.2.0 に含まれる 108 個の既定定義です。
    pub fn defaults() -> &'static [Self] {
        &DEFAULT_EFFECT_TYPES
    }
}

macro_rules! effect_catalog {
    ($( $kind:ident => ($id:literal, $flag:literal, $tracks:literal, $checks:literal, $ext:literal, $name:literal,
        [$($track:expr,)*],
        [$($check:expr,)*]
    ), )+) => {
        /// AupDotNet v0.2.0 が認識する組み込みエフェクトです。
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(i32)]
        pub enum EffectKind {
            $( $kind = $id, )+
        }

        impl EffectKind {
            /// 組み込みエフェクトを ID 順に並べた配列です。
            pub const ALL: [Self; 108] = [$( Self::$kind, )+];

            pub fn from_id(id: i32) -> Option<Self> {
                match id {
                    $( $id => Some(Self::$kind), )+
                    _ => None,
                }
            }

            pub const fn id(self) -> i32 {
                self as i32
            }
        }

        static DEFAULT_EFFECT_TYPES: LazyLock<Vec<EffectType>> = LazyLock::new(|| vec![
            $( EffectType {
                id: $id,
                flag: $flag,
                trackbar_count: $tracks,
                checkbox_count: $checks,
                ext_size: $ext,
                name: $name.to_owned(),
                trackbars: vec![$($track,)*],
                checkboxes: vec![$($check,)*],
            }, )+
        ]);
    };
}

include!("effect_catalog.rs");

bitflags! {
    /// エフェクトの状態フラグです。
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct EffectFlag: u8 {
        const ENABLE = 1;
    }
}

/// タイムラインオブジェクトに設定されたエフェクトです。
///
/// `kind` が `Some` の場合は組み込みエフェクト、`None` の場合は未知のエフェクトです。
/// `ext_data` は既知・未知を問わず元の拡張データを保持します。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Effect {
    pub effect_type: EffectType,
    pub kind: Option<EffectKind>,
    pub flag: EffectFlag,
    pub trackbars: Vec<super::Trackbar>,
    pub checkboxes: Vec<i32>,
    pub ext_data: Vec<u8>,
}

impl Effect {
    /// 組み込みエフェクトを既定値で作成します。
    pub fn from_kind(kind: EffectKind) -> Self {
        let effect_type = EffectType::defaults()[kind.id() as usize].clone();
        let trackbars = effect_type
            .trackbars
            .iter()
            .map(|definition| {
                let value = match definition {
                    Some(definition) => definition.default,
                    None => 0,
                };
                super::Trackbar {
                    current: value,
                    next: value,
                    transition_type: super::TrackbarType::STOP,
                    flag: super::TrackbarFlag::empty(),
                    script_index: 0,
                    parameter: 0,
                }
            })
            .collect();
        let checkboxes = effect_type
            .checkboxes
            .iter()
            .map(|definition| match definition {
                Some(definition) if definition.is_checkbox => definition.default,
                Some(_) => 0,
                None => 0,
            })
            .collect();
        let ext_data = default_ext_data(kind, effect_type.ext_size as usize);
        Self {
            effect_type,
            kind: Some(kind),
            flag: EffectFlag::empty(),
            trackbars,
            checkboxes,
            ext_data,
        }
    }

    /// 名前からトラックバーを取得します。
    pub fn trackbar(&self, name: &str) -> Option<&super::Trackbar> {
        self.effect_type
            .trackbars
            .iter()
            .zip(&self.trackbars)
            .find_map(|(definition, trackbar)| match definition {
                Some(definition) if definition.name == name => Some(trackbar),
                _ => None,
            })
    }

    /// 名前からトラックバーを可変で取得します。
    pub fn trackbar_mut(&mut self, name: &str) -> Option<&mut super::Trackbar> {
        self.effect_type
            .trackbars
            .iter()
            .zip(&mut self.trackbars)
            .find_map(|(definition, trackbar)| match definition {
                Some(definition) if definition.name == name => Some(trackbar),
                _ => None,
            })
    }

    /// 名前からチェックボックス値を取得します。
    pub fn checkbox(&self, name: &str) -> Option<&i32> {
        self.effect_type
            .checkboxes
            .iter()
            .zip(&self.checkboxes)
            .find_map(|(definition, value)| match definition {
                Some(definition) if definition.name == name => Some(value),
                _ => None,
            })
    }

    /// 名前からチェックボックス値を可変で取得します。
    pub fn checkbox_mut(&mut self, name: &str) -> Option<&mut i32> {
        self.effect_type
            .checkboxes
            .iter()
            .zip(&mut self.checkboxes)
            .find_map(|(definition, value)| match definition {
                Some(definition) if definition.name == name => Some(value),
                _ => None,
            })
    }

    /// エフェクト定義と値を指定して作成します。
    ///
    /// 定義が組み込み定義と一致する場合は `kind` が自動的に設定されます。
    pub fn try_new(
        effect_type: EffectType,
        flag: EffectFlag,
        trackbars: Vec<super::Trackbar>,
        checkboxes: Vec<i32>,
        ext_data: Vec<u8>,
    ) -> Result<Self> {
        if trackbars.len() != effect_type.trackbar_count as usize
            || checkboxes.len() != effect_type.checkbox_count as usize
        {
            return Err(AupError::InvalidModel(
                "effect value count does not match definition",
            ));
        }
        if !ext_data.is_empty() && ext_data.len() != effect_type.ext_size as usize {
            return Err(AupError::InvalidModel(
                "effect extension size does not match definition",
            ));
        }
        let kind = EffectKind::from_id(effect_type.id).filter(|kind| {
            let Some(default) = EffectType::defaults().get(kind.id() as usize) else {
                return false;
            };
            default.flag == effect_type.flag
                && default.trackbar_count == effect_type.trackbar_count
                && default.checkbox_count == effect_type.checkbox_count
                && default.ext_size == effect_type.ext_size
                && default.name == effect_type.name
        });
        Ok(Self {
            effect_type,
            kind,
            flag,
            trackbars,
            checkboxes,
            ext_data,
        })
    }
}

fn default_ext_data(kind: EffectKind, size: usize) -> Vec<u8> {
    let mut data = vec![0; size];
    match kind {
        EffectKind::Figure => {
            data[0..4].copy_from_slice(&1_i32.to_le_bytes());
            data[4..7].fill(255);
        }
        EffectKind::Displacement => data[0..4].copy_from_slice(&1_i32.to_le_bytes()),
        EffectKind::Mask | EffectKind::PartialFilter => {
            data[0..4].copy_from_slice(&2_i32.to_le_bytes());
        }
        EffectKind::Border | EffectKind::Shadow => data[3] = 255,
        EffectKind::Text => {
            data[2] = 1;
            data[7] = 1;
            const FONT: &[u8] = b"MS UI Gothic";
            data[0x10..0x10 + FONT.len()].copy_from_slice(FONT);
        }
        EffectKind::SceneChange
        | EffectKind::AnimationEffect
        | EffectKind::CustomObject
        | EffectKind::CameraEffect => data[0x104] = b'*',
        _ => {}
    }
    data
}

#[cfg(test)]
mod tests {
    use super::{Effect, EffectKind, EffectType};

    #[test]
    fn all_effect_kinds_have_matching_defaults() {
        assert_eq!(EffectKind::ALL.len(), EffectType::defaults().len());
        for (id, kind) in EffectKind::ALL.iter().copied().enumerate() {
            assert_eq!(kind.id(), id as i32);
            let effect = Effect::from_kind(kind);
            assert_eq!(effect.kind, Some(kind));
            assert_eq!(
                effect.trackbars.len(),
                effect.effect_type.trackbar_count as usize
            );
            assert_eq!(
                effect.checkboxes.len(),
                effect.effect_type.checkbox_count as usize
            );
            assert_eq!(effect.ext_data.len(), effect.effect_type.ext_size as usize);
        }
    }

    #[test]
    fn typed_default_extension_data_matches_aup_dot_net() {
        let figure = Effect::from_kind(EffectKind::Figure);
        assert_eq!(&figure.ext_data[..8], &[1, 0, 0, 0, 255, 255, 255, 0]);

        let text = Effect::from_kind(EffectKind::Text);
        assert_eq!(text.ext_data[2], 1);
        assert_eq!(text.ext_data[7], 1);
        assert_eq!(&text.ext_data[0x10..0x1d], b"MS UI Gothic\0");

        let animation = Effect::from_kind(EffectKind::AnimationEffect);
        assert_eq!(animation.ext_data[0x104], b'*');
    }
}
