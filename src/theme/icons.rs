use crate::Assets;
use crate::resources;
use crate::Context;

use std::collections::HashMap;

/// A collection of icons used throughout the application.
pub struct IconResources(HashMap<&'static str, resources::Image>);

impl IconResources {
    pub const QUALITY: f32 = 8.0;
    pub fn default(assets: &mut Assets) -> Self {
        let mut icons = HashMap::new();

        icons.insert("accounts", assets.add_svg(&assets.load_file("icons/accounts.svg").unwrap(), Self::QUALITY));
        icons.insert("add", assets.add_svg(&assets.load_file("icons/add.svg").unwrap(), Self::QUALITY));
        icons.insert("app_store", assets.add_svg(&assets.load_file("icons/app_store.svg").unwrap(), Self::QUALITY));
        icons.insert("back", assets.add_svg(&assets.load_file("icons/back.svg").unwrap(), Self::QUALITY));
        icons.insert("block", assets.add_svg(&assets.load_file("icons/block.svg").unwrap(), Self::QUALITY));
        icons.insert("unblock", assets.add_svg(&assets.load_file("icons/unblock.svg").unwrap(), Self::QUALITY));
        icons.insert("boot", assets.add_svg(&assets.load_file("icons/boot.svg").unwrap(), Self::QUALITY));
        icons.insert("unboot", assets.add_svg(&assets.load_file("icons/unboot.svg").unwrap(), Self::QUALITY));
        icons.insert("backspace", assets.add_svg(&assets.load_file("icons/backspace.svg").unwrap(), Self::QUALITY));
        icons.insert("bitcoin", assets.add_svg(&assets.load_file("icons/bitcoin.svg").unwrap(), Self::QUALITY));
        icons.insert("camera", assets.add_svg(&assets.load_file("icons/camera.svg").unwrap(), Self::QUALITY));
        icons.insert("cancel", assets.add_svg(&assets.load_file("icons/cancel.svg").unwrap(), Self::QUALITY));
        icons.insert("capslock", assets.add_svg(&assets.load_file("icons/capslock.svg").unwrap(), Self::QUALITY));
        icons.insert("capslock_on", assets.add_svg(&assets.load_file("icons/capslock_on.svg").unwrap(), Self::QUALITY));
        icons.insert("checkmark", assets.add_svg(&assets.load_file("icons/checkmark.svg").unwrap(), Self::QUALITY));
        icons.insert("close", assets.add_svg(&assets.load_file("icons/close.svg").unwrap(), Self::QUALITY));
        icons.insert("copy", assets.add_svg(&assets.load_file("icons/copy.svg").unwrap(), Self::QUALITY));
        icons.insert("credential", assets.add_svg(&assets.load_file("icons/credential.svg").unwrap(), Self::QUALITY));
        icons.insert("down_arrow", assets.add_svg(&assets.load_file("icons/down_arrow.svg").unwrap(), Self::QUALITY));
        icons.insert("delete", assets.add_svg(&assets.load_file("icons/delete.svg").unwrap(), Self::QUALITY));
        icons.insert("discord", assets.add_svg(&assets.load_file("icons/discord.svg").unwrap(), Self::QUALITY));
        icons.insert("door", assets.add_svg(&assets.load_file("icons/door.svg").unwrap(), Self::QUALITY));
        icons.insert("down", assets.add_svg(&assets.load_file("icons/down.svg").unwrap(), Self::QUALITY));
        icons.insert("edit", assets.add_svg(&assets.load_file("icons/edit.svg").unwrap(), Self::QUALITY));
        icons.insert("emoji", assets.add_svg(&assets.load_file("icons/emoji.svg").unwrap(), Self::QUALITY));
        icons.insert("error", assets.add_svg(&assets.load_file("icons/error.svg").unwrap(), Self::QUALITY));
        icons.insert("explore", assets.add_svg(&assets.load_file("icons/explore.svg").unwrap(), Self::QUALITY));
        icons.insert("facebook", assets.add_svg(&assets.load_file("icons/facebook.svg").unwrap(), Self::QUALITY));
        icons.insert("forward", assets.add_svg(&assets.load_file("icons/forward.svg").unwrap(), Self::QUALITY));
        icons.insert("gif", assets.add_svg(&assets.load_file("icons/gif.svg").unwrap(), Self::QUALITY));
        icons.insert("group", assets.add_svg(&assets.load_file("icons/group.svg").unwrap(), Self::QUALITY));
        icons.insert("heart", assets.add_svg(&assets.load_file("icons/heart.svg").unwrap(), Self::QUALITY));
        icons.insert("home", assets.add_svg(&assets.load_file("icons/home.svg").unwrap(), Self::QUALITY));
        icons.insert("infinite", assets.add_svg(&assets.load_file("icons/infinite.svg").unwrap(), Self::QUALITY));
        icons.insert("info", assets.add_svg(&assets.load_file("icons/info.svg").unwrap(), Self::QUALITY));
        icons.insert("instagram", assets.add_svg(&assets.load_file("icons/instagram.svg").unwrap(), Self::QUALITY));
        icons.insert("left", assets.add_svg(&assets.load_file("icons/left.svg").unwrap(), Self::QUALITY));
        icons.insert("link", assets.add_svg(&assets.load_file("icons/link.svg").unwrap(), Self::QUALITY));
        icons.insert("megaphone", assets.add_svg(&assets.load_file("icons/megaphone.svg").unwrap(), Self::QUALITY));
        icons.insert("messages", assets.add_svg(&assets.load_file("icons/messages.svg").unwrap(), Self::QUALITY));
        icons.insert("microphone", assets.add_svg(&assets.load_file("icons/microphone.svg").unwrap(), Self::QUALITY));
        icons.insert("monitor", assets.add_svg(&assets.load_file("icons/monitor.svg").unwrap(), Self::QUALITY));
        icons.insert("notification", assets.add_svg(&assets.load_file("icons/notification.svg").unwrap(), Self::QUALITY));
        icons.insert("paste", assets.add_svg(&assets.load_file("icons/paste.svg").unwrap(), Self::QUALITY));
        icons.insert("pelican_ui", assets.add_svg(&assets.load_file("icons/pelican_ui.svg").unwrap(), Self::QUALITY));
        icons.insert("photos", assets.add_svg(&assets.load_file("icons/photos.svg").unwrap(), Self::QUALITY));
        icons.insert("play_store", assets.add_svg(&assets.load_file("icons/play_store.svg").unwrap(), Self::QUALITY));
        icons.insert("profile", assets.add_svg(&assets.load_file("icons/profile.svg").unwrap(), Self::QUALITY));
        icons.insert("qr_code", assets.add_svg(&assets.load_file("icons/qr_code.svg").unwrap(), Self::QUALITY));
        icons.insert("radio_filled", assets.add_svg(&assets.load_file("icons/radio_filled.svg").unwrap(), Self::QUALITY));
        icons.insert("radio", assets.add_svg(&assets.load_file("icons/radio.svg").unwrap(), Self::QUALITY));
        icons.insert("right", assets.add_svg(&assets.load_file("icons/right.svg").unwrap(), Self::QUALITY));
        icons.insert("scan", assets.add_svg(&assets.load_file("icons/scan.svg").unwrap(), Self::QUALITY));
        icons.insert("search", assets.add_svg(&assets.load_file("icons/search.svg").unwrap(), Self::QUALITY));
        icons.insert("send", assets.add_svg(&assets.load_file("icons/send.svg").unwrap(), Self::QUALITY));
        icons.insert("settings", assets.add_svg(&assets.load_file("icons/settings.svg").unwrap(), Self::QUALITY));
        icons.insert("up", assets.add_svg(&assets.load_file("icons/up.svg").unwrap(), Self::QUALITY));
        icons.insert("wallet", assets.add_svg(&assets.load_file("icons/wallet.svg").unwrap(), Self::QUALITY));
        icons.insert("warning", assets.add_svg(&assets.load_file("icons/warning.svg").unwrap(), Self::QUALITY));
        icons.insert("x", assets.add_svg(&assets.load_file("icons/x.svg").unwrap(), Self::QUALITY));

        Self(icons)
    }

    pub fn get(&self, name: &'static str) -> resources::Image {
        self.0.get(name).unwrap_or_else(|| self.0.get("pelican_ui").unwrap()).clone()
    }

    pub fn insert(&mut self, ctx: &mut Context, icon_name: &'static str) {
        let path = format!("icons/{icon_name}.svg");
        let svg = &ctx.assets.load_file(&path).unwrap();
        let icon = ctx.assets.add_svg(svg, Self::QUALITY);
        self.0.insert(icon_name, icon);
    }

    pub fn all(&self) -> HashMap<&'static str, resources::Image> {self.0.clone()}
}
