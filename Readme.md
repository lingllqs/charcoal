# Charcoal

An alternative to wudao-dict. With colorized output and optional speech.

## Installation

Arch (-ish):

```sh
yay -S charcoal
```

Brew:

```sh
brew install lighghteeloo/charcoal/charcoal
```

Build from source:

```sh
cargo install charcoal-dict
```

### Small Tips Ahead

Aliasing common command to `chr`, `wd` or anything shorter is recommended (`cc` if you don't mind?)

```sh
alias chr="charcoal query -s"
```

No manually designated dependencies are required. See `Cargo.toml` if you are interested in the crates charcoal is using.


## Usage

### Query

```sh
charcoal query <QUERY>
```

where query can be shortened as `q`, `search`, or `s`.

With `-s` or `--speak-as true` one can force the happening of a speech.

### Edit

You may want to edit the configuration file in an easy way. `charcoal` gets you covered:

```sh
charcoal edit
```

And with `--reset` it will generate a brand new configuration in case anything gets wrong.

For more details on configuration file, see *Configuration* section.

### Clean

Charcoal caches up your queries, both text and audio. While it's nice for repetitive queries, it takes up some space. If you want, you can clean them with:

```sh
charcoal clean
```

For more details on cache, see *Caching Strategy* section.

### Help

To see more options, run

```sh
charcoal help
```

for help on subcommands, or

```sh
charcoal query --help
```

for each subcommand, say, `query`.


## Configuration

Configurations are straight forward. Just change the booleans and they're yours.

## Dev

Clone and build from source:

```sh
git clone git@github.com:lingllqs/charcoal.git
cd charcoal
cargo build --all
```

Alias:

```sh
cd charcoal
cargo build --all
alias chr="RUST_LOG=info target/debug/charcoal query -s"
```

## Caching Strategy

As a tiny cli tool, `charcoal` can't guarantee 100% cache consistency; however, its caching strategy is delicately designed such that inconsistency is rare and of little harm.

Only ascii with out space will be saved by name, under `cache` directory, to achieve better compatibility; the rest will be hashed and then saved under `vault` directory.

Both text and audio will be cached.

## License

This repo uses MIT License.

## Disclaimer and Promises

Though it's unlikely, this software may harm data. Use at your own risk.

No privacy collection will be performed.

## Credits

@TomCC7 for kindly supporting, testing and making suggestions for `charcoal`.

@BinhaoQin for providing advice (and patience!) on PKGBUILD.

@Gorgeous-Patrick for updating outdated readme.
