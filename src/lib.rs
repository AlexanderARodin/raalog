mod backend;
mod interface;
pub use interface::log;


//  //  //  //  //  //  //  //
//      TESTs
//  //  //  //  //  //  //  //
#[cfg(test)]
mod raalog_tests {
    use super::log;
    use super::backend::internal::test_hard_reset as clear_history;

    use sequential_test::sequential;

    #[test]
    #[sequential]
    fn try_outputs() {
        log::info("info 1");
        log::error("error 1");
        print!("show history");
        let history = log::history();
        //print!(">>{history}<<");
    }
    #[test]
    #[sequential]
    fn log_init_state() {
        clear_history();
        let history = log::history();
        assert!( history == "" );
    }
    #[test]
    #[sequential]
    fn log_info() {
        clear_history();
        let validator = "\n : info 1";
        log::info("info 1");
        let history = log::history();
        assert!( history == validator );
    }
    #[test]
    #[sequential]
    fn log_error() {
        clear_history();
        let validator = "\nE: error 1";
        log::error("error 1");
        let history = log::history();
        assert!( history == validator );
    }
    #[test]
    #[sequential]
    fn log_creating() {
        clear_history();
        let validator = "\n+: Struct";
        log::creating("Struct");
        let history = log::history();
        assert!( history == validator );
    }
    #[test]
    #[sequential]
    fn log_droping() {
        clear_history();
        let validator = "\n-: Ob jec t2";
        log::droping("Ob jec t2");
        let history = log::history();
        assert!( history == validator );
    }
    #[test]
    #[sequential]
    fn log_info_error() {
        clear_history();
        let validator = "\n : info 1\n : info 2\nE: error 1\n : info 3\nE: error 2";
        log::info("info 1");
        log::info("info 2");
        log::error("error 1");
        log::info("info 3");
        log::error("error 2");
        let history = log::history();
        assert!( history == validator );
    }
    #[test]
    #[sequential]
    fn log_combo() {
        clear_history();
        let validator = "\n+: info 1\n : info 2\nE: error 1\n-: info 3\nE: error 2";
        log::creating("info 1");
        log::info("info 2");
        log::error("error 1");
        log::droping("info 3");
        log::error("error 2");
        let history = log::history();
        assert!( history == validator );
    }
}
