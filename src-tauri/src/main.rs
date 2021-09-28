
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use shiplift::Docker;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![containers_list, images_list])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn containers_list() -> Result<String, String> {
  let docker = Docker::new();
  match docker.containers().list(&Default::default()).await {
    Err(e) => return Err(e.to_string()),
    Ok(containers) => {
      let mut cont = json::JsonValue::new_array();
      for c in containers {
        let mut data = json::JsonValue::new_object();
        data["created"] = c.created.timestamp().into();
        data["command"] = c.command.into();
        data["id"] = c.id.into();
        data["names"] = c.names.into();
        data["image"] = c.image.into();
        data["state"] = c.state.into();
        data["status"] = c.status.into();
        data["labels"] = c.labels.into();

        let mut ports = json::JsonValue::new_array();
        for port in c.ports {
          let mut temp_port = json::JsonValue::new_object();
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
async fn images_list() -> Result<String, String> {
  let docker = Docker::new();
  match docker.images().list(&Default::default()).await {
    Err(e) => return Err(e.to_string()),
    Ok(images) => {
      let mut cont = json::JsonValue::new_array();
      for i in images {
        let mut data = json::JsonValue::new_object();
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