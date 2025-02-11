mod ui;
mod memory;
mod processes;

use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use sysinfo::{DiskUsage, Pid, System};
use ratatui::{backend::CrosstermBackend, Terminal};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    no_cores: i32,
    mem_vec: Vec<u64>,
    proc_vec: Vec<(Pid, String, f32, DiskUsage)>,
    scroll_offset: usize, // Track scroll position
}

impl App {
    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        ui::build_layouts(self.no_cores, frame, &self.mem_vec, &self.proc_vec, self.scroll_offset);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Up => {
                        if self.scroll_offset > 0 {
                            self.scroll_offset -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if self.scroll_offset < self.proc_vec.len().saturating_sub(10) {
                            self.scroll_offset += 1;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let system = System::new_all();
    let no_cores = system.cpus().len() as i32; // Get number of CPU cores dynamically

    let mem_vec = memory::get_memory_info(&system);
    let proc_vec = processes::get_process_info(&system);
    let app_result = App {
        no_cores,
        mem_vec,
        proc_vec,
        ..Default::default()
    }
    .run(&mut terminal);

    ratatui::restore();
    app_result
}
