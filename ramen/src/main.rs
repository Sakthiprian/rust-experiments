mod ui;
use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use sysinfo::System;
use ratatui::{
    DefaultTerminal, Frame
};


#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    no_cores: i32, // Number of CPU cores
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        ui::build_layouts(self.no_cores, frame);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                if let KeyCode::Char('q') = key_event.code {
                    self.exit = true;
                }
            }
            _ => {}
        };
        Ok(())
    }
}


fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let system = System::new_all();
    let no_cores = system.cpus().len() as i32; // Get number of CPU cores dynamically

    println!("{}", no_cores);
    let app_result = App {
        no_cores,
        ..Default::default()
    }
    .run(&mut terminal);

    ratatui::restore();
    app_result
}
