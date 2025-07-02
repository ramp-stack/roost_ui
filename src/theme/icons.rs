use crate::Assets;
use crate::resources;

use std::collections::HashMap;

/// A collection of icons used throughout the application.
///
/// This struct contains a set of icons that are loaded by the application at runtime.
/// Icons can be accessed by their string name, and they are loaded from SVG files.
/// The icons are loaded with a quality setting to ensure clarity across various screen
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
    /// Creates a new instance of `IconResources` and loads the default set of icons.
    ///
    /// This method loads a collection of SVG icons from the specified paths and stores them
    /// in a `HashMap`. Each icon is given a unique name (as a string), which can later be used
    /// to access the icon resource.
    ///
    /// # Arguments
    ///
    /// * `assets` - A mutable reference to the Assets, used for loading the SVG files.
    ///
    /// # Returns
    ///
    /// A new `IconResources` instance containing the default set of icons.
    pub fn default(assets: &mut Assets) -> Self {
        let mut icons = HashMap::new();
        let quality = 8.0; // Quality factor for the SVG rendering

        // Load each SVG file and insert them into the icons HashMap
        icons.insert("accounts", assets.add_svg(&assets.load_file("icons/accounts.svg").unwrap(), quality));
        icons.insert("add", assets.add_svg(&assets.load_file("icons/add.svg").unwrap(), quality));
        icons.insert("app_store", assets.add_svg(&assets.load_file("icons/app_store.svg").unwrap(), quality));
        icons.insert("back", assets.add_svg(&assets.load_file("icons/back.svg").unwrap(), quality));
        icons.insert("block", assets.add_svg(&assets.load_file("icons/block.svg").unwrap(), quality));
        icons.insert("unblock", assets.add_svg(&assets.load_file("icons/unblock.svg").unwrap(), quality));
        icons.insert("boot", assets.add_svg(&assets.load_file("icons/boot.svg").unwrap(), quality));
        icons.insert("unboot", assets.add_svg(&assets.load_file("icons/unboot.svg").unwrap(), quality));
        icons.insert("back_arrow", assets.add_svg(&assets.load_file("icons/back_arrow.svg").unwrap(), quality));
        icons.insert("back_to", assets.add_svg(&assets.load_file("icons/back_arrow.svg").unwrap(), quality));
        icons.insert("backspace", assets.add_svg(&assets.load_file("icons/backspace.svg").unwrap(), quality));
        icons.insert("bitcoin", assets.add_svg(&assets.load_file("icons/bitcoin.svg").unwrap(), quality));
        icons.insert("camera", assets.add_svg(&assets.load_file("icons/camera.svg").unwrap(), quality));
        icons.insert("cancel", assets.add_svg(&assets.load_file("icons/cancel.svg").unwrap(), quality));
        icons.insert("capslock", assets.add_svg(&assets.load_file("icons/capslock.svg").unwrap(), quality));
        icons.insert("capslock_on", assets.add_svg(&assets.load_file("icons/capslock_on.svg").unwrap(), quality));
        icons.insert("checkmark", assets.add_svg(&assets.load_file("icons/checkmark.svg").unwrap(), quality));
        icons.insert("close", assets.add_svg(&assets.load_file("icons/close.svg").unwrap(), quality));
        icons.insert("copy", assets.add_svg(&assets.load_file("icons/copy.svg").unwrap(), quality));
        icons.insert("credential", assets.add_svg(&assets.load_file("icons/credential.svg").unwrap(), quality));
        icons.insert("down_arrow", assets.add_svg(&assets.load_file("icons/down_arrow.svg").unwrap(), quality));
        icons.insert("delete", assets.add_svg(&assets.load_file("icons/delete.svg").unwrap(), quality));
        icons.insert("door", assets.add_svg(&assets.load_file("icons/door.svg").unwrap(), quality));
        icons.insert("down", assets.add_svg(&assets.load_file("icons/down.svg").unwrap(), quality));
        icons.insert("edit", assets.add_svg(&assets.load_file("icons/edit.svg").unwrap(), quality));
        icons.insert("emoji", assets.add_svg(&assets.load_file("icons/emoji.svg").unwrap(), quality));
        icons.insert("error", assets.add_svg(&assets.load_file("icons/error.svg").unwrap(), quality));
        icons.insert("explore", assets.add_svg(&assets.load_file("icons/explore.svg").unwrap(), quality));
        icons.insert("facebook", assets.add_svg(&assets.load_file("icons/facebook.svg").unwrap(), quality));
        icons.insert("forward", assets.add_svg(&assets.load_file("icons/forward.svg").unwrap(), quality));
        icons.insert("gif", assets.add_svg(&assets.load_file("icons/gif.svg").unwrap(), quality));
        icons.insert("group", assets.add_svg(&assets.load_file("icons/group.svg").unwrap(), quality));
        icons.insert("heart", assets.add_svg(&assets.load_file("icons/heart.svg").unwrap(), quality));
        icons.insert("home", assets.add_svg(&assets.load_file("icons/home.svg").unwrap(), quality));
        icons.insert("infinite", assets.add_svg(&assets.load_file("icons/infinite.svg").unwrap(), quality));
        icons.insert("info", assets.add_svg(&assets.load_file("icons/info.svg").unwrap(), quality));
        icons.insert("instagram", assets.add_svg(&assets.load_file("icons/instagram.svg").unwrap(), quality));
        icons.insert("left", assets.add_svg(&assets.load_file("icons/left.svg").unwrap(), quality));
        icons.insert("link", assets.add_svg(&assets.load_file("icons/link.svg").unwrap(), quality));
        icons.insert("megaphone", assets.add_svg(&assets.load_file("icons/megaphone.svg").unwrap(), quality));
        icons.insert("messages", assets.add_svg(&assets.load_file("icons/messages.svg").unwrap(), quality));
        icons.insert("microphone", assets.add_svg(&assets.load_file("icons/microphone.svg").unwrap(), quality));
        icons.insert("monitor", assets.add_svg(&assets.load_file("icons/monitor.svg").unwrap(), quality));
        icons.insert("notification", assets.add_svg(&assets.load_file("icons/notification.svg").unwrap(), quality));
        icons.insert("paste", assets.add_svg(&assets.load_file("icons/paste.svg").unwrap(), quality));
        icons.insert("photos", assets.add_svg(&assets.load_file("icons/photos.svg").unwrap(), quality));
        icons.insert("play_store", assets.add_svg(&assets.load_file("icons/play_store.svg").unwrap(), quality));
        icons.insert("profile", assets.add_svg(&assets.load_file("icons/profile.svg").unwrap(), quality));
        icons.insert("qr_code", assets.add_svg(&assets.load_file("icons/qr_code.svg").unwrap(), quality));
        icons.insert("radio_filled", assets.add_svg(&assets.load_file("icons/radio_filled.svg").unwrap(), quality));
        icons.insert("radio", assets.add_svg(&assets.load_file("icons/radio.svg").unwrap(), quality));
        icons.insert("right", assets.add_svg(&assets.load_file("icons/right.svg").unwrap(), quality));
        icons.insert("scan", assets.add_svg(&assets.load_file("icons/scan.svg").unwrap(), quality));
        icons.insert("search", assets.add_svg(&assets.load_file("icons/search.svg").unwrap(), quality));
        icons.insert("send", assets.add_svg(&assets.load_file("icons/send.svg").unwrap(), quality));
        icons.insert("settings", assets.add_svg(&assets.load_file("icons/settings.svg").unwrap(), quality));
        icons.insert("up", assets.add_svg(&assets.load_file("icons/up.svg").unwrap(), quality));
        icons.insert("wallet", assets.add_svg(&assets.load_file("icons/wallet.svg").unwrap(), quality));
        icons.insert("warning", assets.add_svg(&assets.load_file("icons/warning.svg").unwrap(), quality));
        icons.insert("x", assets.add_svg(&assets.load_file("icons/x.svg").unwrap(), quality));

        // Return the loaded icons inside the `IconResources` struct
        Self(icons)
    }

    /// Retrieves an icon by its name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the icon to retrieve. Must match the name used when the icon was added.
    ///
    /// # Returns
    ///
    /// The corresponding `resources::Image` for the requested icon. Panics if the icon is not found.
    ///
    /// # Panics
    ///
    /// Panics if the icon with the specified name does not exist in the collection.
    pub fn get(&self, name: &'static str) -> resources::Image {
        self.0.get(name).unwrap_or_else(|| panic!("Could not find icon {:?}", name)).clone()
    }

    /// Adds a new icon to the collection.
    ///
    /// This method inserts a new icon into the `IconResources` collection. If an icon with the same
    /// name already exists, it will not be added again.
    ///
    /// # Arguments
    ///
    /// * `icon_name` - The name of the new icon.
    /// * `icon` - The image resource to associate with the icon name.
    pub fn add_icon(&mut self, icon_name: &'static str, icon: resources::Image) {
        if let std::collections::hash_map::Entry::Vacant(e) = self.0.entry(icon_name) {
            e.insert(icon);
        } else {
            println!("add_icon(): Icon with name {:?} already exists. Use 'set_icon()' instead.", icon_name);
        }
    }

    /// Sets an existing icon with a new image.
    ///
    /// This method replaces the existing image for the specified icon name.
    ///
    /// # Arguments
    ///
    /// * `icon_name` - The name of the icon to replace.
    /// * `icon` - The new image resource to associate with the icon name.
    pub fn set_icon(&mut self, icon_name: &'static str, icon: resources::Image) {
        if let Some(existing) = self.0.get_mut(&icon_name) {
            *existing = icon; 
        } else {
            println!("set_icon(): Icon with name {:?} doesn't exist. Use 'add_icon()' instead.", icon_name);
        }
    }
}
