# crascii

## Overview
`crascii` is a Rust-based command-line tool that converts images into ASCII art. It allows for customization of the output dimensions, color, and character sets, and supports saving or printing the ASCII output directly to the console.

## Features
- Convert images to ASCII art.
- Adjustable output dimensions (width and height).
- Supports colored ASCII output.
- Customizable character sets for ASCII conversion.
- Save the output to a file or print it directly to the terminal.

## Requirements
- Rust (for building and running the tool).
- An image processing library compatible with Rust (e.g., `image`).

## Installation
1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd <repository-folder>
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the executable:
   ```bash
   ./target/release/crascii
   ```

## Usage
Run the tool from the terminal with the following options:

```bash
crascii [OPTIONS] --image <IMAGE> --output-path <OUTPUT_PATH>
```

### Options
- `-i, --image <IMAGE>`: Path to the input image file to be converted.
- `-w, --columns <COLUMNS>`: Number of columns (width) for the ASCII output (optional).
- `-H, --lines <LINES>`: Number of lines (height) for the ASCII output (optional).
- `-C, --color`: Enable colored ASCII output (optional).
- `-c, --charsets <CHARSETS>`: Character set to use for ASCII conversion (default: "default").
- `-o, --output-path <OUTPUT_PATH>`: Path to save the ASCII output file.
- `-p, --print`: Print the ASCII art directly to the terminal (optional).
- `-h, --help`: Show the help message.

### Example Commands
1. Convert an image to ASCII and save it:
   ```bash
   crascii -i my_image.png -o output.txt
   ```

2. Specify custom dimensions for the output:
   ```bash
   crascii -i my_image.png -w 100 -H 50 -o output.txt
   ```

## Development
1. Install Rust: [Rust installation guide](https://www.rust-lang.org/tools/install).
2. Install dependencies:
   ```bash
   cargo build
   ```
3. Run the tool:
   ```bash
   cargo run -- [OPTIONS]
   ```

## Contributing
Contributions are welcome! Feel free to submit issues or pull requests to enhance the tool.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments
- Rust community for support and documentation.


