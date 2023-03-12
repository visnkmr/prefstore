#![allow(warnings)] 
use std::array;
use std::env;
use std::fs::File;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::path::Path;
use std::process;
use std::thread;
use std::time::Duration;
// use chrono::Local;
use prefstore;
use prefstore::ems;
use prefstore::getpreference;
use prefstore::savepreference;

/*
[i32]
[string]
[bool]
[f64]
 */



fn main() {
   const APPNAME:&str="nameofapp";
   // println!("hello");
   savepreference(APPNAME,"boolean",false);

   let set=9;
   savepreference(APPNAME,"i32",set);

   let set=6.8;
   savepreference(APPNAME,"f64",set);
   
   let set:i128=99999999999999999999999999999999999999;
   
   savepreference(APPNAME,"verylongi",set);
//    let mut places = vec!["Paris", "New York"];
//     places.push("Madrid");
//     places.push("Toronto");
//    let set=["test","try"];
//    savepreference("strarr",places);
//    println!("test");
//    thread::spawn(move || loop {
//       // println!("fromhere------------>1");
//       let date = Local::now();
//       let current_date = date.format("%Y-%m-%d").to_string();
//       let k= getpreference(&current_date,0 as u128).parse::<u128>().unwrap();

//       println!("{}",getpreference(&current_date,0 as u128).parse::<u128>().unwrap());

//       thread::sleep(Duration::from_secs(60));
//   });

  
   println!("{}",getpreference(APPNAME,"start",false).tobool());

   // in order to save whole blog posts to strings use the trait
   //for post in posts{savepreference(APPNAME,post.title,post.content);}
}
