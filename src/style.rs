use iced::overlay::menu;
use iced::widget::{button, container, pick_list, scrollable, text_input};
use iced::{Background, Border, Color, Shadow, Theme, Vector};

// Aggressive Solarized Dark Palette
// Darker backgrounds for higher contrast
pub const BASE03: Color = Color::from_rgb(0.0, 0.08, 0.1); // #00141a - Deepest Background
pub const BASE02: Color = Color::from_rgb(0.0, 0.125, 0.15); // #002026 - Panel Background
pub const BASE01: Color = Color::from_rgb(0.345, 0.431, 0.459); // #586e75 - Comments/Secondary Content
pub const BASE00: Color = Color::from_rgb(0.46, 0.55, 0.58); // #758b93 - Content
pub const BASE0: Color = Color::from_rgb(0.67, 0.74, 0.73); // #aabcb9 - Body Text
pub const BASE1: Color = Color::from_rgb(0.8, 0.85, 0.85); // #cceaea - Emphasized Content
pub const BASE2: Color = Color::from_rgb(0.933, 0.91, 0.835); // #eee8d5
pub const BASE3: Color = Color::from_rgb(0.992, 0.965, 0.89); // #fdf6e3

// Neon / Vibrant Accents
pub const YELLOW: Color = Color::from_rgb(0.94, 0.78, 0.0); // #f0c600
pub const ORANGE: Color = Color::from_rgb(1.0, 0.37, 0.0); // #ff5f00
pub const RED: Color = Color::from_rgb(1.0, 0.2, 0.2); // #ff3333
pub const MAGENTA: Color = Color::from_rgb(1.0, 0.2, 0.8); // #ff33cc
pub const VIOLET: Color = Color::from_rgb(0.53, 0.53, 1.0); // #8888ff
pub const BLUE: Color = Color::from_rgb(0.0, 0.53, 1.0); // #0088ff
pub const CYAN: Color = Color::from_rgb(0.0, 1.0, 0.8); // #00ffcc
pub const GREEN: Color = Color::from_rgb(0.2, 1.0, 0.0); // #33ff00

pub const CHART_GREEN: Color = GREEN;
pub const CHART_YELLOW: Color = YELLOW;
pub const CHART_RED: Color = RED;

// --- Container Styles ---

pub struct ColoredBox(pub Color);
impl container::StyleSheet for ColoredBox {
    type Style = Theme;
    fn appearance(&self, _theme: &Theme) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0)),
            ..container::Appearance::default()
        }
    }
}

// --- Joined Input Styles ---

pub struct BorderlessInput;
impl text_input::StyleSheet for BorderlessInput {
    type Style = Theme;
    fn active(&self, _theme: &Theme) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(Color::TRANSPARENT),
            border: Border::with_radius(0.0),
            icon_color: BASE0,
        }
    }

    fn focused(&self, theme: &Theme) -> text_input::Appearance {
        self.active(theme)
    }

    fn hovered(&self, theme: &Theme) -> text_input::Appearance {
        self.active(theme)
    }

    fn disabled(&self, _theme: &Theme) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(Color::TRANSPARENT),
            border: Border::with_radius(0.0),
            icon_color: BASE01,
        }
    }

    fn placeholder_color(&self, _theme: &Theme) -> Color {
        BASE01
    }

    fn value_color(&self, _theme: &Theme) -> Color {
        BASE1
    }

    fn disabled_color(&self, _theme: &Theme) -> Color {
        BASE01
    }

    fn selection_color(&self, _theme: &Theme) -> Color {
        BASE01
    }
}

pub struct InputGroup;
impl container::StyleSheet for InputGroup {
    type Style = Theme;
    fn appearance(&self, _theme: &Theme) -> container::Appearance {
        container::Appearance {
            text_color: Some(BASE0),
            background: Some(Background::Color(BASE02)),
            border: Border {
                color: BASE00,
                width: 1.0,
                radius: 0.0.into(),
            },
            shadow: Shadow::default(),
        }
    }
}


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
                color: BASE00, // Brighter border for panels
                width: 1.0,
                radius: 0.0.into(),
            },
            shadow: Shadow::default(),
        }
    }
}

