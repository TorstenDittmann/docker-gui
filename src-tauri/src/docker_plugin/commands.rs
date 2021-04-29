use serde::{Deserialize, Serialize};
use bollard::{API_DEFAULT_VERSION, Docker};
use futures::executor::block_on;


#[tauri::command]
pub fn hello_world() -> Result<String, String> {
  std::thread::sleep(std::time::Duration::from_secs(5));
  Ok("HELLO WORLD!".into())
}

// Command: connectWithHTTP
// Explanation: This command will create a unsecured HTTP connection with docker
// Parameters: { url: String }



// pub fn ConnectWithHTTP(callback: String, error: String, webview: &mut Webview, payload: PayloadConnectWithHTTP) {
//   tauri::execute_promise(webview, move || {
//     let connection = match Docker::connect_with_http(&payload.url, 10, API_DEFAULT_VERSION) {
//       Ok(result) => result,
//       Err(err) => {
//         return Err(CommandError::new(format!("An error occoured attempting to connect. Error: {}", err)).into())
//       }
//     };
// 
//     connection.ping().await;
// 
//     match block_on(connection.ping()) {
//       Ok(result) => {
//         println!("{}", &result);
//         Ok(())
//       },
//       Err(err) => {
//         return Err(CommandError::new(format!("An error occoured attempting to connect. Error: {}", err)).into())
//       }
//     }
// 
//   }, callback, error)
// }