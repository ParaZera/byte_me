use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
}

impl DataType {
    pub fn all() -> Vec<DataType> {
        vec![
            DataType::U8,
            DataType::U16,
            DataType::U32,
            DataType::U64,
            DataType::U128,
            DataType::I8,
            DataType::I16,
            DataType::I32,
            DataType::I64,
            DataType::I128,
            DataType::F32,
            DataType::F64,
        ]
    }

    pub fn to_string(&self) -> String {
        match self {
            DataType::U8 => "u8".to_string(),
            DataType::U16 => "u16".to_string(),
            DataType::U32 => "u32".to_string(),
            DataType::U64 => "u64".to_string(),
            DataType::U128 => "u128".to_string(),
            DataType::I8 => "i8".to_string(),
            DataType::I16 => "i16".to_string(),
            DataType::I32 => "i32".to_string(),
            DataType::I64 => "i64".to_string(),
            DataType::I128 => "i128".to_string(),
            DataType::F32 => "f32".to_string(),
            DataType::F64 => "f64".to_string(),
        }
    }

    pub fn to_short_string(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
    Little,
    Big,
}

impl Endianness {
    pub fn all() -> Vec<Endianness> {
        vec![Endianness::Little, Endianness::Big]
    }

    pub fn to_string(&self) -> String {
        match self {
            Endianness::Little => "Little".to_string(),
            Endianness::Big => "Big".to_string(),
        }
    }

    pub fn to_short_string(&self) -> String {
        match self {
            Endianness::Little => "LE".to_string(),
            Endianness::Big => "BE".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFormat {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

impl InputFormat {
    pub fn all() -> Vec<InputFormat> {
        vec![
            InputFormat::Binary,
            InputFormat::Octal,
            InputFormat::Decimal,
            InputFormat::Hexadecimal,
        ]
    }

    pub fn to_string(&self) -> String {
        match self {
            InputFormat::Binary => "Binary".to_string(),
            InputFormat::Octal => "Octal".to_string(),
            InputFormat::Decimal => "Decimal".to_string(),
            InputFormat::Hexadecimal => "Hex".to_string(),
        }
    }

    pub fn to_short_string(&self) -> String {
        match self {
            InputFormat::Binary => "Bin".to_string(),
            InputFormat::Octal => "Oct".to_string(),
            InputFormat::Decimal => "Dec".to_string(),
            InputFormat::Hexadecimal => "Hex".to_string(),
        }
    }

    pub fn is_valid_char(&self, c: char) -> bool {
        match self {
            InputFormat::Binary => c == '0' || c == '1',
            InputFormat::Octal => c >= '0' && c <= '7',
            InputFormat::Decimal => c.is_ascii_digit(),
            InputFormat::Hexadecimal => {
                c.is_ascii_digit() || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F')
            }
        }
    }

    pub fn number_prefix(&self) -> &'static str {
        match self {
            InputFormat::Binary => "0b",
            InputFormat::Octal => "0o",
            InputFormat::Decimal => "0d",
            InputFormat::Hexadecimal => "0x",
        }
    }
}

pub struct App {
    pub input: String,
    pub input_mode: bool,
    pub dropdown_open: bool,
    pub active_dropdown: usize,
    pub dropdown_index: usize,
    pub data_type: DataType,
    pub endianness: Endianness,
    pub input_format: InputFormat,
}

impl App {
    pub fn new() -> Self {
        App {
            input: String::new(),
            input_mode: false,
            dropdown_open: false,
            active_dropdown: 0,
            dropdown_index: 0,
            data_type: DataType::U32,
            endianness: Endianness::Little,
            input_format: InputFormat::Decimal,
        }
    }

    pub fn handle_input(&mut self, key: KeyEvent) {
        if self.dropdown_open {
            self.handle_dropdown_input(key);
        } else if self.input_mode {
            self.handle_text_input(key);
        } else {
            self.handle_navigation_input(key);
        }
    }

    fn handle_dropdown_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.dropdown_open = false;
                self.dropdown_index = 0;
            }
            KeyCode::Enter => {
                self.select_dropdown_item();
                self.dropdown_open = false;
                self.dropdown_index = 0;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.dropdown_index > 0 {
                    self.dropdown_index -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let max_index = match self.active_dropdown {
                    0 => DataType::all().len() - 1,
                    1 => Endianness::all().len() - 1,
                    2 => InputFormat::all().len() - 1,
                    _ => 0,
                };
                if self.dropdown_index < max_index {
                    self.dropdown_index += 1;
                }
            }
            _ => {}
        }
    }

    fn handle_text_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = false;
            }
            KeyCode::Char(c) => {
                if self.input_format.is_valid_char(c) {
                    self.input.push(c);
                }
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            _ => {}
        }
    }

    fn handle_navigation_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('1') | KeyCode::Char('m') => {
                self.active_dropdown = 0;
                self.dropdown_open = true;
            }
            KeyCode::Char('2') | KeyCode::Char(',') => {
                self.active_dropdown = 1;
                self.dropdown_open = true;
            }
            KeyCode::Char('3') | KeyCode::Char('.') => {
                self.active_dropdown = 2;
                self.dropdown_open = true;
            }
            KeyCode::Char('i') => {
                self.input_mode = true;
            }
            KeyCode::Tab => {
                self.active_dropdown = (self.active_dropdown + 1) % 3;
                self.dropdown_open = true;
            }
            KeyCode::Char('l') => {
                self.active_dropdown = (self.active_dropdown + 1) % 3;
            }
            KeyCode::Char('h') => {
                if self.active_dropdown == 0 {
                    self.active_dropdown = 2;
                } else {
                    self.active_dropdown = (self.active_dropdown - 1) % 3;
                }
            }
            KeyCode::Enter => {
                self.dropdown_open = true;
            }
            _ => {}
        }
    }

    fn select_dropdown_item(&mut self) {
        match self.active_dropdown {
            0 => {
                self.data_type = DataType::all()[self.dropdown_index];
            }
            1 => {
                self.endianness = Endianness::all()[self.dropdown_index];
            }
            2 => {
                self.input_format = InputFormat::all()[self.dropdown_index];
            }
            _ => {}
        }
    }

    pub fn get_conversion_results(&self) -> Vec<(String, String)> {
        if self.input.is_empty() {
            return vec![(
                "No input".to_string(),
                "Enter a value to see conversions".to_string(),
            )];
        }

        // Convert the input string to a value based on the input format
        let value = match self.parse_input() {
            Some(v) => v,
            None => return vec![("Error".to_string(), "Failed to parse input".to_string())],
        };

        // Generate conversion results using the converter trait
        crate::converter::get_conversions(value, self.data_type, self.endianness)
    }

    fn parse_input(&self) -> Option<u128> {
        let radix = match self.input_format {
            InputFormat::Binary => 2,
            InputFormat::Octal => 8,
            InputFormat::Decimal => 10,
            InputFormat::Hexadecimal => 16,
        };

        u128::from_str_radix(&self.input, radix).ok()
    }
}
