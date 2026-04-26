use crate::api::{Flight, Location};
use ratatui::widgets::TableState;

pub struct App {
    pub location: Option<Location>,
    pub flights: Vec<Flight>,
    pub table_state: TableState,
    pub last_update: chrono::DateTime<chrono::Local>,
}

impl App {
    pub fn new() -> Self {
        Self {
            location: None,
            flights: Vec::new(),
            table_state: TableState::default(),
            last_update: chrono::Local::now(),
        }
    }

    pub fn next(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.flights.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.flights.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }
}
