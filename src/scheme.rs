use tincture::Oklch;

pub(crate) trait Scheme {
    fn base(&self, scale: BaseScale) -> Oklch;
    fn keyword(&self) -> Oklch;
    fn function(&self) -> Oklch;
    fn ty(&self) -> Oklch;
}

pub(crate) enum BaseScale {
    DarkBg,
    Bg,
    Faded,
    Fg,
}
