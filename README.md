# Currency Exchange CLI Application

This is a simple command line application written in Rust for currency exchange.

## Features

- Real-time currency exchange rates using a free API.
- Supports conversion between different currencies.
- User-friendly command line interface.

## Prerequisites

- Rust programming language installed. You can download it from [Rust's official website](https://www.rust-lang.org/tools/install).
- Stable internet connection to fetch real-time exchange rates.

## Installation

1. Clone this repository to your local machine:

```bash
git clone https://github.com/ap211unitech/rust-projects/tree/currency-exchange
```

2. Navigate into the project directory:

```bash
cd rust-projects
git checkout currency-exchange
```

3. Build the project using Cargo:
```bash
cargo build --release
```

## Usage

Once you've built the project, you can run the executable from the command line.

```bash
./target/release/currency-exchange [OPTIONS] <AMOUNT> <FROM_CURRENCY> <TO_CURRENCY>
```

## Example

```bash
./target/release/currency-exchange 100 USD EUR
```