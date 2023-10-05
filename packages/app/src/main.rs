// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ipc;
mod state;

use tauri::{State, Window};
use tower_service::Service;

use self::ipc::{IpcRequest, IpcResponse};
use self::state::AppState;

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![window_did_finish_loading, htmx])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn window_did_finish_loading(window: Window) -> Result<(), tauri::Error> {
    if !window.is_visible()? {
        window.show()?;
    }

    Ok(())
}

#[tauri::command]
async fn htmx(request: IpcRequest, state: State<'_, AppState>) -> Result<IpcResponse, String> {
    let mut router = state.router.lock().unwrap().clone();
    let response = router.call(request.into()).await.expect("infallible");
    let status = response.status().as_u16();

    match status {
        200..=299 => Ok(IpcResponse::from_http_response(response)
            .await
            .map_err(|err| err.to_string())?),
        status => {
            let bytes: Vec<u8> = hyper::body::to_bytes(response.into_body()).await.map_err(|err| err.to_string())?.into();

            Err(format!("Status Code: {status}, Response: {}", String::from_utf8(bytes).map_err(|err| err.to_string())?))
        }
    }
}
