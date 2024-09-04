use crate::color::Color;
use crate::sgr::SelectGraphicRendition;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct AsciiCell<Color> {
    pub background: Color,
    pub foreground: Option<Foreground<Color>>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Foreground<Color> {
    pub color: Color,
    pub character: char,
}

impl<C: Color + PartialEq> AsciiCell<C> {
    pub(crate) fn fmt_with_previous(
        &self,
        f: &mut String,
        previous: Option<AsciiCell<C>>,
    ) -> std::fmt::Result {
        let mut sgr = SelectGraphicRendition::new(f);

        let character;
        let write_background;
        let write_foreground;

        match (previous, self.foreground.as_ref()) {
            (Some(previous), Some(foreground)) => {
                write_background = previous.background != self.background;
                write_foreground = (previous.foreground != self.foreground).then_some(foreground);
            }
            (Some(previous), None) => {
                write_background = previous.background != self.background;
                write_foreground = None;
            }
            (None, Some(foreground)) => {
                write_background = true;
                write_foreground = Some(foreground);
            }
            (None, None) => {
                write_background = true;
                write_foreground = None;
            }
        }

        if write_background {
            self.background
                .write_background(&mut sgr)
                .map_err(|_| std::fmt::Error)?;
        }
        if let Some(foreground) = write_foreground {
            foreground
                .color
                .write_foreground(&mut sgr)
                .map_err(|_| std::fmt::Error)?;

            character = foreground.character;
        } else {
            character = ' ';
        }

        drop(sgr);

        f.push(character);

        Ok(())
    }
}

impl<C: Color + PartialEq> Display for AsciiCell<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        self.fmt_with_previous(&mut string, None)?;
        f.write_str(&string)
    }
}
