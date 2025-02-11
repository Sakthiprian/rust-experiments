use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Style, Stylize}, text::Line, widgets::*, Frame
};
use sysinfo::{DiskUsage, Pid};

pub fn build_layouts(
    no_cores: i32,
    frame: &mut Frame,
    mem_vec: &Vec<u64>,
    proc_vec: &Vec<(Pid, String, f32, DiskUsage)>,
    scroll_offset: usize, 
) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(25), // CPU Section
            Constraint::Length(6),      // Memory Section
            Constraint::Min(0),         // Process List
        ])
        .split(frame.area());

    frame.render_widget(Block::default().title("CPU").borders(Borders::ALL), outer_layout[0]);

    render_cpu_section(no_cores, frame, &outer_layout);
    render_memory_section(frame, &outer_layout, mem_vec);
    render_processes(frame, &outer_layout, proc_vec, scroll_offset); 
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

fn render_processes(
    frame: &mut Frame, 
    outer_layout: &[Rect], 
    proc_vec: &Vec<(Pid, String, f32, DiskUsage)>, 
    scroll_offset: usize
) {
    let area = outer_layout[2]; // Process list area

    let max_visible_rows = 19; // Adjust based on available height
    let total_processes = proc_vec.len();

    // Clamp scroll_offset to prevent out-of-bounds errors
    let scroll_offset = scroll_offset.min(total_processes.saturating_sub(max_visible_rows));

    // Header row
    let mut rows = vec![
        Row::new(vec!["PID", "Process Name", "CPU Usage", "Disk Usage"])
            .style(Style::new())
            .bold()
    ];

    // Render only the visible range of processes
    for (pid, name, cpu_usage, disk_usage) in proc_vec.iter().skip(scroll_offset).take(max_visible_rows) {
        rows.push(Row::new(vec![
            pid.to_string(),
            name.clone(),
            format!("{:.2}%", cpu_usage),
            format!("R: {} W: {}", disk_usage.read_bytes, disk_usage.written_bytes),
        ]));
    }

    let widths = [
        Constraint::Percentage(10), 
        Constraint::Percentage(40), 
        Constraint::Percentage(25), 
        Constraint::Percentage(25),
    ];

    let table = Table::new(rows, widths)
        .column_spacing(1)
        .block(Block::default().title("Processes").borders(Borders::ALL));

    frame.render_widget(table, area);
}


