# Path

This library exists as a way to replace `std::path` in `no_std` environments. 
Some design choices where made in order to be OS and FS agnostic.

However, some choices where made from personal path style preferences, and are open to discussion.

### These choices include:
- Path are UTF-8 as they are implemented with Rust's `String` and `str`. If another encoding is needed, please use `encoding_rs` or similar crates.
- `~` is not supported and always considered as a litteral as there is no concept of current directory.
- Path separator is `/`.
- Paths can not have a prefix such as driver letters or protocols.

## Special thanks
Special thanks to a friend for their huge help in optimization and design of this crate.

## License
MIT License
Copyright © 2025 OwOchlé
