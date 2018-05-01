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
#![feature(nll)]
extern crate num;
extern crate rfmod;
extern crate sfml;

use graphic_handler::GraphicHandler;
use playlist::PlayList;
use rfmod::*;
use sfml::graphics::Font;
use sfml::graphics::RenderWindow;
use sfml::window::{ContextSettings, Style, VideoMode};
use std::env;
mod graphic_button;
mod graphic_element;
mod graphic_handler;
mod graphic_playlist;
mod graphic_sound_position;
mod graphic_spectrum;
mod graphic_timer;
mod playlist;
mod progress_bar;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("USAGE: music_player [music_files ...]");
        println!("For more information: music_player -h");
        println!("Or                  : music_player --help");
        return;
    } else if args.len() == 2 && (args[1] == "-h" || args[1] == "--help") {
        println!("usage: music_player [music_files ...]");
        println!("Here is the list of the binded keyboards keys:");
        println!("* ESC : exit the program");
        println!("* Up / Down : change the music");
        println!("* R : turn on/off song repeat");
        println!("* Add / Subtract : change the music volume");
        println!("* Space : pause / unpause current music");
        println!("* BackSpace : reset user position (in 3D)");
        println!("* Delete : remove the current music\n");
        println!("You can also interact with the software like this :");
        println!("* you can scroll the playlist");
        println!("* you can click on a music to play it");
        println!("* you can click on the music progress bar to go to precise position");
        println!("* you can click on the volume progress bar to change the music's volume");
        println!("* you can click to change your 3D position");
        return;
    }

    let fmod = match Sys::new() {
        Ok(f) => {
            f.init_with_parameters(10i32, InitFlag(rfmod::INIT_NORMAL));
            f
        }
        Err(e) => panic!("FmodSys.new : {:?}", e),
    };
    let mut window = RenderWindow::new(
        VideoMode::new(800, 600, 32),
        "Music Player",
        Style::CLOSE,
        &ContextSettings::default(),
    );
    let font = Font::from_file("font/arial.ttf").unwrap();
    let mut graph = GraphicHandler::new(&window, PlayList::from_slice(&args[1..]), &font);
    window.set_vertical_sync_enabled(true);
    window.set_framerate_limit(30u32);
    graph.start(&mut window, &fmod);
}
