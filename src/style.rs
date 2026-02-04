use iced::widget::{button, container, pick_list, scrollable, text_input};
use iced::overlay::menu;
use iced::{Background, Border, Color, Shadow, Theme, Vector};

// Solarized Dark Palette
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

pub const CHART_GREEN: Color = Color::from_rgb(0.0, 1.0, 0.0);
pub const CHART_YELLOW: Color = Color::from_rgb(1.0, 1.0, 0.0);
pub const CHART_RED: Color = Color::from_rgb(1.0, 0.0, 0.0);

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
                color: BASE01,
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
                color: BASE01,
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
                color: BASE01,
                width: 1.0,
                radius: 0.0.into(),
            },
            icon_color: BASE0,
        }
    }

    fn focused(&self, theme: &Theme) -> text_input::Appearance {
        text_input::Appearance {
            border: Border {
                color: CYAN,
                ..self.active(theme).border
            },
            ..self.active(theme)
        }
    }

    fn hovered(&self, theme: &Theme) -> text_input::Appearance {
        text_input::Appearance {
            border: Border {
                color: BASE0,
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
        BASE0
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
                color: BASE01,
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
                color: BASE0,
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
            background: Some(Background::Color(CYAN)),
            text_color: BASE03,
            border: Border {
                color: BASE01,
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
            background: Some(Background::Color(BLUE)),
            ..active
        }
    }

    fn pressed(&self, theme: &Theme) -> button::Appearance {
        let active = self.active(theme);
        button::Appearance {
            background: Some(Background::Color(BASE02)),
            text_color: CYAN,
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
                    color: BASE01,
                    border: Border {
                        color: BASE01,
                        width: 0.0,
                        radius: 0.0.into(),
                    },
                },
            },
            gap: None,
        }
    }

    fn hovered(&self, theme: &Theme, _is_mouse_over: bool) -> scrollable::Appearance {
        self.active(theme)
    }

    fn dragging(&self, theme: &Theme) -> scrollable::Appearance {
        self.active(theme)
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
                color: BASE01,
                width: 1.0,
                radius: 0.0.into(),
            },
        }
    }

    fn hovered(&self, theme: &Theme) -> pick_list::Appearance {
        let active = self.active(theme);
        pick_list::Appearance {
            border: Border {
                color: BASE0,
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
                color: BASE01,
                width: 1.0,
                radius: 0.0.into(),
            },
            selected_text_color: BASE03,
            selected_background: Background::Color(CYAN),
        }
    }
}
