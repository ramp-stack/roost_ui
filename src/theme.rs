use crate::Assets;

pub mod colors;
pub use colors::*;
pub mod fonts;
pub use fonts::*;
pub mod icons;
pub use icons::*;
pub mod brand;
pub use brand::*;
pub mod layout;
pub use layout::*;

pub struct Theme {
    pub colors: ColorResources,
    pub fonts: FontResources,
    pub icons: IconResources,
    pub brand: BrandResources,
    pub layout: LayoutResources,
}

impl Theme {
    pub fn default(ctx: &mut Assets) -> Self {
        Theme {
            colors: ColorResources::default(),
            fonts: FontResources::default(ctx),
            icons: IconResources::default(ctx),
            brand: BrandResources::default(ctx),
            layout: LayoutResources::default(),
        }
    }

    pub fn new(
        colors: ColorResources, 
        fonts: FontResources, 
        icons: IconResources,
        brand: BrandResources,
        layout: LayoutResources,
    ) -> Self { 
        Theme { colors, fonts, icons, brand, layout } 
    }
}
