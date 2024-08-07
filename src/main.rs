use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::convert::Infallible;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

struct Directions {
    drive: i8,
    turn: i8,
}

pub fn main() {
    let mut current_directions = Directions {
        drive: 0,
        turn: 0,
    };

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("piRoverController", 800, 800)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.clear();
        for event in event_pump.poll_iter() {
            let res: Result<(), _> = match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    current_directions.drive += 1;
                    handle_request(format!("DRIVE:{}", current_directions.drive).as_str());
                    Ok::<(), Infallible>(())
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    current_directions.drive -= 1;
                    handle_request(format!("DRIVE:{}", current_directions.drive).as_str());
                    Ok::<(), Infallible>(())
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    current_directions.turn -= 1;
                    handle_request(format!("TURN:{}", current_directions.turn).as_str());
                    Ok::<(), Infallible>(())
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    current_directions.turn += 1;
                    handle_request(format!("TURN:{}", current_directions.turn).as_str());
                    Ok::<(), Infallible>(())
                },
                _ => {
                    Ok::<(), Infallible>(())
                }
            };
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}

fn handle_request(message: &str) -> io::Result<()> {
    let address = "10.0.0.129:9999"; // Replace with your IP address and port

    let mut stream = TcpStream::connect(address)?;

    println!("Connected to {}", address);

    // Write the message to the stream
    stream.write_all(message.as_bytes())?;

    // Read response from server
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer)?;

    // Print the received response
    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}
