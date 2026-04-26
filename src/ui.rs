use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.size());

    // Header
    let location_info = match &app.location {
        Some(loc) => format!(
            "Location: {}, {} ({:.2}, {:.2})",
            loc.city, loc.country, loc.latitude, loc.longitude
        ),
        None => "Fetching location...".to_string(),
    };
    let header = Paragraph::new(location_info).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Flight Visualizer"),
    );
    frame.render_widget(header, chunks[0]);

    // Flight Table
    let rows = app.flights.iter().map(|f| {
        Row::new(vec![
            Cell::from(f.icao24.clone()),
            Cell::from(f.callsign.clone().unwrap_or_else(|| "N/A".to_string())),
            Cell::from(f.origin_country.clone()),
            Cell::from(format!(
                "{:.0} ft",
                f.baro_altitude.unwrap_or(0.0) * 3.28084
            )),
            Cell::from(format!("{:.0} kt", f.velocity.unwrap_or(0.0) * 1.94384)),
            Cell::from(format!("{:.0}°", f.true_track.unwrap_or(0.0))),
            Cell::from(format!(
                "{:.0} fpm",
                f.vertical_rate.unwrap_or(0.0) * 196.85
            )),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(8),
            Constraint::Length(10),
            Constraint::Length(15),
            Constraint::Length(12),
            Constraint::Length(10),
            Constraint::Length(8),
            Constraint::Length(12),
        ],
    )
    .header(
        Row::new(vec![
            "ICAO24",
            "Callsign",
            "Country",
            "Altitude",
            "Velocity",
            "Track",
            "Vert Rate",
        ])
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
    )
    .block(Block::default().borders(Borders::ALL).title("Live Flights"))
    .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
    .highlight_symbol(">> ");

    frame.render_stateful_widget(table, chunks[1], &mut app.table_state);

    // Footer
    let footer = Paragraph::new(format!(
        "Last update: {} | 'q' to quit, 'r' to refresh",
        app.last_update.format("%H:%M:%S")
    ))
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, chunks[2]);
}
