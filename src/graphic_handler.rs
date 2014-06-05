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

#![allow(dead_code)]

use rsfml::graphics::rc;
use rsfml::system::vector2::{Vector2f, Vector2u};
use rsfml::window::{ContextSettings, VideoMode, event, Close, keyboard, mouse};
use rsfml::graphics::{RenderWindow, Color, Text, Font, RectangleShape};
use rfmod::enums::*;
use rfmod::*;
use playlist::PlayList;
use std::rc::Rc;
use std::cell::RefCell;

struct GraphicSpectrum {
    line: rc::RectangleShape
}

struct GraphicPlayList {
    musics: Vec<String>,
    texts: Vec<rc::Text>,
    graphic_size: Vector2u,
    position: Vector2u,
    to_draw: uint,
    current: uint
}

impl GraphicPlayList {
    fn init(mut self, font: &Font) -> GraphicPlayList {
        for tmp in self.musics.iter() {
            self.texts.push(match rc::Text::new_init(tmp.as_slice().split_terminator('/').last().unwrap(), Rc::new(RefCell::new(font.clone())), 20) {
                            Some(t) => t,
                            None => fail!("Cannot create Text")
                        });
        }
        let tmp = self.position.clone();
        self.set_position(&tmp);
        self.set_current(0u);
        self
    }

    fn new(musics: Vec<String>, font: &Font) -> GraphicPlayList {
        GraphicPlayList {
            musics: musics,
            texts: Vec::new(),
            graphic_size: Vector2u{x: 0u32, y: 0u32},
            position: Vector2u{x: 0u32, y: 0u32},
            to_draw: 0u,
            current: 1u
        }.init(font)
    }

    fn new_init(musics: Vec<String>, font: &Font, position: &Vector2u, size: &Vector2u) -> GraphicPlayList {
        GraphicPlayList {
            musics: musics,
            texts: Vec::new(),
            graphic_size: size.clone(),
            position: position.clone(),
            to_draw: 0u,
            current: 1u
        }.init(font)
    }

    fn set_position(&mut self, position: &Vector2u) {
        let mut pos = position.y;
        let limit = self.graphic_size.y + position.y;

        self.position = position.clone();
        self.to_draw = 0;
        for tmp in self.texts.mut_iter() {
            tmp.set_position(&Vector2f{x: self.position.x as f32, y: pos as f32 + self.position.y as f32});
            if pos < limit {
                self.to_draw += 1;
            }
            pos += 22u32;
        }
    }

    fn draw(&mut self, win: &mut RenderWindow) {
        let mut it = 0u;

        for tmp in self.texts.mut_iter() {
            if it == self.to_draw {
                break;
            }
            win.draw(tmp);
            it += 1;
        }
    }

    fn set_current(&mut self, current: uint) {
        if current != self.current {
            self.texts.get_mut(current).set_color(&Color::new_RGB(255, 125, 25));
            self.texts.get_mut(self.current).set_color(&Color::new_RGB(255, 2555, 255));
            self.current = current;
        }
    }

    fn remove_music(&mut self, pos: uint) {
        self.texts.remove(pos);
        let tmp = self.position;
        self.set_position(&tmp);
    }
}

struct ProgressBar {
    line: rc::RectangleShape,
    progress_size: Vector2u,
    maximum: uint,
    value: uint,
    real_value: uint
}

impl ProgressBar {
    fn new<T>(color: &Color) -> ProgressBar {
        ProgressBar {
            line: rc::RectangleShape::new().unwrap(),
            progress_size: Vector2u{x: 0, y: 0},
            maximum: 1,
            value: 0u,
            real_value: 0u
        }.init(color, &Vector2u{x: 0, y: 0})
    }

