use iced::{Border, Color};

// Color palette - BullBitcoin theme
pub const PRIMARY_RED: Color = Color::from_rgb(0.77, 0.04, 0.04); // #C50909
pub const ON_PRIMARY: Color = Color::from_rgb(1.0, 1.0, 1.0); // #FFFFFF
pub const BACKGROUND: Color = Color::from_rgb(0.96, 0.96, 0.96); // #F5F5F5
pub const SURFACE: Color = Color::from_rgb(1.0, 1.0, 1.0); // #FFFFFF
pub const TEXT: Color = Color::from_rgb(0.08, 0.09, 0.11); // #15171C
pub const TEXT_MUTED: Color = Color::from_rgb(0.44, 0.45, 0.49); // #70747D
pub const BORDER: Color = Color::from_rgb(0.79, 0.79, 0.80); // #C9CACD
pub const GREY_LIGHT: Color = Color::from_rgb(0.91, 0.91, 0.91); // #E8E8E8

pub const WHITE: Color = ON_PRIMARY;
pub const GREY_DARK: Color = TEXT_MUTED;

pub fn primary_button(status: iced::widget::button::Status) -> iced::widget::button::Style {
    let base = iced::widget::button::Style {
        background: Some(iced::Background::Color(PRIMARY_RED)),
        text_color: WHITE,
        border: Border {
            color: PRIMARY_RED,
            width: 0.0,
            radius: 8.0.into(),
        },
        shadow: Default::default(),
    };
    match status {
        iced::widget::button::Status::Hovered => iced::widget::button::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.85, 0.05, 0.05))),
            ..base
        },
        iced::widget::button::Status::Pressed => iced::widget::button::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.65, 0.03, 0.03))),
            ..base
        },
        iced::widget::button::Status::Disabled => iced::widget::button::Style {
            background: Some(iced::Background::Color(GREY_LIGHT)),
            text_color: GREY_DARK,
            border: Border {
                color: BORDER,
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: Default::default(),
        },
        _ => base,
    }
}

pub fn secondary_button(status: iced::widget::button::Status) -> iced::widget::button::Style {
    let base = iced::widget::button::Style {
        background: Some(iced::Background::Color(SURFACE)),
        text_color: TEXT,
        border: Border {
            color: BORDER,
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: Default::default(),
    };
    match status {
        iced::widget::button::Status::Hovered => iced::widget::button::Style {
            background: Some(iced::Background::Color(BACKGROUND)),
            ..base
        },
        iced::widget::button::Status::Pressed => iced::widget::button::Style {
            background: Some(iced::Background::Color(GREY_LIGHT)),
            ..base
        },
        _ => base,
    }
}

pub fn card_container() -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(iced::Background::Color(SURFACE)),
        text_color: Some(TEXT),
        border: Border {
            color: BORDER,
            width: 1.0,
            radius: 12.0.into(),
        },
        shadow: Default::default(),
    }
}

pub fn log_container() -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(iced::Background::Color(SURFACE)),
        text_color: Some(TEXT),
        border: Border {
            color: BORDER,
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: Default::default(),
    }
}

pub fn styled_text_input(status: iced::widget::text_input::Status) -> iced::widget::text_input::Style {
    let base = iced::widget::text_input::Style {
        background: iced::Background::Color(SURFACE),
        border: Border {
            color: BORDER,
            width: 1.0,
            radius: 8.0.into(),
        },
        icon: TEXT,
        placeholder: TEXT_MUTED,
        value: TEXT,
        selection: GREY_LIGHT,
    };
    match status {
        iced::widget::text_input::Status::Focused => iced::widget::text_input::Style {
            border: Border {
                color: PRIMARY_RED,
                width: 2.0,
                radius: 8.0.into(),
            },
            ..base
        },
        iced::widget::text_input::Status::Hovered => iced::widget::text_input::Style {
            border: Border {
                color: TEXT_MUTED,
                width: 1.0,
                radius: 8.0.into(),
            },
            ..base
        },
        _ => base,
    }
}
