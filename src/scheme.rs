use crate::utils::{lerp, oklch};
use tincture::Oklch;

pub(crate) trait Scheme {
    const BASE_SCALE_HUE: f32;

    fn base(&self, scale: BaseScale) -> Oklch {
        let value = match scale {
            BaseScale::DarkBg => 0.0,
            BaseScale::Bg => 0.03,
            BaseScale::LightBg => 0.07,
            BaseScale::BrightBg => 0.3,
            BaseScale::FadedFg => 0.5,
            BaseScale::Fg => 1.0,
        };

        let lightness = lerp(value, 0.24..0.85);

        let chroma = match scale {
            BaseScale::Bg | BaseScale::DarkBg => 0.005,
            _ => lerp(value, 0.01..0.015),
        };

        oklch(lightness, chroma, Self::BASE_SCALE_HUE)
    }

    fn accent(&self) -> Oklch;

    fn keyword(&self) -> Oklch;
    fn function(&self) -> Oklch;
    fn ty(&self) -> Oklch;
    fn data(&self) -> Oklch;
}

pub(crate) enum BaseScale {
    DarkBg,
    Bg,
    LightBg,
    BrightBg,
    FadedFg,
    Fg,
}
