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

impl<C: Color> AsciiCell<C> {
    pub(crate) fn fmt_with_previous(
        &self,
        f: &mut String,
        previous: Option<AsciiCell<C>>,
    ) -> std::fmt::Result {
        let mut sgr = SelectGraphicRendition::new(f);

        let character = if let Some(foreground) = self.foreground {
            if !previous.is_some_and(|previous| previous.foreground == Some(foreground)) {
                self.background
                    .write_background(&mut sgr)
                    .map_err(|_| std::fmt::Error)?;
            }

            foreground.character
        } else {
            ' '
        };

        if !previous.is_some_and(|previous| previous.background == self.background) {
            self.background
                .write_background(&mut sgr)
                .map_err(|_| std::fmt::Error)?;
        }

        drop(sgr);

        f.push(character);

        Ok(())
    }
}

impl<C: Color> Display for AsciiCell<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        self.fmt_with_previous(&mut string, None)?;
        f.write_str(&string)
    }
}
