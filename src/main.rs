const API_URL: &str = "https://api.ipify.org";

fn main() -> Result<(), reqwest::Error> {
    let res = reqwest::blocking::get(API_URL)?;
    let body = res.text()?;
    println!("{body}");
    Ok(())
}
