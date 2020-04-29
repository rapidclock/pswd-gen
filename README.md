# pswd-gen

![Rust](https://github.com/rapidclock/pswd-gen/workflows/Rust/badge.svg)

A Customizable CLI Random Password Generator

## Usage

```
This is a Configurable Random Password Generator, which generates a random password of a specified size. The composition
of the randomly generated string can be configured with the options of the CLI Tool. Check the help menu for available
options.

USAGE:
    pswd-gen [FLAGS] [OPTIONS]

FLAGS:
    -d, --digits        Allows the presence of digits in the generated string
    -h, --help          Prints help information
    -l, --lower-case    Allows the presence of lowercase alphabets in the generated string
    -u, --upper-case    Allows the presence of uppercase alphabets in the generated string
    -v, --version       Prints version information

OPTIONS:
    -s, --req-size <size>      Used to specify the desired size of the generated password [default: 12]
    -y, --symbols <symbols>    Allows ascii symbols in the generated string. This is a non-delimited string containing
                               the various symbols that you need. Eg: "*&#$"
```

## Examples

- Only lowercase with default size (12 chars)
> pswd-gen -l

- lower + upper case only with size = 15
> pswd-gen -lus 15

or
> pswd-gen -lu -s 15

- upper + digits, default size
> pswd-gen -ld

- lower + upper plus symbols `$, #, @, *`
> pswd-gen -lu -y "$#@*"

- lower + upper + digits + symbols `, ; * $ -`, size = 10
> pswd-gen -lud -s 10 -y ",;*$-"

> pswd-gen -lud -y "#@!" -s 10
