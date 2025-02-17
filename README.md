# Crypto Mani

Crypto Mani is a simple blockchain implementation in Rust. It includes basic functionalities such as creating blocks, adding transactions, and mining new blocks.

## Features

- Create a new blockchain
- Add transactions to blocks
- Mine blocks with a specified difficulty
- Display block details with colorful output

## Usage

To use this project, you need to have Rust installed. You can install Rust from [here](https://www.rust-lang.org/tools/install).

### Building the Project

```sh
cargo build
```

### Running the Project

```sh
cargo run
```

### Example

Here is an example of how to create a new blockchain and add a block:

```rust
use crypto_mani::blockchain::Blockchain;
use crypto_mani::transaction::Transaction;

fn main() {
    let mut blockchain = Blockchain::new();
    let transaction = Transaction::new("Alice", "Bob", 10);
    blockchain.add_block(vec![transaction]);
    blockchain.display();
}
```

## Dependencies

- `sha2` for hashing
- `colored` for colorful terminal output

## License

This project is licensed under the MIT License.
