use nix::*;
use std::collections::HashMap;
use std::env;
use std::error::Error as StdError;
use std::process;

pub type BuiltinCmd = fn(&[::Token]);

// I think this is a temporary hack and it probably will make sense to move towards init returning something that can
// be bundled up into a Context object that just gets plumbed around
lazy_static! {
    static ref MAP: HashMap<&'static str, BuiltinCmd> = {
        let mut m = HashMap::new();
        m.insert("cd", cd as BuiltinCmd);
        m.insert("exit", quit);
        m
    };
}

pub fn lookup(command: &str) -> Option<&BuiltinCmd> {
    MAP.get(command)
}

fn cd(args: &[::Token]) {
    if args.len() == 0 {
        let home_dir = env::home_dir().unwrap();
        let home = home_dir.as_path().to_str().unwrap();
        report_if_fails("cd", unistd::chdir(home));
    } else if let Some(&::Token::Word{contents: target}) = args.first() {
        report_if_fails("cd", unistd::chdir(target));
    }
}

fn quit(_: &[::Token]) {
    process::exit(libc::EXIT_SUCCESS);
}

// TODO this should be a macro or something
fn report_if_fails<T>(command: &str, res: Result<T>) {
    if let Err(error) = res {
        println!("{}: {}", command, error.description());
    }
}
