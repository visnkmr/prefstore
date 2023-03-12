#![allow(warnings)] 
use std::array;
use std::env;
use std::fs::File;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::path::Path;
use std::process;
use preferencestore;
use preferencestore::ems;
use preferencestore::getpreference;
use preferencestore::savepreference;

/*
[i32]
[string]
[bool]
[f64]
 */

fn main() {
   println!("hello");
   savepreference("boolean",false);

   let set=9;
   savepreference("i32",set);

   let set=6.8;
   savepreference("f64",set);
   
   let set:i128=99999999999999999999999999999999999999;
   savepreference("verylongi",set);
//    let mut places = vec!["Paris", "New York"];
//     places.push("Madrid");
//     places.push("Toronto");
//    let set=["test","try"];
//    savepreference("strarr",places);
   println!("test");

   println!("{}",getpreference("start",false).tobool());
}
