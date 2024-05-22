use std::io::{Read, Write};
use std::os::fd::AsRawFd;

use nix::sys::signal;

nix::ioctl_read_bad! {
    ioctl_gwinsz,
    libc::TIOCGWINSZ,
    libc::winsize
}

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

fn gwinsz() -> Result<(u32, u32), ()> {
    unsafe {
        let mut ws = libc::winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        if let Ok(_) = ioctl_gwinsz(std::io::stdout().as_raw_fd(), &mut ws) {
            Ok((ws.ws_row as u32, ws.ws_col as u32))
        } else {
            Err(())
        }
    }
}

struct Context {
    col: u32,
    chars: Vec<char>,
    prompt: String,
}

impl Context {
    fn new() -> Self {
        let (_, col) = gwinsz().unwrap();
        Self {
            col: col,
            chars: Vec::<char>::new(),
            prompt: String::from("> "),
        }
    }
    fn prompt(&self) {
        print!("{}", self.prompt);
        let _ = std::io::stdout().flush();
    }
    fn curpos(&self) -> (u32, u32) {
        let mut col = 0;
        col += self.prompt.chars().count() as u32;
        col += self.chars.len() as u32;
        let row = col / self.col;
        let col = col % self.col;
        (row, col)
    }
    fn delc(&mut self) {
        let (_, curcol) = self.curpos();
        if let Some(_) = self.chars.pop() {
            if curcol == 0 {
                eprintln!("Line return");
                print!("\u{1b}[A\u{1b}[{}C\u{1b}[K\n", self.col - 1);
                print!("\r\u{1b}[K\u{1b}[A\u{1b}[{}C", self.col - 1);
            } else {
                print!("\u{8}\u{1b}[K");
            }
            let _ = std::io::stdout().flush();
        }
    }
    fn putc(&mut self, c: char) {
        let (_, curcol) = self.curpos();
        if curcol >= self.col - 1 {
            eprintln!("Line overflow");
            print!("{} \r", c);
        } else {
            print!("{}", c);
        }
        self.chars.push(c);
        let _ = std::io::stdout().flush();
    }
    fn push(&mut self, c: char) -> Result<(), ()> {
        match c {
            '?' => {
                // HELP
                eprintln!("HELP");
                println!("");
                println!("HELP");
                self.prompt();
                print!("{}", String::from_iter(&self.chars));
                let _ = std::io::stdout().flush();
                Ok(())
            }
            '\u{4}' => {
                // EOT(Ctrl-D)
                eprintln!("EOT(Ctrl-D)");
                println!("");
                Err(())
            }
            '\u{8}' | '\u{7f}' => {
                // BS | DEL
                eprintln!("BS | DEL");
                self.delc();
                Ok(())
            }
            '\n' => {
                // LF
                eprintln!("LF");
                println!("");
                println!("INPUT: {}", String::from_iter(&self.chars));
                self.chars.clear();
                self.prompt();
                Ok(())
            }
            _ => {
                self.putc(c);
                Ok(())
            }
        }
    }
}

static CTX: std::sync::Mutex<Option<Context>> = std::sync::Mutex::new(None);

extern "C" fn winch_handler(_: libc::c_int, _: *mut libc::siginfo_t, _: *mut libc::c_void) {
    if let Some(ref mut ctx) = *CTX.lock().unwrap() {
        if let Ok((_, col)) = gwinsz() {
            ctx.col = col;
        }
    }
}

fn main() {
    let termios_orig = match term_setup() {
        Ok(termios_orig) => termios_orig,
        Err(_) => return,
    };
    *CTX.lock().unwrap() = Some(Context::new());
    unsafe {
        let winch_action = signal::SigAction::new(
            signal::SigHandler::SigAction(winch_handler),
            signal::SaFlags::empty(),
            signal::SigSet::empty(),
        );
        let _ = signal::sigaction(signal::SIGWINCH, &winch_action);
    }
    if let Some(ref mut ctx) = *CTX.lock().unwrap() {
        ctx.prompt();
    }
    let mut buf = [0u8; 1];
    loop {
        match std::io::stdin().read(&mut buf) {
            Ok(_) => {
                let c = char::from(buf[0]);
                eprintln!("read: {:?}", c);
                if let Some(ref mut ctx) = *CTX.lock().unwrap() {
                    if let Err(_) = ctx.push(c) {
                        break;
                    }
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(_) => break,
        }
    }
    term_restore(termios_orig);
}
