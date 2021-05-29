use crate::scheme::{BaseScale, Scheme};
use mottle::style::FontStyle;
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, scheme: &impl Scheme) {
    workspace_colors(builder, scheme);
    syntax_highlighting(builder, scheme);
}

fn workspace_colors(builder: &mut ThemeBuilder, scheme: &impl Scheme) {
    builder.add_workspace_rule("editor.background", scheme.base(BaseScale::Bg));
    builder.add_workspace_rules(&["editor.foreground", "foreground"], scheme.base(BaseScale::Fg));

    builder.add_workspace_rule("editor.lineHighlightBackground", scheme.base(BaseScale::LightBg));

    builder.add_workspace_rule("editor.selectionBackground", scheme.base(BaseScale::BrightBg));

    builder.add_workspace_rule("editorLineNumber.foreground", scheme.base(BaseScale::FadedFg));
    builder.add_workspace_rule("editorLineNumber.activeForeground", scheme.base(BaseScale::Fg));

    builder.add_workspace_rule("editorWidget.background", scheme.base(BaseScale::DarkBg));

    builder.add_workspace_rule("list.hoverBackground", scheme.base(BaseScale::Bg));
    builder.add_workspace_rules(
        &["list.focusBackground", "list.activeSelectionBackground", "list.inactiveSelectionBackground"],
        scheme.base(BaseScale::LightBg),
    );
    builder.add_workspace_rule("list.activeSelectionForeground", scheme.base(BaseScale::Fg));
    builder.add_workspace_rule("list.highlightForeground", scheme.accent());

    builder.add_workspace_rules(&["activityBar.background", "sideBar.background"], scheme.base(BaseScale::DarkBg));
    builder.add_workspace_rule("activityBar.foreground", scheme.base(BaseScale::Fg));
    builder.add_workspace_rule("activityBar.inactiveForeground", scheme.base(BaseScale::FadedFg));

    builder.add_workspace_rules(&["tab.inactiveBackground", "tab.border"], scheme.base(BaseScale::DarkBg));
    builder.add_workspace_rule("tab.activeForeground", scheme.base(BaseScale::Fg));
    builder.add_workspace_rule("tab.inactiveForeground", scheme.base(BaseScale::FadedFg));
    builder.add_workspace_rules(&["editorGroupHeader.tabsBackground", "editorGroupHeader.noTabsBackground"], scheme.base(BaseScale::DarkBg));

    builder.add_workspace_rules(
        &["statusBar.background", "statusBar.debuggingBackground", "statusBar.noFolderBackground"],
        scheme.base(BaseScale::DarkBg),
    );
    builder.add_workspace_rules(
        &["statusBar.foreground", "statusBar.debuggingForeground", "statusBar.noFolderForeground"],
        scheme.base(BaseScale::FadedFg),
    );

    builder.add_workspace_rules(&["editorIndentGuide.background", "tree.indentGuidesStroke"], scheme.base(BaseScale::BrightBg));
    builder.add_workspace_rule("editorIndentGuide.activeBackground", scheme.base(BaseScale::FadedFg));

    builder.add_workspace_rule("editorCodeLens.foreground", scheme.base(BaseScale::FadedFg));

    builder.add_workspace_rules(&["editorCursor.foreground", "terminalCursor.foreground"], scheme.base(BaseScale::Fg));
    builder.add_workspace_rules(&["editorCursor.background", "terminalCursor.background"], scheme.base(BaseScale::Bg));

    builder.add_workspace_rule("focusBorder", scheme.base(BaseScale::BrightBg));

    builder.add_workspace_rule("rust_analyzer.inlayHints.foreground", scheme.base(BaseScale::FadedFg));
}

fn syntax_highlighting(builder: &mut ThemeBuilder, scheme: &impl Scheme) {
    builder.add_rules(
        &[
            Semantic("keyword"),
            Textmate("keyword"),
            Textmate("storage"),
            Textmate("variable.language.this"),
            Textmate("storage.type.primitive"),
            Textmate("storage.type.local.java"),
            Textmate("storage.type.js"),
            Textmate("storage.type.rust"),
            Textmate("storage.type.class.js"),
        ],
        scheme.keyword(),
    );

    builder.add_rules(&[Semantic("function"), Semantic("method"), Textmate("entity.name.function")], scheme.function());

    builder.add_rules(
        &[
            Semantic("type"),
            Semantic("class"),
            Semantic("struct"),
            Semantic("enum"),
            Semantic("union"),
            Semantic("interface"),
            Semantic("typeParameter"),
            Semantic("typeAlias"),
            Semantic("builtinType"),
            Textmate("entity.name.type"),
            Textmate("storage.type"),
        ],
        scheme.ty(),
    );
    builder.add_rule(Semantic("enumMember"), (scheme.ty(), FontStyle::Italic));

    builder.add_rules(
        &[
            Semantic("string"),
            Textmate("string"),
            Semantic("number"),
            Textmate("constant.numeric"),
            Semantic("characterLiteral"),
            Semantic("boolean"),
            Textmate("constant.language"),
        ],
        scheme.data(),
    );

    builder.add_rule(Semantic("comment"), (scheme.base(BaseScale::FadedFg), FontStyle::Italic));

    builder.add_rule(Textmate("keyword.operator"), scheme.base(BaseScale::Fg));
}
