/// A data structure for the action we need to take.
use crate::args::ShareIt;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum DataSource {
    String(String),
    File(PathBuf),
    Clipboard,
    Stdin,
}

impl DataSource {
    pub fn from_args(args: &ShareIt) -> DataSource {
        if let Some(Some(filename)) = args.file.clone() {
            DataSource::File(filename)
        } else if let Some(Some(filename)) = args.paste.clone() {
            DataSource::File(filename)
        } else if let Some(Some(url)) = args.link.clone() {
            DataSource::String(url)
        } else if args.from_clipboard {
            DataSource::Clipboard
        } else {
            DataSource::Stdin
        }
    }
}

#[derive(Debug)]
pub struct ActionOptions {
    pub source: DataSource,
    pub name: Option<String>,
    pub expire_after: Option<Duration>,
    pub to_clipboard: bool,
}

impl ActionOptions {
    pub fn from_args(args: &ShareIt) -> ActionOptions {
        ActionOptions {
            source: DataSource::from_args(args),
            name: args.name.clone(),
            expire_after: args.expire_after.map(|d| d.into()),
            to_clipboard: args.to_clipboard,
        }
    }
}

#[derive(Debug)]
pub enum CreateAction {
    Link {
        options: ActionOptions,
    },
    Paste {
        options: ActionOptions,
        highlighting: Option<String>,
    },
    File {
        options: ActionOptions,
        mime_type: Option<String>,
    },
}

impl CreateAction {
    pub fn from_args(args: ShareIt) -> CreateAction {
        let options = ActionOptions::from_args(&args);
        if args.link.is_some() {
            CreateAction::Link { options }
        } else if args.paste.is_some() {
            CreateAction::Paste {
                options,
                highlighting: args.syntax,
            }
        } else if args.file.is_some() {
            CreateAction::File {
                options,
                mime_type: args.mime_type,
            }
        } else {
            // Clap should prevent this.
            panic!("Action unexpectedly missing.");
        }
    }

    pub fn get_options(&self) -> &ActionOptions {
        match self {
            CreateAction::Link { options } => options,
            CreateAction::Paste { options, .. } => options,
            CreateAction::File { options, .. } => options,
        }
    }
}

#[derive(Debug)]
pub enum ConfigAction {
    List,
    Get(String),
    Set(String, String),
}

impl ConfigAction {
    pub fn from_args(args: ShareIt) -> ConfigAction {
        let args = args
            .config
            .expect("Tried to interpret missing config action.");
        match args.len() {
            0 => ConfigAction::List,
            1 => ConfigAction::Get(args[0].clone()),
            _ => ConfigAction::Set(args[0].clone(), args[1..].to_vec().join(" ")),
        }
    }
}

#[derive(Debug)]
pub enum Action {
    Create(CreateAction),
    Config(ConfigAction),
}

impl Action {
    pub fn from_args(args: ShareIt) -> Action {
        if args.config.is_some() {
            Action::Config(ConfigAction::from_args(args))
        } else {
            Action::Create(CreateAction::from_args(args))
        }
    }
}
