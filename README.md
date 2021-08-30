# Shareit CLI

This provides a command line interface to the
[Shareit](https://github.com/pls-shareit/server) service. Shareit is a link
shortner, file uploader and pastebin.

## Installation

### Linux

 1. Download the executable
    (or [click here](https://github.com/pls-shareit/cli/releases/latest/download/shareit-linux)):

    `wget https://github.com/pls-shareit/cli/releases/latest/download/shareit-linux -qO shareit`
 2. Make the file executable:

    `chmod +x shareit`
 3. Add the executable to your PATH, for example:

    `sudo mv shareit /usr/bin`

### MacOS

 1. Download the executable
    (or [click here](https://github.com/pls-shareit/cli/releases/latest/download/shareit-macos)):

    `wget https://github.com/pls-shareit/cli/releases/latest/download/shareit-macos -qO shareit`
 2. Make the file executable:

    `chmod +x shareit`
 3. Add the executable to your PATH, for example:

    `sudo mv shareit /usr/bin`

### Windows

 1. Download the executable with Power shell
    (or [click here](https://github.com/pls-shareit/server/releases/latest/download/shareitd.exe)):

    `iwr -outf clique-collector.exe https://github.com/pls-shareit/server/releases/latest/download/shareitd.exe`
 2. Add the executable to your PATH, for example:

    `Move-Item -Path .\shareitd.exe -Destination C:\Windows\system32`
 3. You may need to restart your shell or computer.

### From source

If you don't want to use a pre-compiled executable, you can compile one using
Rust:

 1. [Install Rust](https://www.rust-lang.org/tools/install).
 2. Run `cargo build --release` in the same directory as this README.
 3. Your binary will be at `target/release/shareit`.

## Usage

First, you should configure the tool with the URL of the Shareit server you'll
be using. For example, to use `https://share.example.com`, run:

```bash
shareit --config base_url https://share.example.com
```

If the instance is password protected, you can configure your password
like so:

```bash
shareit --config password super-secret-password
```

Complete documentation on command behavior and every option is available by
running `shareit --help`. You can get abbreviated help by running `shareit -h`.

Here are some examples:

- `shareit --file ./image.png --name cool-img`

  Uploads the file `./image.png` with the name `cool-img` and maximum expiry.
  Will output the URL to the file if successful.

- `shareit --link https://example.com --to-clipboard`

  Create a shortlink with a random name to the URL `https://example.com`, with
  maximum expiry. Copies the shortened URL to your clipboard if successful.

- `shareit --paste --from-clipboard --syntax python --expire-after 1day`

  Uploads the contents of your clipboard as a paste with a random name and
  python syntax highlighting. Sets the paste to expire after one day. Will
  output the URL to the paste if successful.

Use `shareit --help` for more information on the above options, shorter
aliases for them, and other options.

Currently, the CLI does not support deleting, updating or fetching information
on shares - only creating them.
