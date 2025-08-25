#![allow(warnings)] 

use std::{fs::{File, create_dir_all, remove_file, read_to_string,OpenOptions}, io::{Write,BufReader, self, Read, Error, BufRead}, path::{PathBuf, Path}, collections::HashMap, fmt::format};
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
pub fn savepreference<T: ToString>(app_name:impl Into<String>,key: impl Into<String>,value:T) -> std::io::Result<()> {
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
///
/// # Returns
///
/// A `Result` with an `Ok` value if the operation was successful, or an `Err` value
/// containing an IO error if the operation failed.
pub fn savecustom<T: ToString>(app_name:impl Into<String>,custom_filename_with_extension: impl Into<String>,value:T) -> std::io::Result<()> {
    let key=custom_filename_with_extension.into();
    let app_name=app_name.into();
    let fname=" #savecustom";

    let path = customfile_path(&app_name,&key)?;
    let parent_path = path.parent()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, format!("Cannot find path to {fname}")))?;

    create_dir_all(parent_path)?;

    let mut file = File::create(&path)?;
    write!(file, "{}", value.to_string())?;

    Ok(())
}
#[test]
fn uiouy(){
    save_else_where("/tmp/new/try.json", "value");
}
pub fn save_else_where<T: ToString>(custom_filename_with_extension: impl Into<String>,value:T) -> std::io::Result<()> {
    let key=custom_filename_with_extension.into();
    let fname=" #savecustom";

    let path = Path::new(&key);
    let parent_path = path.parent()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, format!("Cannot find path to {fname}")))?;

    create_dir_all(parent_path)?;

    let mut file = File::create(&path)?;
    write!(file, "{}", value.to_string())?;

    Ok(())
}
/// Initializes the file with the given app name and key, with the given value.
///
/// # Arguments
///
/// * `app_name`: The name of the app.
/// * `custom_filename_with_extension`: The custom filename with extension.
/// * `value`: The value to initialize the file with.
///
/// # Returns
///
/// A `Result` with an `Ok` value if the operation was successful, or an `Err` value
/// containing an error message if the operation failed.
pub fn initcustomfile<T: ToString>(app_name: impl Into<String>, custom_filename_with_extension: impl Into<String>, value: T) -> std::io::Result<()> {
    let key = custom_filename_with_extension.into();
    let app_name = app_name.into();
    let fname = "#initcustom";

    let path = customfile_path(&app_name, &key)?;
    let parent_path = path.parent()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, format!("Cannot find path to {}", fname)))?;

    create_dir_all(parent_path)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)?;
    file.write_all(format!("{}", value.to_string()).as_bytes())?;
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
/// Appends the given value to the file with the given app name and key.
///
/// # Arguments
///
/// * `app_name`: The name of the app.
/// * `custom_filename_with_extension`: The custom filename with extension.
/// * `value`: The value to append.
pub fn appendcustom<T: ToString>(app_name: impl Into<String>, custom_filename_with_extension: impl Into<String>, value: T) -> std::io::Result<()> {
    let key = custom_filename_with_extension.into();
    let fname = "#appendcustom";
    let app_name = app_name.into();

    let path = customfile_path(&app_name, &key)?;
    let parent_path = path.parent()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, format!("Cannot find path to {}", fname)))?;

    create_dir_all(parent_path)?;

    let mut file = File::options().create(true).append(true).open(&path)?;
    write!(file, "{}", value.to_string())?;
    Ok(())
}

