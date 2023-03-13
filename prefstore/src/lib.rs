#![allow(warnings)] 

use std::{fs::{File, create_dir_all, remove_file}, io::{Write,BufReader, self}, path::{PathBuf, Path}};
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

    // let app= var("CARGO_PKG_NAME").unwrap_or_else(|_| env!("CARGO_PKG_NAME").to_string());
    
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