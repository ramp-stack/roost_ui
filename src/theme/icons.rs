use crate::Assets;
use crate::resources;
use crate::Context;

use std::collections::HashMap;

/// A collection of icons used throughout the application.
///
/// This struct contains a set of icons that are loaded by the application at runtime.
/// Icons can be accessed by their string name, and they are loaded from SVG files.
/// The icons are loaded with a Self::QUALITY setting to ensure clarity across various screen
/// resolutions.
///
/// ## Default Icons
/// The following icons are included by default:
/// 
/// 1. `accounts` - An icon representing user accounts.
/// 2. `add` - An icon for adding items or creating new entities.
/// 3. `app_store` - An icon for app store functionality.
/// 4. `back` - An icon representing the back action.
/// 5. `block` - An icon representing blocking actions.
/// 6. `unblock` - An icon representing unblocking actions.
/// 7. `boot` - An icon representing booting user or user removal.
/// 8. `unboot` - An icon representing unbooting user or user un-removal.
/// 9. `back_arrow` - An icon for navigating backward.
/// 10. `back_to` - Another variant of the back arrow icon.
/// 11. `backspace` - An icon representing the backspace action.
/// 12. `bitcoin` - An icon for Bitcoin features.
/// 13. `camera` - An icon for camera functionalities.
/// 14. `cancel` - An icon for canceling actions.
/// 15. `capslock` - An icon representing the Caps Lock key.
/// 16. `capslock_on` - An icon representing Caps Lock being active.
/// 17. `checkmark` - An icon for indicating success or completion.
/// 18. `close` - An icon for closing windows or dialogs.
/// 19. `copy` - An icon representing copy action.
/// 20. `credential` - An icon representing credentials.
/// 21. `down_arrow` - An icon for downward navigation.
/// 22. `delete` - An icon for deleting items.
/// 23. `door` - An icon representing doors or entryways.
/// 24. `down` - Another icon for downward navigation.
/// 25. `edit` - An icon for editing actions.
/// 26. `emoji` - An icon representing emojis.
/// 27. `error` - An icon for error states.
/// 28. `explore` - An icon for exploring or searching.
/// 29. `facebook` - An icon for Facebook-related actions.
/// 30. `forward` - An icon for forwarding actions.
/// 31. `gif` - An icon for GIF-related content.
/// 32. `group` - An icon for group-related features.
/// 33. `heart` - An icon for liking or favoriting content.
/// 34. `home` - An icon for navigating to the home screen.
/// 35. `infinite` - An icon representing infinity or endless loops.
/// 36. `info` - An icon for information-related features.
/// 37. `instagram` - An icon for Instagram-related features.
/// 38. `left` - An icon for leftward navigation.
/// 39. `link` - An icon for linking or sharing content.
/// 40. `megaphone` - An icon representing announcements or promotions.
/// 41. `messages` - An icon for messaging or chat functions.
/// 42. `microphone` - An icon for microphone-related actions.
/// 43. `monitor` - An icon for monitor or display-related features.
/// 44. `paste` - An icon representing the paste action.
/// 45. `photos` - An icon for photos or image-related actions.
/// 46. `play_store` - An icon for the Play Store or app downloads.
/// 47. `profile` - An icon for profile or user-related actions.
/// 48. `qr_code` - An icon for QR code scanning.
/// 49. `radio_filled` - An icon representing a filled radio button or state.
/// 50. `radio` - An icon for a radio button or selection state.
/// 51. `right` - An icon for rightward navigation.
/// 52. `scan` - An icon for scanning actions, typically used for QR codes or barcodes.
/// 53. `search` - An icon for searching functionality.
/// 54. `send` - An icon for sending messages or data.
/// 55. `settings` - An icon for accessing settings or preferences.
/// 56. `up` - An icon for upward navigation.
/// 57. `wallet` - An icon representing wallet or financial features.
/// 58. `warning` - An icon indicating a warning or cautionary state.
/// 59. `x` - An icon for x.com/Twitter-related features.
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
        self.0.get(name).unwrap_or_else(|| panic!("Could not find icon {:?}", name)).clone()
    }

    pub fn insert(&mut self, ctx: &mut Context, icon_name: &'static str) {
        let path = format!("icons/{}.svg", icon_name);
        let svg = &ctx.assets.load_file(&path).unwrap();
        let icon = ctx.assets.add_svg(svg, Self::QUALITY);
        self.0.insert(icon_name, icon);
    }

    pub fn all(&self) -> HashMap<&'static str, resources::Image> {self.0.clone()}
}
