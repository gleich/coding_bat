use clap::Command;
use dialoguer::theme::ColorfulTheme;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DIALOGUER_THEME: ColorfulTheme = ColorfulTheme::default();
}

pub fn setup() -> Command {
    Command::new("coding_bat")
        .version("1.0.0")
        .author("Matt Gleich <email@mattglei.ch")
        .about("small utility for creating coding bat files")
        .arg_required_else_help(true)
        .subcommand(Command::new("new").about("make a new document"))
}
