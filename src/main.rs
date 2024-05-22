use std::io::{Read, Write};
use std::os::fd::AsRawFd;

fn term_setup() -> Result<termios::Termios, ()> {
    let fd = std::io::stdin().as_raw_fd();
    let mut termios = match termios::Termios::from_fd(fd) {
        Ok(termios) => termios,
        Err(_) => return Err(()),
    };
    let termios_orig = termios.clone();
    termios.c_lflag &= !(termios::ICANON | termios::ECHO);
    termios.c_cc[termios::VMIN] = 1;
    termios.c_cc[termios::VTIME] = 0;
    let _ = termios::tcsetattr(fd, termios::TCSAFLUSH, &termios);
    Ok(termios_orig)
}

fn term_restore(termios_orig: termios::Termios) {
    let fd = std::io::stdin().as_raw_fd();
    let _ = termios::tcsetattr(fd, termios::TCSAFLUSH, &termios_orig);
}

fn prompt() {
    print!("> ");
    let _ = std::io::stdout().flush();
}

fn main() {
    let termios_orig = match term_setup() {
        Ok(termios_orig) => termios_orig,
        Err(_) => return,
    };
    let mut chars = Vec::<char>::new();
    prompt();
    let mut buf = [0u8; 1];
    loop {
        match std::io::stdin().read(&mut buf) {
            Ok(_) => {
                let c = char::from(buf[0]);
                eprintln!("read: {:?}", c);
                match c {
                    '?' => {
                        // HELP
                        eprintln!("HELP");
                        println!("");
                        println!("HELP");
                        prompt();
                        print!("{}", String::from_iter(&chars));
                        let _ = std::io::stdout().flush();
                    }
                    '\u{4}' => {
                        // EOT(Ctrl-D)
                        eprintln!("EOT(Ctrl-D)");
                        println!("");
                        break;
                    }
                    '\n' => {
                        // LF
                        eprintln!("LF");
                        println!("");
                        println!("INPUT: {}", String::from_iter(&chars));
                        chars.clear();
                        prompt();
                    }
                    _ => {
                        print!("{}", c);
                        let _ = std::io::stdout().flush();
                        chars.push(c);
                    }
                }
            }
            Err(_) => break,
        }
    }
    term_restore(termios_orig);
}
