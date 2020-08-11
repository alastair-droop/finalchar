Display the Final Character of a File

The `finalchar` tool displays the final character in the given input file(s). This is mainly of use when searching for truncated files.

## Usage

~~~
finalchar 0.1.0

USAGE:
    finalchar [OPTIONS] [path]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output-format <output-fmt>     [default: hex]

ARGS:
    <path>...    
~~~

The `output-format` flag can take one of the following options:

* `dec`: The decimal representation of the final character is displayed;
* `hex`: The hexidecimal representation of the final character is displayed;
* `char`: The UTF8 character representation of the final character is displayed;
* `newline`: `newline` is displayed if the character is a newline character, `other` otherwise.

**NB**: For the `newline` output mode, the following characters are considered to be newline characters:

* `LF` (0x0A);
* `CR` (0x0D);
* `NL` (0x15);
* `RS` (0x1E)

## Building

Before installation, you'll need to install [Rust](https://www.rust-lang.org/).

~~~bash
$ git clone https://github.com/alastair-droop/finalchar.git
$ cd finalchar
$ cargo build --release
$ ./target/release/finalchar -V
finalchar 0.1.0
~~~
