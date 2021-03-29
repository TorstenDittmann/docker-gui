use serde::{Deserialize, Serialize};
use tauri::Webview;

// All commands for the docker plugin are registered here
#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum DockerCmd {
  HelloWorld {
    callback: String,
    error: String,
  }
}

// Basic Command boilerplate
// All commands will use the same error type

#[derive(Debug, Clone)]
struct CommandError<'a> {
  message: &'a str,
}

impl<'a> CommandError<'a> {
  fn new(message: &'a str) -> Self {
    Self { message }
  }
}

impl<'a> std::fmt::Display for CommandError<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl<'a> std::error::Error for CommandError<'a> {}

// All commands will be laid out like so:

// #[derive(Deserialize)] <- Deserialize so we can convert it back into a Rust Struct to use it
// pub struct Payload{{CommandName}} <- This will be the payload for the command in a struct     NOTE: This is optional
//
// #[derive(Serialize)] <- Serialise to convert it back into JSON for the JS Instance to understand it
// pub struct Response{{CommandName}} <- Return Payload to the JS Instance
//
// pub fn {{CommandName}} <- Holds actual command code

// Below is a example command

#[derive(Serialize)]
pub struct ResponseHelloWorld {
  message: String
}

pub fn HelloWorld(callback: String, error: String, webview: &mut Webview) {
  tauri::execute_promise(webview,
move || {
      std::thread::sleep(std::time::Duration::from_secs(5));
      Ok(ResponseHelloWorld { message: "HELLO WORLD!".to_string() })
  },
    callback,
    error)
}