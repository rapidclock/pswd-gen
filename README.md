# pswd-gen [![Build Status](https://travis-ci.com/rapidclock/pswd-gen.svg?token=hJhLfHtyz41UyuLTTdFx&branch=master)](https://travis-ci.com/rapidclock/pswd-gen)
<br>
A Customizable CLI Random Password Generator

## Instructions:

```
This is a Configurable Random Password Generator, which generates a random password of a specified size. 
The composition of the randomly generated string can be configured with the options of the CLI Tool. 
Check the help menu for available options.

USAGE:
    pswd-gen [FLAGS] [OPTIONS]

FLAGS:
    -d, --digits        Allows the presence of digits in the generated string
    -h, --help          Prints help information
    -l, --lower-case    Allows the presence of lowercase alphabets in the generated string
    -u, --upper-case    Allows the presence of uppercase alphabets in the generated string
    -V, --version       Prints version information

OPTIONS:
    -s, --req-size <size>    Used to specify the desired size of the generated password [default: 12]
```
