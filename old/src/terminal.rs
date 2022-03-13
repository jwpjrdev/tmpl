use clap::{app_from_crate, arg};
use console::Term;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Clone)]
pub(crate) struct Terminal {
    term: Term,
}

impl Terminal {
    pub(crate) fn new() -> Self {
        Self {
            term: Term::stdout(),
        }
    }

    pub(crate) fn print<S: AsRef<str>>(&self, message: S) {
        // todo: error handling
        self.term.write_line(message.as_ref()).ok();
    }

    pub(crate) fn spinner(&self, message: String) -> ProgressBar {
        let spinner = ProgressBar::new_spinner();
        spinner.enable_steady_tick(120);
        spinner.set_style(ProgressStyle::default_spinner().template("[{elapsed_precise}] {msg}"));
        spinner.set_message(message);

        spinner
    }
}

pub(crate) fn run_clap() -> String {
    let matches = app_from_crate!()
        .arg(arg!([url] "Remote template repository").required(true))
        .get_matches();

    let url = match matches.value_of("url") {
        Some(url) => url,
        None => panic!("url not found"),
    };

    url.to_string()
}
