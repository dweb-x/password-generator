### Password Generator

To build 
```shell
cargo build --release  
```
copy executable to `~/bin`

```shell
cp ./target/release/password ~/bin
```

### Usage:


```shell
password
```

Generates a random password, 36 characters in length.

to set a custom length 

```shell
password --length 128 
```
or just
```shell
password -l 128
```
There's a maximum length of 512 characters.

You can view help via

```shell
password --help
```
or just
```shell
password -h
```

```
Generates a random string of fixed length made up of the full range of 
alphanumeric characters and symbols acceptable for use in password strings

Usage: password [OPTIONS]

Options:
  -l, --length <LENGTH>  Number of characters, [max: 512]: [default: 36]
  -h, --help             Print help
  -V, --version          Print version

```