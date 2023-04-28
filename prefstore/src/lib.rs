#![allow(warnings)] 

use std::{fs::{File, create_dir_all, remove_file, read_to_string, OpenOptions, self}, io::{Write,BufReader, self, Read, Error}, path::{PathBuf, Path}, collections::HashMap, fmt::format};
use dirs;
// use url::form_urlencoded;
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
    savecustom(app_name, format!("{}.txt",key.into()), value)
}
/// Save custom data to a file with the given app name, filename, and value.
///
/// This function creates a directory with the name `custom_filename_with_extension` in a
/// path defined by the `customfile_path` function. The `create_dir_all` function is called
/// to ensure that all the necessary directories are created for the specified path.
///
/// # Arguments
///
/// * `app_name` - A parameter of type `impl Into<String>`, which means it can accept any type that can be converted into a `String`.
/// * `custom_filename_with_extension` - A parameter of type `impl Into<String>`, which also accepts any type that can be converted into a `String`.
/// * `value` - The last parameter is of type `T` that implements the `ToString` trait.
///
/// # Examples
///
/// ```rust
/// savecustom("my_app", "my_file.txt", "Hello, world!");
/// ```
pub fn savecustom<T: ToString>(app_name:impl Into<String>,custom_filename_with_extension: impl Into<String>,value:T){
    let key=custom_filename_with_extension.into();
    let app_name=app_name.into();
    let fname=" #savecustom";
    create_dir_all(&customfile_path(&app_name,&key).parent()
        .expect(&format!("Cannot find path to {fname}")))
        .expect(&format!("cannot create dirs necessary to {fname}"));
    write!(File::create(&customfile_path(&app_name,&key))
        .expect("Cannot create file."), "{}", value.to_string());
        
}
pub fn initcustomfile<T: ToString>(app_name:impl Into<String>,custom_filename_with_extension: impl Into<String>,value:T)->std::io::Result<()>{
    let key=custom_filename_with_extension.into();
    let app_name=app_name.into();
    let fname=" #initcustom";
    
    create_dir_all(&customfile_path(&app_name,&key).parent()
        .expect(&format!("Cannot find path to {fname}")))
        .expect(&format!("cannot create dirs necessary to {fname}"));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&customfile_path(&app_name,&key))?;
    file.write_all(format!("{}", value.to_string()).as_bytes());
        Ok(())
}
/// Appends a value to a custom file for the given application.
///
/// # Arguments
///
/// * `app_name` - The name of the application.
/// * `custom_filename_with_extension` - The name of the custom file, including its extension.
/// * `value` - The value to append to the file.
///
/// # Example
///
/// ```rust
pub fn appendcustom<T: ToString>(app_name:impl Into<String>,custom_filename_with_extension: impl Into<String>,value:T){
    let key=custom_filename_with_extension.into();
    let fname=" #appendcustom";
    let app_name=app_name.into();
    create_dir_all(&customfile_path(&app_name,&key).parent()
        .expect(&format!("Cannot find path to {fname}")))
        .expect(&format!("cannot create dirs necessary to {fname}"));
    write!(File::options().create(true).append(true).open(&customfile_path(&app_name,&key))
        .expect("Cannot create file."), "{}", value.to_string());
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
    custom_file_name(format!("{}.txt", filename))
}
fn custom_file_name(filename:String) -> String {
    format!("{}", filename)
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
/// Returns the directory path for storing preferences for the given app.
///
/// # Arguments
///
/// * `app_name` - A reference to a string representing the name of the app.
///
/// # Returns
///
/// A `Result` with the `PathBuf` representing the directory path for storing preferences for the app or an empty `()` if the config folder does not exist.
///
/// # Example
///
/// ```
/// use myapp::prefstore_directory;
/// let app_name = String::from("myapp");
/// let directory = prefstore_directory(&app_name);
/// ```
pub fn prefstore_directory(app_name:&String)->Result<PathBuf,()>{
    Ok(
        dirs::config_dir()
        .expect("Config folder does not exist")
        .join(app_name)
    )    
}
/// Returns the path to a custom file given the `app_name` and the `filename` 
/// for the custom file. It joins the system configuration directory, the 
/// `app_name`, and the custom file name to create the full path.
///
/// # Arguments
///
/// * `app_name` - A reference to a `String` containing the name of the application.
/// * `filename` - A value that can be converted into a `String` containing the name of 
///                the custom file.
///
/// # Returns
///
/// The full path to the custom file as a `PathBuf` object.
fn customfile_path(app_name:&String,filename:impl Into<String>) -> PathBuf {
    // Get the system configuration directory.
    match(dirs::config_dir()){
        Some(system_config_dir) =>{
            // Join the system configuration directory, the app name, and the custom file name.
            system_config_dir
                    .join(app_name)
                    .join(custom_file_name(filename.into()))
        },
        None => {
            // If there is no system configuration directory, panic with an error message.
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
    remove_file(&config_path(&app_name.into(),&key.into()))
        .expect("Could not clear preference.");
}
/// Deletes the custom file with the given name for the specified app.
///
/// # Arguments
///
/// * `app_name` - A string slice that represents the name of the app.
/// * `custom_filename_with_extension` - A string slice that represents the name of the custom file with the extension.
///
/// # Panics
///
/// This function panics if it fails to delete the custom file.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use std::fs::{self, File};
///
/// fn customfile_path(app_name:&String,filename:impl Into<String>) -> PathBuf {
///     /* Function body */
///     # PathBuf::new()
/// }
///
/// fn clearcustom_test() {
///     let app_name = "MyApp";
///     let custom_filename_with_extension = "myfile.txt";
///
///     // Create the custom file to be deleted
///     let file_path = customfile_path(&app_name.to_string(), custom_filename_with_extension.to_string());
///     File::create(&file_path).unwrap();
///
///     // Ensure the custom file exists
///     assert_eq!(file_path.exists(), true);
///
///     // Delete the custom file
///     clearcustom(&app_name.to_string(), custom_filename_with_extension.to_string());
///
///     // Ensure the custom file was deleted
///     assert_eq!(file_path.exists(), false);
/// }
/// ```
pub fn clearcustom(app_name:impl Into<String>,custom_filename_with_extension: impl Into<String>){
    remove_file(&customfile_path(&app_name.into(),&custom_filename_with_extension.into()))
        .expect("Could not clear preference.");
}
/// Clears all files with the given extension in the configuration folder for the given application.
///
/// # Arguments
///
/// * `app_name` - The name of the application whose files should be cleared.
/// * `file_extension` - The extension of the files to be cleared.
///
/// # Examples
///
/// ```
/// # use prefstore::clearall;
/// clearall("myapp", "txt");
/// ```
pub fn clearall(app_name: impl Into<String>, file_extension: &str) {
    let app_name = app_name.into();

    // Construct the path to the configuration folder for the application.
    let mut gh = config_folder_path(&app_name)
        .to_str()
        .unwrap()
        .to_string();

    // Append the glob pattern to match files with the given extension.
    gh.push_str(&format!("/*.{}", file_extension));

    // Iterate over all files that match the glob pattern and attempt to remove them.
    for entry in glob::glob(&gh).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                remove_file(path);
            }
            Err(e) => {
                eprintln!("error with glob {:?}", e);
            }
        }
    }
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
    getcustom(app_name, format!("{}.txt", key.into()), defvalue)
    // use io::Read;
    // let key =key.into();
    // let app_name =app_name.into();
	// 			match(File::open(&config_path(&app_name,&key))){
    //                 Ok(mut file) => {
    //                     let mut buf = String::new();
    //                     file.read_to_string(&mut buf)
    //                         .expect("Cannot read to string");
    //                     buf
    //                 },
    //                 Err(_) => {
    //                     savepreference(app_name,&key, &defvalue.to_string());
    //                     defvalue.to_string()
    //                 },
    //             }
}
/// Retrieve the custom data from the specified custom file for the specified app,
/// or save the default value and return it if the file does not exist.
/// 
/// # Arguments
///
/// * `app_name` - The name of the application.
/// * `key` - The key associated with the custom data.
/// * `defvalue` - The default value to be saved and returned if the file does not exist.
///
/// # Returns
///
/// A string containing the custom data retrieved from the specified file, or the default
/// value if the file does not exist.
///
/// # Examples
///
/// ```
/// let app_name = "MyApp";
/// let key = "my_custom_data.txt";
/// let custom_data = getcustom(app_name, key, "default value");
/// println!("Custom data: {}", custom_data);
/// ```

