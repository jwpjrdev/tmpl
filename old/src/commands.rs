use run_script::{spawn, ScriptOptions};

pub(crate) fn run_command_batch(command_batch: &str) {
    let mut options = ScriptOptions::new();
    options.output_redirection = run_script::IoOptions::Inherit; // todo: there's gotta be a cleaner way to do this
    spawn(command_batch, &vec![], &options).unwrap().wait().ok();
}
