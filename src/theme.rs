use crossterm::style::Color;

/// Theme is a simple theme provider. This structure is nothing special. It
/// simply holds some general styling information and can be inserted as a
/// resource into the application.
///
/// If you would like to use different style names just make your own structure
/// which meets your needs and  add it as a resource with App::insert_resource.  
#[derive(Debug)]
pub struct Theme {
    pub bg_primary: Color,
    pub bg_secondary: Color,
    pub bg_tertiary: Color,
    pub bg_selection: Color,
    pub fg_selection: Color,
    pub fg: Color,
    pub accent: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg_primary: Color::Rgb {
                r: 36,
                g: 39,
                b: 58,
            },

            bg_secondary: Color::Rgb {
                r: 20,
                g: 22,
                b: 30,
            },

            bg_tertiary: Color::Rgb {
                r: 76,
                g: 79,
                b: 98,
            },

            bg_selection: Color::Rgb { r: 60, g: 0, b: 60 },
            fg_selection: Color::White,
            fg: Color::White,
            accent: Color::Rgb {
                r: 150,
                g: 0,
                b: 150,
            },
        }
    }
}
