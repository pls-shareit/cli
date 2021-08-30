use crate::action::Action;
use crate::args::ShareIt;
use crate::config::Config;
use clap::Clap;

mod action;
mod api;
mod args;
mod config;

fn run_action(action: Action) -> Result<(), String> {
    let config = Config::new()?;
    match action {
        Action::Create(opts) => api::create(opts, config.data)?,
        Action::Config(opts) => config.do_action(opts)?,
    };
    Ok(())
}

fn main() {
    if let Err(message) = run_action(Action::from_args(ShareIt::parse())) {
        eprintln!("{}", message);
    }
}
