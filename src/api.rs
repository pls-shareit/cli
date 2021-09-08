/// Interaction with the API.
use crate::action::{ActionOptions, CreateAction, DataSource};
use crate::config::ConfigData;
use atty::Stream;
use copypasta_ext::display::DisplayServer;
use reqwest::blocking::{Body, Client, RequestBuilder, Response};
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use std::fs::File;
use std::io::stdin;

macro_rules! add_header {
    ($headers:expr; $name:expr => $value:expr) => {
        $headers.insert($name, $value.parse().unwrap());
    };
}

fn get_headers(action: &CreateAction, options: &ActionOptions, config: &ConfigData) -> HeaderMap {
    let mut headers = HeaderMap::new();
    match action {
        CreateAction::Link { .. } => {
            add_header!(headers; "Share-Type" => "link");
        }
        CreateAction::Paste { highlighting, .. } => {
            add_header!(headers; "Share-Type" => "paste");
            if let Some(highlighting) = highlighting {
                add_header!(headers; "Share-Highlighting" => highlighting);
            }
        }
        CreateAction::File { mime_type, .. } => {
            add_header!(headers; "Share-Type" => "file");
            if let Some(mime_type) = mime_type {
                add_header!(headers; "Content-Type" => mime_type);
            }
        }
    };
    if let Some(expire_after) = options.expire_after {
        add_header!(headers; "Expire-After" => expire_after.as_secs().to_string());
    };
    if let Some(password) = &config.password {
        add_header!(headers; "Authorization" => format!("Password {}", password));
    };
    headers
}

fn add_body(request: RequestBuilder, source: DataSource) -> Result<RequestBuilder, String> {
    Ok(match source {
        DataSource::File(path) => {
            request.body(File::open(path).map_err(|e| format!("Could not open given file: {}", e))?)
        }
        DataSource::String(string) => request.body(string),
        DataSource::Clipboard => request.body(
            DisplayServer::select()
                .try_context()
                .ok_or("Could not access the clipboard.")?
                .get_contents()
                .map_err(|e| format!("Could not read from the clipboard: {}", e))?,
        ),
        DataSource::Stdin => {
            if atty::is(Stream::Stdin) && atty::is(Stream::Stdout) {
                println!("Reading from stdin - press CTRL+D to finish or CTRL+C to cancel.");
            }
            request.body(Body::new(stdin()))
        }
    })
}

fn api_error_message(message: String, status: StatusCode) -> String {
    let hint = match status {
        StatusCode::UNAUTHORIZED => Some("Your password may be incorrect - set it with 'shareit --config password <password>'."),
        StatusCode::FORBIDDEN => Some("You can set your password with 'shareit --config password <password>'."),
        StatusCode::CONFLICT => Some("If you omit the '--name' flag, the server will generate a random one."),
        StatusCode::INTERNAL_SERVER_ERROR => Some("This is probably not your fault, or an issue with the CLI. Try contacting the server administrators."),
        _ => None
    };
    let mut output = String::new();
    output.push_str(&format!(
        "Server returned an error ({}):\n\n  {}",
        status, message
    ));
    if let Some(hint) = hint {
        output.push_str(&format!("\n\nHint: {}", hint));
    };
    output
}

fn handle_response(response: Response, options: &ActionOptions) -> Result<(), String> {
    let status = response.status();
    if status.is_success() {
        let url = response
            .text()
            .map_err(|_| "API success but could not decode response.")?;
        if options.to_clipboard {
            DisplayServer::select()
                .try_context()
                .ok_or("Could not access the clipboard.")?
                .set_contents(url)
                .map_err(|e| format!("Could not write to the clipboard: {}", e))?;
        } else {
            println!("{}", url);
        }
        Ok(())
    } else {
        let message = response
            .text()
            .map_err(|_| "API error and could not decode response.")?;
        Err(api_error_message(message, status))
    }
}

pub fn create(action: CreateAction, config: ConfigData) -> Result<(), String> {
    let options = action.get_options();
    let response = add_body(
        Client::new()
            .post(format!(
                "{}/{}",
                config.base_url.clone().ok_or(
                    "No API URL configured. Try running 'shareit --config base_url <url>'."
                )?,
                options.name.clone().unwrap_or_else(|| "".into())
            ))
            .headers(get_headers(&action, options, &config)),
        options.source.clone(),
    )?
    .send()
    .map_err(|e| format!("Error accessing the API: {}", e))?;
    handle_response(response, options)
}
