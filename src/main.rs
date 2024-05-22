use std::io::Read;

fn prompt() {
    print!("> ");
}

fn main() {
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
                        chars.push(c);
                    }
                }
            }
            Err(_) => break,
        }
    }
}
