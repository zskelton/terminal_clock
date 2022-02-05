use chrono::prelude::*;
use ansi_term::Colour::{White, Cyan, Red};

/* PROCESS FUNCTIONS */
fn display_time() -> Result<(), String> {
    // Get Data
    let local: DateTime<Local> = Local::now();
    let disp = local.to_rfc2822();

    // Build Display
    let intro: String = format!("{}", White.bold().paint("Current:"));
    let data: String = format!("{}", Cyan.bold().paint(disp));

    // Show
    println!("{intro} {data}");

    // Return Success
    Ok(())
}

/* MAIN */
fn main() {
    // Display Time
    if !display_time().is_ok() {
        println!("{}", Red.paint(format!("Error: Date did not compute.")));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_works() {
        let _res = match display_time() {
            Ok(_) => true,
            Err(_) => false
        };
        assert!(_res);
    }
}
