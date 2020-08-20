use std::fmt;

const NOEL: Theme = Theme {
    name: "Noel",
    background: Rgb(0x2A3235),
    darker_background: Rgb(0x252B2D),
    foreground: Rgb(0xDEEEF3),
    faded: Rgb(0x526B6F),
    red: Rgb(0xDC9E8F),
    green: Rgb(0x8BB993),
    orange: Rgb(0xE7BEA6),
    accent1: Rgb(0xB3DCE2),
    accent2: Rgb(0x66B9D2),
    accent3: Rgb(0xEEB9C1),
};

fn main() {
    println!("{}", NOEL);
}

struct Theme {
    name: &'static str,
    background: Rgb,
    darker_background: Rgb,
    foreground: Rgb,
    faded: Rgb,
    red: Rgb,
    green: Rgb,
    orange: Rgb,
    /// This is used for overlays, so the background colour should be derived from this colour.
    accent1: Rgb,
    accent2: Rgb,
    accent3: Rgb,
}

impl Theme {
    fn workspace_colors(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\t\"colors\": {{")?;

        write_scope(f, "activityBar.background", self.darker_background)?;

        writeln!(f, "\t}},")?;

        Ok(())
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{")?;

        writeln!(f, "\t\"name\": {},", self.name)?;
        writeln!(f, "\t\"type\": \"dark\",")?;

        self.workspace_colors(f)?;

        writeln!(f, "}}")?;

        Ok(())
    }
}

fn write_scope(f: &mut fmt::Formatter<'_>, key: &str, style: impl Into<Style>) -> fmt::Result {
    writeln!(f, "\t\t\"{}\": {},", key, style.into())
}

struct Style {
    color: Rgb,
    font_style: Option<FontStyle>,
}

impl From<Rgb> for Style {
    fn from(color: Rgb) -> Self {
        Self {
            color,
            font_style: None,
        }
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(font_style) = self.font_style {
            writeln!(f, "{{")?;
            writeln!(f, "\t\t\t\"foreground\": {}", self.color)?;
            writeln!(f, "\t\t\t\"fontStyle\": {}", font_style)?;
            writeln!(f, "\t\t}}")?;

            Ok(())
        } else {
            write!(f, "{}", self.color)
        }
    }
}

#[derive(Copy, Clone)]
struct FontStyle {
    italic: bool,
    bold: bool,
}

impl fmt::Display for FontStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.italic, self.bold) {
            (true, true) => write!(f, "\"bold italic\""),
            (false, true) => write!(f, "\"bold\""),
            (true, false) => write!(f, "\"italic\""),
            (false, false) => write!(f, "\"\""),
        }
    }
}

#[derive(Copy, Clone)]
struct Rgb(u32);

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"#{:06X}\"", self.0)
    }
}
