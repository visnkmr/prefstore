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

## Usage

Import prefstore:

```rust
use prefstore::*;
```

Save value to disk.

```rust
savepreference("APPNAME",KEY,VALUE);
savecustom("APPNAME",FILE_NAME_WITH_EXTENSION,VALUE);
appendcustom("APPNAME",FILE_NAME_WITH_EXTENSION,APPEND_VALUE);
```

`APPNAME`: folder name to store to on disk in the config folder.  
`KEY`: the name of the preference/config, filename with no extension to store to in APPNAME dir.  
`FILE_NAME_WITH_EXTENSION`: filename with extension to store to in APPNAME dir.  
`VALUE`: the preference/config value to store, stored in the KEY/FILE_NAME_WITH_EXTENSION.txt file in APPNAME dir. Tested using bool,i32,f64,etc.  
`APPEND_VALUE`: the value to append, stored in the KEY/FILE_NAME_WITH_EXTENSION.txt file in APPNAME dir. Tested using bool,i32,f64,etc.  
  
Load value from disk.  
  
```rust
getpreference("APPNAME",KEY,DEFAULT_VALUE);
getcustom("APPNAME",FILE_NAME_WITH_EXTENSION,DEFAULT_VALUE);
getcustomwithnodefault("APPNAME",FILE_NAME_WITH_EXTENSION);
getall("APPNAME"); //returns content of all files with .txt extension as vec<filename,content>
```
  
`APPNAME`: folder name on disk to load from in the config folder.  
`KEY`: the name of the preference/config, filename to load from in APPNAME dir.  
`FILE_NAME_WITH_EXTENSION`: filename with extension to load from in APPNAME dir.  
`DEFAULT_VALUE`: the default preference/config value to return, if the KEY/FILE_NAME_WITH_EXTENSION doesn't exist on disk. Tested using bool,i32,f64,etc.  
  
The saved configs are stored at location as defined by [dirs](https://crates.io/crates/dirs/4.0.0) crate's `config_dir`:   
```
Linux: /home/$USERNAME/.config/APPNAME
Win: Drivename:\Users\$USERNAME\AppData\Roaming\APPNAME 
Mac: /Users/$USERNAME/Library/Application Support\APPNAME
```
  
Delete preference/config file from disk:  
  
```rust
clearpreference("APPNAME",KEY);
clearcustom("APPNAME",FILE_NAME_WITH_EXTENSION);
```
  
`APPNAME`: folder name to delete from on disk in the config folder.  
`KEY`: the name of the preference/config file to delete, filename to delete in APPNAME dir.  
`FILE_NAME_WITH_EXTENSION`: filename with extension to delete in APPNAME dir. 
  
The `VALUE` can be any value that implements ToString trait from rustlib. Note as per doc: This trait is automatically implemented for any type which implements the `[Display]` trait. As such, ToString shouldn't be implemented directly: `[Display]` should be implemented instead, and you get the ToString implementation for free. 	
