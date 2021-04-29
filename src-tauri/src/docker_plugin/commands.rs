use bollard::{API_DEFAULT_VERSION, Docker};


#[tauri::command]
pub fn hello_world() -> Result<String, String> {
  std::thread::sleep(std::time::Duration::from_secs(5));
  println!("Hello World!");
  Ok("HELLO WORLD!".into())
}

// Command: connect_with_http
// Explanation: This command will create a unsecured HTTP connection with docker
// Parameters: { url: String }

#[tauri::command]
pub async fn connect_with_http(url: String) -> Result<String, String> {
    println!("AAAAAAAAAAAAAAA");
    let connection = match Docker::connect_with_http(&url, 10, API_DEFAULT_VERSION) {
      Ok(result) => result,
      Err(err) => {
        return Err(format!("An error occoured attempting to connect. Error: {}", err))
      }
    };

    match connection.ping().await {
      Ok(result) => {
        println!("{}", &result);
        Ok("SUCCESS".to_string())
      },
      Err(err) => {
        return Err(format!("An error occoured attempting to connect. Error: {}", err))
      }
    }
}