pub fn getcustom<T:ToString>(app_name:impl Into<String>,key:impl Into<String>,defvalue:T)->String{
    use io::Read;
    let key =key.into();
    let app_name =app_name.into();
				match(File::open(&customfile_path(&app_name,&key))){
                    Ok(mut file) => {
                        let mut buf = String::new();
                        file.read_to_string(&mut buf)
                            .expect("Cannot read to string");
                        buf
                    },
                    Err(_) => {
                        savecustom(app_name,&key, &defvalue.to_string());
                        defvalue.to_string()
                    },
                }
}
/// Retrieves the preference value associated with the specified key for the given app name.
/// If the preference file does not exist, returns an empty string.
///
/// # Arguments
///
/// * `app_name` - The name of the application.
/// * `key` - The key of the preference value.
///
/// # Examples
///
/// ```
/// let app_name = "my_app";
/// let key = "my_preference_key";
/// let preference_value = getpreferencenodefault(app_name, key);
/// ```
pub fn getpreferencenodefault(app_name:impl Into<String>,key:impl Into<String>)->String{
    use io::Read;
    let key =key.into();
    let app_name =app_name.into();
				match(File::open(&config_path(&app_name,&key))){
                    Ok(mut file) => {
                        let mut buf = String::new();
                        file.read_to_string(&mut buf)
                            .expect("Cannot read to string");
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
// fn getencodedstring(value:String)->String{
//     form_urlencoded::Serializer::new(String::new())
//     .append_key_only(&value)
//     .finish()
// }
// #[test]
// fn encode_and_decode_str_for_list(){
//     println!("{}",getencodedstring(r#"complete the tasl &=&=ta jadmm"#.to_string()));
//     // let encoded = "complete+the+tasl+%26%3D%26%3Dta+jadmm";
//     // let decoded: String = form_urlencoded::parse(encoded.as_bytes())
//     //     .map(|(k, v)| format!("{}={}", k, v))
//     //     .collect::<Vec<String>>()
//     //     .join("&");
//     // println!("{}", decoded);
//     println!("{}",getdecoded(r#"complete+the+tasl+%26%3D%26%3Dta+jadmm"#));
// }
// fn getdecoded(input:&str)->String{
//     (form_urlencoded::parse(input.as_bytes())
//         .map(|(k, _)| format!("{}", k))
//         .collect::<String>())
        
// }
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
// pub fn addlistasjson<T: ToString>(app_name:impl Into<String>,key: impl Into<String>,value:Vec<T>)
// {
//     let app_name:String=app_name.into();
//     let key:String=key.into();
//     let mut listofvecs:Vec<String>=
//     // match(getfromlist(app_name.clone(), key.clone())){
//     //     Ok(alreadycontents) => alreadycontents,
//     //     Err(_) => 
//         Vec::new();
//     //     ,
//     // };
//     // let vecofwhat=value.to_string().as_bytes();
//     // let encoded: String = getencodedstring(value.to_string());
//     // let encoded: String = (value.to_string());//getencodedstring
//     // let utf8_bytes_list: Vec<&[u8]> = value.to_string().chars().into_iter().map(|s| s.as_bytes()).collect();
//     for a in value{
//         listofvecs.push(a.to_string());
//     }
    
//     savepreference(app_name.clone(),key.clone(),serde_json::to_string_pretty(&listofvecs).unwrap());
        
//     // savepreference(app_name.clone(),key.clone(),serde_json::to_string(&listofvecs).unwrap())
// }

// pub fn addtolist<T: ToString>(app_name:impl Into<String>,key: impl Into<String>,value:T){
//     let app_name:String=app_name.into();
//     let key:String=key.into();
//     let mut listofvecs:Vec<String>=match(getfromlist(app_name.clone(), key.clone())){
//         Ok(alreadycontents) => alreadycontents,
//         Err(_) => Vec::new(),
//     };
//     // let vecofwhat=value.to_string().as_bytes();
//     // let encoded: String = getencodedstring(value.to_string());
//     let encoded: String = getencodedstring(value.to_string());
//     // let utf8_bytes_list: Vec<&[u8]> = value.to_string().chars().into_iter().map(|s| s.as_bytes()).collect();
//     listofvecs.push(encoded);
//     savepreference(app_name.clone(),key.clone(),    listofvecs.join("\n"));
        
//     // savepreference(app_name.clone(),key.clone(),serde_json::to_string(&listofvecs).unwrap())
// }
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
// pub fn getfromlist(app_name:impl Into<String>,key:impl Into<String>)->Result<Vec<String>,()>{
//     let key =key.into();
//     let app_name =app_name.into();
//     let input= match(File::open(&config_path(&app_name,&key))){
//         Ok(mut file) => {
//             let mut buf = String::new();
//             file.read_to_string(&mut buf)
//                 .expect("Cannot read to string");
//             buf
//         },
//         Err(_) => {
//             return Err(())
//             // savepreference(app_name,&key, &defvalue.to_string());
//             // defvalue.to_string()
//         },
//     };
//     // let j:Vec<String>=serde_json::from_str(&input).unwrap();
//     let vec_of_string = input.split("\n").map(|s| getdecoded(s).to_string()).collect::<Vec<String>>();
//     // url2str(j)
//     // get_decoded_string(j);
//     // vec![]
//     Ok(vec_of_string)
// }
// pub fn getfromlistasjson(app_name:impl Into<String>,key:impl Into<String>)->Result<Vec<String>,()>{
//     let key =key.into();
//     let app_name =app_name.into();
//     let input= match(File::open(&config_path(&app_name,&key))){
//         Ok(mut file) => {
//             let mut buf = String::new();
//             file.read_to_string(&mut buf)
//                 .expect("Cannot read to string");
//             buf
//         },
//         Err(_) => {
//             return Err(())
//             // savepreference(app_name,&key, &defvalue.to_string());
//             // defvalue.to_string()
//         },
//     };
//     let file=File::open(&config_path(&app_name,&key)).map_err(|err|{
//         eprintln!("Fil coild not be opened:{err}")

//     })?;
//     // let j:Vec<String>=serde_json::from_str(&input).unwrap();
//     let listdecoded:Vec<String>=serde_json::from_reader(BufReader::new(file)).map_err(|err|{
//         eprintln!("Fil coild not be opened:{err}")

//     })?;
//     // let vec_of_string = listdecoded.split("\n").map(|s| getdecoded(s).to_string()).collect::<Vec<String>>();
//     // url2str(j)
//     // get_decoded_string(j);
//     // vec![]
//     Ok(listdecoded)
// }

// pub fn getallfromlist(app_name:impl Into<String>)->Vec<(String,String)>{
//     let app_name=app_name.into();
//     // println!("{app_name}");
//     let mut gh=config_folder_path(&app_name).to_str().unwrap().to_string();
//     // println!("{}",gh);
//     // let key =key.into();
//     gh.push_str("/*.txt");
//     let mut list_of_strings:Vec<(String,String)>=vec![];
//     // println!("{:?}-----------------{:?}",gh,glob::glob(&gh).expect("Failed to read glob pattern"));
//     for entry in glob::glob(&gh)
//         .expect("Failed to read glob pattern") {
//         match entry {

//             Ok(path) =>{
//                 let input= match(File::open(&path)){
//                     Ok(mut file) => {
//                         let mut buf = String::new();
//                         file.read_to_string(&mut buf).expect("Cannot read to string");
//                         buf
//                     },
//                     Err(_) => {
//                         "".to_string()
//                         // savepreference(app_name,&key, &defvalue.to_string());
//                         // defvalue.to_string()
//                     },
//                 };
//                 let vec_of_string = input.split("\n").map(|s| getdecoded(s).to_string()).collect::<Vec<String>>().join("\n");
//                 let file_name =&path.file_stem().unwrap().to_str().unwrap().to_string();

//                 // for i in vec_of_string{
//                     list_of_strings.push((file_name.to_owned(),vec_of_string));
//                 // }
                
//             },
//             Err(e) => {
//                 eprintln!("error with glob {:?}", e);
                
//             },
//         }
//     }
    
//     // let j:Vec<String>=serde_json::from_str(&input).unwrap();
//     // url2str(j)
//     // get_decoded_string(j);
//     // vec![]
//     list_of_strings
// }
// fn readserdefromfile(input:&File)->Result<Vec<String>,()>{
//     let listdecoded:Vec<String>=serde_json::from_reader(BufReader::new(input)).map_err(|err|{
//         eprintln!("Fil could not be opened:{err}")

//     })?;
//     Ok(listdecoded)
// }
// #[test]
// fn test_addtojson(){
//     let g=vec!["date","event","status","description","repeat"];
//     addlistasjson("todo","test", g.clone());
//     addlistasjson("todo","test1", g.clone());
//     addlistasjson("todo","test2", g.clone());
//     // println!("{:?}", getfromjsonlist("todo","test").unwrap());
//     assert_eq!(g,getfromlistasjson("todo","test").unwrap());
//     println!("{:?}",getallasjsonlist("todo"))
// }
// pub fn getallasjsonlist(app_name:impl Into<String>)->Vec<(String,Vec<String>)>{
//     let app_name=app_name.into();
//     // println!("{app_name}");
//     let mut gh=config_folder_path(&app_name).to_str().unwrap().to_string();
//     // println!("{}",gh);
//     // let key =key.into();
//     gh.push_str("/*.txt");
//     let mut list_of_strings:Vec<(String,Vec<String>)>=vec![];
//     // println!("{:?}-----------------{:?}",gh,glob::glob(&gh).expect("Failed to read glob pattern"));
//     for entry in glob::glob(&gh)
//         .expect("Failed to read glob pattern") {
//         match entry {

//             Ok(path) =>{
//                 let input= match(File::open(&path)){
//                     Ok(mut file) => {
//                         // let mut buf = String::new();
//                         // file.read_to_string(&mut buf).expect("Cannot read to string");
//                         // buf
//                         file
//                     },
//                     Err(_) => {
//                         return vec![]
//                         // savepreference(app_name,&key, &defvalue.to_string());
//                         // defvalue.to_string()
//                     },
//                 };
//                 // let vec_of_string = input.split("\n").map(|s| getdecoded(s).to_string()).collect::<Vec<String>>().join("\n");
//                 println!("{:?}",input);
//                 let listdecoded:Vec<String>=readserdefromfile(&input).unwrap();
//                 let file_name =&path.file_stem().unwrap().to_str().unwrap().to_string();

//                 // for i in vec_of_string{
//                     list_of_strings.push((file_name.to_owned(),listdecoded));
//                 // }
                
//             },
//             Err(e) => {
//                 eprintln!("error with glob {:?}", e);
                
//             },
//         }
//     }
    
    // let j:Vec<String>=serde_json::from_str(&input).unwrap();
    // url2str(j)
    // get_decoded_string(j);
    // vec![]
//     list_of_strings
// }
/// Returns a vector of tuples containing all the files in the specified app's configuration directory.
///
/// # Arguments
///
/// * `app_name` - A string slice that holds the name of the app.
///
/// # Example
///
/// ```
/// use prefstore::*;
///
/// let files = getall("my_app_name");
/// println!("{:?}", files);
/// ```
///
/// # Returns
///
/// A vector of tuples containing file names and their contents.
/// ```
/// vec![
///     ("file1".to_owned(), "content1".to_owned()),
///     ("file2".to_owned(), "content2".to_owned()),
///     ("file3".to_owned(), "content3".to_owned())
/// ]
/// ```
pub fn getall(app_name:impl Into<String>)->Vec<(String,String)>{
    getallcustom(app_name, "txt")
}
pub fn save_last_n_to_file<T: ToString>(app_name:impl Into<String>,custom_filename_with_extension: impl Into<String>,value:impl Into<String>,n:i8){
    // getcustom(app_name, key, defvalue);
    let filename=custom_filename_with_extension.into();
    let stringtos=value.into();
    let vectos=saveStringtonlinesbuffer(stringtos, n,&filename);
    saveVecOfStrings(vectos, &filename);
}
// Assume the data structure is a vector of strings
fn saveStringtonlinesbuffer(string: String,n:i8, filename: &str)->Vec<String> {
    let mut ds=readFileToVec(&filename).unwrap();
    // Push the new string to the end of the vector
    ds.push(string);
    // If the vector has more than 5 elements, remove the oldest one from the front
    if ds.len() > n as usize {
        ds.remove(0);
    }
    ds
}
fn saveVecOfStrings(strings: Vec<String>, file_name: &str) -> Result<(), Error> {
    // Open the file for writing, creating it if it doesn't exist
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_name)?;
    // Iterate over the strings in the vector
    for string in strings {
        // Replace any \n characters with empty strings
        let string = string.replace("\n", "");
        // Write the string to the file, followed by a new line
        writeln!(file, "{}", string)?;
    }
    // Return Ok if no errors occurred
    Ok(())
}
// Assume the file name is a string slice
fn readFileToVec(file_name: &str) -> Result<Vec<String>, Error> {
    // Open the file for reading
    let mut file = fs::File::open(file_name)?;
    // Create a string to hold the file contents
    let mut contents = String::new();
    // Read the entire file into the string
    file.read_to_string(&mut contents)?;
    // Split the string by newline characters and collect into a vector of strings
    let lines: Vec<String> = contents.split('\n').map(|s| s.to_string()).collect();
    // Return the vector as Ok if no errors occurred
    Ok(lines)
}
/// Retrieves the contents of all files with the given extension in the configuration folder for the given application.
///
/// # Arguments
///
/// * `app_name` - The name of the application whose files should be retrieved.
/// * `file_extension` - The extension of the files to be retrieved.
///
/// # Returns
///
/// A vector of tuples representing file names and their contents.
///
/// # Examples
///
/// ```
/// # use prefstore::getallcustom;
/// let files = getallcustom("myapp", "txt");
/// for (name, contents) in files {
///     println!("{}: {}", name, contents);
/// }
/// ```
pub fn getallcustom(app_name:impl Into<String>,file_extension:&str)->Vec<(String,String)>{
    let app_name=app_name.into();
    // println!("{app_name}");
    let mut gh=config_folder_path(&app_name).to_str().unwrap().to_string();
    // println!("{}",gh);
    // let key =key.into();
    gh.push_str(&format!("/*.{}",file_extension));
    let mut list_of_strings:Vec<(String,String)>=vec![];
    // println!("{:?}-----------------{:?}",gh,glob::glob(&gh).expect("Failed to read glob pattern"));
    for entry in glob::glob(&gh)
        .expect("Failed to read glob pattern") {
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
// #[test]
//     fn work_with_list(){
//         // clearpreference(app_name, key)
//         let app_name="todo";
//         let key="date";
//         let value="whatsup";
//         addtolist(app_name, key, value);
//         println!("{:?}",getallfromlist(app_name))
//     }
#[cfg(test)]
mod prefstore_test {
    use super::*;
    
    #[test]
    fn test_getall() {
        savecustom("myapp", "custom1", "value1");
        savecustom("myapp", "custom2", "value2");
        let all_custom = getall("myapp");
        assert_eq!(all_custom.len(), 2);
        assert!(all_custom.contains(&("custom1".to_string(), "value1".to_string())));
        assert!(all_custom.contains(&("custom2".to_string(), "value2".to_string())));
    }

    #[test]
    fn test_savecustom() {
        savecustom("myapp", "custom1", "value1");
        assert_eq!(getcustom("myapp", "custom1", "default"), "value1");
    }
    
    #[test]
    fn test_clearcustom() {
        savecustom("myapp", "custom1", "value1");
        clearcustom("myapp", "custom1");
        assert_eq!(getcustom("myapp", "custom1", "default"), "default");
    }

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
