//! SAM reader and iterators.

mod records;

pub use self::records::Records;

use std::io::{self, BufRead, Read, Seek};

use noodles_bgzf as bgzf;

const HEADER_PREFIX: u8 = b'@';
const NEWLINE: u8 = b'\n';

/// A SAM reader.
///
/// The SAM format is comprised to two parts: 1) a header and 2) a list of records.
///
/// Each header line is prefixed with an `@` (at sign). The header is optional and may be empty.
///
/// SAM records are line-based and follow directly after the header or the start of the file until
/// EOF.
///
/// # Examples
///
/// ```no_run
/// # use std::{fs::File, io::{self, BufReader}};
/// use noodles_sam as sam;
///
/// let mut reader = File::open("sample.sam")
///     .map(BufReader::new)
///     .map(sam::Reader::new)?;
///
/// reader.read_header()?;
///
/// for result in reader.records() {
///     let record = result?;
///     println!("{:?}", record);
/// }
/// # Ok::<(), io::Error>(())
/// ```
#[derive(Debug)]
pub struct Reader<R> {
    inner: R,
}

impl<R> Reader<R>
where
    R: BufRead,
{
    /// Creates a SAM reader.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam as sam;
    ///
    /// let data = b"@HD\tVN:1.6
    /// *\t4\t*\t0\t255\t*\t*\t0\t0\t*\t*
    /// ";
    ///
    /// let reader = sam::Reader::new(&data[..]);
    /// ```
    pub fn new(inner: R) -> Self {
        Self { inner }
    }

    /// Reads the raw SAM header.
    ///
    /// The position of the stream is expected to be at the start.
    ///
    /// This returns the raw SAM header as a [`std::string::String`]. It can subsequently be parsed
    /// as a [`crate::Header`].
    ///
    /// The SAM header is optional, and if it is missing, an empty string is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::io;
    /// use noodles_sam as sam;
    ///
    /// let data = b"@HD\tVN:1.6
    /// *\t4\t*\t0\t255\t*\t*\t0\t0\t*\t*
    /// ";
    ///
    /// let mut reader = sam::Reader::new(&data[..]);
    /// let header = reader.read_header()?;
    ///
    /// assert_eq!(header, "@HD\tVN:1.6\n");
    /// # Ok::<(), io::Error>(())
    /// ```
    pub fn read_header(&mut self) -> io::Result<String> {
        let mut header_buf = Vec::new();
        let mut eol = false;

        for i in 0.. {
            let buf = self.inner.fill_buf()?;

            if (i == 0 || eol) && buf.first().map(|&b| b != HEADER_PREFIX).unwrap_or(true) {
                break;
            }

            let (read_eol, len) = match buf.iter().position(|&b| b == NEWLINE) {
                Some(i) => {
                    header_buf.extend(&buf[..=i]);
                    (true, i + 1)
                }
                None => {
                    header_buf.extend(buf);
                    (false, buf.len())
                }
            };

            eol = read_eol;
            self.inner.consume(len);
        }

        String::from_utf8(header_buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    /// Reads a single raw SAM record.
    ///
    /// This reads from the underlying stream until a newline is reached and appends it to the
    /// given buffer, sans the final newline character.
    ///
    /// The stream is expected to be directly after the header or at the start of another record.
    ///
    /// It is more ergonomic to read records using an iterator (see [`Self::records`]), but using
    /// this method allows control of the line buffer and whether the raw record should be parsed.
    ///
    /// If successful, the number of bytes read is returned. If the number of bytes read is 0, the
    /// stream reached EOF.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::io;
    /// use noodles_sam as sam;
    ///
    /// let data = b"@HD\tVN:1.6
    /// *\t4\t*\t0\t255\t*\t*\t0\t0\t*\t*
    /// ";
    ///
    /// let mut reader = sam::Reader::new(&data[..]);
    /// reader.read_header()?;
    ///
    /// let mut buf = String::new();
    /// reader.read_record(&mut buf)?;
    /// assert_eq!(buf, "*\t4\t*\t0\t255\t*\t*\t0\t0\t*\t*");
    ///
    /// assert_eq!(reader.read_record(&mut buf)?, 0);
    /// # Ok::<(), io::Error>(())
    /// ```
    pub fn read_record(&mut self, buf: &mut String) -> io::Result<usize> {
        let result = self.inner.read_line(buf);
        buf.pop();
        result
    }

    /// Returns an iterator over records starting from the current stream position.
    ///
    /// The stream is expected to be directly after the header or at the start of another record.
    ///
    /// Unlike [`Self::read_record`], each record is parsed as a [`crate::Record`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::io;
    /// use noodles_sam as sam;
    ///
    /// let data = b"@HD\tVN:1.6
    /// *\t4\t*\t0\t255\t*\t*\t0\t0\t*\t*
    /// ";
    ///
    /// let mut reader = sam::Reader::new(&data[..]);
    /// reader.read_header()?;
    ///
    /// let mut records = reader.records();
    /// assert!(records.next().is_some());
    /// assert!(records.next().is_none());
    /// # Ok::<(), io::Error>(())
    /// ```
    pub fn records(&mut self) -> Records<'_, R> {
        Records::new(self)
    }
}

impl<R> Reader<bgzf::Reader<R>>
where
    R: Read + Seek,
{
    /// Seeks the underlying BGZF stream to the given virtual position.
    ///
    /// Virtual positions typically come from an associated index.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::{fs::File, io};
    /// use std::io::BufReader;
    ///
    /// use noodles_bgzf as bgzf;
    /// use noodles_sam as sam;
    ///
    /// let mut reader = File::open("sample.sam.gz")
    ///     .map(bgzf::Reader::new)
    ///     .map(sam::Reader::new)?;
    ///
    /// let virtual_position = bgzf::VirtualPosition::from(102334155);
    /// reader.seek(virtual_position)?;
    /// # Ok::<(), io::Error>(())
    /// ```
    pub fn seek(&mut self, pos: bgzf::VirtualPosition) -> io::Result<bgzf::VirtualPosition> {
        self.inner.seek(pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_header_with_no_header() -> io::Result<()> {
        let data = b"*\t4\t*\t0\t255\t*\t*\t0\t0\t*\t*\n";
        let mut reader = Reader::new(&data[..]);
        assert!(reader.read_header()?.is_empty());
        Ok(())
    }

    #[test]
    fn test_read_header_with_no_records() -> io::Result<()> {
        let data = b"@HD\tVN1.6\n";
        let mut reader = Reader::new(&data[..]);
        let header = reader.read_header()?;
        assert_eq!(header, "@HD\tVN1.6\n");
        Ok(())
    }
}
