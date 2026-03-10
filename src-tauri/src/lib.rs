mod ai;
mod commands;
#[doc(hidden)]
pub mod db;
mod error;
mod export;
mod import;
mod license;
mod photos;

use commands::{
    analytics as analytics_commands, brand_voice as brand_voice_commands,
    export as export_commands, generate, import as import_commands, license as license_commands,
    photos as photo_commands, property, settings,
};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .map_err(|e| format!("Failed to get app data dir: {}", e))?;

            let db_path = app_data_dir.join("realestate.db");

            let pool = tauri::async_runtime::block_on(async { db::init_pool(&db_path).await })
                .map_err(|e| format!("Failed to initialize database: {}", e))?;

            app.manage(pool);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            property::create_property,
            property::get_property,
            property::list_properties,
            property::update_property,
            property::delete_property,
            settings::get_setting,
            settings::set_setting,
            generate::generate_listing,
            generate::generate_social,
            generate::list_listings,
            generate::toggle_listing_favorite,
            generate::delete_listing,
            generate::generate_email,
            photo_commands::import_photos,
            photo_commands::list_photos,
            photo_commands::delete_photo,
            photo_commands::reorder_photos,
            export_commands::export_pdf,
            export_commands::export_docx,
            export_commands::copy_to_clipboard,
            brand_voice_commands::create_brand_voice,
            brand_voice_commands::list_brand_voices,
            brand_voice_commands::delete_brand_voice,
            license_commands::validate_license_key,
            license_commands::check_license,
            import_commands::import_properties_csv,
            import_commands::get_csv_template,
            analytics_commands::get_analytics_summary,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
