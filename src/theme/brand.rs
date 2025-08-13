use crate::Assets;
use crate::resources;
use crate::Context;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct BrandResources {
    pub wordmark: resources::Image,
    pub logomark: resources::Image,
    pub app_icon: resources::Image,
    pub illustrations: Illustrations
}

impl BrandResources {
    pub const QUALITY: f32 = 8.0;

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
            logomark: assets.add_svg(&assets.load_file("brand/std_logomark.svg").unwrap(), Self::QUALITY),
            wordmark: assets.add_svg(&assets.load_file("brand/std_wordmark.svg").unwrap(), Self::QUALITY),
            app_icon: assets.add_svg(&assets.load_file("brand/std_app_icon.svg").unwrap(), Self::QUALITY),
            illustrations: Illustrations::default(assets),
        }
    }

    // TODO: Redo all this customization.
    pub fn set_wordmark(&mut self, ctx: &mut Context, path: &'static str) {
        let mut mark: Option<resources::Image> = None;
        if path.ends_with(".png") { mark = Some(ctx.assets.add_image(image::load_from_memory(&ctx.assets.load_file(path).unwrap()).unwrap().into()))}
        if path.ends_with(".svg") { mark = Some(ctx.assets.add_svg(&ctx.assets.load_file(path).unwrap(), Self::QUALITY)) }
        if let Some(wordmark) = mark { self.wordmark = wordmark; }
    }

    pub fn set_app_icon(&mut self, ctx: &mut Context, path: &'static str) {
        let mut icon: Option<resources::Image> = None;
        if path.ends_with(".png") { icon = Some(ctx.assets.add_image(image::load_from_memory(&ctx.assets.load_file(path).unwrap()).unwrap().into()))}
        if path.ends_with(".svg") { icon = Some(ctx.assets.add_svg(&ctx.assets.load_file(path).unwrap(), Self::QUALITY)) }
        if let Some(app_icon) = icon { self.app_icon = app_icon; }
    }

    pub fn set_logomark(&mut self, ctx: &mut Context, path: &'static str) {
        let mut icon: Option<resources::Image> = None;
        if path.ends_with(".png") { icon = Some(ctx.assets.add_image(image::load_from_memory(&ctx.assets.load_file(path).unwrap()).unwrap().into()))}
        if path.ends_with(".svg") { icon = Some(ctx.assets.add_svg(&ctx.assets.load_file(path).unwrap(), Self::QUALITY)) }
        if let Some(logomark) = icon { self.logomark = logomark; }
    }

    pub fn set_error(&mut self, ctx: &mut Context, path: &'static str) {
        self.illustrations.insert(ctx, "error", path);
    }
}

#[derive(Clone, Debug)]
pub struct Illustrations(pub HashMap<String, resources::Image>);

impl Illustrations {
    pub const QUALITY: f32 = 8.0;

    pub fn default(assets: &mut Assets) -> Self {
        let mut illustrations = HashMap::new();

        illustrations.insert("error".to_string(), assets.add_svg(&assets.load_file("brand/std_logomark.svg").unwrap(), Self::QUALITY));
        illustrations.insert("dodo".to_string(), assets.add_svg(&assets.load_file("brand/illustrations/dodo.svg").unwrap(), Self::QUALITY));
        illustrations.insert("hummingbird".to_string(), assets.add_svg(&assets.load_file("brand/illustrations/hummingbird.svg").unwrap(), Self::QUALITY));
        illustrations.insert("toucan".to_string(), assets.add_svg(&assets.load_file("brand/illustrations/toucan.svg").unwrap(), Self::QUALITY));
        illustrations.insert("emu".to_string(), assets.add_svg(&assets.load_file("brand/illustrations/emu.svg").unwrap(), Self::QUALITY));

        Illustrations(illustrations)
    }

    pub fn get(&self, name: &str) -> Option<resources::Image> {
        self.0.get(name).cloned()
    }

    pub fn remove(&mut self, name: &str)  {
        self.0.remove(name);
    }

    pub fn insert(&mut self, ctx: &mut Context, name: &str, path: &str) {
        let mut illustration: Option<resources::Image> = None;
        if path.ends_with(".png") { illustration = Some(ctx.assets.add_image(image::load_from_memory(&ctx.assets.load_file(path).unwrap()).unwrap().into()))}
        if path.ends_with(".svg") { illustration = Some(ctx.assets.add_svg(&ctx.assets.load_file(path).unwrap(), 2.0)) }

        if let Some(i) = illustration {self.0.insert(name.to_string(), i);}
    }
}
