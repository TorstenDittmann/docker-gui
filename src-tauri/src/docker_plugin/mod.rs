use tauri::Webview;

use self::commands::HelloWorld;

mod commands;

pub struct DockerPlugin {

}

impl DockerPlugin {
  pub fn new() -> Self {
    Self {}
  }
}

impl tauri::plugin::Plugin for DockerPlugin {
  fn init_script(&self) -> Option<String> {
    None
  }

  fn created(&self, _webview: &mut Webview<'_>) {

  }

  fn ready(&self, _webview: &mut Webview<'_>) {

  }

  fn extend_api(
      &self, 
      webview: &mut Webview<'_>, 
      payload: &str
  ) -> Result<bool, String> {
    match serde_json::from_str(payload) {
      Err(e) => Err(e.to_string()),
      Ok(command) => {
        handle_commands(command, webview)
      }
    }
  }
}

fn handle_commands(command: commands::DockerCmd, webview: &mut Webview) -> std::result::Result<bool, std::string::String> {
  match command {
    commands::DockerCmd::HelloWorld {callback, error } => HelloWorld(callback, error, webview)
  }
  Ok(true)
}