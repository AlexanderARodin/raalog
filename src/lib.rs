use std::sync::Mutex;

static INTER_LOG: Mutex<String> = Mutex::new( String::new() );

#[allow(dead_code)]
pub mod log {
    pub fn simple(msg: &str){
        super::add_log_line( format!( "> {msg}") , false );
    }
    pub fn create(name: &str){
        super::add_log_line( format!( "+[{name}]") , false );
    }
    pub fn on_drop(name: &str){
        super::add_log_line( format!( "-[{name}]") , false );
    }
    pub fn error(name: &str, error: &str){
        super::add_log_line( format!( "E[{name}]: {error}") , true );
    }
    pub fn info(name: &str, info: &str){
        super::add_log_line( format!( " [{name}]: {info}") , false );
    }
    
    pub fn tick(){
        let mut log = super::INTER_LOG.lock().unwrap();
        *log += "|";
        print!("|");
    }
    
    pub fn get() -> String {
        let res = super::INTER_LOG.lock().unwrap();
        res.clone()
    }
}

fn add_log_line(line: String, is_error: bool) {
    print_echo( &line, is_error);
    let mut log = INTER_LOG.lock().unwrap();
    *log += "\n";
    *log += &line;
}

fn print_echo( msg: &String, is_error: bool ) {
    if is_error {
        eprintln!("{msg}");
    }else{
        println!("{msg}");
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
