![The MSC logo](logo.svg)

# MatrixStack-Code
is a 2d, stack-based, esoteric language.

The pointer traverses a plane / matrix of instructions.
Each 4x4 block of instructions corresponds to a stack.

The language spec is in [LANGUAGE.md](LANGUAGE.md).

## Using the interpreter

Programs can be run using their file paths as command line arguments:
```sh
msc <paths...>
```
This also allows using shebangs on Unix-like systems!

They can be piped in via stdin:
```sh
msc < <path>
```
However, the program will not be able to take inputs!

Or you can write programs straight into it:
```sh
msc
```
Write your program, then press `Enter, Ctrl + D` on Unix-like systems or `Enter, Ctrl + Z` on Windows to run it.  
There is no way to save your programs from here, but you can copy them from your terminal!

## Examples

There are example files in the [examples](examples) directory.
Run one with `msc examples/<name>.msc`
or `cargo run examples/<name>.msc`

## Installing with cargo

Make sure you have [cargo](https://rustup.rs/) installed.
```sh
cargo install --git https://github.com/tomboddaert/msc
```

## Building & Installing

Build:
```sh
cargo build --release
```

Local install on Linux:
```sh
> In ~/.local/bin
ln -s $(realpath target/release/msc) ~/.local/bin

> In ~/.bin
ln -s $(realpath target/release/msc) ~/.bin
```

## License

<p xmlns:cc="http://creativecommons.org/ns#" xmlns:dct="http://purl.org/dc/terms/"><a property="dct:title" rel="cc:attributionURL" href="https://github.com/tomboddaert/msc">MSCode</a> by <a rel="cc:attributionURL dct:creator" property="cc:attributionName" href="https://tomboddaert.com/">Tom Boddaert</a> is licensed under <a href="http://creativecommons.org/licenses/by/4.0/?ref=chooser-v1" target="_blank" rel="license noopener noreferrer" style="display:inline-block;">CC BY 4.0<img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/cc.svg?ref=chooser-v1"><img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/by.svg?ref=chooser-v1"></a></p>
