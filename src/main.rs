use std::fmt;

fn main() {
    println!("{}", NOEL);
}

const NOEL: Theme = Theme {
    name: "Noel",
    bg: Rgb(0x2A3235),
    darker_bg: Rgb(0x252B2D),
    fg: Rgb(0xDEEEF3),
    faded: Rgb(0x526B6F),
    red: Rgb(0xDC9E8F),
    green: Rgb(0x8BB993),
    orange: Rgb(0xE7BEA6),
    primary_accent: Rgb(0xB3DCE2),
    secondary_accent1: Rgb(0x66B9D2),
    secondary_accent2: Rgb(0xEEB9C1),
};

const INVISIBLE: Rgba = Rgba { rgb: Rgb(0x000000), a: 0x00 };

struct Theme {
    name: &'static str,
    bg: Rgb,
    darker_bg: Rgb,
    fg: Rgb,
    faded: Rgb,
    red: Rgb,
    green: Rgb,
    orange: Rgb,
    primary_accent: Rgb,
    secondary_accent1: Rgb,
    secondary_accent2: Rgb,
}

impl Theme {
    fn workspace_colors(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\t\"colors\": {{")?;

        write_scope(f, "activityBar.background", self.darker_bg)?;
        write_scope(f, "activityBar.foreground", self.fg)?;
        write_scope(f, "activityBar.inactiveForeground", Rgba { rgb: self.primary_accent, a: 0x44 })?;
        write_scope(f, "activityBarBadge.background", self.primary_accent)?;
        write_scope(f, "activityBarBadge.foreground", self.darker_bg)?;
        write_scope(f, "badge.background", Rgba { rgb: self.primary_accent, a: 0x33 })?;
        write_scope(f, "debugToolBar.background", self.darker_bg)?;
        write_scope(f, "diffEditor.insertedTextBackground", Rgba { rgb: self.green, a: 0x22 })?;
        write_scope(f, "diffEditor.removedTextBackground", Rgba { rgb: self.red, a: 0x22 })?;
        write_scope(f, "dropdown.background", self.darker_bg)?;
        write_scope(f, "dropdown.foreground", self.fg)?;
        write_scope(f, "editor.background", self.bg)?;
        write_scope(f, "editor.findMatchBackground", Rgba { rgb: self.secondary_accent1, a: 0x77 })?;
        write_scope(f, "editor.findMatchHighlightBackground", Rgba { rgb: self.secondary_accent1, a: 0x44 })?;
        write_scope(f, "editor.foreground", self.fg)?;
        write_scope(f, "editor.lineHighlightBackground", Rgba { rgb: self.primary_accent, a: 0x0D })?;
        write_scope(f, "editor.rangeHighlightBackground", Rgba { rgb: self.primary_accent, a: 0x22 })?;
        write_scope(f, "editor.selectionBackground", Rgba { rgb: self.primary_accent, a: 0x33 })?;
        write_scope(f, "editor.wordHighlightBackground", Rgba { rgb: self.primary_accent, a: 0x33 })?;
        write_scope(f, "editorCodeLens.foreground", Rgba { rgb: self.primary_accent, a: 0x55 })?;
        write_scope(f, "editorCursor.foreground", self.fg)?;
        write_scope(f, "editorError.foreground", self.red)?;
        write_scope(f, "editorGroup.border", Rgba { rgb: self.primary_accent, a: 0x14 })?;
        write_scope(f, "editorGroupHeader.noTabsBackground", self.darker_bg)?;
        write_scope(f, "editorGroupHeader.tabsBackground", self.darker_bg)?;
        write_scope(f, "editorGutter.addedBackground", self.green)?;
        write_scope(f, "editorGutter.deletedBackground", self.red)?;
        write_scope(f, "editorGutter.modifiedBackground", self.orange)?;
        write_scope(f, "editorHoverWidget.background", self.darker_bg)?;
        write_scope(f, "editorIndentGuide.activeBackground", Rgba { rgb: self.primary_accent, a: 0x22 })?;
        write_scope(f, "editorIndentGuide.background", Rgba { rgb: self.primary_accent, a: 0x11 })?;
        write_scope(f, "editorLineNumber.activeForeground", self.fg)?;
        write_scope(f, "editorLineNumber.foreground", Rgba { rgb: self.primary_accent, a: 0x55 })?;
        write_scope(f, "editorLink.activeForeground", self.secondary_accent1)?;
        write_scope(f, "editorOverviewRuler.addedForeground", self.green)?;
        write_scope(f, "editorOverviewRuler.deletedForeground", self.red)?;
        write_scope(f, "editorOverviewRuler.errorForeground", self.red)?;
        write_scope(f, "editorOverviewRuler.findMatchForeground", Rgba { rgb: self.secondary_accent1, a: 0x77 })?;
        write_scope(f, "editorOverviewRuler.modifiedForeground", self.orange)?;
        write_scope(f, "editorOverviewRuler.rangeHighlightForeground", Rgba { rgb: self.primary_accent, a: 0x22 })?;
        write_scope(f, "editorOverviewRuler.warningForeground", self.orange)?;
        write_scope(f, "editorRuler.foreground", Rgba { rgb: self.primary_accent, a: 0x14 })?;
        write_scope(f, "editorWarning.foreground", self.orange)?;
        write_scope(f, "editorWhitespace.foreground", Rgba { rgb: self.primary_accent, a: 0x22 })?;
        write_scope(f, "editorWidget.background", self.darker_bg)?;
        write_scope(f, "editorWidget.border", Rgba { rgb: self.primary_accent, a: 0x14 })?;
        write_scope(f, "errorForeground", self.red)?;
        write_scope(f, "focusBorder", INVISIBLE)?;
        write_scope(f, "foreground", self.fg)?;
        write_scope(f, "gitDecoration.addedResourceForeground", self.green)?;
        write_scope(f, "gitDecoration.deletedResourceForeground", self.red)?;
        write_scope(f, "gitDecoration.modifiedResourceForeground", self.orange)?;
        write_scope(f, "input.background", Rgba { rgb: self.primary_accent, a: 0x11 })?;
        write_scope(f, "input.placeholderForeground", Rgba { rgb: self.primary_accent, a: 0x55 })?;
        write_scope(f, "list.activeSelectionBackground", Rgba { rgb: self.primary_accent, a: 0x33 })?;
        write_scope(f, "list.activeSelectionForeground", self.fg)?;
        write_scope(f, "list.errorForeground", self.red)?;
        write_scope(f, "list.focusBackground", Rgba { rgb: self.primary_accent, a: 0x22 })?;
        write_scope(f, "list.highlightForeground", self.secondary_accent1)?;
        write_scope(f, "list.hoverBackground", Rgba { rgb: self.primary_accent, a: 0x11 })?;
        write_scope(f, "list.inactiveSelectionBackground", Rgba { rgb: self.primary_accent, a: 0x22 })?;
        write_scope(f, "list.warningForeground", self.orange)?;
        write_scope(f, "menu.background", self.darker_bg)?;
        write_scope(f, "minimap.errorHighlight", self.red)?;
        write_scope(f, "minimap.findMatchHighlight", Rgba { rgb: self.secondary_accent1, a: 0x77 })?;
        write_scope(f, "minimap.selectionHighlight", Rgba { rgb: self.primary_accent, a: 0x11 })?;
        write_scope(f, "minimap.warningHighlight", self.orange)?;
        write_scope(f, "minimapGutter.addedBackground", self.green)?;
        write_scope(f, "minimapGutter.deletedBackground", self.red)?;
        write_scope(f, "minimapGutter.modifiedBackground", self.orange)?;
        write_scope(f, "minimapSlider.activeBackground", Rgba { rgb: self.primary_accent, a: 0x33 })?;
        write_scope(f, "minimapSlider.background", Rgba { rgb: self.primary_accent, a: 0x11 })?;
        write_scope(f, "minimapSlider.hoverBackground", Rgba { rgb: self.primary_accent, a: 0x22 })?;
        write_scope(f, "panel.border", Rgba { rgb: self.primary_accent, a: 0x14 })?;
        write_scope(f, "panelTitle.activeForeground", self.fg)?;
        write_scope(f, "panelTitle.inactiveForeground", Rgba { rgb: self.primary_accent, a: 0x55 })?;
        write_scope(f, "peekView.border", self.darker_bg)?;
        write_scope(f, "peekViewEditor.background", self.darker_bg)?;
        write_scope(f, "peekViewEditor.matchHighlightBackground", Rgba { rgb: self.secondary_accent1, a: 0x77 })?;
        write_scope(f, "peekViewResult.background", self.darker_bg)?;
        write_scope(f, "peekViewResult.fileForeground", self.fg)?;
        write_scope(f, "peekViewResult.lineForeground", Rgba { rgb: self.fg, a: 0xBB })?;
        write_scope(f, "peekViewResult.matchHighlightBackground", Rgba { rgb: self.secondary_accent1, a: 0x77 })?;
        write_scope(f, "peekViewResult.selectionBackground", Rgba { rgb: self.primary_accent, a: 0x33 })?;
        write_scope(f, "peekViewResult.selectionForeground", self.fg)?;
        write_scope(f, "peekViewTitle.background", self.darker_bg)?;
        write_scope(f, "peekViewTitleDescription.foreground", Rgba { rgb: self.primary_accent, a: 0x55 })?;
        write_scope(f, "peekViewTitleLabel.foreground", self.fg)?;
        write_scope(f, "problemsErrorIcon.foreground", self.red)?;
        write_scope(f, "problemsWarningIcon.foreground", self.orange)?;
        write_scope(f, "rust_analyzer.inlayHint", Rgba { rgb: self.primary_accent, a: 0x55 })?;
        write_scope(f, "scrollbarSlider.background", Rgba { rgb: self.primary_accent, a: 0x22 })?;
        write_scope(f, "selection.background", Rgba { rgb: self.primary_accent, a: 0x44 })?;
        write_scope(f, "settings.headerForeground", self.fg)?;
        write_scope(f, "sideBar.background", self.darker_bg)?;
        write_scope(f, "sideBar.border", Rgba { rgb: self.primary_accent, a: 0x14 })?;
        write_scope(f, "sideBarSectionHeader.background", Rgba { rgb: self.primary_accent, a: 0x11 })?;
        write_scope(f, "statusBar.background", self.darker_bg)?;
        write_scope(f, "statusBar.border", Rgba { rgb: self.primary_accent, a: 0x14 })?;
        write_scope(f, "statusBar.debuggingBackground", self.darker_bg)?;
        write_scope(f, "statusBar.foreground", Rgba { rgb: self.primary_accent, a: 0x55 })?;
        write_scope(f, "statusBar.noFolderBackground", self.darker_bg)?;
        write_scope(f, "tab.activeBackground", self.bg)?;
        write_scope(f, "tab.activeForeground", self.fg)?;
        write_scope(f, "tab.border", Rgba { rgb: self.primary_accent, a: 0x14 })?;
        write_scope(f, "tab.inactiveBackground", self.darker_bg)?;
        write_scope(f, "tab.inactiveForeground", Rgba { rgb: self.primary_accent, a: 0x55 })?;
        write_scope(f, "terminal.ansiBlack", self.bg)?;
        write_scope(f, "terminal.ansiBlue", self.secondary_accent1)?;
        write_scope(f, "terminal.ansiBrightBlack", self.faded)?;
        write_scope(f, "terminal.ansiBrightBlue", self.secondary_accent1)?;
        write_scope(f, "terminal.ansiBrightCyan", self.primary_accent)?;
        write_scope(f, "terminal.ansiBrightGreen", self.green)?;
        write_scope(f, "terminal.ansiBrightMagenta", self.secondary_accent2)?;
        write_scope(f, "terminal.ansiBrightRed", self.red)?;
        write_scope(f, "terminal.ansiBrightWhite", self.fg)?;
        write_scope(f, "terminal.ansiBrightYellow", self.orange)?;
        write_scope(f, "terminal.ansiCyan", self.primary_accent)?;
        write_scope(f, "terminal.ansiGreen", self.green)?;
        write_scope(f, "terminal.ansiMagenta", self.secondary_accent2)?;
        write_scope(f, "terminal.ansiRed", self.red)?;
        write_scope(f, "terminal.ansiWhite", self.fg)?;
        write_scope(f, "terminal.ansiYellow", self.orange)?;
        write_scope(f, "terminal.foreground", self.fg)?;
        write_scope(f, "textLink.activeForeground", self.secondary_accent1)?;
        write_scope(f, "textLink.foreground", self.secondary_accent1)?;
        write_scope(f, "textPreformat.foreground", self.secondary_accent2)?;
        write_scope(f, "titleBar.activeBackground", self.darker_bg)?;
        write_scope(f, "titleBar.inactiveBackground", self.darker_bg)?;
        write_scope(f, "window.activeBorder", Rgba { rgb: self.primary_accent, a: 0x14 })?;

        writeln!(f, "\t}}")?;

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
    color: Color,
    font_style: Option<FontStyle>,
}

impl<C: Into<Color>> From<C> for Style {
    fn from(color: C) -> Self {
        Self {
            color: color.into(),
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

enum Color {
    Rgba(Rgba),
    Rgb(Rgb),
}

impl From<Rgba> for Color {
    fn from(rgba: Rgba) -> Self {
        Self::Rgba(rgba)
    }
}

impl From<Rgb> for Color {
    fn from(rgb: Rgb) -> Self {
        Self::Rgb(rgb)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rgba(rgba) => write!(f, "{}", rgba),
            Self::Rgb(rgb) => write!(f, "{}", rgb),
        }
    }
}

struct Rgba {
    rgb: Rgb,
    a: u8,
}

impl fmt::Display for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"#{:06X}{:02X}\"", self.rgb.0, self.a)
    }
}

#[derive(Copy, Clone)]
struct Rgb(u32);

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"#{:06X}\"", self.0)
    }
}
