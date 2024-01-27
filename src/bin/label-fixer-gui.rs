#![cfg_attr(windows, windows_subsystem = "windows")]

use iced::{
    alignment, color, executor, mouse, theme,
    widget::{
        self,
        canvas::{self, Frame, Geometry, Program},
        column, image, row, Button, Canvas,
    },
    window, Application, Background, Color, Command, Element, Length, Rectangle, Renderer,
    Settings, Theme, Vector,
};
use label_fixer::fix_label;

const SIZE: (u32, u32) = (600, 600);
const MIN_SIZE: (u32, u32) = (200, 400);
const PRINTER_EMOJI: &str = "🖨️";
const PRINTER_ICON: &[u8] = include_bytes!("../../printer.ico");

fn main() -> iced::Result {
    App::run(Settings {
        window: window::Settings {
            size: SIZE,
            position: window::Position::Centered,
            min_size: Some(MIN_SIZE),
            icon: Some(window::icon::from_file_data(PRINTER_ICON, None).unwrap()),
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug, Clone)]
struct Printer(printers::printer::Printer);

impl PartialEq for Printer {
    fn eq(&self, other: &Self) -> bool {
        self.0.name == other.0.name
    }
}

impl Eq for Printer {}

#[derive(Default)]
enum App {
    #[default]
    Empty,
    Loaded(image::Handle),
    Printers(Vec<Printer>),
    Printing(Printer),
}

#[derive(Debug, Clone)]
enum Message {
    Load,
    Print,
    PrinterSelected(Printer),
    Return,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        "Label Fixer".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Load => {
                match native_dialog::FileDialog::new()
                    .set_title("Open PDF")
                    .show_open_single_file()
                {
                    Ok(Some(path)) => {
                        match fix_label(path) {
                            Ok(out_path) => {
                                *self = Self::Loaded(image::Handle::from_path(out_path));
                            }
                            Err(error) => {
                                native_dialog::MessageDialog::new()
                                    .set_title("Error")
                                    .set_text(&format!("{error}"))
                                    .show_alert()
                                    .unwrap();
                            }
                        }
                        Command::none()
                    }
                    _ => Command::none(),
                }
            }

            Message::Print => {
                *self = Self::Printers(printers::get_printers().into_iter().map(Printer).collect());
                Command::none()
            }

            Message::PrinterSelected(printer) => {
                *self = Self::Printing(printer);
                Command::none()
            }

            Message::Return => {
                *self = Self::Empty;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        fn hot_pink() -> Color {
            color!(0xf7007c)
        }

        struct PinkButtonTheme;

        impl widget::button::StyleSheet for PinkButtonTheme {
            type Style = Theme;

            fn active(&self, style: &Self::Style) -> widget::button::Appearance {
                widget::button::Appearance {
                    border_radius: 15.0.into(),
                    background: Some(Background::Color(hot_pink())),
                    ..style.active(&theme::Button::Primary)
                }
            }
        }

        fn pink_button(title: &str, message: Message) -> Button<'_, Message> {
            widget::button(title)
                .style(theme::Button::custom(PinkButtonTheme))
                .padding(10)
                .on_press(message)
        }

        let open_button = pink_button("Open Label PDF", Message::Load);
        let print_button = pink_button("Print Label", Message::Print);
        let sep = widget::container("")
            .style(|_: &Theme| widget::container::Appearance {
                border_width: 2.0,
                border_color: hot_pink(),
                ..Default::default()
            })
            .width(Length::Fill)
            .max_height(2);

        match self {
            Self::Empty => {
                let bottom_bar = row![
                    widget::horizontal_space(Length::Fill),
                    open_button,
                    widget::horizontal_space(Length::Fill)
                ]
                .padding(20);

                column![
                    Canvas::new(PrinterEmoji)
                        .height(Length::Fill)
                        .width(Length::Fill),
                    sep,
                    bottom_bar,
                ]
                .into()
            }

            Self::Loaded(handle) => {
                let bottom_bar = row![
                    widget::horizontal_space(Length::Fill),
                    open_button,
                    widget::horizontal_space(20),
                    print_button,
                    widget::horizontal_space(Length::Fill)
                ]
                .padding(20);

                column![
                    image::viewer(handle.clone())
                        .height(Length::Fill)
                        .width(Length::Fill),
                    sep,
                    bottom_bar
                ]
                .into()
            }

            Self::Printers(printers) => {
                let buttons = {
                    let mut buttons = vec![];
                    for printer in printers {
                        buttons.push(
                            pink_button(
                                printer.0.name.as_str(),
                                Message::PrinterSelected(printer.clone()),
                            )
                            .into(),
                        );
                        buttons.push(widget::vertical_space(30).into());
                    }
                    buttons.push(pink_button("Return", Message::Return).into());
                    buttons
                };

                column(buttons).into()
            }

            Self::Printing(printer) => {
                widget::text(format!("Printing to {}...", printer.0.name)).into()
            }
        }
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

struct PrinterEmoji;

impl Program<Message> for PrinterEmoji {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer<Theme>,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        frame.translate(Vector::new(bounds.width * 0.5, bounds.height * 0.25));
        frame.fill_text(canvas::Text {
            content: PRINTER_EMOJI.to_string(),
            shaping: widget::text::Shaping::Advanced,
            size: 100.0 + bounds.height * 0.3,
            horizontal_alignment: alignment::Horizontal::Center,
            ..Default::default()
        });
        vec![frame.into_geometry()]
    }
}
