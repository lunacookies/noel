use crate::scheme::BaseScale;
use crate::scheme::Scheme;
use crate::utils::lerp;
use crate::utils::oklch;
use tincture::Oklch;

pub(crate) struct Noel;

impl Noel {
    fn pink(&self) -> Oklch {
        oklch(0.8355564, 0.06198207, 8.320043)
    }

    fn light_blue(&self) -> Oklch {
        oklch(0.8672051, 0.043651294, 207.3837)
    }

    fn blue(&self) -> Oklch {
        oklch(0.7425923, 0.08841091, 219.68378)
    }
}

impl Scheme for Noel {
    const BASE_SCALE_HUE: f32 = 235.0;

    fn base_lightness(&self, scale: BaseScale) -> f32 {
        lerp(scale.value(), 0.3..0.9)
    }

    fn base_chroma(&self, scale: BaseScale) -> f32 {
        match scale {
            BaseScale::DarkBg | BaseScale::Bg | BaseScale::Fg => 0.012,
            _ => lerp(scale.value(), 0.015..0.04),
        }
    }

    fn strong_accent(&self) -> Oklch {
        self.blue()
    }

    fn light_accent(&self) -> Oklch {
        self.light_blue()
    }

    fn keyword(&self) -> Oklch {
        self.pink()
    }

    fn function(&self) -> Oklch {
        self.blue()
    }

    fn ty(&self) -> Oklch {
        self.light_blue()
    }

    fn data(&self) -> Oklch {
        self.blue()
    }
}
