use rusqlite::Connection;
use serde_json::{json};

use crate::chiffrement::chiffrement_mdp;

pub fn call_info() -> Result<String, String> {
     println!(
        "Chemin absolu de la DB: {:?}",
        std::fs::canonicalize("db.sqlite")
    );
    let conn = Connection::open("db.sqlite").map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, site, password FROM mdp_liste;")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(json!({
                "id": row.get::<_, i32>(0)?,
                "url": row.get::<_, String>(1)?,
                "mdp": row.get::<_, String>(2)?,
            }))
        })
        .map_err(|e| e.to_string())?;

    let mut list = Vec::new();
    for row in rows {
        list.push(row.map_err(|e| e.to_string())?);
    }

    let json_string = serde_json::to_string(&list).map_err(|e| e.to_string())?;

    Ok(json_string)
}


#[tauri::command]
pub fn insert(site: &str, mdp:&str) -> Result<bool, String> {
    println!("Insertion de site: {}, mdp: {}", site, mdp);
    let mdpChiffre = chiffrement_mdp(mdp).unwrap();
    println!("mdp chiffré: {}", mdpChiffre);

    let conn = Connection::open("db.sqlite").map_err(|e| e.to_string())?;

    if site.is_empty() || mdp.is_empty() {
        println!("les arguments sont vide");
        return Err("Site and password must not be empty".into());
    } else {

        conn.execute(
            "INSERT INTO mdp_liste (site, password) VALUES (?1, ?2)",
            &[site, &mdpChiffre],
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(true)
}



#[tauri::command]
pub fn supprimer(id: &str) -> Result<bool, String> {
    println!("suppression de id : {}", id);
    
    let conn = Connection::open("db.sqlite").map_err(|e| e.to_string())?;

    if id.is_empty() {
        println!("id est vide explique?");
        return Err("please enter arguments".into())
    } else {
        conn.execute("
            DELETE FROM mdp_liste WHERE id = ?1", [id]).map_err(|e| e.to_string())?;
    }

    Ok(true)

}