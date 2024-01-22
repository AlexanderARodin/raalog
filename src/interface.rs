
//  //  //  //  //  //  //  //
//      interface
//  //  //  //  //  //  //  //
#[allow(dead_code)]
pub mod log {
    use crate::backend::*;

    pub fn history() -> String {
        let res = get_history();
        res.clone()
    }

    pub fn info(msg: &str){
        add_log_line( &format!( " : {msg}") , false );
    }
    pub fn error(msg: &str){
        add_log_line( &format!( "E: {msg}") , true );
    }
    pub fn droping(msg: &str){
        add_log_line( &format!( "-: {msg}") , false );
    }
    pub fn creating(msg: &str){
        add_log_line( &format!( "+: {msg}") , false );
    }
}

