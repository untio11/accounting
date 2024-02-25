# Accounting
I want to get better insights in my money, so why not write something
convoluted from scratch in Rust.


## Dependencies
External dependencies:
- [Nix package manager](https://nixos.org/download#:~:text=Nix%3A%20the%20package%20manager)
  - [With Flakes enabled](https://nixos.wiki/wiki/Flakes)
- [Direnv](https://direnv.net/docs/installation.html)

This project is defined with a Nix Flake. This flake takes care of downloading
all the Rust dependencies. It also provides a shell environment for development
that have these dependencies available in the `PATH`.

This shell environment can be accessed in a few different ways:
- Through VS Code with the direnv extension (see [`.vscode/extensions.json`](.vscode/extensions.json))
- Through your normal shell with direnv
- Through your normal shell with `nix develop` (not recommended, as you
  won't have your normal shell config available there)

## Running the app
Still very much a work in progress. The app can currently only take a single
path to a `.csv` file and it will parse it and print it to stdout.
```shell
cargo run path/to/transactions.csv
```

There's currently no sample csv file with the correct format available, but 
I'll see if I'll make one that doesn't contain sensitive personal data.
