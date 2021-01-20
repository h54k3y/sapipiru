pub mod styling_window {

    use crate::html_parser::handmade_html_parser;
    use crate::css_parser::handmade_css_parser;

    use iced::{
        button, text_input, scrollable, Button, Text, TextInput, Column, Scrollable,
        Container, Element, Length, Row, Sandbox,
        Settings, 
    };
    use std::io::Read;

    #[derive(Default)]
    struct Styling {
        input: text_input::State,
        input_value: String,
        button: button::State,
        scroll: scrollable::State,
        scroll_text: String,
    }

    #[derive(Debug, Clone)]
    enum Message {
        InputChanged(String),
        ButtonPressed,
    }

    pub fn initialize_window() {
        Styling::run(Settings::default());
    }

    impl Sandbox for Styling {
        type Message = Message;

        fn new() -> Self {
            Styling::default()
        }

        fn title(&self) -> String {
            String::from("Sapipiru")
        }

        fn update(&mut self, message: Message) {
            match message {
                Message::InputChanged(value) => self.input_value = value,
                Message::ButtonPressed => {
                    let mut html_text = String::new();
                    reqwest::blocking::get(&self.input_value).unwrap().read_to_string(&mut html_text);
                    self.scroll_text = handmade_html_parser::parse_html(&html_text);
                    /*handmade_html_parser::parse_html(&html_text);
                    self.scroll_text = handmade_css_parser::return_css_text();*/
                },
            }
        }

        fn view(&mut self) -> Element<Message> {

            let text_input = TextInput::new(
                &mut self.input,
                "Plese type URL",
                &self.input_value,
                Message::InputChanged,
            )
            .padding(10)
            .size(20);

            let button = Button::new(&mut self.button, Text::new("Submit"))
                .padding(10)
                .on_press(Message::ButtonPressed);

            let scrollable = Scrollable::new(&mut self.scroll)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(Text::new(&self.scroll_text));

            let content = Column::new()
                .spacing(20)
                .padding(20)
                .width(Length::Fill)
                .push(Row::new().spacing(10).push(text_input).push(button))
                .push(scrollable);

            Container::new(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .into()
        }
    }
}