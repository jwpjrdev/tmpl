use console::style;
use hhmmss::Hhmmss;
use std::env::current_dir;
use stopwatch::Stopwatch;

use crate::terminal::*;

mod commands;
mod config;
mod git;
mod terminal;
mod utils;

// rm -rf testing/ && mkdir testing/ && cd testing/ && cargo run --manifest-path=../Cargo.toml -- https://github.com/jwpjrdev/redwood-tmpl/ && cd ../

// todo:
// - eliminate #unwrap() & #ok() from code. loose ends == bad
// - write unit tests
// - remove as much extra code as possible—especially references
// - support branches (might already be supported with user/repo#branch syntax)
// - check if using gitpod (check for /ide/bin/gp-code or if GITPOD_REPO_ROOT env var exists)
//   - force using gitpod for now
// - support an existing (empty) repo
// - change panic macro usages to eprintln
fn main() {
    let url = run_clap();
    let terminal = Terminal::new();
    let cd = current_dir().unwrap();
    let cd_name = cd.to_str().unwrap();

    if !utils::is_using_gitpod() {
        terminal.print(format!(
            "{}",
            style("Error: TMPL can only be used in Gitpod")
                .red()
                .bright()
                .bold()
        ));
        return;
    }

    let bare_repo_exists = match git2::Repository::open(&cd) {
        Ok(repo) => repo.is_bare(),
        _ => false, // change to err... or true?
    };

    /*
    if using Gitpod {
        then cd exists
        then empty git repo probably exists
    } else {
        if cd exists (it should) {
            if git repo exists {
                then preserve it
            } else {
                then create it?
            }
        } else {
            then create cd (already handled by utils::copy_dir_all)
        }
    }
    */

    if cd.exists() {
        // if it isn't empty
        if !(bare_repo_exists || cd.exists() && cd.read_dir().unwrap().next().is_none()) {
            terminal.print(format!(
                "The provided directory ({}) must be empty",
                &style(cd_name).yellow()
            ));
            return;
        }
    }

    let stopwatch = Stopwatch::start_new();

    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();
    git::clone(&terminal, &url, temp_path);
    std::fs::remove_dir_all(&temp_path.join(".git/")).unwrap();

    let tmpl_config = match std::fs::read_to_string(&temp_path.join("tmpl.toml")) {
        Ok(contents) => contents,
        Err(e) => panic!("failed to read tmpl.toml: {}", e),
    };
    let config = config::parse(&tmpl_config);

    std::fs::remove_file(&temp_path.join("tmpl.toml")).unwrap();

    commands::run_command_batch(&config.commands.pre_install);
    commands::run_command_batch(&config.commands.install);

    // todo: file permission error handling
    utils::copy_dir_all(&temp_path, &cd).unwrap();
    temp_dir.close().ok();

    // 6. delete old git repo & init new repo
    // 6a. if environment is gitpod, use the history in the folder
    // 6b. otherwise, delete history & init repo
    // if !bare_repo_exists {
    git::remove_repo(cd_name);
    git::init(cd_name);
    // println!("repo is not bare");
    // } else {
    //     println!("repo is bare");
    // }

    commands::run_command_batch(&config.commands.post_install);

    terminal.print(format!(
        "Completed in {}",
        Hhmmss::hhmmssxxx(&stopwatch.elapsed())
    ));
}
