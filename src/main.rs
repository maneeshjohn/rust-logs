use std::{fs, io::Result};

fn get_errors(text: &str) -> Vec<&str> {
    let lines = text.split("\n");
    let mut result = vec![];

    for line in lines {
        if line.starts_with("ERROR") {
            result.push(line);
        }
    }
    return result;
}

fn main() -> Result<()> {
    let content = fs::read_to_string("logs.txt")?;
    let errors = get_errors(content.as_str());
    fs::write("out.txt", errors.join("\n"))?;
    Ok(())
}
