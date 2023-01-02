mod cli;
mod cmds;

pub const FOLDER: &str = "/Users/matt/src/coding_bat";

fn main() {
    let matches = cli::setup().get_matches();
    match matches.subcommand() {
        Some(("new", _)) => cmds::new::run(),
        _ => unreachable!(),
    }
}
