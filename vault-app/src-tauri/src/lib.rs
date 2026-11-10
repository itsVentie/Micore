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
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let salt = b"micore_default_salt_v1";
    let key = core_crypto::derive_key(&password, salt).map_err(|e| e.to_string())?;

    let app_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    
    let storage_path = app_dir.join("vault.mic");

    let storage = MicoreStorage::open_or_create(storage_path).map_err(|e| e.to_string())?;

    let mut state_storage = state.storage.lock().unwrap();
    let mut state_key = state.master_key.lock().unwrap();

    *state_storage = Some(storage);
    *state_key = Some(key);

    Ok("Vault successfully unlocked!".into())
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
    name: String,
    email: String,
    avatar: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let storage_guard = state.storage.lock().unwrap();
    let master_key_guard = state.master_key.lock().unwrap();

    let storage = storage_guard.as_ref().ok_or("Vault is not unlocked")?;
    let master_key = master_key_guard.as_ref().ok_or("Master key is missing")?;

    let (enc_name, nonce_name) = core_crypto::encrypt(master_key, name.as_bytes()).map_err(|e| e.to_string())?;
    let mut p_name = nonce_name; p_name.extend(enc_name);
    storage.set_bytes("profile_name", &p_name).map_err(|e| e.to_string())?;

    let (enc_email, nonce_email) = core_crypto::encrypt(master_key, email.as_bytes()).map_err(|e| e.to_string())?;
    let mut p_email = nonce_email; p_email.extend(enc_email);
    storage.set_bytes("profile_email", &p_email).map_err(|e| e.to_string())?;

    let (enc_av, nonce_av) = core_crypto::encrypt(master_key, avatar.as_bytes()).map_err(|e| e.to_string())?;
    let mut p_av = nonce_av; p_av.extend(enc_av);
    storage.set_bytes("profile_avatar", &p_av).map_err(|e| e.to_string())?;

    Ok(())
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

    let name = decrypt_field("profile_name")?;
    let email = decrypt_field("profile_email")?;
    let avatar = decrypt_field("profile_avatar")?;

    Ok(serde_json::json!({
        "name": if name.is_empty() { "User" } else { &name },
        "email": if email.is_empty() { "user@micore.local" } else { &email },
        "avatar": if avatar.is_empty() { "U" } else { &avatar }
    }))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            storage: Mutex::new(None),
            master_key: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            create_or_open_vault,
            save_secret
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}