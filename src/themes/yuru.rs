use super::oklch;
use crate::scheme::{BaseScale, Scheme};
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
    fn base(&self, scale: BaseScale) -> tincture::Oklch {
        match scale {
            BaseScale::DarkBg => oklch(0.25222087, 0.006081627, 17.378693),
            BaseScale::Bg => oklch(0.2688568, 0.0059769503, 17.330532),
            BaseScale::Faded => oklch(0.45792902, 0.013222907, 63.070595),
            BaseScale::Fg => oklch(0.8934077, 0.014947208, 80.69393),
        }
    }

    fn keyword(&self) -> tincture::Oklch {
        self.red()
    }

    fn function(&self) -> tincture::Oklch {
        self.cyan()
    }

    fn ty(&self) -> tincture::Oklch {
        self.brown()
    }
}
