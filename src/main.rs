use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    widgets::{Block, Paragraph},
    DefaultTerminal,
};
use std::{
    env::args,
    io, path,
    time::{Duration, Instant},
};

mod chip8;

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run <ROM file>");
        return Ok(());
    }
    let path = &args[1];

    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(terminal, path);
    ratatui::restore();
    app_result
}

fn run(mut terminal: DefaultTerminal, path: &str) -> io::Result<()> {
    let mut chip8 = chip8::Chip8::new();

    let rom = read_rom(path);
    chip8.load_rom(&rom);

    let cycle_rate = Duration::from_micros(2000);
    let refresh_rate = Duration::from_millis(1000 / 60);
    let mut last_cycle = Instant::now();
    let mut last_refresh = Instant::now();

    // main loop
    loop {
        if last_cycle.elapsed() >= cycle_rate {
            chip8.run_cycle();
            last_cycle = Instant::now();
        }

        if chip8.is_drawing && last_refresh.elapsed() >= refresh_rate {
            let display_data = chip8.get_display_data();
            update_display(&mut terminal, &display_data).unwrap();
            chip8.is_drawing = false;
            last_refresh = Instant::now();
        }

        if event::poll(Duration::from_millis(1))? {
            if let Event::Key(key) = event::read()? {
                match key.kind {
                    KeyEventKind::Press => {
                        if key.code == KeyCode::Esc {
                            return Ok(());
                        }
                        if let Some(key) = key_map(key.code) {
                            chip8.set_key(key);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn read_rom(path: &str) -> Vec<u8> {
    let path = path::Path::new(path);
    let rom = std::fs::read(path).expect("Failed to read ROM file");
    rom
}

fn update_display(terminal: &mut DefaultTerminal, display_data: &[bool]) -> io::Result<()> {
    terminal.draw(|frame| {
        let width = chip8::DISPLAY_WIDTH;
        let height = chip8::DISPLAY_HEIGHT;
        let mut text = String::new();
        for y in 0..height {
            for x in 0..width {
                let index = y * width + x;
                let pixel = display_data[index];
                text.push_str(if pixel { "█" } else { " " });
            }
            text.push_str("\n");
        }
        let block = Paragraph::new(text).block(
            Block::default()
                .title("============= CHIP-8 Emulator (Press ESC to Exit) ==============="),
        );
        frame.render_widget(block, frame.area());
    })?;
    Ok(())
}

fn key_map(key: KeyCode) -> Option<u8> {
    match key {
        KeyCode::Char('1') => Some(0x1),
        KeyCode::Char('2') => Some(0x2),
        KeyCode::Char('3') => Some(0x3),
        KeyCode::Char('4') => Some(0xC),
        KeyCode::Char('q') => Some(0x4),
        KeyCode::Char('w') => Some(0x5),
        KeyCode::Char('e') => Some(0x6),
        KeyCode::Char('r') => Some(0xD),
        KeyCode::Char('a') => Some(0x7),
        KeyCode::Char('s') => Some(0x8),
        KeyCode::Char('d') => Some(0x9),
        KeyCode::Char('f') => Some(0xE),
        KeyCode::Char('z') => Some(0xA),
        KeyCode::Char('x') => Some(0x0),
        KeyCode::Char('c') => Some(0xB),
        KeyCode::Char('v') => Some(0xF),
        _ => None,
    }
}
