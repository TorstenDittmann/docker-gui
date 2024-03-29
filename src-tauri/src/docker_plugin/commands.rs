use bollard::{API_DEFAULT_VERSION, Docker, models::{ContainerInspectResponse, ContainerSummaryInner, SystemInfo}};

use crate::docker_plugin::docker_global_state;

use bollard::container::ListContainersOptions;

use std::collections::HashMap;
use std::default::Default;

// Connection Commands
async fn get_connection_status(instance_option: &Option<Docker>) -> Result<(), String> {
  let instance = match instance_option {
    Some(result) => result,
    None => return Err("ERR_NOT_INITIALISED".to_string())
  };

  match instance.info().await {
    Ok(_data) => Ok(()),
    Err(err) => Err(err.to_string())
  }
}

#[tauri::command]
pub async fn connection_info() -> Result<SystemInfo, String> {
  let docker_instance = docker_global_state().lock().await;

  match get_connection_status(&docker_instance).await {
    Ok(_data) => {},
    Err(err) => {
      return Err(err)
    }
  };

  return match docker_instance.as_ref().unwrap().info().await {
    Ok(data) => {Ok(data)}
    Err(err) => {Err(err.to_string())}
  }
}

#[tauri::command]
pub async fn hello_world() -> Result<String, String> {
  std::thread::sleep(std::time::Duration::from_secs(5));
  println!("Hello World!");

  Ok("HELLO WORLD!".into())
}

#[tauri::command]
pub async fn container_all() -> Result<Vec<ContainerSummaryInner>, String> {
  let mut docker_instance = docker_global_state().lock().await;

  match &docker_instance.as_mut() {
    Some(_) => {},
    None => {return Err("DOCKER IS NOT INITIALISED".into())}
  }

  //TODO: ADD CODE TO CHECK WHETHER CONNECTION IS ALIVE!
  
  let options = Some(ListContainersOptions{
      all: true,
      filters: HashMap::<&str, Vec<&str>>::new(),
      ..Default::default()
  });

  Ok(docker_instance.as_mut().unwrap().list_containers(options).await.unwrap())
}

// Command: connect_with_http
// Explanation: This command will create a unsecured HTTP connection with docker
// Parameters: { url: String, use_defaults: bool }

#[tauri::command]
pub async fn connect_with_http(url: String, use_defaults: Option<bool>) -> Result<SystemInfo, String> {
    let connection = match use_defaults {
      Some(result) => {
        if result == true {
          match Docker::connect_with_http_defaults() {
            Ok(result) => result,
            Err(err) => {
              return Err(format!("An error occoured attempting to connect. Error: {}", err))
            }
          }
        } else {
          match Docker::connect_with_http(&url, 10, API_DEFAULT_VERSION) {
            Ok(result) => result,
            Err(err) => {
              return Err(format!("An error occoured attempting to connect. Error: {}", err))
            }
          }
        }
      },
      None => {
        match Docker::connect_with_http(&url, 10, API_DEFAULT_VERSION) {
          Ok(result) => result,
          Err(err) => {
            return Err(format!("An error occoured attempting to connect. Error: {}", err))
          }
        }
      }
    };

    match connection.info().await {
      Ok(result) => {
        let mut docker_instance = docker_global_state().lock().await;

        *docker_instance = Some(connection);
        Ok(result)
      },
      Err(err) => {
        return Err(format!("An error occoured attempting to connect. Error: {}", err))
      }
    }
}

// Command: connect_with_local
// Explanation: This command will create a connection with docker using a local IP Address
// Parameters: { address: String, use_defaults: bool }


#[tauri::command]
pub async fn connect_with_local(address: String, use_defaults: Option<bool>) -> Result<SystemInfo, String> {
  let connection = match use_defaults {
    Some(result) => {
      if result == true {
        match Docker::connect_with_local_defaults() {
          Ok(result) => result,
          Err(err) => {
            return Err(format!("An error occoured attempting to connect. Error: {}", err))
          }
        }
      } else {
        match Docker::connect_with_local(&address, 10, API_DEFAULT_VERSION) {
          Ok(result) => result,
          Err(err) => {
            return Err(format!("An error occoured attempting to connect. Error: {}", err))
          }
        }
      }
    },
    None => {
      match Docker::connect_with_local(&address, 10, API_DEFAULT_VERSION) {
        Ok(result) => result,
        Err(err) => {
          return Err(format!("An error occoured attempting to connect. Error: {}", err))
        }
      }
    }
  };

  match connection.info().await {
    Ok(result) => {
      let mut docker_instance = docker_global_state().lock().await;

      *docker_instance = Some(connection);
      Ok(result)
    },
    Err(err) => {
      return Err(format!("An error occoured attempting to connect. Error: {}", err))
    }
  }
}

// Command: connect_with_pipe
// Explanation: This command will create a connection with docker using a named pipe
// Parameters: { pipe: String, use_defaults: bool }

#[cfg(target_os = "windows")]
#[tauri::command]
pub async fn connect_with_pipe(pipe: String, use_defaults: Option<bool>) -> Result<SystemInfo, String> {
  let connection = match use_defaults {
    Some(result) => {
      if result == true {
        match Docker::connect_with_named_pipe_defaults() {
          Ok(result) => result,
          Err(err) => {
            return Err(format!("An error occoured attempting to connect. Error: {}", err))
          }
        }
      } else {
        match Docker::connect_with_named_pipe(&pipe, 10, API_DEFAULT_VERSION) {
          Ok(result) => result,
          Err(err) => {
            return Err(format!("An error occoured attempting to connect. Error: {}", err))
          }
        }
      }
    },
    None => {
      match Docker::connect_with_named_pipe(&pipe, 10, API_DEFAULT_VERSION) {
        Ok(result) => result,
        Err(err) => {
          return Err(format!("An error occoured attempting to connect. Error: {}", err))
        }
      }
    }
  };

  match connection.info().await {
    Ok(result) => {
      let mut docker_instance = docker_global_state().lock().await;

      *docker_instance = Some(connection);
      Ok(result)
    },
    Err(err) => {
      return Err(format!("An error occoured attempting to connect. Error: {}", err))
    }
  }
}

#[cfg(not(target_os = "windows"))]
#[tauri::command]
pub async fn connect_with_pipe(pipe: String, use_defaults: Option<bool>) -> Result<String, String> {
  return Err("Function only available in windows".to_string())
}

#[tauri::command]
pub async fn container_get(container: String) -> Result<ContainerInspectResponse, String> {
  let docker_instance = docker_global_state().lock().await;

  match get_connection_status(&docker_instance).await {
    Ok(_data) => {},
    Err(err) => {
      return Err(err)
    }
  };

  let data = docker_instance.as_ref().unwrap().inspect_container(&container, None).await;

  match data {
    Ok(result) => {return Ok(result)}
    Err(err) => {return Err(format!("Error: {}", err.to_string()))}
  };
}