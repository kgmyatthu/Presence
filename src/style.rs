use iced::widget::{button, container, pick_list, scrollable, text_input};
use iced::overlay::menu;
use iced::{Background, Border, Color, Shadow, Theme, Vector};

// Solarized Dark Palette (Standard)
pub const BASE03: Color = Color::from_rgb(0.0, 0.169, 0.212); // #002b36
pub const BASE02: Color = Color::from_rgb(0.027, 0.212, 0.259); // #073642
pub const BASE01: Color = Color::from_rgb(0.345, 0.431, 0.459); // #586e75
pub const BASE00: Color = Color::from_rgb(0.396, 0.482, 0.514); // #657b83
pub const BASE0: Color = Color::from_rgb(0.514, 0.580, 0.588); // #839496
pub const BASE1: Color = Color::from_rgb(0.576, 0.631, 0.631); // #93a1a1
pub const BASE2: Color = Color::from_rgb(0.933, 0.91, 0.835); // #eee8d5
pub const BASE3: Color = Color::from_rgb(0.992, 0.965, 0.89); // #fdf6e3
pub const YELLOW: Color = Color::from_rgb(0.71, 0.537, 0.0); // #b58900
pub const ORANGE: Color = Color::from_rgb(0.796, 0.294, 0.086); // #cb4b16
pub const RED: Color = Color::from_rgb(0.863, 0.196, 0.184); // #dc322f
pub const MAGENTA: Color = Color::from_rgb(0.827, 0.212, 0.51); // #d33682
pub const VIOLET: Color = Color::from_rgb(0.424, 0.443, 0.769); // #6c71c4
pub const BLUE: Color = Color::from_rgb(0.149, 0.545, 0.824); // #268bd2
pub const CYAN: Color = Color::from_rgb(0.165, 0.631, 0.596); // #2aa198
pub const GREEN: Color = Color::from_rgb(0.522, 0.6, 0.0); // #859900

// Expanded Vibrant Palette (Neo-Solarized / Tokyo Night Inspired)
pub const NEON_MAGENTA: Color = Color::from_rgb(1.0, 0.0, 0.55); // Vibrant Pink/Magenta
pub const NEON_BLUE: Color = Color::from_rgb(0.2, 0.65, 1.0); // Electric Blue
pub const NEON_VIOLET: Color = Color::from_rgb(0.7, 0.5, 1.0); // Bright Violet
pub const NEON_ORANGE: Color = Color::from_rgb(1.0, 0.6, 0.0); // Bright Orange
pub const NEON_CYAN: Color = Color::from_rgb(0.0, 0.9, 0.9); // Electric Cyan

// --- Container Styles ---

pub struct MainBg;
impl container::StyleSheet for MainBg {
    type Style = Theme;
    fn appearance(&self, _theme: &Theme) -> container::Appearance {
        container::Appearance {
            text_color: Some(BASE0),
            background: Some(Background::Color(BASE03)),
            ..container::Appearance::default()
        }
    }
}

pub struct Panel;
impl container::StyleSheet for Panel {
    type Style = Theme;
    fn appearance(&self, _theme: &Theme) -> container::Appearance {
        container::Appearance {
            text_color: Some(BASE0),
            background: Some(Background::Color(BASE02)),
            border: Border {
                color: BASE00, // Increased visibility from BASE01
                width: 1.0,
                radius: 4.0.into(), // Slight rounding for modern feel
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                offset: Vector::new(2.0, 2.0),
                blur_radius: 5.0,
            },
        }
    }
}

pub struct BorderedPanel;
impl container::StyleSheet for BorderedPanel {
    type Style = Theme;
    fn appearance(&self, _theme: &Theme) -> container::Appearance {
        container::Appearance {
            text_color: Some(BASE1), // Brighter text
            background: Some(Background::Color(BASE03)),
            border: Border {
                color: NEON_BLUE, // Vibrant border for status/highlights
                width: 1.0,
                radius: 4.0.into(),
            },
            ..container::Appearance::default()
        }
    }
}

// --- Text Input Styles ---

pub struct TextInput;
impl text_input::StyleSheet for TextInput {
    type Style = Theme;
    fn active(&self, _theme: &Theme) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(BASE02),
            border: Border {
                color: BASE01,
                width: 1.0,
                radius: 4.0.into(),
            },
            icon_color: BASE0,
        }
    }

    fn focused(&self, theme: &Theme) -> text_input::Appearance {
        text_input::Appearance {
            border: Border {
                color: NEON_ORANGE, // Exciting focus color
                ..self.active(theme).border
            },
            ..self.active(theme)
        }
    }

    fn hovered(&self, theme: &Theme) -> text_input::Appearance {
        text_input::Appearance {
            border: Border {
                color: BASE1,
                ..self.active(theme).border
            },
            ..self.active(theme)
        }
    }

    fn disabled(&self, theme: &Theme) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(BASE03),
            icon_color: BASE01,
            ..self.active(theme)
        }
    }

    fn placeholder_color(&self, _theme: &Theme) -> Color {
        BASE01
    }

    fn value_color(&self, _theme: &Theme) -> Color {
        BASE1 // Brighter value text
    }

    fn disabled_color(&self, _theme: &Theme) -> Color {
        BASE01
    }

    fn selection_color(&self, _theme: &Theme) -> Color {
        Color::from_rgba(1.0, 0.0, 0.55, 0.3) // Transparent Neon Magenta
    }
}

