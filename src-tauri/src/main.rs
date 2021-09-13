
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use shiplift::Docker;


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![containers_list])
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
        //println!("container -> {:#?}", c);
        let mut data = json::JsonValue::new_object();
        data["id"] = c.id.into();
        data["names"] = c.names.into();
        data["image"] = c.image.into();
        data["state"] = c.state.into();
        data["status"] = c.status.into();
        data["labels"] = c.labels.into();
        cont.push(data).err();
      }
      Ok(json::stringify(cont))
    }

  }
}