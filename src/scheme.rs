use crate::utils::{lerp, oklch};
use tincture::Oklch;

pub(crate) trait Scheme {
    const BASE_SCALE_HUE: f32;

    fn base(&self, scale: BaseScale) -> Oklch {
        let value = match scale {
            BaseScale::DarkBg => 0.0,
            BaseScale::Bg => 0.05,
            BaseScale::Faded => 0.5,
            BaseScale::Fg => 1.0,
        };

        oklch(lerp(value, 0.28..0.9), lerp(value, 0.01..0.018), Self::BASE_SCALE_HUE)
    }

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
