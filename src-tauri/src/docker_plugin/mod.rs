use bollard::Docker;
use tauri::{async_runtime::Mutex, Params, InvokeMessage};
use self::commands::*;
use once_cell::sync::Lazy;

use std::{
    convert::{Into},
    sync::Arc,
};


mod commands;

pub struct DockerPlugin<M: Params> {
  invoke_handler: Box<dyn Fn(InvokeMessage<M>) + Send + Sync>,
  docker_instance: Option<Docker>
}

pub fn docker_global_state() -> &'static Arc<Mutex<Option<Docker>>> {
  static API: Lazy<Arc<Mutex<Option<Docker>>>> = Lazy::new(Default::default);
  &API
}


impl<M: Params> Default for DockerPlugin<M> {
  fn default() -> Self {
    Self {
      docker_instance: None,
      invoke_handler: Box::new(tauri::generate_handler![
        connect_with_http,
        container_all,
        hello_world,
        my_custom_command
      ]),
    }
  }
}

#[tauri::command]
pub fn my_custom_command() -> Result<String, String> {
  std::thread::sleep(std::time::Duration::from_secs(5));
  Ok("HELLO WORLD!".into())
}

impl<M: tauri::Params> tauri::plugin::Plugin<M> for DockerPlugin<M> {
  fn initialization_script(&self) -> Option<String> {
    Some("console.log('HELLO WORLD!')".into())
  }

  fn extend_api(
      &mut self, 
      message: tauri::InvokeMessage<M>
  ) {
    (self.invoke_handler)(message)
  }

  fn name(&self) -> &'static str {
    "docker"
  }
}