    fn new_init(size: &Vector2u, position: &Vector2u, color: &Color, maximum: uint) -> ProgressBar {
        ProgressBar {
            line: match rc::RectangleShape::new_init(&Vector2f{x: 0u as f32, y: size.y as f32}) {
                Some(l) => l,
                None => fail!("Cannot create progress bar")
            },
            progress_size: size.clone(),
            maximum: maximum,
            value: 0u,
            real_value: 0u
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

    fn set_size(&mut self, size: &Vector2u) {
        self.progress_size = size.clone();
        self.line.set_size(&Vector2f{x: 0f32, y: size.y as f32});
        self.maximum = size.x as uint;
    }

    fn set_position(&mut self, position: &Vector2u) {
        self.line.set_position(&Vector2f{x: position.x as f32, y: position.y as f32});
    }

    fn set_progress(&mut self, position: uint) {
        let new_value = position * self.progress_size.x as uint / self.maximum;

        if new_value != self.value {
            self.value = new_value;
            self.real_value = position;
            self.line.set_size(&Vector2f{x: self.value as f32, y: self.progress_size.y as f32});
        }
    }

    fn click(&mut self, pos: &Vector2u, width: u32) {
        let in_order = (pos.x as u32 - self.line.get_position().x as u32) * self.progress_size.x as u32 / width;

        self.set_progress(in_order as uint * self.maximum / self.progress_size.x as uint);
    }
}

pub struct GraphicHandler {
    font: Font,
    musics: GraphicPlayList,
    pub timer: rc::Text,
    music_bar: ProgressBar,
    volume_bar: ProgressBar,
    playlist: PlayList,
}

impl GraphicHandler {
    fn init(mut self) -> GraphicHandler {
        //self.timer.set_progress(&Vector2f{x: 0f32, y: 25f32});
        self
    }

    pub fn new(window: &RenderWindow, playlist: PlayList) -> GraphicHandler {
        let font = match Font::new_from_file("./font/arial.ttf") {
            Some(s) => s,
            None => fail!("Cannot create Font")
        };
        GraphicHandler {
            font: font.clone(),
            musics: GraphicPlayList::new_init(playlist.to_vec(), &font,
                &Vector2u{x: window.get_size().x - (window.get_size().x - 512u32), y: 0},
                &Vector2u{x: window.get_size().x - 512u32, y: window.get_size().y - 20u32}),
            timer: match rc::Text::new_init("", Rc::new(RefCell::new(font)), 20) {
                Some(t) => t,
                None => fail!("Cannot create Text")
            },
            music_bar: ProgressBar::new_init(&Vector2u{x: window.get_size().x, y: 8u32}, &Vector2u{x: 0u32, y: window.get_size().y - 8u32},
                &Color::new_RGB(255, 255, 255), 1u),
            volume_bar: ProgressBar::new_init(&Vector2u{x: window.get_size().x / 5, y: 10u32},
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
                self.musics.remove_music(self.playlist.get_pos());
                self.playlist.remove_current();
                if self.playlist.get_nb_musics() == 0 {
                    return Err(String::from_str("No more music"));
                } else {
                    let tmp_s = self.playlist.get_current();
                    return self.set_music(fmod, tmp_s);
                }
            }
        };
        self.musics.set_current(self.playlist.get_pos());
        self.music_bar.maximum = sound.get_length(FMOD_TIMEUNIT_MS).unwrap() as uint;
        Ok(sound)
    }

    pub fn set_music_position(&mut self, position: uint) {
        self.music_bar.set_progress(position);
    }

    pub fn update(&mut self, win: &mut RenderWindow) {
        win.clear(&Color::new_RGB(0, 0, 0));
        self.musics.draw(win);
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
                            sound = match self.set_music(fmod, tmp_s) {
                                Ok(s) => s,
                                Err(e) => fail!("Error : {}", e)
                            };
                            chan = match sound.play() {
                                Ok(c) => c,
                                Err(e) => fail!("sound.play : {}", e)
                            };
                            self.musics.set_current(self.playlist.get_pos());
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
                            self.musics.set_current(self.playlist.get_pos());
                        }
                        _ => {}
                    },
                    event::MouseButtonReleased{button, x, y} => match button {
                        mouse::MouseLeft => {
                            if y >= window.get_size().y as int - self.music_bar.progress_size.y as int {
                                self.music_bar.click(&Vector2u{x: x as u32, y: y as u32}, window.get_size().x);
                                chan.set_position(self.music_bar.real_value, FMOD_TIMEUNIT_MS);
                            }
                        },
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