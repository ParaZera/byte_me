# byte_me

## Overview
`byte_me` is a terminal user interface (TUI) application that allows users to input numbers in various formats (binary, octal, decimal, or hexadecimal) and convert them into different representations, including little endian, big endian, and floating-point formats.

## Features
- Centered input field for entering numbers in multiple formats.
- Conversion of input numbers into little endian, big endian, and floating-point representations.
- User-friendly interface built using the `ratatui` crate.

## Project Structure
```
byte_me
├── src
│   ├── main.rs          # Entry point of the application
│   ├── app.rs           # Main application logic
│   ├── ui               # User interface components
│   │   ├── mod.rs       # UI module
│   │   └── input.rs     # Input field handling
│   ├── converters        # Conversion logic
│   │   ├── mod.rs       # Converters module
│   │   ├── traits.rs    # Conversion traits
│   │   ├── endian.rs     # Endian conversion logic
│   │   └── float.rs     # Floating-point conversion logic
│   └── utils            # Utility functions
│       └── mod.rs       # Utilities module
├── Cargo.toml           # Project configuration and dependencies
└── README.md            # Project documentation
```

## Setup Instructions
1. Clone the repository:
   ```
   git clone <repository-url>
   cd byte_me
   ```

2. Build the project:
   ```
   cargo build
   ```

3. Run the application:
   ```
   cargo run
   ```

## Usage
- Launch the application and enter a number in the input field.
- Use the provided options to toggle between different number formats.
- View the converted values displayed in the TUI.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request for any enhancements or bug fixes.

## License
This project is licensed under the MIT License. See the LICENSE file for details.