// --- Button Styles ---

pub struct Button;
impl button::StyleSheet for Button {
    type Style = Theme;
    fn active(&self, _theme: &Theme) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(BASE02)),
            text_color: BASE0,
            border: Border {
                color: BASE00,
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: Shadow::default(),
            shadow_offset: Vector::default(),
        }
    }

    fn hovered(&self, theme: &Theme) -> button::Appearance {
        let active = self.active(theme);
        button::Appearance {
            background: Some(Background::Color(BASE01)),
            text_color: BASE3,
            border: Border {
                color: BASE1,
                ..active.border
            },
            ..active
        }
    }

    fn pressed(&self, theme: &Theme) -> button::Appearance {
        let active = self.active(theme);
        button::Appearance {
            background: Some(Background::Color(BASE03)),
            text_color: BASE0,
            ..active
        }
    }

    fn disabled(&self, theme: &Theme) -> button::Appearance {
        let active = self.active(theme);
        button::Appearance {
            background: Some(Background::Color(BASE03)),
            text_color: BASE01,
            border: Border {
                color: BASE02,
                ..active.border
            },
            ..active
        }
    }
}

pub struct PrimaryButton;
impl button::StyleSheet for PrimaryButton {
    type Style = Theme;
    fn active(&self, _theme: &Theme) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(NEON_MAGENTA)), // Vibrant Primary Action
            text_color: BASE3,
            border: Border {
                color: NEON_MAGENTA,
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 2.0,
            },
            shadow_offset: Vector::new(0.0, 1.0),
        }
    }

    fn hovered(&self, theme: &Theme) -> button::Appearance {
        let active = self.active(theme);
        button::Appearance {
            background: Some(Background::Color(NEON_VIOLET)), // Shift on hover
            border: Border {
                color: NEON_VIOLET,
                ..active.border
            },
            ..active
        }
    }

    fn pressed(&self, theme: &Theme) -> button::Appearance {
        let active = self.active(theme);
        button::Appearance {
            background: Some(Background::Color(MAGENTA)),
            text_color: BASE3,
            shadow_offset: Vector::default(),
            ..active
        }
    }

    fn disabled(&self, theme: &Theme) -> button::Appearance {
        let active = self.active(theme);
        button::Appearance {
            background: Some(Background::Color(BASE02)),
            text_color: BASE01,
            border: Border {
                color: BASE02,
                ..active.border
            },
            ..active
        }
    }
}

// --- Scrollable Styles ---

pub struct Scrollable;
impl scrollable::StyleSheet for Scrollable {
    type Style = Theme;
    fn active(&self, _theme: &Theme) -> scrollable::Appearance {
        scrollable::Appearance {
            container: container::Appearance::default(),
            scrollbar: scrollable::Scrollbar {
                background: Some(Background::Color(BASE02)),
                border: Border {
                    color: BASE01,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                scroller: scrollable::Scroller {
                    color: BASE00, // More visible scroller
                    border: Border {
                        color: BASE00,
                        width: 0.0,
                        radius: 4.0.into(),
                    },
                },
            },
            gap: None,
        }
    }

    fn hovered(&self, theme: &Theme, _is_mouse_over: bool) -> scrollable::Appearance {
        let active = self.active(theme);
        scrollable::Appearance {
            scrollbar: scrollable::Scrollbar {
                scroller: scrollable::Scroller {
                    color: BASE1, // Brighter when hovering
                    ..active.scrollbar.scroller
                },
                ..active.scrollbar
            },
            ..active
        }
    }

    fn dragging(&self, theme: &Theme) -> scrollable::Appearance {
        let active = self.active(theme);
        scrollable::Appearance {
            scrollbar: scrollable::Scrollbar {
                scroller: scrollable::Scroller {
                    color: NEON_BLUE, // Highlight when dragging
                    ..active.scrollbar.scroller
                },
                ..active.scrollbar
            },
            ..active
        }
    }
}

// --- PickList Styles ---

pub struct PickList;
impl pick_list::StyleSheet for PickList {
    type Style = Theme;
    fn active(&self, _theme: &Theme) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: BASE1,
            placeholder_color: BASE01,
            handle_color: NEON_BLUE,
            background: Background::Color(BASE02),
            border: Border {
                color: BASE00,
                width: 1.0,
                radius: 4.0.into(),
            },
        }
    }

    fn hovered(&self, theme: &Theme) -> pick_list::Appearance {
        let active = self.active(theme);
        pick_list::Appearance {
            border: Border {
                color: NEON_BLUE,
                ..active.border
            },
            ..active
        }
    }
}

pub struct Menu;
impl menu::StyleSheet for Menu {
    type Style = Theme;
    fn appearance(&self, _theme: &Theme) -> menu::Appearance {
        menu::Appearance {
            text_color: BASE1,
            background: Background::Color(BASE02),
            border: Border {
                color: NEON_BLUE,
                width: 1.0,
                radius: 4.0.into(),
            },
            selected_text_color: BASE3,
            selected_background: Background::Color(NEON_MAGENTA),
        }
    }
}
