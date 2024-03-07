// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod controller;
pub mod datatypes;
pub mod network;
pub mod cache;

pub use std::time::Duration;

use controller::GwentController;
use network::{GwentAPI, GwentAPIConfigBuilder};
use tauri::State;
use tauri::async_runtime::Mutex;


struct GwentControllerWrapped {
    store: Mutex<GwentController>
}

fn create_controller() -> GwentControllerWrapped {
    let config = GwentAPIConfigBuilder::new()
        .set_timeout(Duration::from_secs(10))
        .set_max_retries(5)
        .build();
    let api = GwentAPI::new(config);
    let controller = GwentController::new(api);
    GwentControllerWrapped{
        store: Mutex::new(controller)
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn get_guides<'r>(storage: State<'r, GwentControllerWrapped>) -> Result<datatypes::Decks, ()> {
    let decks = storage.store.lock().await.get_eastern_decks(0, 1000).await;
    return Ok(decks)
}

fn main() {
    tauri::Builder::default()
        .manage(create_controller())
        .invoke_handler(tauri::generate_handler![get_guides])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
