# adventOfCode2020

![Rust](https://github.com/JimmyCastiel/adventOfCode2020/workflows/Rust/badge.svg)

adventOfCode2020 is a rust participation to [Advent Of Code 2020](https://adventofcode.com/2020) event.

## Installation

Use the rust bin cargo to run the project.

```bash
git clone git@github.com:JimmyCastiel/adventOfCode2020.git
```

## Usage

```bash
cd adventOfCode2020
cargo run --package adventOfCode2020 --bin <day_to_run>
```
Special cases (i.e. day4)

```bash
cargo run --package adventOfCode2020 --bin day4 --features day4
```

If you want to build and run binary with production performances :

```bash
cargo build --package adventOfCode2020 --bin <day_to_build> --release
./target/release/<day_to_build>
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.
