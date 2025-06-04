use crate::Assets;
use crate::resources;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct BrandResources {
    pub wordmark: resources::Image,
    pub logomark: resources::Image,
    pub app_icon: resources::Image,
    pub illustrations: Illustrations
}

impl BrandResources {
    pub fn new(
        logomark: resources::Image, 
        wordmark: resources::Image,
        app_icon: resources::Image,
        illustrations: Illustrations
    ) -> Self {
        BrandResources { logomark, wordmark, app_icon, illustrations }
    }

    pub fn default(assets: &mut Assets) -> Self {
        BrandResources {
            logomark: assets.add_svg(&assets.load_file("brand/logomark.svg").unwrap(), 8.0),
            wordmark: assets.add_svg(&assets.load_file("brand/wordmark.svg").unwrap(), 8.0),
            app_icon: assets.add_svg(&assets.load_file("brand/app_icon.svg").unwrap(), 8.0),
            illustrations: Illustrations::default(assets),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Illustrations(HashMap<&'static str, resources::Image>);

impl Illustrations {
    pub fn default(assets: &mut Assets) -> Self {
        let mut illustrations = HashMap::new();

        illustrations.insert("error", assets.add_svg(&assets.load_file("brand/illustrations/error.svg").unwrap(), 8.0));
        illustrations.insert("dodo", assets.add_svg(&assets.load_file("brand/illustrations/dodo.svg").unwrap(), 8.0));
        illustrations.insert("hummingbird", assets.add_svg(&assets.load_file("brand/illustrations/hummingbird.svg").unwrap(), 8.0));
        illustrations.insert("toucan", assets.add_svg(&assets.load_file("brand/illustrations/toucan.svg").unwrap(), 8.0));
        illustrations.insert("emu", assets.add_svg(&assets.load_file("brand/illustrations/emu.svg").unwrap(), 8.0));

        Illustrations(illustrations)
    }

    pub fn get(&self, name: &'static str) -> resources::Image {
        self.0.get(name).unwrap_or_else(|| panic!("Could not find illustration {:?}", name)).clone()
    }

    pub fn add_illustration(&mut self, name: &'static str, illustration: resources::Image) {
        if let Some(existing) = self.0.get_mut(&name) {
            *existing = illustration; 
        } else {
            self.0.insert(name, illustration);
        }
    }
}
