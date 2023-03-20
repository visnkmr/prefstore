#![allow(warnings)] 



use std::{fs::{File, create_dir_all, remove_file}, io::{Write,BufReader, self}, path::{PathBuf, Path}, collections::HashMap};
use dirs;
use std::env::var;
const MSG_NO_SYSTEM_CONFIG_DIR: &str = "no system config directory detected";

// pub fn setname(appname:String)->String{
//     appname
// }
/*
app_name: Folder in config dir to store the file key with value to.
key: filename of the pref, a key.txt can be found at the folder app_name
value: the content of the file specified by key
value can be any type that supports to string
 */
pub fn savepreference<T: ToString>(app_name:impl Into<String>,key: impl Into<String>,value:T){
    let key=key.into();
    let app_name=app_name.into();
    // println!("{}",config_path(key.into()).to_string_lossy());
    //create new file named key and save value
    create_dir_all(&config_path(&app_name,&key).parent().expect("Cannot find some path to create config")).expect("cannot create dirs necessary to create config");
    write!(File::create(&config_path(&app_name,&key)).expect("Cannot create file."), "{}", value.to_string());
    // println!("saved successfully to {}",config_path(&app_name,&key).to_string_lossy())
}

fn default_name(filename:String) -> String {
    format!("{}.txt", filename).to_lowercase()
}

fn config_path(app_name:&String,filename:impl Into<String>) -> PathBuf {
    
    match(dirs::config_dir()){
        Some(system_config_dir) =>{
            system_config_dir
                    .join(app_name)
                    .join(default_name(filename.into()))
        },
        None => {
           println!("{}", MSG_NO_SYSTEM_CONFIG_DIR.to_string());
           todo!()
            // io::Error::new(io::ErrorKind::NotFound, MSG_NO_SYSTEM_CONFIG_DIR)
        },
    }
}
pub fn clearpreference(app_name:impl Into<String>,key: impl Into<String>){
    remove_file(&config_path(&app_name.into(),&key.into())).expect("Could not clear preference.");
}
/*
the methods to parse value returned by getpreference for bool and i32 are provided the remaining will have to parse from returned string */
pub fn getpreference<T:ToString>(app_name:impl Into<String>,key:impl Into<String>,defvalue:T)->String{
    //Try to open file named key, If not found return defvalue
    use io::Read;
    let key =key.into();
    // let defvalue =defvalue.into();
    let app_name =app_name.into();
    // let defvalue=defvalue.into();
				match(File::open(&config_path(&app_name,&key))){
                    Ok(mut file) => {
                        let mut buf = String::new();
                        file.read_to_string(&mut buf).expect("Cannot read to string");
                        buf
                    },
                    Err(_) => {
                        savepreference(app_name,&key, &defvalue.to_string());
                        defvalue.to_string()
                    },
                }
}

// }

/*
methods to convert returned strings to bool,i32, you can also use .parse on string to parse to required format
 */
pub trait ems<T>{
    fn tobool(self)->bool;
    fn toi32(self)->i32;
}

impl ems<String> for String{
    fn tobool(self)->bool{
        let truth_value: bool = match self.as_str() {
            "true" => true,
            "t" => true,
            "false" => false,
            "f" => false,
            // ... any other cases you want
            _ => false  // Or whatever appropriate default value or error.
        };
        truth_value
        }
    fn toi32(self)->i32{
        self.parse::<i32>().unwrap()
    }
    
} 

// #[derive(Debug, PartialEq, Clone, Default)]
// pub struct forsession {
// 	hamap:HashMap<String,String>,
//     // havec:Vec<String>
// }

// impl forsession {
// 	fn save<T: ToString>(&mut self,key: impl Into<String>,value:impl Into<String>){
//     self.hamap.insert(key.into(), value.into());
//     }

//     // fn get<T:ToString>(&self,key:impl Into<String>)->String{
//     //     match(self.hamap.get(&key.into())){
//     //         Some(rstring) => rstring.to_string(),
//     //         None => {
//     //             "No value for key found".into()
//     //         },
//     //     }
//     // }
//     fn safeget<T:ToString>(&mut self,key:impl Into<String>,defvalue:impl ToString)->String{
//         match(self.hamap.get(&key.into())){
//                     Some(rstring) => {
//                         rstring.to_string()
//                     },
//                     None => {
//                         self.save(key, &defvalue.to_string());
//                         "No value for key found".to_string()
//                     },
//                 };
//         defvalue.to_string()
//     }
//     fn get<T:ToString>(&self,key:impl Into<String>)->String{
//         match(self.hamap.get(&key.into())){
//                     Some(rstring) => {
//                         rstring.to_string()
//                     },
//                     None => {
//                         "".to_string();
//                         "No value for key found".to_string()
//                     },
//                 }
        
//     }
    
// }

#[cfg(test)]
mod prefstore_test{
    use super::*;

    #[test]
    fn prefstore_working(){
    //   let key="key".to_string() ;
    //   let value="value".to_string() ;

    const APPNAME:&str="prefstore";
   // println!("hello");
   let set =true;
   savepreference(APPNAME,"boolean",set);
   assert_eq!(getpreference(APPNAME,"boolean",false).tobool(),set) ;
   clearpreference(APPNAME, "boolean");

   let set=9;
   savepreference(APPNAME,"i32",set);
   assert_eq!(getpreference(APPNAME,"i32",0).toi32(),set) ;
   clearpreference(APPNAME, "i32");

   let set=6.8;
   savepreference(APPNAME,"f64",set);
   assert_eq!(getpreference(APPNAME,"f64",0.0).parse::<f64>().unwrap(),set) ;
   clearpreference(APPNAME, "f64");

   let set:i128=99999999999999999999999999999999999999;
   
   savepreference(APPNAME,"verylongi",set);
   assert_eq!(getpreference(APPNAME,"verylongi",0).parse::<i128>().unwrap(),set) ;
   clearpreference(APPNAME, "verylongi");
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

    savepreference(APPNAME,"start",true);
    // let forsession= forsession{
    //     hamap:HashMap<String,String>
    // };
    // forsession::save(this,key,value);
    assert_eq!(true,getpreference(APPNAME,"start",false).tobool());
    clearpreference(APPNAME, "start");
    // println!("{}",getpreference(APPNAME,"start",false).tobool());
}
}
