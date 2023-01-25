use russh_keys::key::KeyPair;
use russh_keys::{decode_secret_key, load_secret_key, Error};
use tauri::api::path::home_dir;

use crate::device_manager::PrivateKey;

impl PrivateKey {
    pub fn priv_key(&self, passphrase: Option<&str>) -> Result<KeyPair, Error> {
        return match self {
            PrivateKey::Path { name } => {
                let key_path = home_dir().unwrap().join(".ssh").join(name);
                load_secret_key(key_path, passphrase.clone())
            }
            PrivateKey::Data { data } => decode_secret_key(data, passphrase.clone()),
        };
    }
}
