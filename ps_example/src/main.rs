use prefstore::*;
use std::io::Result;

fn main() -> Result<()> {
    const APPNAME: &str = "prefstore_demo";

    println!("=== Prefstore Library Comprehensive Demo ===\n");

    // 1. Basic preference operations
    println!("1. Basic Preference Operations:");
    println!("------------------------------");

    // Save different types of preferences
    savepreference(APPNAME, "username", "john_doe")?;
    savepreference(APPNAME, "age", 25)?;
    savepreference(APPNAME, "height", 5.9)?;
    savepreference(APPNAME, "is_active", true)?;
    savepreference(APPNAME, "big_number", 99999999999999999999999999999999999999i128)?;

    // Retrieve preferences with defaults
    println!("Username: {}", getpreference(APPNAME, "username", "default_user"));
    println!("Age: {}", getpreference(APPNAME, "age", 0));
    println!("Height: {}", getpreference(APPNAME, "height", 0.0));
    println!("Active: {}", getpreference(APPNAME, "is_active", false));
    println!("Big Number: {}", getpreference(APPNAME, "big_number", 0i128));

    // Get preference without default (returns Result)
    match getpreferencenodefault(APPNAME, "username") {
        Ok(value) => println!("Username (no default): {}", value),
        Err(e) => println!("Error getting username: {}", e),
    }

    // 2. Custom file operations
    println!("\n2. Custom File Operations:");
    println!("-------------------------");

    // Save custom data
    savecustom(APPNAME, "config.json", r#"{"theme": "dark", "lang": "en"}"#)?;
    savecustom(APPNAME, "notes.txt", "This is a note.")?;

    // Get custom data
    let config = getcustom(APPNAME, "config.json", "{}")?;
    let notes = getcustom(APPNAME, "notes.txt", "No notes")?;
    println!("Config: {}", config);
    println!("Notes: {}", notes);

    // Initialize file with value
    initcustomfile(APPNAME, "counter.txt", "0")?;

    // Append to files
    appendcustom(APPNAME, "log.txt", "Application started.")?;
    appendcustomnewline(APPNAME, "log.txt", "User logged in.")?;
    appendcustomnewline(APPNAME, "log.txt", "Operation completed.")?;

    // Read file lines into vector
    let log_lines = opencustomperlinetovec(APPNAME, "log.txt")?;
    println!("Log entries:");
    for (i, line) in log_lines.iter().enumerate() {
        println!("  {}: {}", i + 1, line);
    }

    // 3. Buffer operations
    println!("\n3. Buffer Operations:");
    println!("--------------------");

    // Save to buffer (maintains last N items)
    savebuffer(APPNAME, "recent_actions", "User login", 5);
    savebuffer(APPNAME, "recent_actions", "File saved", 5);
    savebuffer(APPNAME, "recent_actions", "Settings changed", 5);
    savebuffer(APPNAME, "recent_actions", "Profile updated", 5);

    // Get last item from buffer
    let last_action = get_last_from_buffer(APPNAME, "recent_actions");
    println!("Last action: {}", last_action);

    // Get entire buffer
    let all_actions = getbuffer(APPNAME, "recent_actions");
    println!("All recent actions:");
    for (i, action) in all_actions.iter().enumerate() {
        println!("  {}: {}", i + 1, action);
    }

    // 4. Bulk operations
    println!("\n4. Bulk Operations:");
    println!("------------------");

    // Get all preferences
    let all_prefs = getall(APPNAME);
    println!("All preferences:");
    for (key, value) in all_prefs {
        println!("  {}: {}", key, value);
    }

    // Get all custom files
    let all_custom = getallcustom(APPNAME, "txt");
    println!("All custom txt files:");
    for (name, content) in all_custom {
        println!("  {}: {}", name, content);
    }

    // 5. String parsing with ems trait
    println!("\n5. String Parsing Operations:");
    println!("-----------------------------");

    // Parse strings to different types
    let bool_str = "true".to_string();
    let int_str = "42".to_string();
    let float_str = "3.14159".to_string();
    let big_int_str = "99999999999999999999999999999999999999".to_string();

    println!("Parsed values:");
    println!("  Boolean: {}", bool_str.tobool());
    println!("  Integer: {}", int_str.toi32().unwrap_or(0));
    println!("  Float: {}", float_str.tof64().unwrap_or(0.0));
    println!("  Big Int: {}", big_int_str.toi128().unwrap_or(0));

    // 6. Path and directory operations
    println!("\n6. Path and Directory Operations:");
    println!("---------------------------------");

    // Get prefstore directory
    match prefstore_directory(&APPNAME.to_string()) {
        Ok(path) => println!("Prefstore directory: {}", path.display()),
        Err(e) => println!("Error getting directory: {}", e),
    }

    // Clear specific preferences
    println!("\n7. Cleanup Operations:");
    println!("---------------------");

    clearpreference(APPNAME, "temporary_data")?;
    clearcustom(APPNAME, "temp.txt");

    // Clear all files with specific extension
    clearall(APPNAME, "log")?;
    println!("Cleared all .log files");

    // 8. Error handling demonstration
    println!("\n8. Error Handling:");
    println!("------------------");

    // Try to get non-existent preference
    match getpreferencenodefault(APPNAME, "nonexistent") {
        Ok(value) => println!("Found value: {}", value),
        Err(e) => println!("Expected error for non-existent key: {}", e),
    }

    // Try to save to invalid path
    match save_else_where("/invalid/path/test.txt", "test data") {
        Ok(()) => println!("Saved to custom path successfully"),
        Err(e) => println!("Expected error for invalid path: {}", e),
    }

    println!("\n=== Demo completed successfully! ===");

    Ok(())
}
