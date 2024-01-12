use std::fmt;
use std::fmt::Formatter;
use eframe::egui::Color32;
use serde::{Serialize, Deserialize};

pub const COLOR_BLUE_300: Color32 = Color32::from_rgb(147, 197, 253);
pub const COLOR_BLUE_400: Color32 = Color32::from_rgb(96, 165, 250);
pub const COLOR_EMERALD_300: Color32 = Color32::from_rgb(110, 231, 183);
pub const COLOR_EMERALD_400: Color32 = Color32::from_rgb(52, 211, 153);
pub const COLOR_ORANGE_300: Color32 = Color32::from_rgb(253, 186, 116);
pub const COLOR_ORANGE_400: Color32 = Color32::from_rgb(251, 146, 60);
pub const COLOR_RED_300: Color32 = Color32::from_rgb(252, 165, 165);
pub const COLOR_RED_400: Color32 = Color32::from_rgb(248, 113, 113);

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Wavelength {
    #[default]
    W470nm,
    W570nm,
    W630nm,
    W850nm,
}

impl fmt::Display for Wavelength {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Wavelength::W470nm => write!(f, "470nm"),
            Wavelength::W570nm => write!(f, "570nm"),
            Wavelength::W630nm => write!(f, "630nm"),
            Wavelength::W850nm => write!(f, "850nm"),
        }
    }
}

impl Wavelength {
    /// Converts the `Wavelength` enum variant to a corresponding `u8` value.
    ///
    /// Each wavelength variant is mapped to a unique `u8` value. This method can be used
    /// to convert wavelength information into a compact numerical format, useful for
    /// storage, comparison, or interfacing with systems where numerical representation is required.
    ///
    /// # Returns
    /// A `u8` value corresponding to the `Wavelength` variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use dashboard::wavelength::{Wavelength, COLOR_BLUE_300};
    ///
    /// let numeric_value = Wavelength::W630nm.to_u8();
    /// assert_eq!(numeric_value, 2); // 630nm wavelength corresponds to the value 2
    /// ```
    pub fn to_u8(&self) -> u8 {
        match self {
            Wavelength::W470nm => 0,
            Wavelength::W570nm => 3,
            Wavelength::W630nm => 2,
            Wavelength::W850nm => 1,
        }
    }
    /// Returns the color associated with the wavelength when hovered over in the UI.
    ///
    /// This method maps each wavelength to a distinct color value defined in the constants,
    /// providing a visual cue when a wavelength is interacted with in the UI. The returned
    /// `Color32` value is a lighter variant of the color representing the 'hovered' state.
    ///
    /// # Examples
    ///
    /// ```
    /// use dashboard::wavelength::{Wavelength, COLOR_BLUE_300};
    ///
    /// let color = Wavelength::W470nm.get_hovered_color();
    /// assert_eq!(color, COLOR_BLUE_300); // Light blue color for 470nm wavelength
    /// ```
    pub fn get_hovered_color(&self) -> Color32 {
        match self {
            Wavelength::W470nm => COLOR_BLUE_300,
            Wavelength::W570nm => COLOR_EMERALD_300,
            Wavelength::W630nm => COLOR_ORANGE_300,
            Wavelength::W850nm => COLOR_RED_300,
        }
    }

    /// Returns the primary color associated with the wavelength.
    ///
    /// This method provides the main color representation for each wavelength. The returned
    /// `Color32` value is a darker variant compared to the `get_hovered_color` method, representing
    /// the standard state of the wavelength in the UI.
    ///
    /// # Examples
    ///
    /// ```
    /// use dashboard::wavelength::{Wavelength, COLOR_ORANGE_400};
    ///
    /// let color = Wavelength::W630nm.get_color();
    /// assert_eq!(color, COLOR_ORANGE_400); // Dark orange color for 630nm wavelength
    /// ```
    pub fn get_color(&self) -> Color32 {
        match self {
            Wavelength::W470nm => COLOR_BLUE_400,
            Wavelength::W570nm => COLOR_EMERALD_400,
            Wavelength::W630nm => COLOR_ORANGE_400,
            Wavelength::W850nm => COLOR_RED_400,
        }
    }
}
