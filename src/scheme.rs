use crate::utils::oklch;
use tincture::Oklch;

pub(crate) trait Scheme {
    const BASE_SCALE_HUE: f32;

    fn base(&self, scale: BaseScale) -> Oklch {
        oklch(self.base_lightness(scale), self.base_chroma(scale), Self::BASE_SCALE_HUE)
    }
    fn base_lightness(&self, scale: BaseScale) -> f32;
    fn base_chroma(&self, scale: BaseScale) -> f32;

    fn strong_accent(&self) -> Oklch;
    fn light_accent(&self) -> Oklch;

    fn keyword(&self) -> Oklch;
    fn function(&self) -> Oklch;
    fn ty(&self) -> Oklch;
    fn data(&self) -> Oklch;

    fn ansi_red(&self) -> Oklch;
    fn ansi_green(&self) -> Oklch;
    fn ansi_yellow(&self) -> Oklch;
    fn ansi_blue(&self) -> Oklch;
    fn ansi_magenta(&self) -> Oklch;
    fn ansi_cyan(&self) -> Oklch;

    fn added(&self) -> Oklch;
    fn modified(&self) -> Oklch;
    fn deleted(&self) -> Oklch;
}

#[derive(Clone, Copy)]
pub(crate) enum BaseScale {
    DarkBg,
    Bg,
    LightBg,
    BrightBg,
    BrighterBg,
    FadedFg,
    Fg,
}

impl BaseScale {
    pub(crate) fn value(self) -> f32 {
        match self {
            Self::DarkBg => 0.0,
            Self::Bg => 0.03,
            Self::LightBg => 0.07,
            Self::BrightBg => 0.2,
            Self::BrighterBg => 0.3,
            Self::FadedFg => 0.5,
            Self::Fg => 1.0,
        }
    }
}
