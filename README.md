
# `trashctl`

A command line interface for trash

[![GPLv3 License](https://img.shields.io/badge/License-GPL%20v3-yellow.svg)](https://opensource.org/licenses/)
[![Continuous integration](https://github.com/0xMRTT/trashctl/actions/workflows/matrix.yml/badge.svg)](https://github.com/0xMRTT/trashctl/actions/workflows/matrix.yml)
[![Deploy](https://github.com/0xMRTT/trashctl/actions/workflows/deploy.yml/badge.svg)](https://github.com/0xMRTT/trashctl/actions/workflows/deploy.yml)
[![Rust](https://github.com/0xMRTT/trashctl/actions/workflows/rust.yml/badge.svg)](https://github.com/0xMRTT/trashctl/actions/workflows/rust.yml)


## Features

- Add file to trash
- List files
- Permanently delete a file
- Restore file
- Empty the trash


## Documentation

If you want a complete [documentation](https://docs.rs/trashctl), see the doc on [docs.rs](https://docs.rs/trashctl). Otherwise, read the [book](https://0xMRTT.github.io/trashctl)


## Screenshots

![App Screenshot](https://via.placeholder.com/468x300?text=App+Screenshot+Here)


## Installation

Install with `cargo`

```bash
cargo install trashctl
```

### AUR

Install from the AUR

```bash
yay -S trashctl
```

### Install from source

For this, you need `git` and `rustup`

```bash
git clone https://github.com/0xMRTT/trashctl
cd trashctl
cargo b
./target/debug/trashctl
```
    
### Use github releases

Download binaries on the releases page (in the right pane)
## Usage/Examples

TODO
```shell
trashctl put foo
```


## Contributing

Contributions are always welcome!

See [`CONTRIBUTING.md`](./CONTRIBUTING.md) for ways to get started.

Please adhere to this project's [`CODE_OF_CONDUCT`](./CODE_OF_CONDUCT.md).


## Running Tests

To run tests, run the following command

```bash
cargo test
```


## Feedback

If you have any feedback, please open an discussion.


## Support

For support, open an discussion or join the matrix channel.


## FAQ

#### What can I do if y accidently `empty` my trash ?

Stop using your system. Boot into a live cd and use tools like `photo rec`



## Authors and contributors

- [@0xMRTT](https://www.github.com/0xMRTT)
- [Maybe you ?](#Contributing)


## Tech Stack

**cli:** `clap`, `clap_complete`



## Related

Here are some related projects

[`trash-cli`](https://github.com/andreafrancia/trash-cli)


## Acknowledgements

 - [Clap](https://docs.rs/clap)
## License

[GPLv3](https://choosealicense.com/licenses/gpl-3.0/)

