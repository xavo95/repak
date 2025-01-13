use std::io;

use byteorder::{LE, WriteBytesExt};

use crate::Version;

pub(crate) fn flag_writer<W: io::Write>(writer: &mut W,
                                        version: super::Version,
                                        flags: u32) -> Result<(), super::Error> {
    #[cfg(not(feature = "wuthering-waves"))]
    writer.write_u32::<LE>(flags)?;
    #[cfg(feature = "wuthering-waves")]
    if version == Version::V12 {
        let tmp = ((flags & 0x3f) << 16) | ((flags >> 6) & 0xFFFF) |
            ((flags << 6) & (1 << 28)) | ((flags >> 1) & 0x0FC00000) | flags & 0xE0000000;
        writer.write_u32::<LE>(tmp)?;
        writer.write_u8(0)?;
    } else {
        writer.write_u32::<LE>(flags)?;
    }
    Ok(())
}

pub(crate) fn offset_writer<W: io::Write>(writer: &mut W,
                                          version: super::Version,
                                          offset: u64,
                                          offset_safe_32: bool,
                                          uncompressed: u64,
                                          uncompressed_safe_32: bool) -> Result<(), super::Error> {
    #[cfg(not(feature = "wuthering-waves"))]
    {
        write_safe(writer, offset_safe_32, offset)?;
        write_safe(writer, uncompressed_safe_32, uncompressed)?;
    }
    #[cfg(feature = "wuthering-waves")]
    if version == Version::V12 {
        write_safe(writer, uncompressed_safe_32, uncompressed)?;
        write_safe(writer, offset_safe_32, offset)?;
    } else {
        write_safe(writer, offset_safe_32, offset)?;
        write_safe(writer, uncompressed_safe_32, uncompressed)?;
    }
    Ok(())
}

#[inline(always)]
pub(crate) fn write_safe<W: io::Write>(writer: &mut W, safe: bool, value: u64) -> Result<(), super::Error> {
    if safe {
        writer.write_u32::<LE>(value as u32)?
    } else {
        writer.write_u64::<LE>(value)?
    }
    Ok(())
}