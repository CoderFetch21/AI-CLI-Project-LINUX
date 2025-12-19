use std::fs;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Eq)]
pub struct DistroDetails {
    pub id: String,
    pub id_like: Option<String>,
}

pub fn get_distro() -> io::Result<DistroDetails> {
    let file = fs::File::open("/etc/os-release")?;
    let reader = io::BufReader::new(file);

    let mut id = String::new();
    let mut id_like: Option<String> = None;

    for line in reader.lines() {
        let line = line?;
        if let Some(captures) = line.strip_prefix("ID=") {
            id = captures.trim_matches('"').to_string();
        } else if let Some(captures) = line.strip_prefix("ID_LIKE=") {
            id_like = Some(captures.trim_matches('"').to_string());
        }
    }

    Ok(DistroDetails { id, id_like })
}