/// Appends the given value to the file with the given app name and key, followed by a newline character.
///
/// # Arguments
///
/// * `app_name`: The name of the app.
/// * `custom_filename_with_extension`: The custom filename with extension.
/// * `value`: The value to append.
pub fn appendcustomnewline<T: ToString>(app_name: impl Into<String>, custom_filename_with_extension: impl Into<String>, value: T) -> std::io::Result<()> {
    let key = custom_filename_with_extension.into();
    let fname = "#appendcustomnewline";
    let app_name = app_name.into();

    let path = customfile_path(&app_name, &key)?;
    let parent_path = path.parent()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, format!("Cannot find path to {}", fname)))?;

    create_dir_all(parent_path)?;

    let mut file = File::options().create(true).append(true).open(&path)?;
    writeln!(file, "{}", value.to_string())?;
    Ok(())
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
/// # Returns
///
/// A `Result` containing the path or an IO error if the system config directory cannot be found.
///
/// # Examples
///
/// ```
/// use prefstore::config_path;
///
/// let app_name = "myapp";
/// let filename = "myconfig";
/// let path = config_path(&app_name.to_string(), &filename.to_string()).unwrap();
/// ```
fn config_path(app_name:&String,filename:impl Into<String>) -> std::io::Result<PathBuf> {
    match(dirs::config_dir()){
        Some(system_config_dir) =>{
            Ok(system_config_dir
                    .join(app_name)
                    .join(default_name(filename.into())))
        },
        None => {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, MSG_NO_SYSTEM_CONFIG_DIR))
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
pub fn prefstore_directory(app_name:&String)->Result<PathBuf,std::io::Error>{
    let config_dir = dirs::config_dir()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Config folder does not exist"))?;

    Ok(config_dir.join(app_name))
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
/// A `Result` containing the full path to the custom file or an IO error if the system config directory cannot be found.
fn customfile_path(app_name:&String,filename:impl Into<String>) -> std::io::Result<PathBuf> {
    // Get the system configuration directory.
    match(dirs::config_dir()){
        Some(system_config_dir) =>{
            // Join the system configuration directory, the app name, and the custom file name.
            Ok(system_config_dir
                    .join(app_name)
                    .join(custom_file_name(filename.into())))
        },
        None => {
            // If there is no system configuration directory, return an error.
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, MSG_NO_SYSTEM_CONFIG_DIR))
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
pub fn clearpreference(app_name:impl Into<String>,key: impl Into<String>) -> std::io::Result<()> {
    let path = config_path(&app_name.into(),&key.into())?;
    remove_file(&path)?;
    Ok(())
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
    let path = match customfile_path(&app_name.into(),&custom_filename_with_extension.into()) {
        Ok(path) => path,
        Err(_) => return,
    };

    match remove_file(&path) {
        Ok(_) => {},
        Err(_) => {},
    };
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
pub fn clearall(app_name: impl Into<String>, file_extension: &str) -> std::io::Result<()> {
    let app_name = app_name.into();

    // Construct the path to the configuration folder for the application.
    let config_path = config_folder_path(&app_name)?;
    let path_str = config_path
        .to_str()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid path characters"))?
        .to_string();

    // Append the glob pattern to match files with the given extension.
    let gh = format!("{}/**/*.{}", path_str, file_extension);

    // Iterate over all files that match the glob pattern and attempt to remove them.
    for entry in glob::glob(&gh).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))? {
        match entry {
            Ok(path) => {
                if let Err(e) = remove_file(path) {
                    eprintln!("Failed to remove file: {:?}", e);
                }
            }
            Err(e) => {
                eprintln!("error with glob {:?}", e);
            }
        }
    }
    Ok(())
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
    let defvalue_str = defvalue.to_string();
    getcustom(app_name, format!("{}.txt", key.into()), defvalue).unwrap_or(defvalue_str)
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

pub fn getcustom<T:ToString>(app_name:impl Into<String>,key:impl Into<String>,defvalue:T)->std::io::Result<String>{
    let key =key.into();
    let app_name =app_name.into();
    if(!key.is_empty()){
        let path = customfile_path(&app_name,&key)?;
        match(File::open(&path)){
            Ok(mut file) => {
                let mut buf = String::new();
                file.read_to_string(&mut buf)?;
                Ok(buf)
            },
            Err(_) => {
                savecustom(app_name,&key, &defvalue.to_string())?;
                Ok(defvalue.to_string())
            },
        }
    }
    else{
        Ok(String::new())
    }
}
/// Opens the file with the given app name and key and reads the lines into a vector.
///
/// # Arguments
///
/// * `app_name`: The name of the app.
/// * `key`: The key of the file.
///
/// # Returns
///
/// A vector of strings containing the lines of the file.
pub fn opencustomperlinetovec(app_name: impl Into<String>, key: impl Into<String>) -> std::io::Result<Vec<String>> {
    let key = key.into();
    let app_name = app_name.into();
    // Create an empty vector to store the lines
    let mut lines = Vec::new();
    // Open the file and create a buffered reader
    let path = customfile_path(&app_name, &key)?;
    match File::open(&path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            // Iterate over the lines of the reader
            for line in reader.lines() {
                // Get the line as a string and push it to the vector
                let line = line?;
                lines.push(line);
            }
        },
        Err(_) => {
            // Do nothing - return empty vector
        },
    }
    // Return the vector of lines
    Ok(lines)
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
pub fn getpreferencenodefault(app_name:impl Into<String>,key:impl Into<String>)->std::io::Result<String>{
    use io::Read;
    let key =key.into();
    let app_name =app_name.into();
    let path = config_path(&app_name,&key)?;
    match(File::open(&path)){
        Ok(mut file) => {
            let mut buf = String::new();
            file.read_to_string(&mut buf)?;
            Ok(buf)
        },
        Err(_) => {
            Ok(String::new())
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
    /// # Returns
    ///
    /// A Result containing the parsed i32 value or an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefstore::ems;
    ///
    /// let value = "42";
    /// let i32_value = value.toi32().unwrap();
    /// assert_eq!(i32_value, 42);
    /// ```
    fn toi32(self)->Result<i32, std::num::ParseIntError>;

    /// Converts a String to an i128.
    ///
    /// # Arguments
    ///
    /// * `self` - A string slice that holds the value to be converted.
    ///
    /// # Returns
    ///
    /// A Result containing the parsed i128 value or an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefstore::ems;
    ///
    /// let value = "99999999999999999999999999999999999999";
    /// let i128_value = value.toi128().unwrap();
    /// assert_eq!(i128_value, 99999999999999999999999999999999999999);
    /// ```
    fn toi128(self)->Result<i128, std::num::ParseIntError>;

    /// Converts a String to an f64.
    ///
    /// # Arguments
    ///
    /// * `self` - A string slice that holds the value to be converted.
    ///
    /// # Returns
    ///
    /// A Result containing the parsed f64 value or an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefstore::ems;
    ///
    /// let value = "3.14";
    /// let f64_value = value.tof64().unwrap();
    /// assert_eq!(f64_value, 3.14);
    /// ```
    fn tof64(self)->Result<f64, std::num::ParseFloatError>;
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
    fn toi32(self)->Result<i32, std::num::ParseIntError>{
        self.parse::<i32>()
    }
    fn toi128(self)->Result<i128, std::num::ParseIntError>{
        self.parse::<i128>()
    }
    fn tof64(self)->Result<f64, std::num::ParseFloatError>{
        self.parse::<f64>()
    }
}
/// Gets the path to the config folder for the given app name.
///
/// # Arguments
///
/// * `app_name`: The name of the app.
///
/// # Returns
///
/// The path to the config folder.
pub fn config_folder_path(app_name: &str) -> std::io::Result<PathBuf> {
    match dirs::config_dir() {
        Some(system_config_dir) => {
            Ok(system_config_dir.join(app_name))
        },
        None => {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, MSG_NO_SYSTEM_CONFIG_DIR))
        },
    }
}

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
/// Gets all the entries in the buffer for the given app name.
///
/// # Arguments
///
/// * `app_name`: The name of the app.
///
/// # Returns
///
/// A vector of tuples containing the key and value of each entry in the buffer.
pub fn getall(app_name: impl Into<String>) -> Vec<(String, String)> {
    getallcustom(app_name, "txt")
}

/// Saves the given value to the buffer for the given app name and custom filename with extension.
///
/// # Arguments
///
/// * `app_name`: The name of the app.
/// * `custom_filename_with_extension`: The custom filename with extension.
/// * `value`: The value to save.
/// * `buffersize`: The maximum number of strings to keep in the buffer.
///
/// # Returns
///
/// The updated buffer.
pub fn savebuffer(app_name: impl Into<String>, custom_filename_with_extension: impl Into<String>, value: impl Into<String>, buffersize: i8) {
    // getcustom(app_name, key, defvalue);
    let app_name = app_name.into();
    let filename = custom_filename_with_extension.into();
    let stringtos = value.into();
    let mut vectos = saveStringtobuffer(&app_name, stringtos, buffersize, &filename);
    // vectos.reverse();
    saveVecOfStrings(&app_name, vectos, &filename);
}

/// Gets the last string from the buffer for the given app name and custom filename with extension.
///
/// # Arguments
///
/// * `app_name`: The name of the app.
/// * `custom_filename_with_extension`: The custom filename with extension.
///
/// # Returns
///
/// The last string from the buffer, or an empty string if the buffer is empty.
pub fn get_last_from_buffer(app_name: impl Into<String>, custom_filename_with_extension: impl Into<String>) -> String {
    getbuffer(&app_name.into(), &custom_filename_with_extension.into()).pop().unwrap_or(String::new()).to_string()
}

/// Saves the given string to the buffer for the given app name and filename.
///
/// # Arguments
///
/// * `app_name`: The name of the app.
/// * `string`: The string to save.
/// * `n`: The maximum number of strings to keep in the buffer.
/// * `filename`: The filename of the buffer.
///
/// # Returns
///
/// The updated buffer.
fn saveStringtobuffer(app_name: &str, string: String, n: i8, filename: &str) -> Vec<String> {
    let mut ds = getbuffer(app_name, filename);
    // Push the new string to the end of the vector
    ds.push(string.clone());
    // println!("{}", string);
    // println!("{}/{}--------{:?}", ds.len(), n as usize, ds);
    if ds.len() > n as usize {
        println!("removing");
        ds.remove(0);
    }
    ds
}

/// Saves a vector of strings to a file.
///
/// # Arguments
///
/// * `app_name`: The name of the app.
/// * `strings`: The vector of strings to save.
/// * `file_name`: The filename of the file to save to.
///
/// # Returns
///
/// A `Result` with an `Ok` value if the operation was successful, or an `Err` value
/// containing an error message if the operation failed.
fn saveVecOfStrings(app_name: &str, strings: Vec<String>, file_name: &str) -> Result<(), Error> {
    // Open the file for writing, creating it if it doesn't exist
    let _ = opencustomperlinetovec(app_name, file_name).unwrap_or_default();
    clearcustom(app_name, file_name);
    initcustomfile(app_name, file_name, "")?;
    // Iterate over the strings in the vector
    for string in strings {
        if !string.is_empty() {
            appendcustomnewline(app_name, file_name, string)?;
        }
    }
    // Return Ok if no errors occurred
    Ok(())
}

/// Gets the buffer for the given app name and file name.
///
/// # Arguments
///
/// * `app_name`: The name of the app.
/// * `file_name`: The filename of the file to read from.
///
/// # Returns
///
/// A vector of strings containing the contents of the buffer.
pub fn getbuffer(app_name: &str, file_name: &str) -> Vec<String> {
    opencustomperlinetovec(app_name, file_name).unwrap_or_default()
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
    getallcustomwithin(app_name,"", file_extension).unwrap_or_default().into_iter().collect()
}
pub fn getallcustomwithin(app_name:impl Into<String>,sub_path:&str,file_extension:&str)->std::io::Result<HashMap<String,String>>{
    let app_name=app_name.into();
    // println!("{app_name}");
    let config_path = config_folder_path(&app_name)?;
    let path_str = config_path
        .to_str()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "could not find config folder path"))?
        .to_string();
    // println!("{}",path_str);
    // let key =key.into();
    let y=format!("/{}",sub_path);
    let subpath=if(sub_path==""){
        ""}else{
           &y};
    let gh = format!("{}/**/*.{}",path_str,file_extension);
    let mut list_of_strings:HashMap<String,String>=HashMap::new();
    // println!("{:?}-----------------{:?}",gh,glob::glob(&gh).expect("Failed to read glob pattern"));
    for entry in glob::glob(&gh).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))? {
        match entry {

            Ok(path) =>{
                let input= match(File::open(&path)){
                    Ok(mut file) => {
                        let mut buf = String::new();
                        file.read_to_string(&mut buf)?;
                        buf
                        // file
                    },
                    Err(_) => {
                        return Ok(HashMap::new())
                        // savepreference(app_name,&key, &defvalue.to_string());
                        // defvalue.to_string()
                    },
                };
                // let vec_of_string = input.split("\n").map(|s| getdecoded(s).to_string()).collect::<Vec<String>>().join("\n");
                println!("{:?}",input);
                // let listdecoded:Vec<String>=readserdefromfile(&input).unwrap();
                let file_name = path.file_stem()
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "no file name found"))?
                .to_str()
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "could not convert file name to str"))?
                .to_string();

                // for i in vec_of_string{
                    list_of_strings.insert(file_name.to_owned(),input);
                // }

            },
            Err(e) => {
                eprintln!("error with glob {:?}", e);

            },
        }
    }


    Ok(list_of_strings)
}

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
        savecustom("myapp", "custom1", "value1").unwrap();
        assert_eq!(getcustom("myapp", "custom1", "default").unwrap(), "value1");
    }
    
    #[test]
    fn test_clearcustom() {
        savecustom("myapp", "custom1", "value1").unwrap();
        clearcustom("myapp", "custom1");
        assert_eq!(getcustom("myapp", "custom1", "default").unwrap(), "default");
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

        let i32_value = "42".to_string().toi32().unwrap();
        assert_eq!(i32_value, 42);

        let i128_value = "99999999999999999999999999999999999999".to_string().toi128().unwrap();
        assert_eq!(i128_value, 99999999999999999999999999999999999999);

        let f64_value = "3.14".to_string().tof64().unwrap();
        assert_eq!(f64_value, 3.14);
    }

    // #[test]
    // fn test_buffer(){
    //     savebuffer("prefstore", "last.save", "yu", 3);
    //     savebuffer("prefstore", "last.save", "", 3);
    //     savebuffer("prefstore", "last.save", "yu3", 3);
    //     savebuffer("prefstore", "last.save", "yu4", 3);
    //     savebuffer("prefstore", "last.save", "yu5", 3);
    //     savebuffer("prefstore", "last.save", "yu6", 3);
    //     println!("{:?}",getbuffer("prefstore", "last.save"));
    //     println!("{:?}",get_last_from_buffer("prefstore", "last.save"));
    // }
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

#[test]
fn globtest(){
    println!("1--------->{:?}",getallcustomwithin("filedime", "custom_scripts","fds").unwrap_or_default());
//     let mut gh = config_folder_path("filedime")
//     .to_str()
//     .unwrap()
//     .to_string();

//     // Append the glob pattern to match files with the given extension.
//     gh.push_str(&format!("/**/*.{}", "fds"));

// // Iterate over all files that match the glob pattern and attempt to remove them.
// for entry in glob::glob(&gh).expect("Failed to read glob pattern") {
//     match entry {
//         Ok(path) => {
//             println!("{:?}",path);
//             // remove_file(path);
//         }
//         Err(e) => {
//             eprintln!("error with glob {:?}", e);
//         }
//     }
// }
}

#[test]
fn trvy(){
    crate::getall("perlink").is_empty();
}