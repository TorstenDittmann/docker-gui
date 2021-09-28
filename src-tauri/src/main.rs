#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use futures::StreamExt;
use json::JsonValue;
use shiplift::Docker;

struct GlobalState {
  docker: Docker,
}

#[derive(serde::Serialize)]
struct Payload {
  message: String,
}

fn main() {
  tauri::Builder::default()
    .manage(GlobalState {
      docker: Docker::new().into(),
    })
    .invoke_handler(tauri::generate_handler![
      containers_list,
      images_list,
      docker_ping,
      init_process
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn init_process(window: tauri::Window) {
  let docker = Docker::new();
  while let Some(event_result) = docker.events(&Default::default()).next().await {
    match event_result {
      Ok(event) => window.emit("docker", event),
      Err(e) => window.emit("docker", e.to_string())
    };
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
  match state.docker.containers().list(&Default::default()).await {
    Err(e) => return Err(e.to_string()),
    Ok(containers) => {
      let mut cont = JsonValue::new_array();
      for c in containers {
        let mut data = JsonValue::new_object();
        data["created"] = c.created.timestamp().into();
        data["command"] = c.command.into();
        data["id"] = c.id.into();
        data["names"] = c.names.into();
        data["image"] = c.image.into();
        data["state"] = c.state.into();
        data["status"] = c.status.into();
        data["labels"] = c.labels.into();

        let mut ports = JsonValue::new_array();
        for port in c.ports {
          let mut temp_port = JsonValue::new_object();
          temp_port["ip"] = port.ip.into();
          temp_port["private_port"] = port.private_port.into();
          temp_port["public_port"] = port.public_port.into();
          temp_port["typ"] = port.typ.into();
          ports.push(temp_port).err();
        }
        data["ports"] = ports;
        cont.push(data).err();
      }
      Ok(json::stringify(cont))
    }
  }
}

#[tauri::command]
async fn images_list(state: tauri::State<'_, GlobalState>) -> Result<String, String> {
  match state.docker.images().list(&Default::default()).await {
    Err(e) => return Err(e.to_string()),
    Ok(images) => {
      let mut cont = JsonValue::new_array();
      for i in images {
        let mut data = JsonValue::new_object();
        data["created"] = i.created.timestamp().into();
        data["id"] = i.id.into();
        data["parent_id"] = i.parent_id.into();
        data["labels"] = i.labels.into();
        data["repo_tags"] = i.repo_tags.into();
        data["repo_digests"] = i.repo_digests.into();
        data["virtual_size"] = i.virtual_size.into();
        cont.push(data).err();
      }
      Ok(json::stringify(cont))
    }
  }
}
