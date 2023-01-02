use std::{env, ffi::OsStr, process::Command};

use git2::{Repository, Status};
use walkdir::WalkDir;

use crate::FOLDER;

pub fn run() {
    let repo = Repository::open(FOLDER).expect("opening repo failed");
    let cwd = env::current_dir().expect("getting current working dir failed");

    let new_file_states = [Status::INDEX_NEW, Status::WT_NEW];
    let modified_file_states = [
        Status::INDEX_MODIFIED,
        Status::WT_MODIFIED,
        Status::INDEX_RENAMED,
        Status::WT_RENAMED,
    ];
    let deleted_file_states = [Status::INDEX_DELETED, Status::WT_DELETED];
    for entry in WalkDir::new(FOLDER)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
        .filter(|e| e.path().extension() == Some(OsStr::new("py")))
    {
        let path = entry.path();
        let status = repo
            .status_file(&pathdiff::diff_paths(&path, &cwd).unwrap())
            .expect("getting status of file failed");
        let subject = path
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        let name = path.file_name().unwrap().to_str().unwrap();
        let mut commit_msg: Option<String> = None;
        if new_file_states.contains(&status) {
            commit_msg = Some(format!("feat[{}]: finish {}", subject, name));
        } else if modified_file_states.contains(&status) {
            commit_msg = Some(format!("fix[{}]: {}", subject, name));
        } else if deleted_file_states.contains(&status) {
            commit_msg = Some(format!("feat[{}]: remove {}", subject, name));
        }

        if commit_msg.is_some() {
            Command::new("git")
                .current_dir(FOLDER)
                .arg("add")
                .arg(path)
                .output()
                .expect(&format!("staging {} with git failed", path.display()));
            Command::new("git")
                .current_dir(FOLDER)
                .arg("commit")
                .arg("-m")
                .arg(commit_msg.as_ref().unwrap())
                .output()
                .expect(&format!("committing {} with git failed", path.display()));
            println!("committed {} ({})", name, commit_msg.unwrap());
        }
    }
}
