#![allow(warnings)] 

use std::{fs::{File, create_dir_all, remove_file, read_to_string}, io::{Write,BufReader, self, Read}, path::{PathBuf, Path}, collections::HashMap};
use dirs;
use url::form_urlencoded;
use std::env::var;
const MSG_NO_SYSTEM_CONFIG_DIR: &str = "no system config directory detected";

// #[no_mangle]
/// Saves a preference with the given key and value for the given app_name.
///
/// # Arguments
///
/// * `app_name` - A string slice that holds the name of the application.
/// * `key` - A string slice that holds the key of the preference.
/// * `value` - A generic type that holds the value of the preference.
///
/// # Examples
///
/// ```
/// use prefstore::savepreference;
///
/// savepreference("myapp", "mykey", "myvalue");
/// ```
///
/// # Errors
///
/// This function will return an error if it is unable to create the necessary directories or file.
pub fn savepreference<T: ToString>(app_name:impl Into<String>,key: impl Into<String>,value:T){
    let key=key.into();
    let app_name=app_name.into();
    create_dir_all(&config_path(&app_name,&key).parent().expect("Cannot find some path to create config")).expect("cannot create dirs necessary to create config");
    write!(File::create(&config_path(&app_name,&key)).expect("Cannot create file."), "{}", value.to_string());
}
pub fn savecustom<T: ToString>(app_name:impl Into<String>,key: impl Into<String>,value:T){
    let key=key.into();
    let app_name=app_name.into();
    create_dir_all(&customfile_path(&app_name,&key).parent().expect("Cannot find some path to create config")).expect("cannot create dirs necessary to create config");
    write!(File::create(&customfile_path(&app_name,&key)).expect("Cannot create file."), "{}", value.to_string());
}

/// Returns a default name for a preference file.
///
/// # Arguments
///
/// * `filename` - A string slice that holds the name of the preference file.
///
/// # Examples
///
/// ```
/// use prefstore::default_name;
///
/// let filename = "mykey";
/// let default_name = default_name(filename.to_string());
/// assert_eq!(default_name, "mykey.txt");
/// ```
fn default_name(filename:String) -> String {
    format!("{}.txt", filename).to_lowercase()
}
fn custom_default_name(filename:String) -> String {
    format!("{}", filename).to_lowercase()
}

/// Returns the path to the configuration file for the given app_name and filename.
///
/// # Arguments
///
/// * `app_name` - A string slice that holds the name of the application.
/// * `filename` - A string slice that holds the name of the configuration file.
///
/// # Examples
///
/// ```
/// use prefstore::config_path;
///
/// let app_name = "myapp";
/// let filename = "myconfig";
/// let path = config_path(&app_name.to_string(), &filename.to_string());
/// assert_eq!(path.to_str().unwrap(), "/home/user/.config/myapp/myconfig.txt");
/// ```
///
/// # Errors
///
/// This function will panic if the system config directory cannot be found.
fn config_path(app_name:&String,filename:impl Into<String>) -> PathBuf {
    match(dirs::config_dir()){
        Some(system_config_dir) =>{
            system_config_dir
                    .join(app_name)
                    .join(default_name(filename.into()))
        },
        None => {
           panic!("{}", MSG_NO_SYSTEM_CONFIG_DIR);
        },
    }
}
fn customfile_path(app_name:&String,filename:impl Into<String>) -> PathBuf {
    match(dirs::config_dir()){
        Some(system_config_dir) =>{
            system_config_dir
                    .join(app_name)
                    .join(custom_default_name(filename.into()))
        },
        None => {
           panic!("{}", MSG_NO_SYSTEM_CONFIG_DIR);
        },
    }
}
/// Removes the preference with the given key for the given app_name.
///
/// # Arguments
///
/// * `app_name` - A string slice that holds the name of the application.
/// * `key` - A string slice that holds the key of the preference.
///
/// # Examples
///
/// ```
/// use prefstore::clearpreference;
///
/// clearpreference("myapp", "mykey");
/// ```
///
/// # Errors
///
/// This function will return an error if it is unable to remove the preference file.
pub fn clearpreference(app_name:impl Into<String>,key: impl Into<String>){
    remove_file(&config_path(&app_name.into(),&key.into())).expect("Could not clear preference.");
}
pub fn clearcustom(app_name:impl Into<String>,key: impl Into<String>){
    remove_file(&customfile_path(&app_name.into(),&key.into())).expect("Could not clear preference.");
}

