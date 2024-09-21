//use anyhow::Result;
//use log::{Level, Metadata, Record};

mod logger;

use logger::RaaLogger;

//  //  //  //  //  //  //  //

pub fn init() -> anyhow::Result<&'static RaaLogger> {
    RaaLogger::init()
}

