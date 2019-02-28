extern crate winapi;

#[macro_use]
extern crate clap;

use clap::App;
use std::io::Error;
use std::os::windows::prelude::*;
use std::path::Path;

use winapi::um::utilapiset::Beep;
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::TRUE;
use std::ffi::{OsString, OsStr};

type Res<T> = std::result::Result<T, std::io::Error>;

fn beep(freq: u32, dur: u32) -> Res<String> {
    let ret = unsafe {
        Beep(
            freq as DWORD,
            dur as DWORD
        )
    };
    match ret {
        TRUE => Ok("ok".to_string()),
        _ => Err(Error::last_os_error().into()),
    }
}

fn main() {
    let yaml = load_yaml!("args.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let duration = matches.value_of("duration").unwrap_or("200");
    let frequency = matches.value_of("frequency").unwrap_or("800");
    let ret = beep(
                  frequency.parse::<u32>().unwrap(),
                  duration.parse::<u32>().unwrap(),
              ).unwrap();
    println!("{}", ret);
}
