# Flight-Vis

A TUI-based flight visualizer written in Rust. It shows live flights in your area using data from the OpenSky Network.

## Features
- Automatic geolocation based on your IP.
- Live flight data (altitude, velocity, track, vertical rate).
- Command-line arguments for custom location and radius.
- Built with Nix for reproducible development.

## Prerequisites
- Rust (stable)
- Nix (optional, for development environment)

## Usage

### Using Nix
```bash
nix develop
cargo run
```

### Using Cargo
```bash
cargo run -- --radius 1.5
```

### Arguments
- `-l`, `--latitude`: Custom latitude.
- `-o`, `--longitude`: Custom longitude.
- `-r`, `--radius`: Search radius in degrees (default: 1.0).

### Keybindings
- `q`: Quit.
- `r`: Force refresh.
- `Up/Down`: Select flight (visual feedback only).

## Data Sources
- Flight Data: [OpenSky Network API](https://opensky-network.org/)
- Geolocation: [ipapi.co](https://ipapi.co/)

## License
MIT
