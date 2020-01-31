use feox::emulator::Emulator;
use feox::cpu::Cpu;
use feox::gui::Gui;
use feox::joypad::Button;

use std::env;
use std::fs::File;

use sdl2::keyboard::Keycode;
use sdl2::event::Event;


// TODO: clean this thing up
fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: feox [bootrom] [rom]");
        std::process::exit(-1);
    };
    let mut bootrom = File::open(&args[1])
        .expect(&format!("expected to find '{}'", args[1]));
    let mut rom = File::open(&args[2])
        .expect(&format!("expected to find '{}'", args[2]));

    let mut emulator = Emulator::new();
    emulator.load_bootrom(&mut bootrom).expect("failed to read bootrom");
    emulator.load_rom(&mut rom).expect("failed to read rom");

    let mut cpu = Cpu::new();

    let mut gui = Gui::new(120)?;
    let mut event_pump = gui.context.event_pump()
        .map_err(|e| e.to_string())?;
    'running: loop {
        let mut cycles = 0;
        while cycles < 17556 {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                    Event::KeyDown { keycode: Some(key), .. } => {
                        if let Some(button) = map_keycode_to_joypad(key) {
                            emulator.joypad_press(button);
                        }
                    }
                    Event::KeyUp { keycode: Some(key), .. } => {
                        if let Some(button) = map_keycode_to_joypad(key) {
                            emulator.joypad_clear(button);
                        }
                    }
                    _ => (),
                }
            }

            cpu.step(&mut emulator);
            cycles += emulator.catch_up_cycles();
        }

        gui.update_screen(&emulator.ppu.framebuffer())?;
        gui.delay();
    }
    Ok(())
}
 
fn map_keycode_to_joypad(keycode: Keycode) -> Option<Button> {
    match keycode {
        Keycode::Up => Some(Button::Up),
        Keycode::Down => Some(Button::Down),
        Keycode::Left => Some(Button::Left),
        Keycode::Right => Some(Button::Right),
        Keycode::A => Some(Button::A),
        Keycode::B => Some(Button::B),
        Keycode::Space => Some(Button::Select),
        Keycode::Return => Some(Button::Start),
        _ => None,
    }
}
