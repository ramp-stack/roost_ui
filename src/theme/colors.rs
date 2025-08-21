use wgpu_canvas::Color;
use std::collections::HashMap;

/// Represents a collection of color resources used throughout the UI, including background, text, button, and status colors.
#[derive(Clone, Debug, Default)]
pub struct ColorResources {
    /// Defines the background colors.
    pub background: BackgroundColor,
    /// Defines the outline colors.
    pub outline: OutlineColor,
    /// Defines the colors representing various status indicators.
    pub status: StatusColor,
    /// Defines the colors for various text elements.
    pub text: TextColor,
    /// Defines the colors for buttons in various states (default, disabled, hover, etc.).
    pub button: ButtonColors,
    /// Defines brand-specific colors.
    pub brand: BrandColor,
    /// Defines various color shades. (Black, White, Transparent, etc.)
    pub shades: ShadesColor,
    /// Colors used for illustrations and other visual assets.
    pub illustration: IllustrationColors,
}

impl ColorResources {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        background: BackgroundColor,
        outline: OutlineColor,
        status: StatusColor,
        text: TextColor,
        brand: BrandColor,
        button: ButtonColors,
        illustration: IllustrationColors,
    ) -> Self {
        ColorResources { background, outline, status, text, brand, shades: ShadesColor::default(), button, illustration }
    }
}

/// Defines various shades used for UI elements.
#[derive(Copy, Clone, Debug)]
pub struct ShadesColor {
    /// Pure black color.
    pub black: Color,
    /// Pure white color.
    pub white: Color,
    /// A lighter shade of white.
    pub lighten: Color,
    /// A further lightened shade of white.
    pub lighten2: Color,
    /// A darkened shade of black.
    pub darken: Color,
    /// A further darkened shade of black.
    pub darken2: Color,
    /// Transparent color.
    pub transparent: Color,
}

impl Default for ShadesColor {
    fn default() -> Self {
        ShadesColor {
            black: Color::from_hex("000000", 255),
            white: Color::from_hex("ffffff", 255),
            lighten: Color::from_hex("ffffff", 110),
            lighten2: Color::from_hex("ffffff", 180),
            darken: Color::from_hex("000000", 110),
            darken2: Color::from_hex("000000", 180),
            transparent: Color::from_hex("000000", 0),
        }
    }
}

/// Defines the background colors.   
#[derive(Copy, Clone, Debug)]
pub struct BackgroundColor {
    pub primary: Color,
    pub secondary: Color,
}

impl Default for BackgroundColor {
    fn default() -> Self {
        BackgroundColor {
            primary: Color::from_hex("000000", 255),
            secondary: Color::from_hex("262322", 255),
        }
    }
}

/// Defines the brand-specific colors.
#[derive(Copy, Clone, Debug)]
pub struct BrandColor {
    /// The brand's main color.
    pub primary: Color,
    /// The brand's secondary color. (typically black or white)
    pub secondary: Color,
}

impl Default for BrandColor {
    fn default() -> Self {
        BrandColor {
            primary: Color::from_hex("eb343a", 255),
            secondary: Color::from_hex("ffffff", 255),
        }
    }
}

/// Defines the outline colors.
#[derive(Copy, Clone, Debug)]
pub struct OutlineColor {
    pub primary: Color,
    pub secondary: Color,
}

impl Default for OutlineColor {
    fn default() -> Self {
        OutlineColor {
            primary: Color::from_hex("ffffff", 255),
            secondary: Color::from_hex("585250", 255),
        }
    }
}

/// Defines the colors of text elements.
#[derive(Copy, Clone, Debug)]
pub struct TextColor {
    pub heading: Color,
    pub primary: Color,
    pub secondary: Color,
}

impl Default for TextColor {
    fn default() -> Self {
        TextColor {
            heading: Color::from_hex("ffffff", 255),
            primary: Color::from_hex("e2e1df", 255),
            secondary: Color::from_hex("a7a29d", 255),
        }
    }
}

/// Defines the colors representing various status indicators.
#[derive(Copy, Clone, Debug)]
pub struct StatusColor {
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
}

