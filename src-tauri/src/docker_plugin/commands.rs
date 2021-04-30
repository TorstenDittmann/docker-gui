use bollard::{API_DEFAULT_VERSION, Docker, models::ContainerSummaryInner};

use crate::docker_plugin::docker_global_state;

use bollard::container::ListContainersOptions;

use std::collections::HashMap;
use std::default::Default;

async fn is_connection_alive(instance_option: Option<Docker>) -> Result<(), String> {
  let instance = match instance_option {
    Some(result) => result,
    None => return Err("ERR_NOT_INITIALISED".to_string())
  };

  match instance.ping().await {
    Ok(_data) => Ok(()),
    Err(err) => Err(err.to_string())
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

  let mut filters = HashMap::new();
  filters.insert("health", vec!["unhealthy"]);
  
  let options = Some(ListContainersOptions{
      all: true,
      filters,
      ..Default::default()
  });

  Ok(docker_instance.as_mut().unwrap().list_containers(options).await.unwrap())
}

// Command: connect_with_http
// Explanation: This command will create a unsecured HTTP connection with docker
// Parameters: { url: String, use_defaults: bool }

#[tauri::command]
pub async fn connect_with_http(url: String, use_defaults: Option<bool>) -> Result<String, String> {
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
        println!("{:?}", result);
        let mut docker_instance = docker_global_state().lock().await;

        *docker_instance = Some(connection);
        Ok("SUCCESS".to_string())
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
pub async fn connect_with_local(address: String, use_defaults: Option<bool>) -> Result<String, String> {
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
      println!("{:?}", result);

      let mut docker_instance = docker_global_state().lock().await;

      *docker_instance = Some(connection);
      Ok("SUCCESS".to_string())
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
pub async fn connect_with_pipe(pipe: String, use_defaults: Option<bool>) -> Result<String, String> {
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
      println!("{:?}", result);

      let mut docker_instance = docker_global_state().lock().await;

      *docker_instance = Some(connection);
      Ok("SUCCESS".to_string())
    },
    Err(err) => {
      return Err(format!("An error occoured attempting to connect. Error: {}", err))
    }
  }
}