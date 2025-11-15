use argon2::Argon2;

//key derivation function
pub fn kdf(password: &str, salt: &[u8]) -> Result<[u8; 32], argon2::Error>{
    let mut output_key_material = [0u8; 32]; // Can be any desired size
    let _ = Argon2::default().hash_password_into(password.as_bytes(), salt, &mut output_key_material)?;
    Ok(output_key_material)
}
