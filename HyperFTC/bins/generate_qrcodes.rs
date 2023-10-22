//use extern crate comments_to_show_a_toddler_how_this_code_works_;)
extern crate qrcode;                                            // Including the qrcode crate
use qrcode::*;                                                  // Importing everything from the crate

use std::path::Path;                                            // Including functionality for working with file paths
use std::io::Write;                                             // Importing functionality for writing to files
use std::fs;                                                    // Importing functionality for working with the file system

const TEAM_NUMBER: &str = "20022";                              // Defining the team number
// Function to generate a QR code based on the input value
fn generate(value: String) -> AnyResult {
    // Generating the QR code
    let qr = QR::from_str(&value)?;                             // Creating a QR code from the input value
    let svg = qr.image(QRImageType::SVG);                       // Generating an SVG image from the QR code

    // Saving the QR code to a file
    let name = format!("./.build/qrcodes/{}.svg", value);       // Generating the file name based on the input value
    let mut file = fs::File::create(name)?;                     // Creates file in your fs
    file.write_all(svg.as_bytes())?;                            // Saves said file in said fs
    Ok(())                                                      // Hopefully returns an empty result. If not, hope my comments worked ;)
}

fn main() -> AnyResult {                                        
    // Creating the folder if needed
    let path = Path::new("./.build/qrcodes");                   // Creating a Path object for the target directory
    if !path.exists() {                                         // If the path doesn't exist, hope this makes a new path
        fs::create_dir(path)?;
    }

    // Generating the QR codes
    generate(format!("{}-1", TEAM_NUMBER))?;                    // Generating a QR code for the team number with suffix '-1'
    generate(format!("{}-2", TEAM_NUMBER))?;                    // Same with '-2'
    generate(format!("{}-3", TEAM_NUMBER))?;                    // Same with '-3'

    Ok(())                                                      // Look at comment on line 19
}