/*
* Rust-music-player - Copyright (c) 2014 Gomez Guillaume.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

#![feature(globs)]

extern crate rsfml;
extern crate rfmod;

use rsfml::window::{ContextSettings, VideoMode, event, Close, keyboard};
use rsfml::graphics::{RenderWindow, Color, Text, Font, RectangleShape};
use rsfml::system::vector2::{Vector2f};
use rfmod::enums::*;
use rfmod::*;
use std::io::timer::sleep;
use graphic_handler::GraphicHandler;

mod graphic_handler;

fn main_loop(chan: &Channel, graph: &mut GraphicHandler, old_position: uint, length: u32) -> uint {
    match chan.is_playing() {
        Ok(b) => {
            if b == true {
                let position = chan.get_position(FMOD_TIMEUNIT_MS).unwrap();

                if position != old_position {
                    graph.timer.set_string(format!("{:02u}:{:02u} / {:02u}:{:02u}",
                        position / 1000 / 60, position / 1000 % 60, length / 1000 / 60, length / 1000 % 60).as_slice());
                    position
                } else {
                    old_position
                }
            } else {
                old_position
            }
        }
        Err(e) => fail!("fmod error : {}", e)
    }
}

fn main() {
    let fmod = match FmodSys::new() {
        Ok(f) => {
            f.init();
            f
        },
        Err(e) => fail!("FmodSys.new : {}", e)
    };
    let mut window = match RenderWindow::new(VideoMode::new_init(800, 600, 32), 
                                             "SFML Example", 
                                             Close, 
                                             &ContextSettings::default()) {
        Some(window) => window,
        None => fail!("Cannot create a new Render Window.")
    };
    let sound = match fmod.create_sound(String::from_str("/windows/Users/User/Music/Daddy DJ-Daddy DJ.mp3"), None, None) {
        Ok(s) => s,
        Err(err) => fail!("FmodSys.create_sound failed : {}", err),
    };
    let mut graph = GraphicHandler::new(&window);
    let mut old_position = 100u;
    let chan = match sound.play() {
        Ok(c) => c,
        Err(e) => fail!("sound.play : {}", e)
    };
    let length = sound.get_length(FMOD_TIMEUNIT_MS).unwrap();

    graph.set_music(sound.get_name(100u32).unwrap(), sound.get_length(FMOD_TIMEUNIT_MS).unwrap() as uint);
    window.set_vertical_sync_enabled(true);

    while window.is_open() {
        loop {
            match window.poll_event() {
                event::Closed => window.close(),
                event::KeyPressed{code, ..} => match code {
                    keyboard::Escape => window.close(),
                    _ => {}
                },
                event::NoEvent => break,
                _ => {}
            }
        }

        old_position = main_loop(&chan, &mut graph, old_position, length);
        graph.set_music_position(old_position);

        graph.update(&mut window);
    }
}