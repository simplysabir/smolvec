# SmolVec

SmolVec is a lightweight vector implementation with small-vector optimization for Rust, providing excellent performance for small collections while maintaining API compatibility with the standard `Vec<T>`.

## Features

- Stores small arrays inline, avoiding heap allocation for small vectors
- Fully API compatible with standard `Vec<T>`
- Optimized for performance and memory efficiency with small collections
- Seamlessly switches to heap allocation for larger collections

## Performance

Based on our benchmarks:

- SmolVec is about 23 times faster than `std::Vec` for small push operations
- SmolVec is about 22 times faster than `std::Vec` for small pop operations
- For larger collections (1000+ elements), `std::Vec` outperforms SmolVec in push operations

These results demonstrate that SmolVec is particularly efficient for use cases involving small collections or frequent creation and destruction of vectors.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
smolvec = "1.0.0"
```

Then, in your Rust code:

```rust
use smolvec::SmolVec;

fn main() {
    let mut vec = SmolVec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("Vector: {:?}", vec);
    
    // SmolVec implements standard Vec methods
    vec.pop();
    vec.extend([4, 5, 6].iter().cloned());
    println!("Modified vector: {:?}", vec);
}
```

## When to Use SmolVec

SmolVec is ideal for scenarios where:

- You frequently create and destroy small vectors
- You work with vectors that often contain just a few elements
- You want to optimize memory usage for collections that are usually small but can occasionally grow larger

For consistently large collections, consider using the standard `Vec<T>` instead.

## Documentation

For full documentation, including all available methods and their usage, run `cargo doc --open` after adding SmolVec to your project dependencies.

## Benchmarking

To run the benchmarks yourself:

1. Clone the repository
2. Run `cargo bench`

This will execute the benchmark suite and provide detailed performance comparisons between SmolVec and std::Vec.

## License

This project is licensed under
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. Here are some ways you can contribute:

- Implement new features
- Improve documentation
- Report bugs
- Suggest improvements

Before making a significant change, please open an issue to discuss your proposed changes and ensure they align with the project's goals.
