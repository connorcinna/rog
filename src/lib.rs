use log::{Record, Metadata, SetLoggerError, LevelFilter};
use std::fs::File;
use std::io::prelude::*;
use chrono;
//#[macro_export]
//macro_rules! log {
//    () => {
//        log(line!(), file!().to_string());
//    };
//}

struct Rog;

impl log::Log for Rog
{
    fn enabled(&self, _metadata: &Metadata) -> bool 
    {
        true
    }

    fn log(&self, record: &Record) 
    {
        if self.enabled(record.metadata()) 
        {
            let pre : String = prefix(&record).expect("Expected prefix string"); 

            println!("[{}] {}", pre, record.args());
            let _ = log_to_file(record, pre);
        }
    }

    fn flush(&self) {}
}

static LOGGER: Rog = Rog;

pub fn init() -> Result<(), SetLoggerError>
{
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace))
}

pub fn prefix(record: &Record) -> Result<String, SetLoggerError>
{
    let mut res : String = Default::default();
    let time = chrono::offset::Local::now();
    res += &record.level().as_str();
    res += " ";
    res += &time.to_string();
    res += " ";
    res += &record.file().expect("Expected file name"); 
    res += " ";
    res += &record.line().expect("Expected line number").to_string(); 
    return Ok(res);
}

pub fn log_to_file(record: &Record, pre: String) -> std::io::Result<()>
{
    std::fs::create_dir_all("log")?;
    let s_file = format!("log/{}", &record.file().expect("Expected file name"));
    let mut file = File::create(s_file)?;
    let line = format!("[{}] {}", pre, record.args());
    Ok(file.write_all(&line.into_bytes())?)
}
