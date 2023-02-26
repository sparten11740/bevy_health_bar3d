use std::marker::PhantomData;
use bevy::prelude::{Color, Component, Resource};
use bevy::utils::default;
use crate::constants::{DEFAULT_BACKGROUND_COLOR, DEFAULT_HIGH_COLOR, DEFAULT_LOW_COLOR, DEFAULT_MODERATE_COLOR};

/// Component to configure the Y-offset of the bar relative to the entity its attached to
#[derive(Component)]
pub struct BarOffset(pub f32);

impl BarOffset {
    pub fn get(&self) -> f32 {
        self.0
    }
}

/// Component to configure the width of the bar
#[derive(Component)]
pub struct BarWidth(pub f32);

impl BarWidth {
    pub fn get(&self) -> f32 {
        self.0
    }
}

/// Component to configure the width of the bar
#[derive(Component)]
pub struct BarHeight(pub f32);

impl BarHeight {
    pub fn get(&self) -> f32 {
        self.0
    }
}

/// Trait implemented by the component to be tracked by the health bar
pub trait Percentage {
    /// Value between 0 and 1
    fn value(&self) -> f32;
}

/// ForegroundColor enum. The foreground color can either be static or a tri-color spectrum
/// The tri-color spectrum defines three colors: high, moderate, and low.
/// The high color is applied when the tracked component's value is more than or equal to 80%,
/// moderate when it's between 40% and 80%, and low when it is less than 40%.
#[derive(Debug, Clone)]
pub enum ForegroundColor {
    Static(Color),
    TriSpectrum {
        high: Color,
        moderate: Color,
        low: Color,
    },
}

/// Resource to customize the appearance of bars per tracked component type.
#[derive(Resource, Debug, Clone)]
pub struct ColorScheme<T: Percentage + Component> {
    pub foreground_color: ForegroundColor,
    pub background_color: Color,
    phantom_data: PhantomData<T>,
}

impl<T: Percentage + Component> ColorScheme<T> {
    /// Returns a default initialized ColorScheme for the given component type
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy_health_bar3d::prelude::ColorScheme;
    /// let color_scheme = ColorScheme::<Health>::new();
    /// ```
    pub fn new() -> Self {
        Self {
            phantom_data: PhantomData,
            ..default()
        }
    }


    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    /// Sets the foreground color to either a static value or a tri-color spectrum
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::prelude::Color;
    /// use bevy_health_bar3d::prelude::{ColorScheme, ForegroundColor};
    /// let mana_scheme = ColorScheme::<Mana>::new().foreground_color(ForegroundColor::Static(Color::BLUE));
    /// let health_scheme = ColorScheme::<Health>::new().foreground_color(ForegroundColor::TriSpectrum {
    ///     high: Color::GREEN,
    ///     moderate: Color::ORANGE,
    ///     low: Color::RED
    /// });
    /// ```
    pub fn foreground_color(mut self, color: ForegroundColor) -> Self {
        self.foreground_color = color;
        self
    }
}

impl<T: Percentage + Component> Default for ColorScheme<T> {
    fn default() -> Self {
        Self {
            foreground_color: ForegroundColor::TriSpectrum {
                high: DEFAULT_HIGH_COLOR,
                moderate: DEFAULT_MODERATE_COLOR,
                low: DEFAULT_LOW_COLOR,
            },
            background_color: DEFAULT_BACKGROUND_COLOR,
            phantom_data: PhantomData,
        }
    }
}