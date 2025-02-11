

use ratatui::{
    widgets::*,
    layout::{Layout, Direction, Constraint},
    Frame
};

pub fn build_layouts(no_cores: i32, frame: &mut Frame) {
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

    render_cpu_section(no_cores, frame, &outer_layout);
    render_memory_section(frame, &outer_layout);
    render_processes(frame, &outer_layout);
    }
    
fn render_cpu_section(no_cores: i32, frame: &mut Frame<'_>, outer_layout: &std::rc::Rc<[ratatui::prelude::Rect]>) {
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

fn render_memory_section(frame: &mut Frame , outer_layout: &std::rc::Rc<[ratatui::prelude::Rect]> ){
}

fn render_processes(frame: &mut Frame, outer_layout:  &std::rc::Rc<[ratatui::prelude::Rect]>){
}