//use anyhow::Result;
//use log::{Level, Metadata, Record};

mod logger;

use logger::RaaLogger;
//use logger::RAA_LOGGER as LOG;

//  //  //  //  //  //  //  //

pub fn init() -> anyhow::Result<&'static RaaLogger> {
    RaaLogger::init()
}

