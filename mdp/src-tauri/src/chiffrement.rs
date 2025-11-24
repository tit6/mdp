use argon2::Argon2;
use hmac_sha256::HKDF;


use aes_gcm::{
    Aes256Gcm,                       // AES-256 en mode GCM
    aead::{Aead, KeyInit},          // traits pour new()/encrypt()/decrypt()
    Key, Nonce,
};
use rand::RngCore;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;


#[derive(Debug)]
pub struct EncryptedPassword {
    pub nonce: [u8; 12],    // IV/nonce GCM (96 bits)
    pub ciphertext: Vec<u8>
}


use once_cell::sync::OnceCell;

pub struct Authpass {
    pub db : [u8; 32],
    pub password : [u8; 32],
}

static AUTH_KEYS: OnceCell<Authpass> = OnceCell::new();

pub fn set_auth_keys(keys: Authpass) {
    let _ = AUTH_KEYS.set(keys);
}

pub fn get_db_key() -> Option<[u8; 32]> {
    AUTH_KEYS.get().map(|k| k.db)
}

pub fn get_password_key() -> Option<[u8; 32]> {
    AUTH_KEYS.get().map(|k| k.password)
}


//key derivation function
pub fn kdf(password: &str, salt: &[u8]) -> Result<[u8; 32], argon2::Error>{
    let mut output_key_material = [0u8; 32]; // Can be any desired size
    let _ = Argon2::default().hash_password_into(password.as_bytes(), salt, &mut output_key_material)?;
    Ok(output_key_material)
}

pub fn hkdf(master_key: &[u8], hkdf_salt : &[u8], info: &[u8]) -> [u8; 32] {

    let prk = HKDF::extract(hkdf_salt, master_key);

    // Étape Expand : OKM (Output Key Material)
    let mut output = [0u8; 32]; // 32 bytes -> clé 256 bits
    HKDF::expand(&mut output, prk, info);

    output
}


pub fn encrypt_password(key_bytes: &[u8; 32], password: &[u8]) -> Result<EncryptedPassword, aes_gcm::Error> {
    // 1) Construire la clé AES-256-GCM
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);

    // 2) Générer un nonce (IV) aléatoire de 12 octets
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes); // &[u8; 12] -> Nonce

    // 3) Chiffrer
    let ciphertext = cipher.encrypt(nonce, password)?;

    Ok(EncryptedPassword {
        nonce: nonce_bytes,
        ciphertext,
    })
}

pub fn decrypt_password(key_bytes: &[u8; 32], enc: &EncryptedPassword) -> Result<Vec<u8>, aes_gcm::Error> {
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);

    let nonce = Nonce::from_slice(&enc.nonce);

    let plaintext = cipher.decrypt(nonce, enc.ciphertext.as_ref())?;

    Ok(plaintext)
}


pub fn encrypted_db() -> Result<(), Box<dyn std::error::Error>>  {
    let mut file = File::open("db.sqlite")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    println!("voicie la db : {:?}", buffer);

    
    let key = get_db_key().ok_or("clé DB non initialisée")?;
    let encrypt = encrypt_password(&key, &buffer);


    match encrypt {
        Ok(enc) => {
            let mut output = File::create("encrypted_db.enc")?;
            let mut writer = BufWriter::new(output);

            // 1) écrire le nonce (IV)
            writer.write_all(&enc.nonce)?;

            // 2) écrire le ciphertext
            writer.write_all(&enc.ciphertext)?;

            writer.flush()?;

        },
        Err(err) => println!("encrypt error : {:?}", err)
    }


    Ok(())
}


pub fn read_encrypted_db(path: &str) -> std::io::Result<EncryptedPassword> {
    let data = std::fs::read(path)?;

    if data.len() < 12 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Fichier trop court pour contenir un nonce AES-GCM",
        ));
    }

    let mut nonce = [0u8; 12];
    nonce.copy_from_slice(&data[..12]);

    let ciphertext = data[12..].to_vec();

    Ok(EncryptedPassword { nonce, ciphertext })
}



pub fn decrypted_db(key : [u8; 32]) -> Result<(), Box<dyn std::error::Error>>  {

    let encrypted = read_encrypted_db("encrypted_db.enc")?;
  


    let decrypt = decrypt_password(&key, &encrypted);
    let mut output = File::create("db.sqlite")?;
    let mut writer = BufWriter::new(output);

    match decrypt {
        Ok(d) => writer.write_all(&d)?,
        Err(e) => return Err(e.into()),
    }

    //writer.write_all(&decrypt.unwrap())?;
    writer.flush()?;

    Ok(())


}

pub fn derivation(password: String) -> bool{
    let salt: Vec<u8> = vec![
    1, 2, 3, 4, 5, 6, 7, 8,
    9, 10, 11, 12, 13, 14, 15, 16
    ];

    let t = kdf(&password, &salt);
    match t {
        Ok(u) => println!("{:?}", u),
        Err(err) => println!("{:?}", err),

    }
    let hkdf_salt: Vec<u8> = vec![
    1, 2, 3, 4, 5, 6, 7, 8,
    9, 10, 11, 12, 13, 14, 15, 16
    ];

    let db_key : [u8; 32] = hkdf(&t.unwrap(), &hkdf_salt, b"bdb");
    let hkdf_password : [u8; 32] = hkdf(&t.unwrap(), &hkdf_salt, b"hkdf password");

    let keys = Authpass {
        db: db_key,
        password: hkdf_password,
    };

    // On stocke globalement
    set_auth_keys(keys);

    println!("key db : {:?}", db_key);
    println!("key password : {:?}", hkdf_password);

    let file_db = decrypted_db(db_key);
    match file_db {
        Ok(u) => {
            println!("{:?}", u);
            true
        },
        Err(err) => {
            println!("{:?}", err);
            false
        },
    }
}

// executer quand on a perdu le db enc fonction destiner à être supprimer
pub fn encrypted_db_temp() -> Result<(), Box<dyn std::error::Error>>  {
    let mut file = File::open("db.sqlite")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    println!("voicie la db : {:?}", buffer);

    
    let key :[u8; 32]= [11, 29, 72, 227, 171, 25, 2, 39, 105, 83, 231, 79, 115, 154, 211, 103, 32, 230, 174, 151, 211, 151, 29, 117, 150, 239, 35, 38, 209, 125, 79, 207];
    let encrypt = encrypt_password(&key, &buffer);


    match encrypt {
        Ok(enc) => {
            let mut output = File::create("encrypted_db.enc")?;
            let mut writer = BufWriter::new(output);

            // 1) écrire le nonce (IV)
            writer.write_all(&enc.nonce)?;

            // 2) écrire le ciphertext
            writer.write_all(&enc.ciphertext)?;

            writer.flush()?;

        },
        Err(err) => println!("encrypt error : {:?}", err)
    }


    Ok(())
}

pub fn chiffrement_mdp(password:&str) -> Result<String, Box<dyn std::error::Error>> {
    let db_key : [u8; 32] =  get_password_key().unwrap();

    let temp_mdp = encrypt_password(&db_key, password.as_bytes());
    
    match temp_mdp {
        Ok(enc) => {
            

           let mdp = enc
            .nonce
            .iter()
            .chain(enc.ciphertext.iter())
            .map(|b| format!("{:02x}", b))
            .collect::<String>();

            println!("mdp chiffré hex : {}", mdp);

            return Ok(mdp);


        },
        Err(err) => {
            println!("encrypt error : {:?}", err);
            return Err(err.into());
        }
    }

}