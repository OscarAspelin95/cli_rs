# cli_rs
Toy example of a cli that prints the file contents of a directory.

## Requirements
- Linux OS (Ubuntu 24.04.2)
- Rust >= 1.88.0

## Installation
Clone the repository or download the source code. Enter the cli_rs directory and run:<br>
`cargo build --release`

The generated binary is available in `target/release/cli_rs`.

## Usage
Run with:<br>
`cli_rs -d <directory>`

Optional arguments:
<pre>
<b>--by-size</b> [false] - Sort files by size.

<b>--by-modified</b> [false] - Sort files by modification date.
</pre>

![image](https://github.com/OscarAspelin95/cli_rs/blob/main/assets/example.png)
