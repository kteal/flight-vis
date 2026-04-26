mod api;
mod app;
mod ui;

use crate::app::App;
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io,
    time::{Duration, Instant},
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'l', long)]
    latitude: Option<f64>,
    #[arg(short = 'o', long)]
    longitude: Option<f64>,
    #[arg(short, long, default_value_t = 1.0)]
    radius: f64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app and ensure we cleanup terminal even on error
    let res = run(&mut terminal, args).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{:?}", err);
    }

    Ok(())
}

async fn run<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, args: Args) -> Result<()> {
    let mut app = App::new();

    // Initial data fetch
    if let (Some(lat), Some(lon)) = (args.latitude, args.longitude) {
        app.location = Some(api::Location {
            latitude: lat,
            longitude: lon,
            city: "Custom".to_string(),
            country: "User Specified".to_string(),
        });
    } else {
        if let Ok(loc) = api::get_current_location().await {
            app.location = Some(loc);
        }
    }

    if let Some(loc) = &app.location {
        if let Ok(flights) = api::get_flights(loc.latitude, loc.longitude, args.radius).await {
            app.flights = flights;
            app.last_update = chrono::Local::now();
        }
    }

    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();
    let mut last_fetch = Instant::now();
    let fetch_interval = Duration::from_secs(1);

    loop {
        terminal.draw(|f| ui::render(&mut app, f))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_default();

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('r') => {
                        if let Some(loc) = &app.location {
                            if let Ok(flights) =
                                api::get_flights(loc.latitude, loc.longitude, args.radius).await
                            {
                                app.flights = flights;
                                app.last_update = chrono::Local::now();
                            }
                        }
                    }
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }

        if last_fetch.elapsed() >= fetch_interval {
            if let Some(loc) = &app.location {
                if let Ok(flights) =
                    api::get_flights(loc.latitude, loc.longitude, args.radius).await
                {
                    app.flights = flights;
                    app.last_update = chrono::Local::now();
                }
            }
            last_fetch = Instant::now();
        }
    }
}
