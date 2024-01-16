

pub(crate) fn add_log_line(line: String, is_error: bool) {
    internal::print_echo( &line, is_error);
    internal::add_to_history( &line, is_error );
}
pub(crate) fn get_history() -> String {
    internal::get_history()
}

//  //  //  //  //  //  //  //
//      internal
//  //  //  //  //  //  //  //
pub(crate) mod internal {
    use std::sync::Mutex;
    static HISTORY: Mutex<String> = Mutex::new( String::new() );

    #[cfg(test)]
    pub(crate) fn test_hard_reset() {
        let mut hist = HISTORY
                        .lock()
                        .expect("Panic on locking logs HISTORY!!");
        hist.clear();
    }

    pub(super) fn get_history() -> String {
        let hist = HISTORY
                        .lock()
                        .expect("Panic on locking logs HISTORY!!");
        hist.clone()
    }

    pub(super) fn add_to_history( msg: &String, is_error: bool ) {
        let mut hist = HISTORY
                        .lock()
                        .expect("Panic on locking logs HISTORY!!");
        *hist += "\n";
        if is_error {
            *hist += "";
        }else{
            *hist += "";
        }
        *hist += &msg;
    }
    pub(super) fn print_echo( msg: &String, is_error: bool ) {
        if is_error {
            eprintln!("[APP] {msg}");
        }else{
            println!("[APP] {msg}");
        }
    }
}



