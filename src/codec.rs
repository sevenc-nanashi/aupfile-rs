use std::borrow::Cow;
use std::io::Write;

use encoding_rs::SHIFT_JIS;
use nom::Parser;
use nom::bytes::complete::{tag, take};
use nom::number::complete::{le_i16, le_i32, le_u8, le_u16, le_u24, le_u32};
use nom::sequence::preceded;

use crate::{AupError, Result};

pub(crate) fn parse<'a, O>(
    input: &mut &'a [u8],
    mut parser: impl Parser<&'a [u8], Output = O, Error = AupError>,
) -> Result<O> {
    let (remaining, output) = parser.parse(*input)?;
    *input = remaining;
    Ok(output)
}

pub(crate) fn header(input: &mut &[u8], expected: &[u8], kind: &'static str) -> Result<()> {
    // Keep truncated headers distinct from complete but invalid headers.
    let bytes = parse(input, take(expected.len()))?;
    tag::<_, _, AupError>(expected)(bytes).map_err(|_| AupError::InvalidHeader { kind })?;
    Ok(())
}

pub(crate) fn write_i32<W: Write>(writer: &mut W, value: i32) -> Result<()> {
    writer.write_all(&value.to_le_bytes())?;
    Ok(())
}

pub(crate) fn write_u32<W: Write>(writer: &mut W, value: u32) -> Result<()> {
    writer.write_all(&value.to_le_bytes())?;
    Ok(())
}

pub(crate) fn checked_len(value: i32, field: &'static str) -> Result<usize> {
    usize::try_from(value).map_err(|_| AupError::InvalidValue {
        field,
        value: i128::from(value),
    })
}

pub(crate) fn decode_sjis(bytes: &[u8], field: &'static str) -> Result<String> {
    let bytes = match bytes.iter().position(|byte| *byte == 0) {
        Some(end) => &bytes[..end],
        None => bytes,
    };
    SHIFT_JIS
        .decode_without_bom_handling_and_without_replacement(bytes)
        .map(Cow::into_owned)
        .ok_or(AupError::InvalidString {
            field,
            encoding: "CP932",
        })
}

pub(crate) fn encode_sjis<'a>(value: &'a str, field: &'static str) -> Result<Cow<'a, [u8]>> {
    let (encoded, _, had_errors) = SHIFT_JIS.encode(value);
    if had_errors {
        return Err(AupError::InvalidString {
            field,
            encoding: "CP932",
        });
    }
    Ok(encoded)
}

pub(crate) fn encode_sjis_fixed(value: &str, len: usize, field: &'static str) -> Result<Vec<u8>> {
    let encoded = encode_sjis(value, field)?;
    if encoded.len() >= len {
        return Err(AupError::StringTooLong {
            field,
            max: len,
            actual: encoded.len(),
        });
    }
    let mut output = vec![0; len];
    output[..encoded.len()].copy_from_slice(&encoded);
    Ok(output)
}

pub(crate) fn decompress_into(input: &mut &[u8], output: &mut [u8]) -> Result<()> {
    let mut index = 0usize;
    while index < output.len() {
        let control = parse(input, le_u8)?;
        let compressed = control & 0x80 != 0;
        let short_len = usize::from(control & 0x7f);
        let len = if short_len != 0 {
            short_len
        } else {
            parse(input, le_u24)? as usize
        };
        if len == 0 {
            return Err(AupError::InvalidValue {
                field: "compressed run length",
                value: 0,
            });
        }
        let end = index
            .checked_add(len)
            .ok_or(AupError::Overflow("compressed run end"))?;
        if end > output.len() {
            return Err(AupError::UnexpectedEnd {
                context: "decompressed output",
                offset: index,
                end,
                actual: output.len(),
            });
        }
        if compressed {
            let value = parse(input, le_u8)?;
            output[index..end].fill(value);
        } else {
            output[index..end].copy_from_slice(parse(input, take(len))?);
        }
        index = end;
    }
    Ok(())
}

pub(crate) fn compress<W: Write>(writer: &mut W, mut data: &[u8]) -> Result<()> {
    while !data.is_empty() {
        if data.len() >= 4 && data[..4].iter().all(|byte| *byte == data[0]) {
            let value = data[0];
            let mut len = 4usize;
            while len < data.len() && data[len] == value && len < 0x7f_ffff {
                len += 1;
            }
            write_run_length(writer, len, true)?;
            writer.write_all(&[value])?;
            data = &data[len..];
        } else {
            let mut len = 0usize;
            while len < data.len()
                && (data.len() - len < 4
                    || !data[len..len + 4].iter().all(|byte| *byte == data[len]))
                && len < 0x7f_ffff
            {
                len += 1;
            }
            write_run_length(writer, len, false)?;
            writer.write_all(&data[..len])?;
            data = &data[len..];
        }
    }
    Ok(())
}

fn write_run_length<W: Write>(writer: &mut W, len: usize, compressed: bool) -> Result<()> {
    if len == 0 || len > 0x7f_ffff {
        return Err(AupError::InvalidValue {
            field: "compressed run length",
            value: len as i128,
        });
    }
    let flag = if compressed { 0x80 } else { 0 };
    if len < 0x80 {
        writer.write_all(&[(len as u8) | flag])?;
    } else {
        writer.write_all(&[flag, len as u8, (len >> 8) as u8, (len >> 16) as u8])?;
    }
    Ok(())
}

