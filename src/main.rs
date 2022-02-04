use chrono::prelude::*;

fn main() {
    let local: DateTime<Local> = Local::now();

    println!("Localtime: {}", local);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}