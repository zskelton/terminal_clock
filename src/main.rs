use chrono::prelude::*;
use ansi_term::Colour;

/* PROCESS FUNCTIONS */
fn display_time() -> Result<(), String> {
    let local: DateTime<Local> = Local::now();
    let disp = local.to_rfc2822();

    println!("{disp}\n");
    // debug_show(format!("{}\n", disp));

    Ok(())
}

/* MAIN */
fn main() {
    // Display Time
    if !display_time().is_ok() {
        println!("{}", Colour::Red.paint(format!("Error: Date did not compute.\n")));
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
