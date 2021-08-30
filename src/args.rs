/// Parse command line arguments.
use clap::{crate_authors, crate_version, AppSettings, ArgGroup, Clap};
use humantime::Duration;
use std::path::PathBuf;

const ACTION_HEADING: Option<&str> = Some("ACTIONS");
const OPTION_HEADING: Option<&str> = Some("OPTIONS");

#[derive(Clap, Debug)]
#[clap(
    name = "shareit",
    author = crate_authors!(),
    version = crate_version!(),
    setting = AppSettings::DeriveDisplayOrder,
    groups = &[
        ArgGroup::new("action").required(true),
        ArgGroup::new("create_action")
    ]
)]
/// A tool for creating file uploads, pastes or shortlinks with shareit.
///
/// Exactly one action option must be specified.
pub struct ShareIt {
    #[clap(short, long, groups = &["action", "create_action"], help_heading = ACTION_HEADING)]
    /// Create a shortlink.
    ///
    /// If this option is provided without arguments, the link will be read
    /// from stdin, or from the clipboard if --from-clipboard is passed. If an
    /// argument is passed to it, that argument will be used as the link to
    /// shorten.
    pub link: Option<Option<String>>,

    #[clap(short, long, groups = &["action", "create_action"], value_name = "FILE", help_heading = ACTION_HEADING)]
    /// Create a paste.
    ///
    /// If this option is provided without arguments, the paste will be read
    /// from stdin, or from the clipboard if --from-clipboard is passed. If an
    /// argument is passed to it, that argument will treated as a filename to
    /// read the paste from.
    pub paste: Option<Option<PathBuf>>,

    #[clap(short, long, groups = &["action", "create_action"], help_heading = ACTION_HEADING)]
    /// Upload a file.
    ///
    /// If this option is provided without arguments, the file will be read
    /// from stdin, or from the clipboard if --from-clipboard is passed. If an
    /// argument is passed to it, that argument will be treated as a filename
    /// to upload.
    pub file: Option<Option<PathBuf>>,

    #[clap(long, group = "action", help_heading = ACTION_HEADING, value_name = "OPTIONS", max_values = 2)]
    /// Set or view a config option.
    ///
    /// If this option is provided without arguments, the available config
    /// options will be listed. If a single config option is passed, the
    /// current value of that option will be shown. If two config options are
    /// passed, the config option specified by the first will be set to the
    /// value of the second.
    pub config: Option<Vec<String>>,

    #[clap(short, long, requires = "create_action", help_heading = OPTION_HEADING)]
    /// A custom name for the share.
    ///
    /// If this is not passed, the server will generate a random name. The
    /// name acts as the path part of the share URL.
    pub name: Option<String>,

    #[clap(short, long, requires = "create_action", value_name = "EXPIRY", help_heading = OPTION_HEADING)]
    /// How long the share should last before expiring, eg. "3days".
    ///
    /// If this is not passed, the maximum expiry time supported by the server
    /// will be used. This may mean that the share never expires.
    pub expire_after: Option<Duration>,

    #[clap(short, long, name = "TYPE", requires = "file", help_heading = OPTION_HEADING)]
    /// The MIME type of the uploaded file.
    ///
    /// This can only be used with the --file option.
    pub mime_type: Option<String>,

    #[clap(short, long, name = "LANG", requires = "paste", help_heading = OPTION_HEADING)]
    /// The language for syntax highlighting the paste.
    ///
    /// This can only be used with the --paste option.
    pub syntax: Option<String>,

    #[clap(short = 'c', long, requires = "create_action", help_heading = OPTION_HEADING)]
    /// Read the share from the clipboard instead of stdin.
    pub from_clipboard: bool,

    #[clap(short = 'C', long, requires = "create_action", help_heading = OPTION_HEADING)]
    /// Copy the created share URL to the clipboard.
    pub to_clipboard: bool,
}
