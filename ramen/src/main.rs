use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use sysinfo::System;
use ratatui::{
    widgets::*,
    layout::{Layout, Direction, Constraint},
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
        build_layouts(self.no_cores, frame);
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

fn build_layouts(no_cores: i32, frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(25), // CPU Section
            Constraint::Percentage(25), // Memory Section
            Constraint::Percentage(50), // Process List
        ])
        .split(frame.area());

    frame.render_widget(Block::default().title("CPU").borders(Borders::ALL), outer_layout[0]);
    frame.render_widget(Block::default().title("Memory").borders(Borders::ALL), outer_layout[1]);
    frame.render_widget(Block::default().title("Processes").borders(Borders::ALL), outer_layout[2]);

    let cpu_section = outer_layout[0];

    // **Split CPU section into 2 rows**
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(cpu_section);

    let cores_per_row = (no_cores as f32 / 2.0).ceil() as usize;

    // **Split each row horizontally**
    let row_1_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(100 / cores_per_row as u16); cores_per_row])
        .split(vertical_layout[0]);

    let row_2_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(100 / cores_per_row as u16); cores_per_row])
        .split(vertical_layout[1]);

    // **Render cores for Row 1**
    for (i, area) in row_1_layout.iter().enumerate() {
        frame.render_widget(
            Block::default().title(format!("Core {}", i + 1)).borders(Borders::ALL),
            *area,
        );
    }

    // **Render cores for Row 2**
    for (i, area) in row_2_layout.iter().enumerate() {
        let core_index = i + cores_per_row;
        if core_index < no_cores as usize {
            frame.render_widget(
                Block::default().title(format!("Core {}", core_index + 1)).borders(Borders::ALL),
                *area,
            );
        }
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
