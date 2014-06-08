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
use playlist::PlayList;
use graphic_handler::GraphicHandler;
use std::os;

mod graphic_handler;
mod playlist;
mod graphic_playlist;
mod graphic_timer;
mod progress_bar;
mod graphic_spectrum;

fn main() {
    let args = Vec::from_slice(os::args().tail());

    if args.len() < 1 {
        fail!("USAGE: ./music_player [music_file1 music_file2 ...]");
    }

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

    
    let mut graph = GraphicHandler::new(&window, PlayList::from_vec(&args));

    window.set_vertical_sync_enabled(true);
    window.set_framerate_limit(30u);
    graph.start(&mut window, &fmod);
}