pub struct BorderedPanel;
impl container::StyleSheet for BorderedPanel {
    type Style = Theme;
    fn appearance(&self, _theme: &Theme) -> container::Appearance {
        container::Appearance {
            text_color: Some(BASE0),
            background: Some(Background::Color(BASE03)),
            border: Border {
                color: BLUE, // Aggressive Blue Border
                width: 1.0,
                radius: 0.0.into(),
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
                color: BASE00,
                width: 1.0,
                radius: 0.0.into(),
            },
            icon_color: BASE0,
        }
    }

    fn focused(&self, theme: &Theme) -> text_input::Appearance {
        text_input::Appearance {
            border: Border {
                color: CYAN, // Neon Cyan focus
                width: 1.5,
                ..self.active(theme).border
            },
            ..self.active(theme)
        }
    }

    fn hovered(&self, theme: &Theme) -> text_input::Appearance {
        text_input::Appearance {
            border: Border {
                color: BASE1, // Bright hover
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
        BASE1 // Brighter text value
    }

    fn disabled_color(&self, _theme: &Theme) -> Color {
        BASE01
    }

    fn selection_color(&self, _theme: &Theme) -> Color {
        BASE01
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
                color: BASE00, // Stronger border
                width: 1.0,
                radius: 0.0.into(),
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
                color: CYAN, // Cyan hover
                ..active.border
            },
            ..active
        }
    }

    fn pressed(&self, theme: &Theme) -> button::Appearance {
        let active = self.active(theme);
        button::Appearance {
            background: Some(Background::Color(BASE03)),
            text_color: CYAN,
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
            background: Some(Background::Color(BLUE)), // Electric Blue
            text_color: BASE03, // Dark text on bright bg
            border: Border {
                color: CYAN,
                width: 1.0,
                radius: 0.0.into(),
            },
            shadow: Shadow::default(),
            shadow_offset: Vector::default(),
        }
    }

    fn hovered(&self, theme: &Theme) -> button::Appearance {
        let active = self.active(theme);
        button::Appearance {
            background: Some(Background::Color(CYAN)), // Cyan on hover
            ..active
        }
    }

    fn pressed(&self, theme: &Theme) -> button::Appearance {
        let active = self.active(theme);
        button::Appearance {
            background: Some(Background::Color(BASE03)),
            text_color: CYAN,
            border: Border {
                color: BLUE,
                ..active.border
            },
            ..active
        }
    }

    fn disabled(&self, theme: &Theme) -> button::Appearance {
        let active = self.active(theme);
        button::Appearance {
            background: Some(Background::Color(BASE02)),
            text_color: BASE01,
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
                    color: BASE00, // Visible scroller
                    border: Border {
                        color: BASE00,
                        width: 0.0,
                        radius: 0.0.into(),
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
                    color: BLUE, // Blue scroller on hover
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
                    color: CYAN, // Cyan scroller dragging
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
            text_color: BASE0,
            placeholder_color: BASE01,
            handle_color: BASE0,
            background: Background::Color(BASE02),
            border: Border {
                color: BASE00,
                width: 1.0,
                radius: 0.0.into(),
            },
        }
    }

    fn hovered(&self, theme: &Theme) -> pick_list::Appearance {
        let active = self.active(theme);
        pick_list::Appearance {
            border: Border {
                color: CYAN, // Cyan border hover
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
            text_color: BASE0,
            background: Background::Color(BASE02),
            border: Border {
                color: BLUE, // Blue border for menu
                width: 1.0,
                radius: 0.0.into(),
            },
            selected_text_color: BASE03,
            selected_background: Background::Color(CYAN),
        }
    }
}
