use super::oklch;
use crate::scheme::{BaseScale, Scheme};
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
    fn base(&self, scale: BaseScale) -> tincture::Oklch {
        match scale {
            BaseScale::DarkBg => oklch(0.2840086, 0.009262296, 219.91037),
            BaseScale::Bg => oklch(0.3107568, 0.012263376, 222.50056),
            BaseScale::Faded => oklch(0.5093168, 0.030170973, 208.38818),
            BaseScale::Fg => oklch(0.9386827, 0.018355545, 218.56363),
        }
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
}
