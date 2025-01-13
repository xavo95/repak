use std::io;

use byteorder::{LE, ReadBytesExt};

use crate::Version;

pub(crate) fn flag_reader<R: io::Read>(reader: &mut R,
                                       version: super::Version) -> Result<u32, super::Error> {
    let bits = reader.read_u32::<LE>()?;
    #[cfg(not(feature = "wuthering-waves"))]
    { Ok(bits) }
    #[cfg(feature = "wuthering-waves")]
    if version == Version::V12 {
        reader.read_u8()?;
        Ok(
            (bits >> 16) & 0x3f |
                (bits & 0xFFFF) << 6 |
                (bits & (1 << 28)) >> 6 |
                (bits & 0x0FC00000) << 1 |
                bits & 0xE0000000
        )
    } else {
        Ok(bits)
    }
}

pub(crate) fn offset_reader<R: io::Read>(reader: &mut R,
                                         version: super::Version,
                                         bits: u32) -> Result<(u64, u64), super::Error> {
    let offset = read_safe(reader, bits, 31)?;
    let uncompressed = read_safe(reader, bits, 30)?;
    #[cfg(not(feature = "wuthering-waves"))]
    { Ok((offset, uncompressed)) }
    #[cfg(feature = "wuthering-waves")]
    if version == Version::V12 {
        Ok((uncompressed, offset))
    } else {
        Ok((offset, uncompressed))
    }
}

#[inline(always)]
pub(crate) fn read_safe<R: io::Read>(reader: &mut R,
                                     bits: u32,
                                     bit: u32) -> Result<u64, super::Error> {
    Ok(if (bits & (1 << bit)) != 0 {
        reader.read_u32::<LE>()? as u64
    } else {
        reader.read_u64::<LE>()?
    })
}