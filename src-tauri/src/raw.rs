/// Copied from https://github.com/cdown/rawtojpg/blob/master/src/main.rs under MIT License.
use anyhow::{ensure, Result};
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use memmap2::{Advice, Mmap};
use std::fs::File;
use std::path::Path;

/// Map a RAW file into memory using `mmap()`. The file must be static.
pub fn mmap_raw(file: File) -> Result<Mmap> {
    // SAFETY: mmap in general is unsafe because the lifecycle of the backing bytes are mutable
    // from outside the program.
    //
    // This means that, among other things, I/O errors can abort the program (e.g. by SIGBUS). This
    // is not a big problem, since we are just a command line program and have control over the
    // entire execution lifecycle.
    //
    // Also, any guarantees around validation (like taking a string slice from the &[u8]) are also
    // only enforced at creation time, so it's possible for the underlying file to cause corruption
    // (and thus UB). However, in our case, that's not a problem: we don't rely on such
    // enforcement.
    let raw_buf = unsafe { Mmap::map(&file)? };

    // Avoid overread into the rest of the RAW, which degrades performance substantially. We will
    // later update the advice for the JPEG section with Advice::WillNeed. Until then, our accesses
    // are essentially random: we walk the IFDs, but these are likely in non-sequential pages.
    raw_buf.advise(Advice::Random)?;
    Ok(raw_buf)
}

/// An embedded JPEG in a RAW file.
#[derive(Default, Eq, PartialEq)]
struct EmbeddedJpegInfo {
    offset: usize,
    length: usize,
}

/// Find the largest embedded JPEG in a memory-mapped RAW buffer.
///
/// This function parses the IFDs in the TIFF structure of the RAW file to find the largest JPEG
/// thumbnail embedded in the file.
///
/// We hand roll the IFD parsing because libraries do not fit requirements. For example:
///
/// - kamadak-exif: Reads into a big `Vec<u8>`, which is huge for our big RAW.
/// - quickexif: Cannot iterate over IFDs.
pub fn find_largest_embedded_jpeg(raw_buf: &[u8]) -> Result<EmbeddedJpegInfo> {
    const IFD_ENTRY_SIZE: usize = 12;
    const TIFF_MAGIC_LE: &[u8] = b"II*\0";
    const TIFF_MAGIC_BE: &[u8] = b"MM\0*";
    const JPEG_TAG: u16 = 0x201;
    const JPEG_LENGTH_TAG: u16 = 0x202;

    let is_le = &raw_buf[0..4] == TIFF_MAGIC_LE;
    ensure!(
        is_le || &raw_buf[0..4] == TIFF_MAGIC_BE,
        "Not a valid TIFF file"
    );

    let read_u16 = if is_le {
        LittleEndian::read_u16
    } else {
        BigEndian::read_u16
    };

    let read_u32 = if is_le {
        LittleEndian::read_u32
    } else {
        BigEndian::read_u32
    };

    let mut next_ifd_offset = read_u32(&raw_buf[4..8]).try_into()?;
    let mut largest_jpeg = EmbeddedJpegInfo::default();

    while next_ifd_offset != 0 {
        let cursor = &raw_buf[next_ifd_offset..];
        let num_entries = read_u16(&cursor[..2]).into();
        let entries_cursor = &cursor[2..];

        let mut cur_offset = None;
        let mut cur_length = None;

        for entry in entries_cursor
            .chunks_exact(IFD_ENTRY_SIZE)
            .take(num_entries)
        {
            let tag = read_u16(&entry[..2]);

            match tag {
                JPEG_TAG => cur_offset = Some(read_u32(&entry[8..12]).try_into()?),
                JPEG_LENGTH_TAG => cur_length = Some(read_u32(&entry[8..12]).try_into()?),
                _ => {}
            }

            if let (Some(offset), Some(length)) = (cur_offset, cur_length) {
                if length > largest_jpeg.length {
                    largest_jpeg = EmbeddedJpegInfo { offset, length };
                }
                break;
            }
        }

        next_ifd_offset = read_u32(&cursor[2 + num_entries * IFD_ENTRY_SIZE..][..4]).try_into()?;
    }

    ensure!(
        largest_jpeg != EmbeddedJpegInfo::default(),
        "No JPEG data found"
    );
    ensure!(
        largest_jpeg.offset + largest_jpeg.length <= raw_buf.len(),
        "JPEG data exceeds file size"
    );

    Ok(largest_jpeg)
}

pub fn extract_jpeg(raw_buf: &Mmap) -> Result<&[u8]> {
    let jpeg = find_largest_embedded_jpeg(raw_buf)?;
    raw_buf.advise_range(Advice::WillNeed, jpeg.offset, jpeg.length)?;
    Ok(&raw_buf[jpeg.offset..jpeg.offset + jpeg.length])
}

/// Process a single RAW file to extract the embedded JPEG, and then write the extracted JPEG to
/// the output directory.
async fn process_file(entry_path: &Path) -> Result<()> {
    let in_file = File::open(entry_path)?;
    let raw_buf = mmap_raw(in_file)?;
    let jpeg_buf = extract_jpeg(&raw_buf)?;
    Ok(())
}
