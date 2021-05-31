use iced::{ button, executor, Align, Application, Button, Clipboard, Column, Command, Container, Element, Length, HorizontalAlignment, Row, Settings, Text };


pub fn main() -> iced::Result {
    Calculator::run(Settings::default())
}

struct Calculator {
    x: String,
    y: String,
    operation: String,
    result: f64,
    state: State,
    key_1: button::State,
    key_2: button::State,
    key_3: button::State,
    key_4: button::State,
    key_5: button::State,
    key_6: button::State,
    key_7: button::State,
    key_8: button::State,
    key_9: button::State,
    key_0: button::State,
    key_decimal: button::State,
    key_equal: button::State,
    key_add: button::State,
    key_sub: button::State,
    key_mul: button::State,
    key_div: button::State,
    key_clear: button::State,
}

enum State {
    Input1,
    Input2,
}

#[derive(Debug, Clone)]
enum Message {
    Number1,
    Number2,
    Number3,
    Number4,
    Number5,
    Number6,
    Number7,
    Number8,
    Number9,
    Number0,
    Add,
    Sub,
    Multiply,
    Divide,
    Solve,
    Clear,
    Decimal
}

impl Application for Calculator {

    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Calculator, Command<Self::Message>) {
        (
            Calculator {
                state: State::Input1,
                x: String::from(""),
                y: String::from(""),
                operation: String::from(""),
                result: 0.0,
                key_1: button::State::new(),
                key_2: button::State::new(),
                key_3: button::State::new(),
                key_4: button::State::new(),
                key_5: button::State::new(),
                key_6: button::State::new(),
                key_7: button::State::new(),
                key_8: button::State::new(),
                key_9: button::State::new(),
                key_0: button::State::new(),
                key_decimal: button::State::new(),
                key_equal: button::State::new(),
                key_add: button::State::new(),
                key_sub: button::State::new(),
                key_mul: button::State::new(),
                key_div: button::State::new(),
                key_clear: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Rusty Icy Calculator")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {

        match message {
            Message::Number1 => match self.state {
                State::Input1 => {
                    self.x.push_str("1");
                    println!("Num1 is {}", self.x);
                }
                State::Input2 => {
                    self.y.push_str("1");
                    println!("Num2 is {}", self.y);
                }
            }

            Message::Number2 => match self.state {
                State::Input1 => {
                    self.x.push_str("2");
                    println!("Num1 is {}", self.x);
                }
                State::Input2 => {
                    self.y.push_str("2");
                    println!("Num2 is {}", self.y);
                }
            }

            Message::Number3 => match self.state {
                State::Input1 => {
                    self.x.push_str("3");
                    println!("Num1 is {}", self.x);
                }
                State::Input2 => {
                    self.y.push_str("3");
                    println!("Num2 is {}", self.y);
                }
            }

            Message::Number4 => match self.state {
                State::Input1 => {
                    self.x.push_str("4");
                    println!("Num1 is {}", self.x);
                }
                State::Input2 => {
                    self.y.push_str("4");
                    println!("Num2 is {}", self.y);
                }
            }

            Message::Number5 => match self.state {
                State::Input1 => {
                    self.x.push_str("5");
                    println!("Num1 is {}", self.x);
                }
                State::Input2 => {
                    self.y.push_str("5");
                    println!("Num2 is {}", self.y);
                }
            }

            Message::Number6 => match self.state {
                State::Input1 => {
                    self.x.push_str("6");
                    println!("Num1 is {}", self.x);
                }
                State::Input2 => {
                    self.y.push_str("6");
                    println!("Num2 is {}", self.y);
                }
            }

            Message::Number7 => match self.state {
                State::Input1 => {
                    self.x.push_str("7");
                    println!("Num1 is {}", self.x);
                }
                State::Input2 => {
                    self.y.push_str("7");
                    println!("Num2 is {}", self.y);
                }
            }

            Message::Number8 => match self.state {
                State::Input1 => {
                    self.x.push_str("8");
                    println!("Num1 is {}", self.x);
                }
                State::Input2 => {
                    self.y.push_str("8");
                    println!("Num2 is {}", self.y);
                }
            }

            Message::Number9 => match self.state {
                State::Input1 => {
                    self.x.push_str("9");
                    println!("Num1 is {}", self.x);
                }
                State::Input2 => {
                    self.y.push_str("9");
                    println!("Num2 is {}", self.y);
                }
            }

            Message::Number0 => match self.state {
                State::Input1 => {
                    self.x.push_str("0");
                    println!("Num1 is {}", self.x);
                }
                State::Input2 => {
                    self.y.push_str("0");
                    println!("Num2 is {}", self.y);
                }
            }

            Message::Add => match &mut self.state {
                State::Input1 => {
                    self.operation = String::from("+");
                    self.state = State::Input2;
                    println!("{}", self.operation);
                }
                State::Input2 => {
                    
                }
            }

            Message::Sub => match &mut self.state {
                State::Input1 => {
                    self.operation = String::from("-");
                    self.state = State::Input2;
                    println!("{}", self.operation);
                }
                State::Input2 => {

                }
            }

            Message::Multiply => match &mut self.state {
                State::Input1 => {
                    self.operation = String::from("*");
                    self.state = State::Input2;
                    println!("{}", self.operation);
                }
                State::Input2 => {

                }
            }

            Message::Divide => match &mut self.state {
                State::Input1 => {
                    self.operation = String::from("/");
                    self.state = State::Input2;
                    println!("{}", self.operation);
                }
                State::Input2 => {

                }
            }

            Message::Solve => match &mut self.state {
                State::Input1 => {
                    self.result = self.x.parse().unwrap();
                    println!("Result: {}", self.result);
                }
                State::Input2 => {
                    let x: f64 = self.x.parse().unwrap();
                    let y: f64 = self.y.parse().unwrap();

                    if self.operation == "+" {
                        self.result = x + y;
                    }
                    else if self.operation == "-" {
                        self.result = x - y;
                    }
                    else if self.operation == "*" {
                        self.result = x * y;
                    }
                    else if self.operation == "/" {
                        self.result = x / y;
                    }
                    println!("Result: {}", self.result);
                    self.x = String::from("");
                    self.y = String::from("");
                    self.operation = String::from("");
                    self.state = State::Input1;
                }
            }

            Message::Clear => match &mut self.state {
                State::Input1 => {

                }
                State::Input2 => {

                }
            }

            Message::Decimal => match &mut self.state {
                State::Input1 => {

                }
                State::Input2 => {

                }
            }

        }
        
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let key = |state, label, style| {
            Button::new(
                state,
                Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
            )
            .min_width(80)
            .padding(10)
            .style(style)
        };

        let key_1 = key(&mut self.key_1, "1", style::Button::Primary)
            .on_press(Message::Number1);

        let key_2 = key(&mut self.key_2, "2", style::Button::Primary)
            .on_press(Message::Number2);

        let key_3 = key(&mut self.key_3, "3", style::Button::Primary)
            .on_press(Message::Number3);

        let key_4 = key(&mut self.key_4, "4", style::Button::Primary)
            .on_press(Message::Number4);

        let key_5 = key(&mut self.key_5, "5", style::Button::Primary)
            .on_press(Message::Number5);

        let key_6 = key(&mut self.key_6, "6", style::Button::Primary)
            .on_press(Message::Number6);

        let key_7 = key(&mut self.key_7, "7", style::Button::Primary)
            .on_press(Message::Number7);

        let key_8 = key(&mut self.key_8, "8", style::Button::Primary)
            .on_press(Message::Number8);

        let key_9 = key(&mut self.key_9, "9", style::Button::Primary)
            .on_press(Message::Number9);

        let key_0 = key(&mut self.key_0, "0", style::Button::Primary)
            .on_press(Message::Number0);
        
        let key_decimal = key(&mut self.key_decimal, ".", style::Button::Primary)
            .on_press(Message::Decimal);

        let key_equal = key(&mut self.key_equal, "=", style::Button::Primary)
            .on_press(Message::Solve);

        let key_clear = key(&mut self.key_clear, "CL", style::Button::Secondary)
            .on_press(Message::Clear);

        let key_div = key(&mut self.key_div, "/", style::Button::Secondary)
            .on_press(Message::Divide);

        let key_mul = key(&mut self.key_mul, "x", style::Button::Secondary)
            .on_press(Message::Multiply);

        let key_sub = key(&mut self.key_sub, "-", style::Button::Secondary)
            .on_press(Message::Sub);
            
        let key_add = key(&mut self.key_add, "+", style::Button::Secondary)
            .on_press(Message::Add);

        let keypad_row_1 = Row::new()
            .spacing(20)
            .push(key_7)
            .push(key_8)
            .push(key_9)
            .push(key_div);

        let keypad_row_2 = Row::new()
            .spacing(20)
            .push(key_4)
            .push(key_5)
            .push(key_6)
            .push(key_mul);
        
        let keypad_row_3 = Row::new()
            .spacing(20)
            .push(key_1)
            .push(key_2)
            .push(key_3)
            .push(key_sub);

        let keypad_row_4 = Row::new()
            .spacing(20)
            .push(key_0)
            .push(key_decimal)
            .push(key_clear)
            .push(key_add);

        let keypad = Column::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(keypad_row_1)
            .push(keypad_row_2)
            .push(keypad_row_3)
            .push(keypad_row_4)
            .push(key_equal);

        Container::new(keypad)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Primary,
        Secondary,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.74, 0.179, 0.223),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Operation {
    x: i64,
    y: i64,
    op: String,
    res: i64, 
}

impl Operation {
    fn add(&self) -> i64 {
        let res = self.x + self.y;
        res
    }
    
    fn sub(&self) -> i64 {
        let res = self.x - self.y;
        res
    }
    
    fn mul(&self) -> i64 {
        let res = self.x * self.y;
        res
    }
    
    fn div(&self) -> i64 {
        let res = self.x / self.y;
        res
    }
}
