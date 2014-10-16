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

use rsfml::window::{ContextSettings, VideoMode, Close};
use rsfml::graphics::{RenderWindow};
use rfmod::*;
use rfmod::types::*;
use playlist::PlayList;
use graphic_handler::GraphicHandler;
use std::os;

mod graphic_handler;
mod playlist;
mod graphic_playlist;
mod graphic_timer;
mod progress_bar;
mod graphic_spectrum;
mod graphic_button;
mod graphic_sound_position;
mod graphic_element;

fn main() {
    let args = os::args().tail().to_vec();

    if args.len() < 1 {
        println!("USAGE: music_player [music_files ...]");
        println!("For more information: music_player -h");
        println!("Or more information: music_player --help");
        return;
    } else if args.len() == 1 && (args[0].as_slice() == "-h" || args[0].as_slice() == "--help") {
        println!("usage: music_player [music_files ...]");
        println!("Here is the list of the binded keyboards keys:");
        println!("* ESC : exit the program");
        println!("* Up / Down : change the music");
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

    let fmod = match FmodSys::new() {
        Ok(f) => {
            f.init_with_parameters(10i32, FmodInitFlag(enums::FMOD_INIT_NORMAL));
            f
        },
        Err(e) => fail!("FmodSys.new : {}", e)
    };
    let mut window = match RenderWindow::new(VideoMode::new_init(800, 600, 32), 
                                             "Music Player", 
                                             Close, 
                                             &ContextSettings::default()) {
        Some(window) => window,
        None => fail!("Cannot create a new Render Window.")
    };

    
    let mut graph = GraphicHandler::new(&window, PlayList::from_vec(&args));
    window.set_vertical_sync_enabled(true);
    window.set_framerate_limit(30u);
    graph.start(&mut window, &fmod);
}