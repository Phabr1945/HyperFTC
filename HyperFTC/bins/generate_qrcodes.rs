extern crate qrcode;use qrcode::*;

use std::path::Path;
use std::io::Write;
use std::fs;

const TEAM_NUMBER: &str = "20022";

fn generate(value: String) -> AnyResult {
    // Generating the QR code
    let qr = QR::from_str(&value)?;
    let svg = qr.image(QRImageType::SVG);

    // Saving the QR code to a file
    let name = format!("./.build/qrcodes/{}.svg", value);
    let mut file = fs::File::create(name)?;
    file.write_all(svg.as_bytes())?;
    Ok(())
}

fn main() -> AnyResult {
    // Creating the folder if needed
    let path = Path::new("./.build/qrcodes");
    if !path.exists() {
        fs::create_dir(path)?;
    }

    // Generating the QR codes
    generate(format!("{}-1", TEAM_NUMBER))?;
    generate(format!("{}-2", TEAM_NUMBER))?;
    generate(format!("{}-3", TEAM_NUMBER))?;

    Ok(())
}