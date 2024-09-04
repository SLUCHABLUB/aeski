use std::fmt::Write as _;
use std::io::Write;

const CONTROL_SEQUENCE_INTRODUCER: &str = "\x1b[";
const SELECT_GRAPHIC_RENDITION: char = 'm';

pub(crate) struct SelectGraphicRendition<'f> {
    started: bool,
    // TODO: Use `Formatter`.
    f: &'f mut String,
}

impl<'a> SelectGraphicRendition<'a> {
    pub(crate) fn new(f: &'a mut String) -> Self {
        SelectGraphicRendition { started: false, f }
    }

    pub(crate) fn write_zero(mut self) {
        self.f.push_str(CONTROL_SEQUENCE_INTRODUCER);
        self.started = true;
    }
}

impl Write for SelectGraphicRendition<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        if self.started {
            self.f.push(';');
        } else {
            self.f.push_str(CONTROL_SEQUENCE_INTRODUCER);
        }

        self.started = true;
        self.f
            .write_fmt(format_args!("{}", buf[0]))
            .map_err(std::io::Error::other)?;

        Ok(1)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Drop for SelectGraphicRendition<'_> {
    fn drop(&mut self) {
        if self.started {
            self.f.push(SELECT_GRAPHIC_RENDITION);
        }
    }
}
