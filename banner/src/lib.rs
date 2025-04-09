use std::num::ParseFloatError;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Flag {
   pub short_hand: String,
   pub long_hand: String,
   pub desc: String,
}

impl<'a> Flag {
    pub fn opt_flag(name: &'a str, d: &'a str) -> Self {
        let short = format!(
            "-{}",
            name.chars()
            .next()
            .expect("Flag name cannot be empty")
        );
        let long = format!("--{}", name);
        Flag {
            short_hand: short,
            long_hand: long,
            desc: d.to_string(),
        }
    }
}

// Type alias for the callback function pointer.
// Takes two string slices (arguments) and returns a Result containing
// either a success String or a ParseFloatError.
pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        // Check if the provided input flag exists in the map
        match self.flags.get(input) {
            Some(callback) => {
                // Ensure we have at least two arguments for the callback
                if argv.len() < 2 {
                    return Err(format!(
                        "Insufficient arguments for flag '{}'. Expected 2, got {}",
                        input,
                        argv.len()
                    ));
                }
                // Execute the callback function with the first two arguments
                // Map the ParseFloatError from the callback to a String for the final result
                callback(argv[0], argv[1]).map_err(|e| e.to_string())
            }
            None => {
                // Flag not found in the handler
                Err(format!("Flag '{}' not found", input))
            }
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
     let num_a = a.parse::<f64>()?;
     let num_b = b.parse::<f64>()?;
 
     // Perform division
     let result = num_a / num_b;
 
     // Convert the result back to a String and wrap it in Ok
     Ok(result.to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
     let num_a = a.parse::<f64>()?;
     let num_b = b.parse::<f64>()?;
 
     // Perform remainder operation
     let result = num_a % num_b;
 
     // Convert the result back to a String and wrap it in Ok
     Ok(result.to_string())
}
