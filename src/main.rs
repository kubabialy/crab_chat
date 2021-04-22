use druid::widget::prelude::*;
use druid::widget::{Flex, Label, TextBox, Button};
use druid::{AppLauncher, Data, Lens, UnitPoint, WidgetExt, WindowDesc};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const BUTTON_BOX_WIDTH: f64 = 100.0;
const TEXT_BOX_WIDTH: f64 = 200.0;

#[derive(Clone, Data, Lens)]
struct AuthState {
    login: String,
    password: String
}

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title("Crab Chat 🦀")
        .window_size((800.0, 600.0));

    // create the initial app state
    let initial_state: AuthState = AuthState {
        login: "Crabber!!!".into(),
        password: "".into()
    };

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<AuthState> {
    // a label that will determine its text based on the current app data.
    let label = Label::new("Hey, who are you? Log in!")
        .with_text_size(32.0);

    let login = Label::new("Login");
    let textbox = TextBox::new()
        .with_placeholder("What's your email?")
        .with_text_size(18.0)
        .fix_width(TEXT_BOX_WIDTH)
        .lens(AuthState::login);

    let password = Label::new("Password");
    let password_textbox = TextBox::new()
        .with_placeholder("What's your password?")
        .with_text_size(18.0)
        .fix_width(TEXT_BOX_WIDTH)
        .lens(AuthState::password);

    let button = Button::new("Sing in").fix_width(BUTTON_BOX_WIDTH);

    // arrange the two widgets vertically, with some padding
    Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(login)
        .with_child(textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(password)
        .with_child(password_textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(button)
        .align_vertical(UnitPoint::CENTER)
}
