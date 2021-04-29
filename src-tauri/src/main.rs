#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
mod docker_plugin;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .plugin(docker_plugin::DockerPlugin::default())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
fn my_custom_command() {
  println!("I was invoked from JS!");
}