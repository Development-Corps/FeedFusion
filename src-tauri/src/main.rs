// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod internal; // Declare the internal module
use internal::app_initialization::tauri_init::{
    setup_app, handle_menu_event, generate_menu,
};

fn main() {
    let menu = generate_menu();
    
    // create sqliteDB -- including tables, generate fake data
    internal::sqlite_db::create_db().expect("Panic create DB");
    internal::sqlite_db::create_fake_data().expect("Panic create fake data");
    
    // Retrieve fake data
    // let fake_data = internal::sqlite_db::retrieve_articles().expect("Panic query fake data");
    let fake_data = internal::sqlite_db::db_fetch_feed();

    // Display fake data
    println!("\nMAIN FUNCTION\n---------------------------------------------");

    for e in fake_data {
        println!("FeedItem: {:?}", e);
    }

    println!("---------------------------------------------\n");

    tauri::Builder::default()
        .setup(setup_app)
        .menu(menu)
        .on_menu_event(handle_menu_event)
        .invoke_handler(tauri::generate_handler![
            internal::api::endpoints::greet,
            internal::api::endpoints::add_feed,
            internal::api::endpoints::load_feeds,
            internal::api::endpoints::get_articles
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}