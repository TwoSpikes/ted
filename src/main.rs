use ::std::io::Write;

use ::termion::cursor::DetectCursorPos;
use termion::raw::IntoRawMode;

enum Mode {
    NORMAL,
    INSERT,
}
impl ToString for Mode {
    fn to_string(&self) -> String {
        String::from(match self {
            Mode::NORMAL => "NORMAL",
            Mode::INSERT => "INSERT",
        })
    }
}

fn main() {
    let mut mode = Mode::NORMAL;

    let mut stdout = ::std::io::stdout().into_raw_mode().expect("Cannot get stdout in raw mode");
    _ = ::crossterm::terminal::enable_raw_mode();
    let ::termsize::Size{rows, cols: _} = ::termsize::get().expect("Cannot get terminal size");
    print!("{}",
        ::termion::clear::All
    );
    print!("{}{}{}",
        ::termion::cursor::Goto(0, rows),
        mode.to_string(),
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
            match mode {
                Mode::NORMAL => {
                    match code {
                        ::crossterm::event::KeyCode::Char('i') => {
                            let (x, y) = stdout.cursor_pos().expect("Cannot get cursor position");
                            mode = Mode::INSERT;
                            print!("{}{}{}",
                                ::termion::cursor::Goto(0, rows),
                                mode.to_string(),
                                ::termion::cursor::Goto(x, y),
                            );
                            _ = stdout.flush();
                        },
                        _ => {

                        },
                    }
                },
                Mode::INSERT => {
                    match code {
                        ::crossterm::event::KeyCode::Esc => {
                            let (x, y) = stdout.cursor_pos().expect("Cannot get cursor position");
                            mode = Mode::NORMAL;
                            print!("{}{}{}",
                                ::termion::cursor::Goto(0, rows),
                                mode.to_string(),
                                ::termion::cursor::Goto(x, y),
                            );
                            _ = stdout.flush();
                        },
                        ::crossterm::event::KeyCode::Backspace => {
                            let (x, y) = stdout.cursor_pos().expect("Cannot get cursor position");
                            print!("{} {}",
                                ::termion::cursor::Goto(x-1, y),
                                ::termion::cursor::Goto(x-1, y),
                            );
                            _ = stdout.flush();
                        },
                        ::crossterm::event::KeyCode::Enter => {
                            let (_, y) = stdout.cursor_pos().expect("Cannot get cursor position");
                            println!("{}",
                                ::termion::cursor::Goto(0, y)
                            );
                        },
                        ::crossterm::event::KeyCode::Char(c) => {
                            print!("{}", c);
                            _ = stdout.flush();
                        },
                        _ => todo!(),
                    }
                },
            }
        }
    }
}