pub(crate) struct SliceReader<'a> {
    data: &'a [u8],
    context: &'static str,
}

impl<'a> SliceReader<'a> {
    pub(crate) fn new(data: &'a [u8], context: &'static str) -> Self {
        Self { data, context }
    }

    pub(crate) fn bytes(&self, offset: usize, len: usize) -> Result<&'a [u8]> {
        let end = offset
            .checked_add(len)
            .ok_or(AupError::Overflow("slice end"))?;
        preceded(take(offset), take(len))
            .parse(self.data)
            .map(|(_, bytes)| bytes)
            .map_err(|_: nom::Err<AupError>| AupError::UnexpectedEnd {
                context: self.context,
                offset,
                end,
                actual: self.data.len(),
            })
    }

    pub(crate) fn u8(&self, offset: usize) -> Result<u8> {
        parse(&mut self.bytes(offset, 1)?, le_u8)
    }

    pub(crate) fn i16(&self, offset: usize) -> Result<i16> {
        parse(&mut self.bytes(offset, 2)?, le_i16)
    }

    pub(crate) fn u16(&self, offset: usize) -> Result<u16> {
        parse(&mut self.bytes(offset, 2)?, le_u16)
    }

    pub(crate) fn i32(&self, offset: usize) -> Result<i32> {
        parse(&mut self.bytes(offset, 4)?, le_i32)
    }

    pub(crate) fn u32(&self, offset: usize) -> Result<u32> {
        parse(&mut self.bytes(offset, 4)?, le_u32)
    }
}

pub(crate) fn put_bytes(
    output: &mut [u8],
    offset: usize,
    value: &[u8],
    context: &'static str,
) -> Result<()> {
    let actual = output.len();
    let end = offset
        .checked_add(value.len())
        .ok_or(AupError::Overflow("slice write end"))?;
    let target = output.get_mut(offset..end).ok_or(AupError::UnexpectedEnd {
        context,
        offset,
        end,
        actual,
    })?;
    target.copy_from_slice(value);
    Ok(())
}

pub(crate) fn put_i16(
    output: &mut [u8],
    offset: usize,
    value: i16,
    context: &'static str,
) -> Result<()> {
    put_bytes(output, offset, &value.to_le_bytes(), context)
}

pub(crate) fn put_u16(
    output: &mut [u8],
    offset: usize,
    value: u16,
    context: &'static str,
) -> Result<()> {
    put_bytes(output, offset, &value.to_le_bytes(), context)
}

pub(crate) fn put_i32(
    output: &mut [u8],
    offset: usize,
    value: i32,
    context: &'static str,
) -> Result<()> {
    put_bytes(output, offset, &value.to_le_bytes(), context)
}

pub(crate) fn put_u32(
    output: &mut [u8],
    offset: usize,
    value: u32,
    context: &'static str,
) -> Result<()> {
    put_bytes(output, offset, &value.to_le_bytes(), context)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{compress, decompress_into};

    #[test]
    fn compression_round_trip() {
        let cases: &[&[u8]] = &[b"", b"a", b"aaab", b"aaaab", b"hello world", &[7; 300]];
        for input in cases {
            let mut compressed = Vec::new();
            compress(&mut compressed, input).unwrap();
            let mut actual = vec![0; input.len()];
            decompress_into(&mut compressed.as_slice(), &mut actual).unwrap();
            assert_eq!(*input, actual);
        }
    }

    #[test]
    fn decompression_checks_runs_and_preserves_remaining_input() {
        for invalid in [
            &[][..],
            &[0],
            &[0, 1, 0],
            &[0, 0, 0, 0],
            &[0x80, 0, 0, 0],
            &[0x81],
            &[2, b'a'],
            &[0x83, b'a'],
        ] {
            assert!(decompress_into(&mut &invalid[..], &mut [0; 2]).is_err());
        }
        // A literal run followed by an extended repeat run and a following section.
        let mut input = &[2, b'a', b'b', 0x80, 3, 0, 0, b'c', 0xff][..];
        let mut output = [0; 5];
        decompress_into(&mut input, &mut output).unwrap();
        assert_eq!(&output, b"abccc");
        assert_eq!(input, &[0xff]);
    }

    #[test]
    fn compression_matches_aup_dot_net_fixtures() {
        let directory = fs::read_dir("testdata/CompDecomp").unwrap();
        let mut fixture_count = 0;
        for entry in directory {
            let raw_path = entry.unwrap().path();
            let Some(filename) = raw_path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            let Some(prefix) = filename.strip_suffix("_raw.dat") else {
                continue;
            };
            fixture_count += 1;
            let compressed_path = raw_path.with_file_name(format!("{prefix}_comp.dat"));
            let raw = fs::read(&raw_path).unwrap();
            let expected_compressed = fs::read(&compressed_path).unwrap();

            let mut actual_compressed = Vec::new();
            compress(&mut actual_compressed, &raw).unwrap();
            assert_eq!(expected_compressed, actual_compressed, "{prefix}");

            let mut actual_raw = vec![0; raw.len()];
            decompress_into(&mut expected_compressed.as_slice(), &mut actual_raw).unwrap();
            assert_eq!(raw, actual_raw, "{prefix}");
        }
        assert_eq!(fixture_count, 17);
    }
}
