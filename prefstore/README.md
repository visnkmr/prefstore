# prefstore

[![Crates.io](https://img.shields.io/crates/v/prefstore)](https://crates.io/crates/prefstore)
[![Crates.io](https://img.shields.io/crates/d/prefstore)](https://crates.io/crates/prefstore)
[![Crates.io](https://img.shields.io/crates/l/prefstore)](https://crates.io/crates/prefstore)

A rust crate to Easily store and retrieve preferences in rust.

## Installation

Install the crate as a dependency in your app's Cargo.toml file:

```toml
[dependencies]
prefstore = "0.1.0"
```

## Usage

Import prefstore:

```rust
use prefstore::*;
```

Save value to disk.

```rust
savepreference("APPNAME",KEY,VALUE);
```

`APPNAME`: folder name to store to on disk in the config folder.  
`KEY`: the name of the preference/config, filename to store to in APPNAME dir.  
`VALUE`: the preference/config value to store, stored in the KEY.txt file in APPNAME dir. Tested using bool,i32,f64,etc.  
  
Load value from disk.  
  
```rust
getpreference("APPNAME",KEY,DEFAULT_VALUE);
```
  
`APPNAME`: folder name on disk to load from in the config folder.  
`KEY`: the name of the preference/config, filename to load from in APPNAME dir.  
`DEFAULT_VALUE`: the default preference/config value to return, if the KEY doesn't exist on disk. Tested using bool,i32,f64,etc.  
  
The saved configs are stored at location as defined by [dirs](https://crates.io/crates/dirs/4.0.0) crate's `config_dir`:   
```
Linux: /home/$USERNAME/.config/APPNAME
Win: Drivename:\Users\$USERNAME\AppData\Roaming\APPNAME 
Mac: /Users/$USERNAME/Library/Application Support\APPNAME
```
  
Delete preference/config file from disk:  
  
```rust
clearpreference("APPNAME",KEY);
```
  
`APPNAME`: folder name to delete from on disk in the config folder.  
`KEY`: the name of the preference/config file to delete, filename to delete in APPNAME dir.  
  
The `VALUE` can be any value that implements ToString trait from rustlib. Note as per doc: This trait is automatically implemented for any type which implements the `[Display]` trait. As such, ToString shouldn't be implemented directly: `[Display]` should be implemented instead, and you get the ToString implementation for free. 	
