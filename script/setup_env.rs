use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    println!("hello world");

    let mut file = File::create("config.toml")?;
    file.write_all(content().as_bytes())?;
    println!("{}", content());
    Ok(())
}

fn content() -> String {
    let env_api = std::env::vars()
        .into_iter()
        .filter(|(key, _)| key.as_str() == "SUTOM_API_KEY")
        .map(|(key, value)| {
            format!("{key} = \"{value}\"")
        })
        .collect::<Vec<String>>();

    format!(
        "{}\n{}",
        String::from("[api]"),
        env_api.join("\n")
    )
}