// #[no_mangle]
/// Retrieves the preference with the given key for the given app_name. If the preference does not exist, it returns the default value provided.
///
/// # Arguments
///
/// * `app_name` - A string slice that holds the name of the application.
/// * `key` - A string slice that holds the key of the preference.
/// * `defvalue` - A generic type that holds the default value of the preference.
///
/// # Examples
///
/// ```
/// use prefstore::getpreference;
///
/// let app_name = "myapp";
/// let key = "mykey";
/// let defvalue = "defaultvalue";
/// let value = getpreference(app_name, key, defvalue);
/// assert_eq!(value, "defaultvalue");
/// ```
///
/// # Errors
///
/// This function will return an error if it is unable to read the preference file or create a new preference file with the default value.
pub fn getpreference<T:ToString>(app_name:impl Into<String>,key:impl Into<String>,defvalue:T)->String{
    use io::Read;
    let key =key.into();
    let app_name =app_name.into();
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
pub fn getcustom<T:ToString>(app_name:impl Into<String>,key:impl Into<String>,defvalue:T)->String{
    use io::Read;
    let key =key.into();
    let app_name =app_name.into();
				match(File::open(&customfile_path(&app_name,&key))){
                    Ok(mut file) => {
                        let mut buf = String::new();
                        file.read_to_string(&mut buf).expect("Cannot read to string");
                        buf
                    },
                    Err(_) => {
                        savecustom(app_name,&key, &defvalue.to_string());
                        defvalue.to_string()
                    },
                }
}
pub fn getpreferencenodefault(app_name:impl Into<String>,key:impl Into<String>)->String{
    use io::Read;
    let key =key.into();
    let app_name =app_name.into();
				match(File::open(&config_path(&app_name,&key))){
                    Ok(mut file) => {
                        let mut buf = String::new();
                        file.read_to_string(&mut buf).expect("Cannot read to string");
                        buf
                    },
                    Err(_) => {
                        String::new()
                    },
                }
}

pub trait ems<T>{
    /// Converts a String to a bool.
    ///
    /// # Arguments
    ///
    /// * `self` - A string slice that holds the value to be converted.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefstore::ems;
    ///
    /// let value = "true";
    /// let bool_value = value.tobool();
    /// assert_eq!(bool_value, true);
    /// ```
    fn tobool(self)->bool;
    
    /// Converts a String to an i32.
    ///
    /// # Arguments
    ///
    /// * `self` - A string slice that holds the value to be converted.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefstore::ems;
    ///
    /// let value = "42";
    /// let i32_value = value.toi32();
    /// assert_eq!(i32_value, 42);
    /// ```
    fn toi32(self)->i32;
    
    /// Converts a String to an i128.
    ///
    /// # Arguments
    ///
    /// * `self` - A string slice that holds the value to be converted.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefstore::ems;
    ///
    /// let value = "99999999999999999999999999999999999999";
    /// let i128_value = value.toi128();
    /// assert_eq!(i128_value, 99999999999999999999999999999999999999);
    /// ```
    fn toi128(self)->i128;
    
    /// Converts a String to an f64.
    ///
    /// # Arguments
    ///
    /// * `self` - A string slice that holds the value to be converted.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefstore::ems;
    ///
    /// let value = "3.14";
    /// let f64_value = value.tof64();
    /// assert_eq!(f64_value, 3.14);
    /// ```
    fn tof64(self)->f64;
}

impl ems<String> for String{
    fn tobool(self)->bool{
        let truth_value: bool = match self.as_str() {
            "true" => true,
            "t" => true,
            "false" => false,
            "f" => false,
            _ => false
        };
        truth_value
    }
    fn toi32(self)->i32{
        self.parse::<i32>().unwrap()
    }
    fn toi128(self)->i128{
        self.parse::<i128>().unwrap()
    }
    fn tof64(self)->f64{
        self.parse::<f64>().unwrap()
    }
} 
fn getencodedstring(value:String)->String{
    form_urlencoded::Serializer::new(String::new())
    .append_key_only(&value)
    .finish()
}
#[test]
fn encode_and_decode_str_for_list(){
    println!("{}",getencodedstring(r#"complete the tasl &=&=ta jadmm"#.to_string()));
    // let encoded = "complete+the+tasl+%26%3D%26%3Dta+jadmm";
    // let decoded: String = form_urlencoded::parse(encoded.as_bytes())
    //     .map(|(k, v)| format!("{}={}", k, v))
    //     .collect::<Vec<String>>()
    //     .join("&");
    // println!("{}", decoded);
    println!("{}",getdecoded(r#"complete+the+tasl+%26%3D%26%3Dta+jadmm"#));
}
fn getdecoded(input:&str)->String{
    (form_urlencoded::parse(input.as_bytes())
        .map(|(k, _)| format!("{}", k))
        .collect::<String>())
        
}
// fn get_decoded_string(j:Vec<String>)->Vec<String>{
//     // for a in j{
//         let encoded = form_urlencoded::parse(&a.as_bytes())
//         .map(|(_, v)| {
//             // todo!();
            
//             format!("{}", v)
//         })
//         .collect::<Vec<String>>();   
//     // }
//     vec![]
// }
pub fn addlistasjson<T: ToString>(app_name:impl Into<String>,key: impl Into<String>,value:Vec<T>)
{
    let app_name:String=app_name.into();
    let key:String=key.into();
    let mut listofvecs:Vec<String>=
    // match(getfromlist(app_name.clone(), key.clone())){
    //     Ok(alreadycontents) => alreadycontents,
    //     Err(_) => 
        Vec::new();
    //     ,
    // };
    // let vecofwhat=value.to_string().as_bytes();
    // let encoded: String = getencodedstring(value.to_string());
    // let encoded: String = (value.to_string());//getencodedstring
    // let utf8_bytes_list: Vec<&[u8]> = value.to_string().chars().into_iter().map(|s| s.as_bytes()).collect();
    for a in value{
        listofvecs.push(a.to_string());
    }
    
    savepreference(app_name.clone(),key.clone(),serde_json::to_string_pretty(&listofvecs).unwrap());
        
    // savepreference(app_name.clone(),key.clone(),serde_json::to_string(&listofvecs).unwrap())
}

pub fn addtolist<T: ToString>(app_name:impl Into<String>,key: impl Into<String>,value:T){
    let app_name:String=app_name.into();
    let key:String=key.into();
    let mut listofvecs:Vec<String>=match(getfromlist(app_name.clone(), key.clone())){
        Ok(alreadycontents) => alreadycontents,
        Err(_) => Vec::new(),
    };
    // let vecofwhat=value.to_string().as_bytes();
    // let encoded: String = getencodedstring(value.to_string());
    let encoded: String = getencodedstring(value.to_string());
    // let utf8_bytes_list: Vec<&[u8]> = value.to_string().chars().into_iter().map(|s| s.as_bytes()).collect();
    listofvecs.push(encoded);
    savepreference(app_name.clone(),key.clone(),    listofvecs.join("\n"));
        
    // savepreference(app_name.clone(),key.clone(),serde_json::to_string(&listofvecs).unwrap())
}
fn config_folder_path(app_name:&String) -> PathBuf {
    match(dirs::config_dir()){
        Some(system_config_dir) =>{
            system_config_dir
                    .join(app_name)
        },
        None => {
           panic!("{}", MSG_NO_SYSTEM_CONFIG_DIR);
        },
    }
}
pub fn getfromlist(app_name:impl Into<String>,key:impl Into<String>)->Result<Vec<String>,()>{
    let key =key.into();
    let app_name =app_name.into();
    let input= match(File::open(&config_path(&app_name,&key))){
        Ok(mut file) => {
            let mut buf = String::new();
            file.read_to_string(&mut buf).expect("Cannot read to string");
            buf
        },
        Err(_) => {
            return Err(())
            // savepreference(app_name,&key, &defvalue.to_string());
            // defvalue.to_string()
        },
    };
    // let j:Vec<String>=serde_json::from_str(&input).unwrap();
    let vec_of_string = input.split("\n").map(|s| getdecoded(s).to_string()).collect::<Vec<String>>();
    // url2str(j)
    // get_decoded_string(j);
    // vec![]
    Ok(vec_of_string)
}
pub fn getfromlistasjson(app_name:impl Into<String>,key:impl Into<String>)->Result<Vec<String>,()>{
    let key =key.into();
    let app_name =app_name.into();
    let input= match(File::open(&config_path(&app_name,&key))){
        Ok(mut file) => {
            let mut buf = String::new();
            file.read_to_string(&mut buf).expect("Cannot read to string");
            buf
        },
        Err(_) => {
            return Err(())
            // savepreference(app_name,&key, &defvalue.to_string());
            // defvalue.to_string()
        },
    };
    let file=File::open(&config_path(&app_name,&key)).map_err(|err|{
        eprintln!("Fil coild not be opened:{err}")

    })?;
    // let j:Vec<String>=serde_json::from_str(&input).unwrap();
    let listdecoded:Vec<String>=serde_json::from_reader(BufReader::new(file)).map_err(|err|{
        eprintln!("Fil coild not be opened:{err}")

    })?;
    // let vec_of_string = listdecoded.split("\n").map(|s| getdecoded(s).to_string()).collect::<Vec<String>>();
    // url2str(j)
    // get_decoded_string(j);
    // vec![]
    Ok(listdecoded)
}

pub fn getallfromlist(app_name:impl Into<String>)->Vec<(String,String)>{
    let app_name=app_name.into();
    // println!("{app_name}");
    let mut gh=config_folder_path(&app_name).to_str().unwrap().to_string();
    // println!("{}",gh);
    // let key =key.into();
    gh.push_str("/*.txt");
    let mut list_of_strings:Vec<(String,String)>=vec![];
    // println!("{:?}-----------------{:?}",gh,glob::glob(&gh).expect("Failed to read glob pattern"));
    for entry in glob::glob(&gh).expect("Failed to read glob pattern") {
        match entry {

            Ok(path) =>{
                let input= match(File::open(&path)){
                    Ok(mut file) => {
                        let mut buf = String::new();
                        file.read_to_string(&mut buf).expect("Cannot read to string");
                        buf
                    },
                    Err(_) => {
                        "".to_string()
                        // savepreference(app_name,&key, &defvalue.to_string());
                        // defvalue.to_string()
                    },
                };
                let vec_of_string = input.split("\n").map(|s| getdecoded(s).to_string()).collect::<Vec<String>>().join("\n");
                let file_name =&path.file_stem().unwrap().to_str().unwrap().to_string();

                // for i in vec_of_string{
                    list_of_strings.push((file_name.to_owned(),vec_of_string));
                // }
                
            },
            Err(e) => {
                eprintln!("error with glob {:?}", e);
                
            },
        }
    }
    
    // let j:Vec<String>=serde_json::from_str(&input).unwrap();
    // url2str(j)
    // get_decoded_string(j);
    // vec![]
    list_of_strings
}
fn readserdefromfile(input:&File)->Result<Vec<String>,()>{
    let listdecoded:Vec<String>=serde_json::from_reader(BufReader::new(input)).map_err(|err|{
        eprintln!("Fil could not be opened:{err}")

    })?;
    Ok(listdecoded)
}
#[test]
fn test_addtojson(){
    let g=vec!["date","event","status","description","repeat"];
    addlistasjson("todo","test", g.clone());
    addlistasjson("todo","test1", g.clone());
    addlistasjson("todo","test2", g.clone());
    // println!("{:?}", getfromjsonlist("todo","test").unwrap());
    assert_eq!(g,getfromlistasjson("todo","test").unwrap());
    println!("{:?}",getallasjsonlist("todo"))
}
pub fn getallasjsonlist(app_name:impl Into<String>)->Vec<(String,Vec<String>)>{
    let app_name=app_name.into();
    // println!("{app_name}");
    let mut gh=config_folder_path(&app_name).to_str().unwrap().to_string();
    // println!("{}",gh);
    // let key =key.into();
    gh.push_str("/*.txt");
    let mut list_of_strings:Vec<(String,Vec<String>)>=vec![];
    // println!("{:?}-----------------{:?}",gh,glob::glob(&gh).expect("Failed to read glob pattern"));
    for entry in glob::glob(&gh).expect("Failed to read glob pattern") {
        match entry {

            Ok(path) =>{
                let input= match(File::open(&path)){
                    Ok(mut file) => {
                        // let mut buf = String::new();
                        // file.read_to_string(&mut buf).expect("Cannot read to string");
                        // buf
                        file
                    },
                    Err(_) => {
                        return vec![]
                        // savepreference(app_name,&key, &defvalue.to_string());
                        // defvalue.to_string()
                    },
                };
                // let vec_of_string = input.split("\n").map(|s| getdecoded(s).to_string()).collect::<Vec<String>>().join("\n");
                println!("{:?}",input);
                let listdecoded:Vec<String>=readserdefromfile(&input).unwrap();
                let file_name =&path.file_stem().unwrap().to_str().unwrap().to_string();

                // for i in vec_of_string{
                    list_of_strings.push((file_name.to_owned(),listdecoded));
                // }
                
            },
            Err(e) => {
                eprintln!("error with glob {:?}", e);
                
            },
        }
    }
    
    // let j:Vec<String>=serde_json::from_str(&input).unwrap();
    // url2str(j)
    // get_decoded_string(j);
    // vec![]
    list_of_strings
}pub fn getall(app_name:impl Into<String>)->Vec<(String,String)>{
    let app_name=app_name.into();
    // println!("{app_name}");
    let mut gh=config_folder_path(&app_name).to_str().unwrap().to_string();
    // println!("{}",gh);
    // let key =key.into();
    gh.push_str("/*.txt");
    let mut list_of_strings:Vec<(String,String)>=vec![];
    // println!("{:?}-----------------{:?}",gh,glob::glob(&gh).expect("Failed to read glob pattern"));
    for entry in glob::glob(&gh).expect("Failed to read glob pattern") {
        match entry {

            Ok(path) =>{
                let input= match(File::open(&path)){
                    Ok(mut file) => {
                        let mut buf = String::new();
                        file.read_to_string(&mut buf).expect("Cannot read to string");
                        buf
                        // file
                    },
                    Err(_) => {
                        return vec![]
                        // savepreference(app_name,&key, &defvalue.to_string());
                        // defvalue.to_string()
                    },
                };
                // let vec_of_string = input.split("\n").map(|s| getdecoded(s).to_string()).collect::<Vec<String>>().join("\n");
                println!("{:?}",input);
                // let listdecoded:Vec<String>=readserdefromfile(&input).unwrap();
                let file_name =&path.file_stem().unwrap().to_str().unwrap().to_string();

                // for i in vec_of_string{
                    list_of_strings.push((file_name.to_owned(),input));
                // }
                
            },
            Err(e) => {
                eprintln!("error with glob {:?}", e);
                
            },
        }
    }
    
    // let j:Vec<String>=serde_json::from_str(&input).unwrap();
    // url2str(j)
    // get_decoded_string(j);
    // vec![]
    list_of_strings
}
#[test]
    fn work_with_list(){
        // clearpreference(app_name, key)
        let app_name="todo";
        let key="date";
        let value="whatsup";
        addtolist(app_name, key, value);
        println!("{:?}",getallfromlist(app_name))
    }
#[cfg(test)]
mod prefstore_test {
    use super::*;

    #[test]
    fn test_savepreference() {
        let app_name = "myapp";
        let key = "mykey";
        let value = "myvalue";
        savepreference(app_name, key, value);

        // Test that the preference was saved correctly
        let result = getpreference(app_name, key, "defaultvalue");
        assert_eq!(result, "myvalue");

        // Test that the default value is returned when the preference does not exist
        let result = getpreference(app_name, "nonexistentkey", "defaultvalue");
        assert_eq!(result, "defaultvalue");

        // Test that the preference can be cleared
        clearpreference(app_name, key);
        let result = getpreference(app_name, key, "defaultvalue");
        assert_eq!(result, "defaultvalue");

        // Test that the ems trait works correctly
        let bool_value = "true".to_string().tobool();
        assert_eq!(bool_value, true);

        let i32_value = "42".to_string().toi32();
        assert_eq!(i32_value, 42);

        let i128_value = "99999999999999999999999999999999999999".to_string().toi128();
        assert_eq!(i128_value, 99999999999999999999999999999999999999);

        let f64_value = "3.14".to_string().tof64();
        assert_eq!(f64_value, 3.14);
    }
}

// This is a Rust module that provides a preference store. It allows you to save and retrieve preferences for your application. The preferences are stored in the system's configuration directory.

// The module provides the following functions:

// savepreference(app_name: String, key: String, value: T): saves a preference with the given key and value for the given app_name.
// getpreference(app_name: String, key: String, defvalue: T) -> String: retrieves the preference with the given key for the given app_name. If the preference does not exist, it returns the default value provided.
// clearpreference(app_name: String, key: String): removes the preference with the given key for the given app_name.
// The preferences are stored in the system's configuration directory. The directory is determined by the dirs::config_dir() function. If the function returns None, the module will panic with the message "no system config directory detected".

// The preferences are stored in a file named after the key, with a .txt extension. The file is stored in a directory named after the app_name. If the directory does not exist, it will be created.

// The module also provides a trait ems<T> with two methods:

// tobool(self) -> bool: converts a String to a bool.
// toi32(self) -> i32: converts a String to an i32.
// These methods can be used to convert the string returned by getpreference to the desired type.

// The module also includes a test module prefstore_test with tests for the savepreference, getpreference, and clearpreference functions.
