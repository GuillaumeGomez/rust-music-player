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

use rsfml::graphics::rc;
use rsfml::system::vector2::{Vector2f, Vector2u};
use rsfml::window::{ContextSettings, VideoMode, event, Close, keyboard};
use rsfml::graphics::{RenderWindow, Color, Text, Font, RectangleShape};
use rfmod::enums::*;
use rfmod::*;
use playlist::PlayList;
use std::rc::Rc;
use std::cell::RefCell;

struct ProgressBar {
    line: rc::RectangleShape,
    progress_size: Vector2u,
    maximum: uint,
    value: uint,
    //callback: fn(uint, uint) -> ()
}

impl ProgressBar {
    fn new(size: &Vector2u, position: &Vector2u, color: &Color, maximum: uint) -> ProgressBar {
        ProgressBar {
            line: match rc::RectangleShape::new_init(&Vector2f{x: size.x as f32, y: size.y as f32}) {
                Some(l) => l,
                None => fail!("Cannot create progress bar")
            },
            progress_size: size.clone(),
            maximum: maximum,
            value: 0u
        }.init(color, position)
    }

    fn init(mut self, color: &Color, position: &Vector2u) -> ProgressBar {
        self.line.set_position(&Vector2f{x: position.x as f32, y: position.y as f32});
        self.line.set_fill_color(color);
        self
    }

    fn draw(&self, win: &mut RenderWindow) {
        win.draw(&self.line);
    }

    fn set_position(&mut self, position: uint) {
        let new_value = position as f32 / self.maximum as f32 * self.progress_size.x as f32;

        if new_value != self.value as f32 {
            self.value = new_value as uint;
            self.line.set_size(&Vector2f{x: self.value as f32, y: self.progress_size.y as f32});
        }
    }
}

pub struct GraphicHandler {
    font: Font,
    pub text: rc::Text,
    pub timer: rc::Text,
    music_bar: ProgressBar,
    volume_bar: ProgressBar,
    playlist: PlayList,
}

impl GraphicHandler {
    fn init(mut self) -> GraphicHandler {
        self.timer.set_position(&Vector2f{x: 0f32, y: 25f32});
        self
    }

    pub fn new(window: &RenderWindow, playlist: PlayList) -> GraphicHandler {
        let font = match Font::new_from_file("./font/arial.ttf") {
            Some(s) => s,
            None => fail!("Cannot create Font")
        };
        GraphicHandler {
            font: font.clone(),
            text: match rc::Text::new_init("", Rc::new(RefCell::new(font.clone())), 20) {
                Some(t) => t,
                None => fail!("Cannot create Text")
            },
            timer: match rc::Text::new_init("", Rc::new(RefCell::new(font)), 20) {
                Some(t) => t,
                None => fail!("Cannot create Text")
            },
            music_bar: ProgressBar::new(&Vector2u{x: window.get_size().x, y: 8u32}, &Vector2u{x: 0u32, y: window.get_size().y - 8u32},
                &Color::new_RGB(255, 255, 255), 1u),
            volume_bar: ProgressBar::new(&Vector2u{x: window.get_size().x / 5, y: 10u32},
                &Vector2u{x: window.get_size().x - window.get_size().x / 5, y: window.get_size().y - 19u32},
                &Color::new_RGB(255, 25, 25), 100u),
            playlist: playlist
        }.init()
    }

    pub fn set_music(&mut self, fmod: &FmodSys, name: String) -> Result<Sound, String> {
        let sound = match fmod.create_sound(name.clone(), None, None) {
            Ok(s) => s,
            Err(err) => {
                println!("FmodSys.create_sound failed on this file : {}\nError : {}", name, err);
                self.playlist.remove_current();
                if self.playlist.get_nb_musics() == 0 {
                    return Err(String::from_str("No more music"));
                } else {
                    let tmp_s = self.playlist.get_current();
                    return self.set_music(fmod, tmp_s);
                }
            }
        };
        self.text.set_string(name.as_slice());
        self.music_bar.maximum = sound.get_length(FMOD_TIMEUNIT_MS).unwrap() as uint;
        Ok(sound)
    }

    pub fn set_music_position(&mut self, position: uint) {
        self.music_bar.set_position(position);
    }

    pub fn update(&mut self, win: &mut RenderWindow) {
        win.clear(&Color::new_RGB(0, 0, 0));
        win.draw(&self.text);
        win.draw(&self.timer);
        self.music_bar.draw(win);
        self.volume_bar.draw(win);
        win.display();
    }

    fn main_loop(&mut self, chan: &Channel, old_position: uint, length: u32) -> Option<uint> {
        match chan.is_playing() {
            Ok(b) => {
                if b == true {
                    let position = chan.get_position(FMOD_TIMEUNIT_MS).unwrap();

                    if position != old_position {
                        self.timer.set_string(format!("{:02u}:{:02u} / {:02u}:{:02u}",
                            position / 1000 / 60, position / 1000 % 60, length / 1000 / 60, length / 1000 % 60).as_slice());
                        Some(position)
                    } else {
                        Some(old_position)
                    }
                } else {
                    None
                }
            }
            Err(e) => fail!("fmod error : {}", e)
        }
    }

    pub fn start(&mut self, window: &mut RenderWindow, fmod: &FmodSys) {
        let mut old_position = 100u;
        let mut tmp_s = self.playlist.get_current();
        let mut sound = match self.set_music(fmod, tmp_s) {
            Ok(s) => s,
            Err(e) => fail!("Error : {}", e)
        };
        let mut chan = match sound.play() {
            Ok(c) => c,
            Err(e) => fail!("sound.play : {}", e)
        };
        let length = self.music_bar.maximum as u32;

        while window.is_open() {
            loop {
                match window.poll_event() {
                    event::Closed => window.close(),
                    event::KeyReleased{code, ..} => match code {
                        keyboard::Escape => window.close(),
                        keyboard::Up => {
                            tmp_s = self.playlist.get_prev();
                            println!("new song : {}", tmp_s);
                            sound = match self.set_music(fmod, tmp_s) {
                                Ok(s) => s,
                                Err(e) => fail!("Error : {}", e)
                            };
                            chan = match sound.play() {
                                Ok(c) => c,
                                Err(e) => fail!("sound.play : {}", e)
                            };
                        }
                        keyboard::Down => {
                            tmp_s = self.playlist.get_next();
                            sound = match self.set_music(fmod, tmp_s) {
                                Ok(s) => s,
                                Err(e) => fail!("Error : {}", e)
                            };
                            chan = match sound.play() {
                                Ok(c) => c,
                                Err(e) => fail!("sound.play : {}", e)
                            };
                        }
                        _ => {}
                    },
                    event::NoEvent => break,
                    _ => {}
                }
            }

            old_position = match self.main_loop(&chan, old_position, length) {
                Some(p) => {
                    self.set_music_position(p);
                    p
                },
                None => {
                    tmp_s = self.playlist.get_next();
                    sound = match self.set_music(fmod, tmp_s) {
                        Ok(s) => s,
                        Err(e) => fail!("Error : {}", e)
                    };
                    chan = match sound.play() {
                        Ok(c) => c,
                        Err(e) => fail!("sound.play : {}", e)
                    };
                    100u
                }
            };

            self.update(window);
        }
    }
}