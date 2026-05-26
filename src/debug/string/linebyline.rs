use std::fmt::Display;
use log::debug;

pub trait DebugOutLineByLine {
    fn debug<S: Display>(&self, prefix: Option<S>);
    fn debug_noprefix(&self) {
       self.debug(None::<&str>);
    }
}

impl DebugOutLineByLine for &str {
    fn debug<S: Display>(&self, prefix: Option<S>) {
        if let Some(prefix) = prefix {
            debug!("{}", prefix);
        }
        for line in self.lines() {
            debug!("{}", line);
        }
    }
}
impl DebugOutLineByLine for String {
    fn debug<S: Display>(&self, prefix: Option<S>) {
        self.as_str().debug(prefix);
    }
}

#[cfg(all(test, feature = "debug-out-lbl"))]
mod debug_out_lbl_tests {
    use super::*;
    use env_logger::try_init;

    const SPLSTR: &str = "This string\n is split\n  on multiple\n   lines!\n";

    #[test]
    fn debug_multiline_str_noprefix() {
        let _ = try_init();
        SPLSTR.debug_noprefix();
        SPLSTR.debug(None::<&str>);
    }

    #[test]
    fn debug_multiline_string() {
        let _ = try_init();
        let s = String::from(SPLSTR);
        s.debug(Some("A prefix"));
    }
}
