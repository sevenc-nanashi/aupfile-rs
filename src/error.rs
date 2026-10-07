use std::io;

/// `aupfile` の処理中に発生するエラーです。
#[derive(Debug, thiserror::Error)]
pub enum AupError {
    /// 入出力エラーです。
    #[error(transparent)]
    Io(#[from] io::Error),

    /// バイナリの構造が正しくありません。
    #[error("binary parse error: {kind:?} ({remaining} bytes remaining)")]
    Parse {
        kind: nom::error::ErrorKind,
        remaining: usize,
    },

    /// ファイルまたはセクションの識別子が正しくありません。
    #[error("invalid {kind} header")]
    InvalidHeader { kind: &'static str },

    /// 入力が必要な長さに満たないか、範囲外を参照しています。
    #[error("{context}: need bytes {offset}..{end}, but input length is {actual}")]
    UnexpectedEnd {
        context: &'static str,
        offset: usize,
        end: usize,
        actual: usize,
    },

    /// ファイル内のサイズ・個数が不正です。
    #[error("invalid {field}: {value}")]
    InvalidValue { field: &'static str, value: i128 },

    /// 整数演算がオーバーフローしました。
    #[error("integer overflow while calculating {0}")]
    Overflow(&'static str),

    /// 文字列を指定された文字コードとして扱えません。
    #[error("invalid {encoding} string in {field}")]
    InvalidString {
        field: &'static str,
        encoding: &'static str,
    },

    /// 文字列が固定長領域に収まりません。
    #[error("{field} requires fewer than {max} encoded bytes, got {actual}")]
    StringTooLong {
        field: &'static str,
        max: usize,
        actual: usize,
    },

    /// 参照先のインデックスが存在しません。
    #[error("{field} index {index} is out of bounds for length {len}")]
    InvalidIndex {
        field: &'static str,
        index: usize,
        len: usize,
    },

    /// モデルの状態がファイル形式の制約を満たしていません。
    #[error("invalid model: {0}")]
    InvalidModel(&'static str),
}

/// `aupfile` の結果型です。
pub type Result<T> = std::result::Result<T, AupError>;

impl nom::error::ParseError<&[u8]> for AupError {
    fn from_error_kind(input: &[u8], kind: nom::error::ErrorKind) -> Self {
        if kind == nom::error::ErrorKind::Eof {
            return io::Error::from(io::ErrorKind::UnexpectedEof).into();
        }
        Self::Parse {
            kind,
            remaining: input.len(),
        }
    }

    fn append(_: &[u8], _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

impl nom::error::FromExternalError<&[u8], AupError> for AupError {
    fn from_external_error(_: &[u8], _: nom::error::ErrorKind, error: Self) -> Self {
        error
    }
}

impl From<nom::Err<AupError>> for AupError {
    fn from(error: nom::Err<AupError>) -> Self {
        match error {
            nom::Err::Error(error) | nom::Err::Failure(error) => error,
            nom::Err::Incomplete(_) => io::Error::from(io::ErrorKind::UnexpectedEof).into(),
        }
    }
}
