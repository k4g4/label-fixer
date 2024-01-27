#![cfg_attr(windows, windows_subsystem = "window")]

use iced::{
    alignment, executor, mouse,
    widget::{
        self,
        canvas::{self, Frame, Geometry, Program},
        column, image, row, Canvas,
    },
    window, Application, Command, Element, Length, Rectangle, Renderer, Settings, Theme, Vector,
};
use label_fixer::fix_label;

const SIZE: (u32, u32) = (600, 600);
const MIN_SIZE: (u32, u32) = (200, 400);
const PRINTER_EMOJI: char = '🖶';
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

#[derive(Default)]
enum App {
    #[default]
    Empty,
    Loaded(image::Handle),
}

#[derive(Debug, Clone, Hash)]
enum Message {
    Load,
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
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let open_button = widget::button("Open PDF")
            .padding(10)
            .on_press(Message::Load);

        let bottom_bar = row![
            widget::horizontal_space(Length::Fill),
            open_button,
            widget::horizontal_space(Length::Fill)
        ]
        .padding(20);

        let sep = widget::container("")
            .style(|theme: &Theme| widget::container::Appearance {
                border_width: 2.0,
                border_color: theme.palette().primary,
                ..Default::default()
            })
            .width(Length::Fill)
            .max_height(2);

        match self {
            Self::Empty => column![
                Canvas::new(PrinterEmoji)
                    .height(Length::Fill)
                    .width(Length::Fill),
                sep,
                bottom_bar,
            ],
            Self::Loaded(handle) => column![image::viewer(handle.clone()), sep, bottom_bar],
        }
        .into()
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
