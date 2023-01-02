use std::{fs, path::Path, process::Command};

use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};

use crate::{cli::DIALOGUER_THEME, FOLDER};

pub fn run() {
    let opts = ask().expect("asking user for options failed");
    let path = Path::new(FOLDER)
        .join(opts.language)
        .join(opts.section)
        .join(format!("{}.py", opts.name));
    fs::write(&path, "").expect("creating file failed");
    Command::new("code")
        .arg(path)
        .output()
        .expect("opening document with code failed");
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
    let sections = ["Warmup-1", "Warmup-2"];
    let section = sections
        .get(
            FuzzySelect::with_theme(theme)
                .items(&sections)
                .interact()
                .context("asking for section name failed")?,
        )
        .unwrap();
    Ok(Options {
        name,
        language,
        section: section.to_string(),
    })
}
