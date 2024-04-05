






use std::fmt::Display;
use std::io::{self, Stdout};
use std::io::Write;
use std::thread;
use std::time::{Duration, Instant};

use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::cursor;
use termion::cursor::Goto;
use termion::color::*;





fn main() -> io::Result<()> {
    let text = "The quick brown fox jumps over the lazy dog.";
    let mut out = io::stdout()
        .into_raw_mode()
        .expect("Failed to get raw mode");

    write!(out, "{}{}Press Enter to start...", Goto(1, 1), termion::clear::All)?;
    out.flush()?;

    
    let mut keys = io::stdin().keys();
    let mut validations = vec![];
    let mut time = Instant::now();
    let mut started = false;
    let mut wpm = 0;

    while let Some(Ok(key)) = keys.next() {
        match key {
            Key::Ctrl('c') => break,
            Key::Char('\n') if !started => {
                start(&mut out, &text)?;
                time = Instant::now();
                started = true;
            },

            Key::Char(c) if started => {
                let charat = text
                    .chars()
                    .nth(validations.len())
                    .unwrap_or(' ');

                let validation = if c == charat {
                    Validation::Correct
                } else {
                    Validation::Incorrect
                };

                validations.push(validation.clone());
                write!(out, "{}{}", 
                    validation,
                    if c == ' ' {
                        '·' 
                    } else { c }
                )?;

                if validations.len() == text.len() {
                    break;
                }
            },
            
            Key::Backspace if started => {
                validations.pop();
                write!(out, "{}{} {}", 
                    Fg(Reset),
                    termion::cursor::Left(1),
                    termion::cursor::Left(1)
                )?;
            },


            _ => (),
        }

        wpm = calc_wpm(&validations, time.elapsed());
        write!(out, "{}{}{}{}{}WPM: {wpm}{}{}",
            cursor::Save,
            cursor::Hide,
            Goto(1, 3),
            Fg(Reset),
            termion::clear::AfterCursor,
            cursor::Restore,
            cursor::Show,
        )?;
        out.flush()?;
    }

    write!(out, "{}{}{}WPM: {wpm}{}",
        Goto(1, 3),
        Fg(Reset),
        termion::clear::AfterCursor,
        Goto(1, 4),
    )?;
    out.flush()?;

    Ok(())
}

fn start(out: &mut RawTerminal<Stdout>, text: &str) -> io::Result<()> {
    write!(out,
        "{}{text}{}",
        Goto(1, 1),
        Goto(1, 2),
    )?;
    out.flush()?;

    for i in 1..=3 {
        write!(out, "{}Start in: {}{}", 
            termion::clear::CurrentLine,
            4 - i,
            Goto(1, 2)
        )?;
        out.flush()?;
        thread::sleep(Duration::from_secs(1));
    }

    write!(out, "{}", termion::clear::CurrentLine)?;
    out.flush()?;
    Ok(())
}



fn calc_wpm(
    validations: &Vec<Validation>,
    time: Duration,
) -> usize {
    let time = time.as_secs_f64() / 60.0;
    let correct = validations
        .iter()
        .filter(|v| **v == Validation::Correct)
        .count();   


    return ((correct as f64 / 5f64) / time + 0.5f64) as usize
}



#[derive(Debug, Clone, PartialEq)]
pub enum Validation {
    Correct,
    Incorrect,
}

impl Display for Validation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Validation::Correct => write!(f, "{}", Fg(Green)),
            Validation::Incorrect => write!(f, "{}", Fg(Red)),
        }
    }
}