impl Default for StatusColor {
    fn default() -> Self {
        StatusColor {
            success: Color::from_hex("3ccb5a", 255),
            warning: Color::from_hex("f5bd14", 255),
            danger: Color::from_hex("ff330a", 255),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct IllustrationColors {
    pub colors: HashMap<&'static str, Color>,
}

/// Defines the colors for buttons in various states, including default, disabled, hover, pressed, etc.
#[derive(Copy, Clone, Debug)]
pub struct ButtonColors {
    /// Button color scheme for the primary default state.
    pub primary_default: ButtonColorScheme,
    /// Button color scheme for the primary disabled state.
    pub primary_disabled: ButtonColorScheme,
    /// Button color scheme for the primary hover state.
    pub primary_hover: ButtonColorScheme,
    /// Button color scheme for the primary selected state.
    pub primary_selected: ButtonColorScheme,
    /// Button color scheme for the primary pressed state.
    pub primary_pressed: ButtonColorScheme,

    /// Button color scheme for the secondary default state.
    pub secondary_default: ButtonColorScheme,
    /// Button color scheme for the secondary disabled state.
    pub secondary_disabled: ButtonColorScheme,
    /// Button color scheme for the secondary hover state.
    pub secondary_hover: ButtonColorScheme,
    /// Button color scheme for the secondary selected state.
    pub secondary_selected: ButtonColorScheme,
    /// Button color scheme for the secondary pressed state.
    pub secondary_pressed: ButtonColorScheme,

    /// Button color scheme for the ghost default state.
    pub ghost_default: ButtonColorScheme,
    /// Button color scheme for the ghost disabled state.
    pub ghost_disabled: ButtonColorScheme,
    /// Button color scheme for the ghost hover state.
    pub ghost_hover: ButtonColorScheme,
    /// Button color scheme for the ghost selected state.
    pub ghost_selected: ButtonColorScheme,
    /// Button color scheme for the ghost pressed state.
    pub ghost_pressed: ButtonColorScheme,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            primary_default: ButtonColorScheme {
                background: Color::from_hex("eb343a", 255),
                label: Color::from_hex("ffffff", 255),
                outline: Color::from_hex("000000", 0),
            },
            primary_disabled: ButtonColorScheme {
                background: Color::from_hex("443f3f", 255),
                label: Color::from_hex("000000", 255),
                outline: Color::from_hex("000000", 0),
            },
            primary_hover: ButtonColorScheme {
                background: Color::from_hex("da282e", 255),
                label: Color::from_hex("ffffff", 255),
                outline: Color::from_hex("000000", 0),
            },
            primary_selected: ButtonColorScheme {
                background: Color::from_hex("da282e", 255),
                label: Color::from_hex("ffffff", 255),
                outline: Color::from_hex("000000", 0),
            },
            primary_pressed: ButtonColorScheme {
                background: Color::from_hex("da282e", 255),
                label: Color::from_hex("ffffff", 255),
                outline: Color::from_hex("000000", 0),
            },

            secondary_default: ButtonColorScheme {
                background: Color::from_hex("000000", 0),
                label: Color::from_hex("ffffff", 255),
                outline: Color::from_hex("585250", 255),
            },
            secondary_disabled: ButtonColorScheme {
                background: Color::from_hex("78716c", 255),
                label: Color::from_hex("000000", 255),
                outline:Color::from_hex("585250", 255),
            },
            secondary_hover: ButtonColorScheme {
                background: Color::from_hex("262322", 255),
                label: Color::from_hex("ffffff", 255),
                outline: Color::from_hex("585250", 255),
            },
            secondary_selected: ButtonColorScheme {
                background: Color::from_hex("000000", 255),
                label: Color::from_hex("ffffff", 255),
                outline: Color::from_hex("ffffff", 255),
            },
            secondary_pressed: ButtonColorScheme {
                background: Color::from_hex("262322", 255),
                label: Color::from_hex("ffffff", 255),
                outline: Color::from_hex("585250", 255),
            },

            ghost_default: ButtonColorScheme {
                background: Color::from_hex("000000", 0),
                label: Color::from_hex("ffffff", 255),
                outline: Color::from_hex("000000", 0),
            },
            ghost_disabled: ButtonColorScheme {
                background: Color::from_hex("000000", 0),
                label: Color::from_hex("78716c", 255),
                outline: Color::from_hex("000000", 0),
            },
            ghost_hover: ButtonColorScheme {
                background: Color::from_hex("262322", 255),
                label: Color::from_hex("ffffff", 255),
                outline: Color::from_hex("000000", 0),
            },
            ghost_selected: ButtonColorScheme {
                background: Color::from_hex("262322", 255),
                label: Color::from_hex("ffffff", 255),
                outline: Color::from_hex("000000", 0),
            },
            ghost_pressed: ButtonColorScheme {
                background: Color::from_hex("262322", 255),
                label: Color::from_hex("ffffff", 255),
                outline: Color::from_hex("000000", 0),
            },
        }
    }
}

/// Defines a color scheme for a button, including background, label, and outline colors.
#[derive(Copy, Clone, Debug)]
pub struct ButtonColorScheme {
    /// The background color of the button.
    pub background: Color,
    /// The color of the button label (text).
    pub label: Color,
    /// The color of the button outline (border).
    pub outline: Color,
}
