# Password Generator

A command-line utility that generates cryptographically secure, random passwords.

## Features

- 🔐 Cryptographically secure using ChaCha20
- 🎛️ Configurable password length (up to 512 characters)
- 🔤 Multiple character sets:
    - Alphanumeric (A-Z, a-z, 0-9)
    - Special symbols (!@#$%^&*()-_=+[]{}|;:,.<>?)
    - Extended symbols (`\"'/\\)
    - Optional space character
- 🛡️ No logging or storage of generated passwords
- 📦 Easy to install and use


## Quick Start

### Install

```shell
cargo install --path .
```

### Build

```shell
cargo build --release  
```

Copy the executable to your bin directory:

```shell
cp ./target/release/password ~/bin
```


## Usage:

### Basic Command Format

```shell
password [OPTIONS]
```

### Options:
```
  -l, --length <LENGTH>   Number of characters, [max: 512]: [default: 36]
  -n, --no-symbols        Exclude special symbols from the password
  -e, --extended-symbols  Include extended symbols set (`\"'/\)
  -s, --allow-space       Allow space character in password
  -h, --help              Print help
  -V, --version           Print version

```


## Examples

#### default (no args) 
Outputs a 36 digit password. Password will include alphanumeric characters and the symbols \!\@\#\$\%\^\&\*\(\)-\_\=\+\[\]\{\}\|\;\:\,\.\<\>\?

```shell
password
```
#### Set a custom length (128 characters)

```shell
password -l 128 
```

There's a maximum length of 512 characters.

#### Long password with all characters:

```shell
password -l 50 -e -s
```

#### Alphanumeric only:

```shell
password -n
```

#### With spaces but no symbols:
```shell
password -n -s
```

#### Maximum strength (all characters):
```shell
password -e -s
```


### Character Sets

- **Alphanumeric**: A-Z, a-z, 0-9
- **Symbols**: !@#$%^&*()-_=+[]{}|;:,.<>?
- **Extended**: `\"'/\\
- **Space**: When enabled with `-s`


### Security

- Uses ChaCha20 for cryptographically secure random generation
- Implements secure password generation best practices
- No logging or storage of generated passwords


### Version

Current version: 1.0.0


### Author

David Carruthers (david@dweb-x.com)


### License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
