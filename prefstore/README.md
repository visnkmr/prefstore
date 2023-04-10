# prefstore

[![Crates.io](https://img.shields.io/crates/v/prefstore)](https://crates.io/crates/prefstore)
[![Crates.io](https://img.shields.io/crates/d/prefstore)](https://crates.io/crates/prefstore)
[![Crates.io](https://img.shields.io/crates/l/prefstore)](https://crates.io/crates/prefstore)

A rust crate to Easily store and retrieve preferences in rust.

## Installation

Install the crate as a dependency in your app's Cargo.toml file:

```toml
[dependencies]
prefstore = "0.5.0"
```

Then you can use it in your code like this:

```rust
// Import prefstore
use prefstore::*;

fn main() {
    // Save value to disk using savepreference
    savepreference("MyApp", "name", "Alice");

    // Save value to disk using savecustom
    savecustom("MyApp", "age.txt", 25);

    // Save value to disk using appendcustom
    appendcustom("MyApp", "hobbies.txt", "reading");

    // Load value from disk using getpreference
    let name = getpreference("MyApp", "name", "Bob");
    println!("Name: {}", name);

    // Load value from disk using getcustom
    let age = getcustom("MyApp", "age.txt", 0);
    println!("Age: {}", age);

    // Load value from disk using getcustomwithnodefault
    let hobbies = getcustomwithnodefault("MyApp", "hobbies.txt");
    println!("Hobbies: {}", hobbies);

    // Delete preference file from disk using clearpreference
    clearpreference("MyApp", "name");

    // Delete preference file from disk using clearcustom
    clearcustom("MyApp", "age.txt");
}
```  
## Features

- Supports any type that implements `[Display]` for values
- Stores each preference in separate file, ensuring quick access and minimum disk read/write operations.
- Provides methods for setting, getting, removing preferebces.
- Provides methods for loading and saving preference from and to a file.
- Provides methods for clearing.

## License

This project is licensed under the MIT license. See the [LICENSE](LICENSE) file for more details.
