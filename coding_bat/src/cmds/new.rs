use std::{fs, path::Path, process::Command};

use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Input};

use crate::{cli::DIALOGUER_THEME, FOLDER};

pub fn run() {
    let opts = ask().expect("asking user for options failed");
    let path = Path::new(FOLDER)
        .join(opts.language)
        .join(opts.section)
        .join(format!("{}.py", opts.name));
    fs::write(&path, format!("def {}():\n    ", opts.name)).expect("creating file failed");
    Command::new("code")
        .arg(&path)
        .output()
        .expect("opening document with code failed");
    println!("created {}", path.display());
}

struct Options {
    pub name: String,
    pub language: String,
    pub section: String,
}

fn ask() -> Result<Options> {
    let theme: &ColorfulTheme = &DIALOGUER_THEME;
    let name = Input::with_theme(theme)
        .with_prompt("Name")
        .interact_text()
        .context("asking for assignment name failed")?;
    let language = String::from("python");
    let section = String::from("Warmup-2");
    Ok(Options {
        name,
        language,
        section,
    })
}
