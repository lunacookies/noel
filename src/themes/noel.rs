use crate::scheme::BaseScale;
use crate::scheme::Scheme;
use crate::utils::lerp;
use crate::utils::oklch;
use tincture::Oklch;

pub(crate) struct Noel;

impl Noel {
    fn pink(&self) -> Oklch {
        oklch(0.8, 0.06, 8.0)
    }

    fn red(&self) -> Oklch {
        oklch(0.75, 0.09, 35.0)
    }

    fn yellow(&self) -> Oklch {
        oklch(0.8, 0.06, 80.0)
    }

    fn green(&self) -> Oklch {
        oklch(0.75, 0.09, 140.0)
    }

    fn light_blue(&self) -> Oklch {
        oklch(0.85, 0.04, 205.0)
    }

    fn blue(&self) -> Oklch {
        oklch(0.75, 0.09, 220.0)
    }
}

impl Scheme for Noel {
    const BASE_SCALE_HUE: f32 = 235.0;

    fn base_lightness(&self, scale: BaseScale) -> f32 {
        lerp(scale.value(), 0.3..0.9)
    }

    fn base_chroma(&self, scale: BaseScale) -> f32 {
        match scale {
            BaseScale::DarkBg | BaseScale::Bg => 0.013,
            BaseScale::Fg => 0.02,
            _ => lerp(scale.value(), 0.014..0.04),
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

    fn ansi_red(&self) -> Oklch {
        self.red()
    }

    fn ansi_green(&self) -> Oklch {
        self.green()
    }

    fn ansi_yellow(&self) -> Oklch {
        self.yellow()
    }

    fn ansi_blue(&self) -> Oklch {
        self.blue()
    }

    fn ansi_magenta(&self) -> Oklch {
        self.pink()
    }

    fn ansi_cyan(&self) -> Oklch {
        self.light_blue()
    }

    fn added(&self) -> Oklch {
        self.green()
    }

    fn modified(&self) -> Oklch {
        self.yellow()
    }

    fn deleted(&self) -> Oklch {
        self.red()
    }
}
