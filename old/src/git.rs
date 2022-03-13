// mostly ripped from https://github.com/rust-lang/git2-rs/blob/master/examples/clone.rs

use console::style;
use git2::{
    build::{CheckoutBuilder, RepoBuilder},
    FetchOptions, Progress, RemoteCallbacks, Repository,
};
use indicatif::ProgressBar;
use std::{
    cell::RefCell,
    io::{self, Write},
    path::{Path, PathBuf},
};

use crate::terminal::Terminal;

pub(crate) fn remove_repo(path_name: &str) {
    // match for errors
    std::fs::remove_dir_all(Path::new(&format!("{}/.git/", &path_name))).ok();
}

pub(crate) fn init(path_name: &str) -> Repository {
    match Repository::init(path_name) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    }
}

pub(crate) fn clone(terminal: &Terminal, url: &str, path: &Path) -> Repository {
    // shallow clone via command (libgit2 doesn't support it and so neither does git2-rs)
    // rm -rf .git/ folder for an empty history... if integrated with Gitpod, some effort should be put into adding the remote and setting up the git repo again
    // todo: support cloning via SSH
    let spinner = terminal.spinner(format!("Cloning from {}", &style(&url).yellow()));

    let state = RefCell::new(State {
        progress: None,
        total: 0,
        current: 0,
        path: None,
    });
    let mut cb = RemoteCallbacks::new();
    cb.transfer_progress(|stats| {
        let mut state = state.borrow_mut();
        state.progress = Some(stats.to_owned());
        update(&spinner, &mut *state);
        true
    });

    let mut co = CheckoutBuilder::new();
    co.progress(|path, cur, total| {
        let mut state = state.borrow_mut();
        state.path = path.map(|p| p.to_path_buf());
        state.current = cur;
        state.total = total;
        update(&spinner, &mut *state);
    });

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);
    let repo = RepoBuilder::new()
        .fetch_options(fo)
        .with_checkout(co)
        .clone(url, path);

    spinner.finish_with_message(format!("Cloned from {}", &style(&url).yellow()));

    repo.unwrap()
}

struct State {
    progress: Option<Progress<'static>>,
    total: usize,
    current: usize,
    path: Option<PathBuf>,
}

fn update(spinner: &ProgressBar, state: &mut State) {
    let stats = state.progress.as_ref().unwrap();
    let network_pct = (100 * stats.received_objects()) / stats.total_objects();
    let index_pct = (100 * stats.indexed_objects()) / stats.total_objects();
    let co_pct = if state.total > 0 {
        (100 * state.current) / state.total
    } else {
        0
    };
    let kbytes = stats.received_bytes() / 1024;
    if stats.received_objects() == stats.total_objects() {
        spinner.set_message(format!(
            "Resolving deltas {}/{}\r",
            stats.indexed_deltas(),
            stats.total_deltas()
        ));
    } else {
        spinner.set_message(format!(
            "net {:3}% ({:4} kb, {:5}/{:5})  /  idx {:3}% ({:5}/{:5})  \
             /  chk {:3}% ({:4}/{:4}) {}\r",
            network_pct,
            kbytes,
            stats.received_objects(),
            stats.total_objects(),
            index_pct,
            stats.indexed_objects(),
            stats.total_objects(),
            co_pct,
            state.current,
            state.total,
            state
                .path
                .as_ref()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default()
        ));
    }
    io::stdout().flush().unwrap();
}
