use crate::scheme::BaseScale;
use crate::scheme::Scheme;
use crate::utils::lerp;
use crate::utils::oklch;
use tincture::Oklch;

pub(crate) struct Yuru;

impl Yuru {
    fn red(&self) -> Oklch {
        oklch(0.68488437, 0.09384193, 8.52984)
    }

    fn brown(&self) -> Oklch {
        oklch(0.7924896, 0.025996776, 81.10153)
    }

    fn cyan(&self) -> Oklch {
        oklch(0.81816167, 0.07638612, 187.41963)
    }
}

impl Scheme for Yuru {
    const BASE_SCALE_HUE: f32 = 15.0;

    fn base_lightness(&self, scale: BaseScale) -> f32 {
        lerp(scale.value(), 0.27..0.9)
    }

    fn base_chroma(&self, scale: BaseScale) -> f32 {
        lerp(scale.value(), 0.006..0.015)
    }

    fn strong_accent(&self) -> Oklch {
        self.red()
    }

    fn light_accent(&self) -> Oklch {
        self.cyan()
    }

    fn keyword(&self) -> Oklch {
        self.red()
    }

    fn function(&self) -> Oklch {
        self.cyan()
    }

    fn ty(&self) -> Oklch {
        self.brown()
    }

    fn data(&self) -> Oklch {
        self.cyan()
    }
}
