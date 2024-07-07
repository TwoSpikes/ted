use std::io::Write;

fn main() {
    let term = ::console::Term::stdout();
    let mut stdout = ::std::io::stdout();
    loop {
        let ch = term.read_char().expect("Cannot read char");
        if ch == 'q' {
            break;
        }
        print!("{}", ch);
        _ = stdout.flush();
    }
}
