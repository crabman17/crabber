// source: https://github.com/ardaku/whoami


#![allow(dead_code)]


use std::{
    io::{
        self, 
        Error as IOError
    },
    os::{
        raw::{c_char, c_int, c_uchar, c_ulong},
        windows::ffi::OsStringExt,
    },
    ffi::OsString,
    ptr::null_mut,
};

const UNKNOW: &str = "unknown";

#[allow(unused)]
#[repr(C)]
enum ExtendedNameFormat {
    Unknown,          // Nothing
    FullyQualifiedDN, // Nothing
    SamCompatible,    // Hostname Followed By Username
    Display,          // Full Name
    UniqueId,         // Nothing
    Canonical,        // Nothing
    UserPrincipal,    // Nothing
    CanonicalEx,      // Nothing
    ServicePrincipal, // Nothing
    DnsDomain,        // Nothing
    GivenName,        // Nothing
    Surname,          // Nothing
}

#[allow(unused)]
#[repr(C)]
enum ComputerNameFormat {
    NetBIOS,                   // Same as GetComputerNameW
    DnsHostname,               // Fancy Name
    DnsDomain,                 // Nothing
    DnsFullyQualified,         // Fancy Name with, for example, .com
    PhysicalNetBIOS,           // Same as GetComputerNameW
    PhysicalDnsHostname,       // Same as GetComputerNameW
    PhysicalDnsDomain,         // Nothing
    PhysicalDnsFullyQualified, // Fancy Name with, for example, .com
    Max,
}

#[link(name = "secur32")]
extern "system" {
    fn GetUserNameExW(
        a: ExtendedNameFormat,
        b: *mut c_char,
        c: *mut c_ulong,
    ) -> c_uchar;
    fn GetUserNameW(a: *mut c_char, b: *mut c_ulong) -> c_int;
    fn GetComputerNameExW(
        a: ComputerNameFormat,
        b: *mut c_char,
        c: *mut c_ulong,
    ) -> c_int;
}

pub fn get_username() -> String {
    let mut size = 256 + 1;

    // Step 2. Allocate memory to put the Windows (UTF-16) string.
    let mut name: Vec<u16> = Vec::with_capacity(256 + 1);

    let fail =
        unsafe { GetUserNameW(name.as_mut_ptr().cast(), &mut size) == 0 };
    if fail {
        return UNKNOW.to_owned();
    }

    unsafe {
        name.set_len(usize::try_from(size).unwrap());
    } 

    // Step 3. Convert to Rust String
    OsString::from_wide(&name).to_string_lossy().to_string()
}

pub fn get_realname() -> String {
    // Step 1. Retrieve the entire length of the username
    let mut size = 0;
    unsafe {
        GetUserNameExW(
            ExtendedNameFormat::Display,
            null_mut(),
            &mut size,
        );
    };

    // Step 2. Allocate memory to put the Windows (UTF-16) string.
    let mut name: Vec<u16> = vec![0; usize::try_from(size).unwrap()];

    let fail = unsafe {
        GetUserNameExW(
            ExtendedNameFormat::Display,
            name.as_mut_ptr().cast(),
            &mut size,
        ) == 0
    };
    if fail {
        return UNKNOW.to_owned();
    }

    // Step 3. Convert to Rust String
    OsString::from_wide(&name).to_string_lossy().to_string()
}

pub fn get_devicename() -> String {
    // Step 1. Retreive the entire length of the device name
    let mut size = 0;
    unsafe {
        // Ignore error, we know that it will be ERROR_INSUFFICIENT_BUFFER
        GetComputerNameExW(
            ComputerNameFormat::DnsHostname,
            null_mut(),
            &mut size,
        );
    };

    // Step 2. Allocate memory to put the Windows (UTF-16) string.
    let mut name: Vec<u16> = vec![0; usize::try_from(size).unwrap()];

    let fail = unsafe {
        GetComputerNameExW(
            ComputerNameFormat::DnsHostname,
            name.as_mut_ptr().cast(),
            &mut size,
        ) == 0
    };
    if fail {
        return UNKNOW.to_owned();
    }

    // Step 3. Convert to Rust String
    OsString::from_wide(&name).to_string_lossy().to_string()
}

pub fn get_hostname() -> String {
    // Step 1. Retreive the entire length of the username
    let mut size = 0;
    unsafe {
        // Ignore error, we know that it will be ERROR_INSUFFICIENT_BUFFER
        GetComputerNameExW(
            ComputerNameFormat::NetBIOS,
            null_mut(),
            &mut size,
        );
    };

    // Step 2. Allocate memory to put the Windows (UTF-16) string.
    let mut name: Vec<u16> = vec![0; usize::try_from(size).unwrap()];

    let fail = unsafe {
        GetComputerNameExW(
            ComputerNameFormat::NetBIOS,
            name.as_mut_ptr().cast(),
            &mut size,
        ) == 0
    };
    if fail {
        return UNKNOW.to_owned();
    }
    
    // Step 3. Convert to Rust String
    OsString::from_wide(&name).to_string_lossy().to_string()
}