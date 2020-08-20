use std::{fmt, fs, io};

fn main() -> io::Result<()> {
    let path = format!("themes/{}-color-theme.json", NOEL.name);
    let json = NOEL.to_string();

    fs::write(path, json.as_bytes())
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

const INVISIBLE: Color = Color { rgb: Rgb(0x000000), a: Some(0x00) };

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
        write_scope(f, "activityBar.inactiveForeground", (self.primary_accent, 0x44))?;
        write_scope(f, "activityBarBadge.background", self.primary_accent)?;
        write_scope(f, "activityBarBadge.foreground", self.darker_bg)?;
        write_scope(f, "badge.background", (self.primary_accent, 0x33))?;
        write_scope(f, "debugToolBar.background", self.darker_bg)?;
        write_scope(f, "diffEditor.insertedTextBackground", (self.green, 0x22))?;
        write_scope(f, "diffEditor.removedTextBackground", (self.red, 0x22))?;
        write_scope(f, "dropdown.background", self.darker_bg)?;
        write_scope(f, "dropdown.foreground", self.fg)?;
        write_scope(f, "editor.background", self.bg)?;
        write_scope(f, "editor.findMatchBackground", (self.secondary_accent1, 0x77))?;
        write_scope(f, "editor.findMatchHighlightBackground", (self.secondary_accent1, 0x44))?;
        write_scope(f, "editor.foreground", self.fg)?;
        write_scope(f, "editor.lineHighlightBackground", (self.primary_accent, 0x0D))?;
        write_scope(f, "editor.rangeHighlightBackground", (self.primary_accent, 0x22))?;
        write_scope(f, "editor.selectionBackground", (self.primary_accent, 0x33))?;
        write_scope(f, "editor.wordHighlightBackground", (self.primary_accent, 0x33))?;
        write_scope(f, "editorCodeLens.foreground", (self.primary_accent, 0x55))?;
        write_scope(f, "editorCursor.foreground", self.fg)?;
        write_scope(f, "editorError.foreground", self.red)?;
        write_scope(f, "editorGroup.border", (self.primary_accent, 0x14))?;
        write_scope(f, "editorGroupHeader.noTabsBackground", self.darker_bg)?;
        write_scope(f, "editorGroupHeader.tabsBackground", self.darker_bg)?;
        write_scope(f, "editorGutter.addedBackground", self.green)?;
        write_scope(f, "editorGutter.deletedBackground", self.red)?;
        write_scope(f, "editorGutter.modifiedBackground", self.orange)?;
        write_scope(f, "editorHoverWidget.background", self.darker_bg)?;
        write_scope(f, "editorIndentGuide.activeBackground", (self.primary_accent, 0x22))?;
        write_scope(f, "editorIndentGuide.background", (self.primary_accent, 0x11))?;
        write_scope(f, "editorLineNumber.activeForeground", self.fg)?;
        write_scope(f, "editorLineNumber.foreground", (self.primary_accent, 0x55))?;
        write_scope(f, "editorLink.activeForeground", self.secondary_accent1)?;
        write_scope(f, "editorOverviewRuler.addedForeground", self.green)?;
        write_scope(f, "editorOverviewRuler.deletedForeground", self.red)?;
        write_scope(f, "editorOverviewRuler.errorForeground", self.red)?;
        write_scope(f, "editorOverviewRuler.findMatchForeground", (self.secondary_accent1, 0x77))?;
        write_scope(f, "editorOverviewRuler.modifiedForeground", self.orange)?;
        write_scope(f, "editorOverviewRuler.rangeHighlightForeground", (self.primary_accent, 0x22))?;
        write_scope(f, "editorOverviewRuler.warningForeground", self.orange)?;
        write_scope(f, "editorRuler.foreground", (self.primary_accent, 0x14))?;
        write_scope(f, "editorWarning.foreground", self.orange)?;
        write_scope(f, "editorWhitespace.foreground", (self.primary_accent, 0x22))?;
        write_scope(f, "editorWidget.background", self.darker_bg)?;
        write_scope(f, "editorWidget.border", (self.primary_accent, 0x14))?;
        write_scope(f, "errorForeground", self.red)?;
        write_scope(f, "focusBorder", INVISIBLE)?;
        write_scope(f, "foreground", self.fg)?;
        write_scope(f, "gitDecoration.addedResourceForeground", self.green)?;
        write_scope(f, "gitDecoration.deletedResourceForeground", self.red)?;
        write_scope(f, "gitDecoration.modifiedResourceForeground", self.orange)?;
        write_scope(f, "input.background", (self.primary_accent, 0x11))?;
        write_scope(f, "input.placeholderForeground", (self.primary_accent, 0x55))?;
        write_scope(f, "list.activeSelectionBackground", (self.primary_accent, 0x33))?;
        write_scope(f, "list.activeSelectionForeground", self.fg)?;
        write_scope(f, "list.errorForeground", self.red)?;
        write_scope(f, "list.focusBackground", (self.primary_accent, 0x22))?;
        write_scope(f, "list.highlightForeground", self.secondary_accent1)?;
        write_scope(f, "list.hoverBackground", (self.primary_accent, 0x11))?;
        write_scope(f, "list.inactiveSelectionBackground", (self.primary_accent, 0x22))?;
        write_scope(f, "list.warningForeground", self.orange)?;
        write_scope(f, "menu.background", self.darker_bg)?;
        write_scope(f, "minimap.errorHighlight", self.red)?;
        write_scope(f, "minimap.findMatchHighlight", (self.secondary_accent1, 0x77))?;
        write_scope(f, "minimap.selectionHighlight", (self.primary_accent, 0x11))?;
        write_scope(f, "minimap.warningHighlight", self.orange)?;
        write_scope(f, "minimapGutter.addedBackground", self.green)?;
        write_scope(f, "minimapGutter.deletedBackground", self.red)?;
        write_scope(f, "minimapGutter.modifiedBackground", self.orange)?;
        write_scope(f, "minimapSlider.activeBackground", (self.primary_accent, 0x33))?;
        write_scope(f, "minimapSlider.background", (self.primary_accent, 0x11))?;
        write_scope(f, "minimapSlider.hoverBackground", (self.primary_accent, 0x22))?;
        write_scope(f, "panel.border", (self.primary_accent, 0x14))?;
        write_scope(f, "panelTitle.activeForeground", self.fg)?;
        write_scope(f, "panelTitle.inactiveForeground", (self.primary_accent, 0x55))?;
        write_scope(f, "peekView.border", self.darker_bg)?;
        write_scope(f, "peekViewEditor.background", self.darker_bg)?;
        write_scope(f, "peekViewEditor.matchHighlightBackground", (self.secondary_accent1, 0x77))?;
        write_scope(f, "peekViewResult.background", self.darker_bg)?;
        write_scope(f, "peekViewResult.fileForeground", self.fg)?;
        write_scope(f, "peekViewResult.lineForeground", (self.fg, 0xBB))?;
        write_scope(f, "peekViewResult.matchHighlightBackground", (self.secondary_accent1, 0x77))?;
        write_scope(f, "peekViewResult.selectionBackground", (self.primary_accent, 0x33))?;
        write_scope(f, "peekViewResult.selectionForeground", self.fg)?;
        write_scope(f, "peekViewTitle.background", self.darker_bg)?;
        write_scope(f, "peekViewTitleDescription.foreground", (self.primary_accent, 0x55))?;
        write_scope(f, "peekViewTitleLabel.foreground", self.fg)?;
        write_scope(f, "problemsErrorIcon.foreground", self.red)?;
        write_scope(f, "problemsWarningIcon.foreground", self.orange)?;
        write_scope(f, "rust_analyzer.inlayHint", (self.primary_accent, 0x55))?;
        write_scope(f, "scrollbarSlider.background", (self.primary_accent, 0x22))?;
        write_scope(f, "selection.background", (self.primary_accent, 0x44))?;
        write_scope(f, "settings.headerForeground", self.fg)?;
        write_scope(f, "sideBar.background", self.darker_bg)?;
        write_scope(f, "sideBar.border", (self.primary_accent, 0x14))?;
        write_scope(f, "sideBarSectionHeader.background", (self.primary_accent, 0x11))?;
        write_scope(f, "statusBar.background", self.darker_bg)?;
        write_scope(f, "statusBar.border", (self.primary_accent, 0x14))?;
        write_scope(f, "statusBar.debuggingBackground", self.darker_bg)?;
        write_scope(f, "statusBar.foreground", (self.primary_accent, 0x55))?;
        write_scope(f, "statusBar.noFolderBackground", self.darker_bg)?;
        write_scope(f, "tab.activeBackground", self.bg)?;
        write_scope(f, "tab.activeForeground", self.fg)?;
        write_scope(f, "tab.border", (self.primary_accent, 0x14))?;
        write_scope(f, "tab.inactiveBackground", self.darker_bg)?;
        write_scope(f, "tab.inactiveForeground", (self.primary_accent, 0x55))?;
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
        write_scope(f, "window.activeBorder", (self.primary_accent, 0x14))?;

        writeln!(f, "\t}},")?;

        Ok(())
    }

    fn semantic_highlighting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\t\"semanticHighlighting\": true,")?;
        writeln!(f, "\t\"semanticTokenColors\": {{")?;

        write_scope(f, "keyword", self.secondary_accent2)?;

        write_scope(f, "boolean", self.secondary_accent2)?;

        write_scope(f, "variable", self.fg)?;
        write_scope(f, "property", self.fg)?;
        write_scope(f, "parameter", self.fg)?;

        write_scope(f, "type", self.primary_accent)?;
        write_scope(f, "class", self.primary_accent)?;
        write_scope(f, "struct", self.primary_accent)?;
        write_scope(f, "enum", self.primary_accent)?;
        write_scope(
            f,
            "enumMember",
            Style {
                color: self.primary_accent.into(),
                font_style: Some(FontStyle { italic: true, bold: false }),
            },
        )?;
        write_scope(f, "interface", self.primary_accent)?;
        write_scope(f, "typeAlias", self.primary_accent)?;
        write_scope(f, "typeParameter", self.primary_accent)?;
        write_scope(f, "builtinType", self.primary_accent)?;

        write_scope(f, "function", self.secondary_accent1)?;

        write_scope(f, "namespace", self.fg)?;

        write_scope(f, "number", self.secondary_accent1)?;
        write_scope(f, "string", self.secondary_accent1)?;

        write_scope(
            f,
            "comment",
            Style {
                color: self.faded.into(),
                font_style: Some(FontStyle { italic: true, bold: false }),
            },
        )?;

        write_scope(f, "punctuation", (self.fg, 0xBB))?;
        write_scope(f, "operator", (self.fg, 0xBB))?;

        writeln!(f, "\t}},")?;

        Ok(())
    }

    fn textmate_highlighting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\t\"tokenColors\": [")?;

        write_textmate(
            f,
            &["keyword", "keyword.control", "storage.modifier", "keyword.other.using", "constant.language"],
            self.secondary_accent2,
        )?;

        write_textmate(
            f,
            &["variable", "entity.name.variable", "support.type.property-name", "punctuation.support.type.property-name"],
            self.fg,
        )?;

        write_textmate(f, &["entity.name.function"], self.secondary_accent1)?;

        write_textmate(f, &["entity.name.type", "keyword.type"], self.primary_accent)?;

        write_textmate(
            f,
            &["entity.name.variable.enum-member"],
            Style {
                color: self.primary_accent.into(),
                font_style: Some(FontStyle { italic: true, bold: false }),
            },
        )?;

        write_textmate(f, &["constant.numeric", "string", "punctuation.definition.string"], self.secondary_accent1)?;
        write_textmate(f, &["entity.name.type.namespace"], self.fg)?;
        write_textmate(f, &["punctuation", "keyword.operator"], (self.fg, 0xBB))?;

        write_textmate(
            f,
            &["comment", "punctuation.definition.comment"],
            Style {
                color: self.faded.into(),
                font_style: Some(FontStyle { italic: true, bold: false }),
            },
        )?;

        writeln!(f, "\t]")?;

        Ok(())
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{")?;

        writeln!(f, "\t\"name\": \"{}\",", self.name)?;
        writeln!(f, "\t\"type\": \"dark\",")?;

        self.workspace_colors(f)?;
        self.semantic_highlighting(f)?;
        self.textmate_highlighting(f)?;

        writeln!(f, "}}")?;

        Ok(())
    }
}

