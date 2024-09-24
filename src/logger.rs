use anyhow::Result;
use log::{Metadata, Record};
use std::sync::RwLock;

mod convertor;

//  //  //  //  //  //  //  //
static RAA_LOGGER: RaaLogger = RaaLogger {
    mode: RwLock::new(RaaLoggerMode::StdErr),
};
//  //  //  //  //  //  //  //
enum RaaLoggerMode {
    Off,
    StdErr,
    StdOut,
    Buffer(&'static RwLock<Vec<String>>),
    File(std::fs::File),
}

//  //  //  //  //  //  //  //
pub struct RaaLogger {
    mode: RwLock<RaaLoggerMode>,
}

#[allow(dead_code)]
impl RaaLogger {
    pub fn init() -> Result<&'static Self> {
        if let Err(ee) = log::set_logger(&RAA_LOGGER) {
            return Err(anyhow::anyhow!(ee.to_string()));
        }
        log::set_max_level(log::LevelFilter::Info);
        Ok(&RAA_LOGGER)
    }

    pub fn set_level(&self, level: log::LevelFilter) -> &Self {
        log::set_max_level(level);
        self
    }

    pub fn set_file_mode(&self, file_name: &str) -> Result<&Self> {
        let file = std::fs::File::options()
            .append(true)
            .create(true)
            .open(file_name)?;
        self.set_mode(RaaLoggerMode::File(file));
        Ok(self)
    }
    pub fn set_buffer_mode(&self, buf: &'static RwLock<Vec<String>>) -> &Self {
        self.set_mode(RaaLoggerMode::Buffer(buf))
    }
    pub fn set_stderr_mode(&self) -> &Self {
        self.set_mode(RaaLoggerMode::StdErr)
    }
    pub fn set_stdout_mode(&self) -> &Self {
        self.set_mode(RaaLoggerMode::StdOut)
    }
    pub fn set_silent_mode(&self) -> &Self {
        self.set_mode(RaaLoggerMode::Off)
    }
}

//  //  //  //  //  //  //  //
//        internal          //
//  //  //  //  //  //  //  //
impl RaaLogger {
    fn set_mode(&self, new_mode: RaaLoggerMode) -> &Self {
        let mode: &mut RaaLoggerMode = &mut self.mode.write().expect("set_pts: RwLock");
        *mode = new_mode;

        self
    }

    fn log_to_buffer(b: &RwLock<Vec<String>>, s: String) {
        let buf: &mut Vec<String> = &mut b
            .write()
            .expect("RwLock can't read RaaLogLogger.Buffer(buf)");
        buf.push(s);
    }

    fn log_to_file(mut file: &std::fs::File, s: String) -> Result<()> {
        use std::io::Write;
        writeln!(file, "{}", s)?;
        Ok(())
    }

    fn reformat(record: &Record) -> String {
        let message = record.args().to_string();
        let parsed = convertor::convert_to_lines(Some(&message));
        //let parsed = convertor::convert_to_lines(record.args().as_str());
        let mut result = String::new();

        for line in parsed {
            if !result.is_empty() {
                result.push('\n');
            }
            //result.push_str(&line.0);
            let sign = match line.1 {
                    convertor::LinePosition::NULL   => '?',
                    convertor::LinePosition::Single => ':',
                    convertor::LinePosition::First  => '*',
                    convertor::LinePosition::Middle => '│',
                    convertor::LinePosition::Last   => '└',
            };
            result.push_str(
                &format!("[{:>5}] {} {}", record.level(), sign, line.0)
            );
        }

        result
    }
}

//  //  //  //  //  //  //  //
//        log::Log          //
//  //  //  //  //  //  //  //
impl log::Log for RaaLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let mode: &RaaLoggerMode = &self.mode.read().expect("RwLock can't read RaaLogger.moode");
        match mode {
            RaaLoggerMode::Off => return false,
            _ => metadata.level() <= log::max_level(),
        }
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let mode: &RaaLoggerMode = &self.mode.read().expect("RaaLog.log: RwLock");
        match mode {
            RaaLoggerMode::Off => return,
            RaaLoggerMode::File(file) => {
                let s = Self::reformat(record);
                let _ = RaaLogger::log_to_file(file, s);
            }
            RaaLoggerMode::Buffer(buf) => {
                //let s = format!("[{:>5}]: {}", record.level(), record.args());
                let s = Self::reformat(record);
                RaaLogger::log_to_buffer(buf, s);
            }
            RaaLoggerMode::StdErr => {
                let s = Self::reformat(record);
                eprintln!("{}", s);
            }
            RaaLoggerMode::StdOut => {
                let s = Self::reformat(record);
                println!("{}", s);
            }
        }
    }

    fn flush(&self) {
        todo!("RaaLoggerMode needs flush()");
    }
}

//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod basic_validation {
    use super::*;
    use crate as raalogger;

    #[test]
    fn single_init() -> Result<()> {
        let _ = raalogger::init();
        Ok(())
    }

    #[test]
    #[should_panic]
    fn double_init() {
        let _ = raalogger::init();
        RaaLogger::init().expect("double init of RaaLogger");
    }

    static BUF_E: RwLock<Vec<String>> = RwLock::new(Vec::new());
    #[test]
    fn buffer_err() -> Result<()> {
        let _ = raalogger::init();
        RAA_LOGGER.set_buffer_mode(&BUF_E);

        a_few_loggings();
        log::set_max_level(log::LevelFilter::Warn);
        a_few_loggings();
        log::set_max_level(log::LevelFilter::Debug);
        a_few_loggings();
        log::set_max_level(log::LevelFilter::Trace);
        a_few_loggings();
        log::set_max_level(log::LevelFilter::Error);
        a_few_loggings();

        let buf_content = format!("{:#?}", BUF_E.read().unwrap());
        let comparator = format!(
            "{:#?}",
            vec![
                "[ INFO] : some info",
                "[ WARN] : some warn",
                "[ERROR] : some error",
                "[ WARN] : some warn",
                "[ERROR] : some error",
                "[DEBUG] : some debug",
                "[ INFO] : some info",
                "[ WARN] : some warn",
                "[ERROR] : some error",
                "[TRACE] : some trace",
                "[DEBUG] : some debug",
                "[ INFO] : some info",
                "[ WARN] : some warn",
                "[ERROR] : some error",
                "[ERROR] : some error",
            ]
        );

        assert!(buf_content == comparator);

        Ok(())
    }
    //  //  //  //  //  //  //  //
    fn a_few_loggings() {
        log::trace!("some trace");
        log::debug!("some debug");
        log::info!("some info");
        log::warn!("some warn");
        log::error!("some error");
    }
    //  //  //  //  //  //  //  //

    #[test]
    fn to_stderr() -> Result<()> {
        let _ = raalogger::init();
        //RAA_LOGGER.set_pts("uni-pts");
        //log::info!("test info_msg");

        Ok(())
    }
}
