use std::io::{Result, Write};

pub struct PrefixWriter<'s, W> {
    prefix: &'s [u8],
    w: W,
    start: bool,
}

impl<'s, W> PrefixWriter<'s, W> {
    pub fn new(prefix: &'s [u8], w: W) -> Self {
        Self {
            prefix,
            w,
            start: true,
        }
    }
}

impl<W: Write> Write for PrefixWriter<'_, W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if self.start {
            self.w.write_all(self.prefix)?;
            self.start = false;
        }

        // let mut start = 0;
        // for i in memchr::memchr_iter(b'\n', buf) {
        //     self.w.write_all(self.prefix)?;
        //     self.w.write_all(&buf[start..i])?;
        //     start = i;
        // }
        // self.w.write_all(&buf[start..])?;
        for i in buf.split_inclusive(|x| *x == b'\n') {
            self.w.write_all(i)?;
            if i.last() == Some(&b'\n') {
                self.w.write_all(self.prefix)?;
            }
            // self.w.write_all(self.prefix)?;
            // self.w.write_all(i)?;
            // self.w.write_all(b"\n")?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        self.w.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_line() {
        let mut w = Vec::new();
        let mut wraper = PrefixWriter::new("(sdb) ".as_bytes(), &mut w);

        write!(wraper, "Hello").unwrap();
    }

    #[test]
    fn two_lines() {
        let mut w = Vec::new();
        let mut wraper = PrefixWriter::new("(sdb) ".as_bytes(), &mut w);
        write!(wraper, "abc\ndefg\ndd").unwrap();
        assert_eq!(
            String::from_utf8(w).unwrap(),
            "(sdb) abc
(sdb) defg
(sdb) dd"
        );
    }

    #[test]
    fn trailing_newline() {
        let mut w = Vec::new();
        let mut wraper = PrefixWriter::new("(sdb) ".as_bytes(), &mut w);
        write!(wraper, "a\nb\n\nc\n").unwrap();
        assert_eq!(
            String::from_utf8(w).unwrap(),
            "(sdb) a
(sdb) b
(sdb) 
(sdb) c
(sdb) "
        );
    }
}
