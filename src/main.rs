#[macro_use]
extern crate itertools;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate maplit;
extern crate nix;

mod builtin;

mod index;
use index::Index;

mod lex;
use lex::Lex;
use lex::Token::*;

mod parse;
use parse::Parse;

mod logger;
use logger::Logger;

use nix::*;
use std::env;
use std::ffi::CString;
use std::io;
use std::io::Write;
use std::process;

fn result<T, E>(t: std::result::Result<T, E>) -> T
where
    E: std::fmt::Debug,
{
    if t.is_ok() {
        return t.unwrap();
    }

    debug!("Panicking at");
    return t.unwrap();
}

pub enum Token<'a> {
    Quit,
    Pipe,
    Eol,
    Word { contents: &'a str },
}

fn lex(line: &str) -> Vec<Token> {
    //debug!("lexing '{}'", line.replace("\n", "\\n"));
    let chunks = line.split_whitespace();
    let mut cantrip = Vec::new();
    for word in chunks {
        cantrip.push(Token::Word { contents: word });
    }
    cantrip
}

fn main() {
    Logger::init();

    let paths = ["/bin", "/sbin", "/usr/bin", "/usr/sbin", "/usr/local/bin"];

    let index = Index::new(&paths);

    loop {
        let cwd = result(env::current_dir());
        print!("{} ▶ ", cwd.display());

        // TODO need to consider handling this on potential unexpected hangups?
        io::stdout().flush().unwrap();

        let mut input = String::new();

        if let Ok(bytes_read) = io::stdin().read_line(&mut input) {
            if bytes_read == 0 {
                // EOF!
                process::exit(libc::EXIT_SUCCESS);
            }

            debug!("********* {}", input.trim());

            let mut p = Parse::new(Lex::new(&input));
            let foo = p.parse();

            println!("Parse result: {:?}", foo);

            let token_stream = Lex::new(&input);

            for token in token_stream {
                debug!("Token: {:?}", token);
            }

            let cantrip = lex(input.trim());

            let (cmd, rest) = match cantrip.split_first() {
                Some((&Token::Word { contents: cmd }, rest)) => (cmd, rest),
                Some(_) => continue, // not implemented right now
                None => continue,
            };

            if let Some(command) = builtin::lookup(cmd) {
                command(rest);
            } else if let Some(entry) = index.lookup(cmd) {
                if let Ok(pid) = unistd::fork() {
                    if pid.is_child() {
                        let centry = result(CString::new(entry.as_str()));
                        let ccmd = result(CString::new(cmd));
                        let mut carguments = Vec::new();
                        carguments.push(ccmd);
                        for arg in rest {
                            if let &Token::Word { contents: carg } = arg {
                                carguments.push(CString::new(carg).unwrap());
                            }
                        }

                        unistd::execv(&centry, &carguments);

                        // It shouldn't be possible to reach this unless something went wrong with execv
                        process::exit(libc::EXIT_FAILURE);
                    } else if let unistd::ForkResult::Parent { child: child_id } = pid {
                        loop {
                            if let Ok(exit_status) =
                                sys::wait::waitpid(child_id, Some(sys::wait::WUNTRACED))
                            {
                                match exit_status {
                                    sys::wait::WaitStatus::Exited(_, _)
                                    | sys::wait::WaitStatus::Signaled(_, _, _) => break,
                                    // continue waiting
                                    _ => continue,
                                }
                            } else {
                                // panic?
                            }
                        }
                    }
                } else {
                    // couldn't fork, do something useful here (exit w/ error message)
                }
            } else {
                println!("Not found: {:?}", cmd);
            }
        }
    }
}
