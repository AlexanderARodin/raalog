

pub(crate) fn add_log_line(line: String, is_error: bool) {
    let echo_line = format!("[APP] {line}");
    internal::print_echo( &echo_line, is_error);
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

    pub(super) fn add_to_history( msg: &str, is_error: bool ) {
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
    #[ cfg(not(target_arch = "wasm32")) ]
    pub(super) fn print_echo( msg: &String, is_error: bool ) {
        if is_error {
            eprintln!("{msg}");
        }else{
            println!("{msg}");
        }
    }
    #[ cfg(target_arch = "wasm32") ]
    pub(super) fn print_echo( msg: &str, is_error: bool ) {
        if is_error {
            super::wasming::error(msg.to_string());
        }else{
            super::wasming::log(msg);
        }
    }



}

#[ cfg(target_arch = "wasm32") ]
pub(super) mod wasming {
    #[ cfg(target_arch = "wasm32") ]
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        pub(super) fn log(value: &str);

        #[wasm_bindgen(js_namespace = console)]
        pub(super) fn error(msg: String);
    }
}


