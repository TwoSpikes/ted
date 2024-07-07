use ::std::io::Write;

use ::termion::cursor::DetectCursorPos;
use termion::raw::IntoRawMode;

fn main() {
    let mut stdout = ::std::io::stdout().into_raw_mode().expect("Cannot get stdout in raw mode");
    _ = ::crossterm::terminal::enable_raw_mode();
    let ::termsize::Size{rows, cols: _} = ::termsize::get().expect("Cannot get terminal size");
    print!("{}",
        ::termion::clear::All
    );
    print!("{}NORMAL{}",
        ::termion::cursor::Goto(0, rows),
        ::termion::cursor::Goto(0, 1),
    );
    _ = stdout.flush();
    loop {
        let ch = ::crossterm::event::read().expect("Cannot read key from terminal");
        if let ::crossterm::event::Event::Key(::crossterm::event::KeyEvent {
            code,
            ..
        }) = ch {
            if code == ::crossterm::event::KeyCode::Char('q') {
                break;
            }
            match code {
                ::crossterm::event::KeyCode::Backspace => {
                    let (x, y) = stdout.cursor_pos().expect("Cannot get cursor position");
                    print!("{} {}",
                        ::termion::cursor::Goto(x-1, y),
                        ::termion::cursor::Goto(x-1, y),
                    );
                    _ = stdout.flush();
                },
                ::crossterm::event::KeyCode::Char(c) => {
                    print!("{}", c);
                    _ = stdout.flush();
                },
                _ => todo!(),
            }
        }
    }
}
