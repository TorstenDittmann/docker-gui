use bollard::Docker;
use tauri::Window;

// use self::commands::{hello_world};

mod commands;

pub struct DockerPlugin {
  dockerInstance: Option<Docker>
}

impl DockerPlugin {
  pub fn new() -> Self {
    Self {
      dockerInstance: None
    }
  }
}

#[tauri::command]
pub fn my_custom_command() -> Result<String, String> {
  std::thread::sleep(std::time::Duration::from_secs(5));
  Ok("HELLO WORLD!".into())
}

impl<M: tauri::Params> tauri::plugin::Plugin<M> for DockerPlugin {
  fn initialization_script(&self) -> Option<String> {
    Some("console.log('HELLO WORLD!')".into())
  }

  fn extend_api(
      &mut self, 
      message: tauri::InvokeMessage<M>
  ) {
    let command = message.command();

    println!("{} handled", command);
  }

  fn name(&self) -> &'static str {
    "docker"
  }
}