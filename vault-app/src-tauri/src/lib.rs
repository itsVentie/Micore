use core_crypto;
use core_storage::MicoreStorage;
use std::sync::Mutex;
use tauri::State;
use tauri::Manager;

struct AppState {
    storage: Mutex<Option<MicoreStorage>>,
    master_key: Mutex<Option<[u8; 32]>>,
}


#[tauri::command]
fn create_or_open_vault(
    password: String,
    vault_name: Option<String>, 
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let salt = b"micore_default_salt_v1";
    let key = core_crypto::derive_key(&password, salt).map_err(|e| e.to_string())?;

    let mut state_storage = state.storage.lock().unwrap();
    let mut state_key = state.master_key.lock().unwrap();

    let app_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    
    let file_name = vault_name.filter(|s| !s.trim().is_empty()).unwrap_or_else(|| "vault.mic".into());
    let storage_path = app_dir.join(file_name);

    let _exists = storage_path.exists();

    let storage = MicoreStorage::open_or_create(&storage_path).map_err(|e| e.to_string())?;
    
    *state_storage = Some(storage);
    *state_key = Some(key);

    Ok(storage_path.to_string_lossy().into_owned())
}

#[tauri::command]
fn get_default_vault_info(app_handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let app_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let storage_path = app_dir.join("vault.mic");
    let exists = storage_path.exists();

    Ok(serde_json::json!({
        "path": storage_path.to_string_lossy(),
        "name": "vault.mic",
        "exists": exists
    }))
}

#[tauri::command]
fn save_secret(
    key_name: String,
    secret_value: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let storage_guard = state.storage.lock().unwrap();
    let master_key_guard = state.master_key.lock().unwrap();

    let storage = storage_guard.as_ref().ok_or("Vault is not unlocked")?;
    let master_key = master_key_guard.as_ref().ok_or("Master key is missing")?;

    let (encrypted_data, nonce) =
        core_crypto::encrypt(master_key, secret_value.as_bytes()).map_err(|e| e.to_string())?;

    let mut payload = nonce;
    payload.extend(encrypted_data);

    storage.set_bytes(&key_name, &payload).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_secret(
    key_name: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let storage_guard = state.storage.lock().unwrap();
    let master_key_guard = state.master_key.lock().unwrap();

    let storage = storage_guard.as_ref().ok_or("Vault is not unlocked")?;
    let master_key = master_key_guard.as_ref().ok_or("Master key is missing")?;

    let payload = storage.get_bytes(&key_name).map_err(|e| e.to_string())?
        .ok_or("Secret not found")?;

    let decrypted_bytes = core_crypto::decrypt(master_key, &payload)
        .map_err(|e| e.to_string())?;

    String::from_utf8(decrypted_bytes).map_err(|e| e.to_string())
}

#[tauri::command]
fn save_profile(
    first_name: String,
    last_name: String,
    email: String,
    avatar_path: String,
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let storage_guard = state.storage.lock().unwrap();
    let master_key_guard = state.master_key.lock().unwrap();

    let storage = storage_guard.as_ref().ok_or("Vault is not unlocked")?;
    let master_key = master_key_guard.as_ref().ok_or("Master key is missing")?;

    let mut final_avatar_path = avatar_path.clone();

    if !avatar_path.is_empty() {
        if let Ok(app_dir) = app_handle.path().app_data_dir() {
            let avatars_dir = app_dir.join("avatars");
            let _ = std::fs::create_dir_all(&avatars_dir);
            
            let ext = std::path::Path::new(&avatar_path)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("png");
            
            let target_path = avatars_dir.join(format!("avatar.{}", ext));

            if std::fs::copy(&avatar_path, &target_path).is_ok() {
                final_avatar_path = target_path.to_string_lossy().into_owned();
            }
        }
    }

    let save_encrypted = |key: &str, val: &str| -> Result<(), String> {
        let (enc, nonce) = core_crypto::encrypt(master_key, val.as_bytes()).map_err(|e| e.to_string())?;
        let mut payload = nonce;
        payload.extend(enc);
        storage.set_bytes(key, &payload).map_err(|e| e.to_string())?;
        Ok(())
    };

    save_encrypted("profile_first_name", &first_name)?;
    save_encrypted("profile_last_name", &last_name)?;
    save_encrypted("profile_email", &email)?;
    save_encrypted("profile_avatar_path", &final_avatar_path)?;

    Ok(final_avatar_path)
}

#[tauri::command]
fn get_profile(
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let storage_guard = state.storage.lock().unwrap();
    let master_key_guard = state.master_key.lock().unwrap();

    let storage = storage_guard.as_ref().ok_or("Vault is not unlocked")?;
    let master_key = master_key_guard.as_ref().ok_or("Master key is missing")?;

    let decrypt_field = |key: &str| -> Result<String, String> {
        if let Some(payload) = storage.get_bytes(key).map_err(|e| e.to_string())? {
            let bytes = core_crypto::decrypt(master_key, &payload).map_err(|e| e.to_string())?;
            String::from_utf8(bytes).map_err(|e| e.to_string())
        } else {
            Ok("".into())
        }
    };

    let first_name = decrypt_field("profile_first_name")?;
    let last_name = decrypt_field("profile_last_name")?;
    let email = decrypt_field("profile_email")?;
    let avatar_path = decrypt_field("profile_avatar_path")?;

    let mut avatar_data_url = "".to_string();
    if !avatar_path.is_empty() {
        if let Ok(bytes) = std::fs::read(&avatar_path) {
            let ext = std::path::Path::new(&avatar_path)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("png");
            
            let mime = match ext.to_lowercase().as_str() {
                "jpg" | "jpeg" => "image/jpeg",
                "png" => "image/png",
                "webp" => "image/webp",
                "svg" => "image/svg+xml",
                _ => "image/png",
            };
            
            use base64::Engine;
            let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
            avatar_data_url = format!("data:{};base64,{}", mime, b64);
        }
    }

    Ok(serde_json::json!({
        "first_name": first_name,
        "last_name": last_name,
        "email": email,
        "avatar_path": avatar_path,
        "avatar_url": avatar_data_url
    }))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            storage: Mutex::new(None),
            master_key: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            create_or_open_vault,
            save_secret,
            get_secret,
            save_profile,
            get_profile 
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}