
# PyIFT

Image-Foresting Transform algorithms in Python (and rust).

This is an experimental implementation of the [pyift](https://github.com/PyIFT/pyift) using [rust](https://www.rust-lang.org/).

The rust ift library is wrapped using [pyo3](https://github.com/PyO3/pyo3).

## Installation

First clone the repository and move into the directory:

```bash
git clone git@github.com:JoOkuma/pyift_rust.git
cd pyift_rust
```

To install it using pip, run:

```bash
pip install -e .
```

To install in development mode, run the command below, this requires having rust and cargo installed.

```bash
cargo build --no-default-features
```

To install in release mode, run the command below, it makes a huge difference.

```bash
cargo build --release --no-default-features
```

## Usage

```bash
python examples/watershed_from_minima.py
```
