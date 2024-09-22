//use anyhow::Result;
//use log::{Level, Metadata, Record};

mod logger;

use logger::RaaLogger;

#[allow(unused_imports)]
pub use log::{debug, error, info, trace, warn};

pub use log::LevelFilter;

//  //  //  //  //  //  //  //

pub fn init() -> anyhow::Result<&'static RaaLogger> {
    RaaLogger::init()
}

