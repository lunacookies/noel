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
}

fn syntax_highlighting(builder: &mut ThemeBuilder, scheme: &impl Scheme) {
    builder.add_rule(Semantic("keyword"), scheme.keyword());

    builder.add_rules(&[Semantic("function"), Semantic("method")], scheme.function());

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
        ],
        scheme.ty(),
    );
    builder.add_rule(Semantic("enumMember"), (scheme.ty(), FontStyle::Italic));

    builder.add_rule(Semantic("comment"), (scheme.base(BaseScale::Faded), FontStyle::Italic));
}
