#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use futures::StreamExt;
use shiplift::tty::TtyChunk;
use shiplift::{ContainerListOptions, Docker, LogsOptions};
use std::time::Duration;

struct GlobalState {
  docker: Docker,
}

fn main() {
  tauri::Builder::default()
    .manage(GlobalState {
      docker: Docker::new().into(),
    })
    .invoke_handler(tauri::generate_handler![
      containers_list,
      container_logs,
      images_list,
      docker_ping,
      init_process,
      stop_container,
      start_container,
      restart_container,
      delete_container
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn init_process(window: tauri::Window) {
  let docker = Docker::new();
  while let Some(event_result) = docker.events(&Default::default()).next().await {
    if let Ok(event) = event_result {
      if let Err(e) = window.emit("docker", event) {
        eprintln!("Error: {}", e)
      }
    }
  }
}

#[tauri::command]
async fn container_logs(container_id: String, window: tauri::Window) {
  let docker = Docker::new();
  let mut logs_stream = docker.containers().get(&container_id).logs(
    &LogsOptions::builder()
      .stdout(true)
      .stderr(true)
      .follow(true)
      .build(),
  );

  while let Some(log_result) = logs_stream.next().await {
    if let Ok(chunk) = log_result {
      if let Err(e) = window.emit("logs", return_chunk(chunk)) {
        eprintln!("Error: {}", e)
      }
    }
  }
}

fn return_chunk(chunk: TtyChunk) -> String {
  match chunk {
    TtyChunk::StdOut(bytes) => String::from(std::str::from_utf8(&bytes).unwrap()),
    TtyChunk::StdErr(bytes) => String::from(std::str::from_utf8(&bytes).unwrap()),
    TtyChunk::StdIn(_) => String::new(),
  }
}

#[tauri::command]
async fn docker_ping(state: tauri::State<'_, GlobalState>) -> Result<String, String> {
  match state.docker.ping().await {
    Err(e) => Err(e.to_string()),
    Ok(pong) => Ok(pong),
  }
}

#[tauri::command]
async fn containers_list(state: tauri::State<'_, GlobalState>) -> Result<String, String> {
  match state
    .docker
    .containers()
    .list(&ContainerListOptions::builder().all().build())
    .await
  {
    Err(e) => return Err(e.to_string()),
    Ok(containers) => return Ok(serde_json::to_string(&containers).unwrap()),
  }
}

#[tauri::command]
async fn images_list(state: tauri::State<'_, GlobalState>) -> Result<String, String> {
  match state.docker.images().list(&Default::default()).await {
    Err(e) => return Err(e.to_string()),
    Ok(containers) => return Ok(serde_json::to_string(&containers).unwrap()),
  }
}

#[tauri::command]
async fn stop_container(
  container_id: String,
  state: tauri::State<'_, GlobalState>,
) -> Result<(), String> {
  match state
    .docker
    .containers()
    .get(&container_id)
    .stop(Some(Duration::new(5, 0)))
    .await
  {
    Err(e) => return Err(e.to_string()),
    Ok(_c) => return Ok(()),
  }
}

#[tauri::command]
async fn delete_container(
  container_id: String,
  state: tauri::State<'_, GlobalState>,
) -> Result<(), String> {
  match state.docker.containers().get(&container_id).delete().await {
    Err(e) => return Err(e.to_string()),
    Ok(_c) => return Ok(()),
  }
}

#[tauri::command]
async fn start_container(
  container_id: String,
  state: tauri::State<'_, GlobalState>,
) -> Result<(), String> {
  match state.docker.containers().get(&container_id).start().await {
    Err(e) => return Err(e.to_string()),
    Ok(_c) => return Ok(()),
  }
}

#[tauri::command]
async fn restart_container(
  container_id: String,
  state: tauri::State<'_, GlobalState>,
) -> Result<(), String> {
  let wait = Some(Duration::new(5, 0));
  match state
    .docker
    .containers()
    .get(&container_id)
    .restart(wait)
    .await
  {
    Err(e) => return Err(e.to_string()),
    Ok(_c) => return Ok(()),
  }
}
