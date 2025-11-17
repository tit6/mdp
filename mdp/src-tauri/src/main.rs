// src-tauri/src/main.rs
use std::sync::{Arc, Mutex};
use tauri::State;

mod db;
use db::{insert, call_info};

mod chiffrement;
use chiffrement::EncryptedPassword;

struct Session {
    authenticated: bool,
}

type SharedSession<'a> = State<'a, Arc<Mutex<Session>>>;

#[tauri::command]
fn login_backend(session: SharedSession<'_>, password: String) -> bool {
    let mut login = chiffrement::derivation(password);
    //let mut login = chiffrement::encrypted_db_temp();
    let mut guard = session.lock().unwrap();
    //guard.authenticated = true;
    guard.authenticated = login;
    guard.authenticated
}

#[tauri::command]
fn logout_backend(session: SharedSession<'_>) {
    let _ = chiffrement::encrypted_db();
    session.lock().unwrap().authenticated = false;
}

#[tauri::command]
fn secure_action(session: SharedSession<'_>) -> Result<String, String> {
    if session.lock().unwrap().authenticated {
        match call_info() {
            Ok(data) => Ok(data.to_string()),
            Err(e) => Err(e),
        }
    } else {
        Err("unauthorized".into())
    }
}




fn main() {
  tauri::Builder::default()
    .manage(Arc::new(Mutex::new(Session { authenticated: false })))
    .invoke_handler(tauri::generate_handler![login_backend, logout_backend, secure_action, insert])
    .run(tauri::generate_context!())
    .expect("error running app");
}
    
