use std::{env, io::{stdout, Cursor, Result, Stdout, Write}, path::PathBuf, time::Duration};

use crossterm::{cursor, event::{self, Event, KeyCode, KeyModifiers}, terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType}, tty, ExecutableCommand, QueueableCommand};


const POLL_DURATION: Duration = Duration::from_millis(100);

fn main() -> Result<()> {
    println!("Hello, world!");

    let mut stdout = stdout();

    enable_raw_mode()?; //Remove need for enter

    let (_term_width, term_height) = terminal::size()?;

    let mut curr_dir = env::current_dir().expect("Failed To Load Current Directory");

    clear_terminal()?;
    stdout.queue(cursor::MoveTo(0, term_height))?;
    stdout.queue(cursor::EnableBlinking)?;
    stdout.queue(cursor::MoveTo(0, term_height))?;
    print!("{} $ ", curr_dir.display());
    stdout.flush()?;


    let mut buffer: String = String::new();
    loop {

        let poll_res = event::poll(POLL_DURATION);
        match poll_res {
            Ok(_ready) => {

                // Ready means enter pressed?
                // if ready {
                //     println!("Enter Pressed");
                // }
            
                if let Event::Key(key_event) = event::read()? {

                    match key_event.code {
                        KeyCode::Char(c) => {

                            let new_c = if key_event.modifiers.contains(KeyModifiers::SHIFT) {
                                c.to_ascii_uppercase()
                            } else {
                                c
                            };
                            print!("{new_c}");
                            buffer.push(new_c);
                        },
                        KeyCode::Enter => {

                            // Move cursor to start of next line
                            println!();
                            stdout.queue(cursor::MoveToColumn(0))?;

                            println!("Command = {buffer}");

                            stdout.queue(cursor::MoveToNextLine(1))?;
                            print!("{} $ ", curr_dir.display());

                            buffer.clear();

                        }
                        KeyCode::Esc => {
                            break;
                        }
                        _ => {
                            stdout.queue(cursor::MoveTo(0, term_height))?;
                            println!("Unrecognized Event = {:?}", key_event);
                        }
                        
                    }
                }
            }
            Err(e) => {
                println!("Poll Failed due to {e}");
            }
        }
        stdout.flush()?;

    }
    disable_raw_mode()?;
    clear_terminal()?;
    stdout.queue(cursor::MoveToNextLine(1))?;
    stdout.flush()?;

    Ok(())
}

fn clear_terminal() -> Result<()> {
    stdout().execute(Clear(ClearType::All))?;
    Ok(())
}