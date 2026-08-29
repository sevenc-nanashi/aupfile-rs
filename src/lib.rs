//! AviUtl 1.x のプロジェクトファイル (`.aup`) を読み書きするライブラリです。

mod aup;
mod codec;
mod error;
pub mod exedit;

pub use aup::{
    AviUtlProject, ClippedImage, EditFrameEditFlag, EditHandle, FilterConfig, FilterProject,
    FrameStatus, FrameStatusInter, RawFilterProject,
};
pub use error::{AupError, Result};
