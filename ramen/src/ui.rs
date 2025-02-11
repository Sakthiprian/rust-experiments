use ratatui::{
    widgets::*,
    text::Line,
    layout::{Layout, Direction, Constraint, Rect, Alignment},
    Frame
};

pub fn build_layouts(no_cores: i32, frame: &mut Frame, mem_vec: &Vec<u64>) {
    let outer_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(25), // CPU Section
        Constraint::Length(6),      // Memory Section (Fixed height of 4 lines)
        Constraint::Min(0),         // Process List (Takes up remaining space)
    ])
    .split(frame.area());


    frame.render_widget(Block::default().title("CPU").borders(Borders::ALL), outer_layout[0]);
    frame.render_widget(Block::default().title("Processes").borders(Borders::ALL), outer_layout[2]);

    render_cpu_section(no_cores, frame, &outer_layout);
    render_memory_section(frame, &outer_layout, mem_vec);
    render_processes(frame, &outer_layout);
}
    
fn render_cpu_section(no_cores: i32, frame: &mut Frame<'_>, outer_layout: &[Rect]) {
    let cpu_section = outer_layout[0];

    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(cpu_section);
    
    let cores_per_row = (no_cores as f32 / 2.0).ceil() as usize;
    
    let row_1_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(100 / cores_per_row as u16); cores_per_row])
        .split(vertical_layout[0]);
    
    let row_2_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(100 / cores_per_row as u16); cores_per_row])
        .split(vertical_layout[1]);
    
    for (i, area) in row_1_layout.iter().enumerate() {
        frame.render_widget(
            Block::default().title(format!("Core {}", i + 1)).borders(Borders::ALL),
            *area,
        );
    }

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

fn create_mem_lines(mem_vec: &Vec<u64>) -> Vec<Line<'static>> {
    vec![
        Line::from(format!("Total Memory: {} MB", mem_vec[0])),
        Line::from(format!("Used Memory: {} MB", mem_vec[1])),
        Line::from(format!("Total Swap: {} MB", mem_vec[2])),
        Line::from(format!("Used Swap: {} MB", mem_vec[3])),
    ]
}

fn render_memory_section(frame: &mut Frame, outer_layout: &[Rect], mem_vec: &Vec<u64>) {
    let area = outer_layout[1];

    let paragraph = Paragraph::new(create_mem_lines(mem_vec))
        .block(Block::default().title("Memory").borders(Borders::ALL))
        .alignment(Alignment::Left) // Ensures text is aligned properly
        .wrap(Wrap { trim: false }); // Enables wrapping to fit inside the area

    frame.render_widget(paragraph, area);
}

fn render_processes(_frame: &mut Frame, _outer_layout: &[Rect]) {
    // TODO: Implement process rendering
}
