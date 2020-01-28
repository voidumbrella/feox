use feox::bus::Bus;
use feox::cpu::Cpu;

use std::env;
use std::fs::File;

use sdl2;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

#[allow(dead_code)]
const COLOR1: Color = Color::RGB(0x08, 0x18, 0x20);
#[allow(dead_code)]
const COLOR2: Color = Color::RGB(0x34, 0x68, 0x56);
#[allow(dead_code)]
const COLOR3: Color = Color::RGB(0x88, 0xC0, 0x70);
#[allow(dead_code)]
const COLOR4: Color = Color::RGB(0xE0, 0xF8, 0xD0);

// TODO: clean this thing up
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: feox [bootrom] [rom]");
        std::process::exit(-1);
    };
    let mut bootrom = File::open(&args[1])
        .expect(&format!("expected to find '{}'", args[1]));
    let mut rom = File::open(&args[2])
        .expect(&format!("expected to find '{}'", args[2]));

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("feox", 160, 144)
        .position_centered()
        .build()
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
     
    canvas.set_draw_color(COLOR1);
    canvas.clear();
    canvas.present();

    let mut bus = Bus::new();
    bus.load_bootrom(&mut bootrom).expect("failed to read bootrom");
    bus.load_rom(&mut rom).expect("failed to read rom");

    let mut cpu = Cpu::new();

    // const CYCLES_PER_FRAME: usize = 69905;
    let mut debugging: bool = false;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    debugging = !debugging;
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    if debugging {
                        println!("{:?}", cpu);
                        cpu.step(&mut bus);
                    }
                },
                _ => {}
            }
        }

        if !debugging {
            cpu.step(&mut bus);
            bus.catch_up_cycles();
        }
    }
}