fn write_scope(f: &mut fmt::Formatter<'_>, key: &str, style: impl Into<Style>) -> fmt::Result {
    writeln!(f, "\t\t\"{}\": {},", key, style.into())
}

fn write_textmate(f: &mut fmt::Formatter<'_>, scopes: &[&str], style: impl Into<Style>) -> fmt::Result {
    writeln!(f, "\t\t{{")?;

    writeln!(f, "\t\t\t\"scope\": [")?;
    for scope in scopes {
        writeln!(f, "\t\t\t\t\"{}\",", scope)?;
    }
    writeln!(f, "\t\t\t],")?;

    writeln!(f, "\t\t\t\"settings\": {:#}", style.into())?;

    writeln!(f, "\t\t}},")?;

    Ok(())
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
        if self.font_style.is_some() || f.alternate() {
            writeln!(f, "{{")?;
            write!(f, "\t\t\t\"foreground\": {}", self.color)?;

            if let Some(font_style) = self.font_style {
                writeln!(f, ",")?;
                writeln!(f, "\t\t\t\"fontStyle\": {}", font_style)?;
            } else {
                writeln!(f)?;
            }

            write!(f, "\t\t\t}}")?;

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

struct Color {
    rgb: Rgb,
    a: Option<u8>,
}

impl From<Rgb> for Color {
    fn from(rgb: Rgb) -> Self {
        Self { rgb, a: None }
    }
}

impl From<(Rgb, u8)> for Color {
    fn from((rgb, a): (Rgb, u8)) -> Self {
        Self { rgb, a: Some(a) }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(a) = self.a {
            write!(f, "\"#{:06X}{:02X}\"", self.rgb.0, a)
        } else {
            write!(f, "{}", self.rgb)
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
