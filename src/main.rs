extern crate regex;

use std::process::Command;
use regex::RegexBuilder;

fn main() {
    let output = Command::new("tmux").arg("ls").output().expect("failed to run tmux ls");
    if output.status.success() {
        let raw = match String::from_utf8(output.stdout) {
            Err(_) => "non-utf8 output".to_string(),
            Ok(s) => s
        };
        let re = RegexBuilder::new(r"^([^:]*):.+?(\(attached\))?$").multi_line(true).compile().unwrap();
        let mut sessions = Vec::new();
        for cap in re.captures_iter(&raw) {
            let session = cap.at(1).unwrap().to_owned();
            if cap.at(2).is_some() {
                let active_session = format!("<fc=orange>{}</fc>", session);
                sessions.push(active_session);
            } else {
                sessions.push(session);
            }
        }
        if sessions.is_empty() {
            println!("tmux: [none]");
        } else {
            println!("tmux: {}", sessions.join(", "));
        }
    }
}
