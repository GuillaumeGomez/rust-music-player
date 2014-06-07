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
    spectrum: Vec<rc::RectangleShape>,
    height: uint
}

impl GraphicSpectrum {
    fn init(mut self) -> GraphicSpectrum {
        let mut it = 0;

        while it < 512 {
            self.spectrum.push(match rc::RectangleShape::new_init(&Vector2f{x: 1f32, y: self.height as f32}) {
                Some(l) => l,
                None => fail!("Cannot create spectrum")
            });
            self.spectrum.get_mut(it).set_fill_color(&Color::new_RGB(50, 100, 30));
            self.spectrum.get_mut(it).set_position(&Vector2f{x: it as f32, y: self.height as f32});
            it += 1;
        }
        self
    }

    fn new(height: uint) -> GraphicSpectrum {
        GraphicSpectrum {
            spectrum: Vec::new(),
            height: height
        }.init()
    }

    fn update_spectrum(&mut self, data: Vec<f32>) {
        let mut it = 0u;

        for tmp in data.iter() {
            self.spectrum.get_mut(it).set_size(&Vector2f{x: 1f32, y: self.height as f32 * *tmp * -20f32});
            it += 1;
        }
    }

    fn draw(&mut self, win: &mut RenderWindow) {
        for tmp in self.spectrum.mut_iter() {
            win.draw(tmp);
        }
    }
}

struct GraphicPlayList {
    musics: Vec<String>,
    texts: Vec<rc::Text>,
    graphic_size: Vector2u,
    position: Vector2u,
    to_draw: uint,
    current: uint,
    border: rc::RectangleShape
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
            current: 1u,
            border: match rc::RectangleShape::new_init(&Vector2f{x: 0f32, y: 1f32}) {
                Some(l) => l,
                None => fail!("Cannot create border for GraphicPlayList")
            }
        }.init(font)
    }

    fn new_init(musics: Vec<String>, font: &Font, position: &Vector2u, size: &Vector2u) -> GraphicPlayList {
        GraphicPlayList {
            musics: musics,
            texts: Vec::new(),
            graphic_size: size.clone(),
            position: position.clone(),
            to_draw: 0u,
            current: 1u,
            border: match rc::RectangleShape::new_init(&Vector2f{x: 1f32, y: size.y as f32}) {
                Some(l) => l,
                None => fail!("Cannot create border for GraphicPlayList")
            }
        }.init(font)
    }

    fn set_position(&mut self, position: &Vector2u) {
        let mut pos = position.y;
        let limit = self.graphic_size.y + position.y;

        self.position = position.clone();
        self.to_draw = 0;
        self.border.set_position(&Vector2f{x: position.x as f32 - 1f32, y: position.y as f32});
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
        win.draw(&self.border);
    }

    fn set_current(&mut self, current: uint) {
        if current != self.current {
            self.texts.get_mut(current).set_color(&Color::new_RGB(255, 125, 25));
            self.texts.get_mut(self.current).set_color(&Color::new_RGB(255, 255, 255));
            self.current = current;
        }
    }

    fn remove_music(&mut self, pos: uint) {
        self.texts.remove(pos);
        let tmp = self.position;
        self.set_position(&tmp);
    }

    fn is_inside(&self, pos: &Vector2u) -> bool {
        pos.y >= self.position.y && pos.y <= self.position.y + self.graphic_size.y &&
        pos.x >= self.position.x && pos.y <= self.position.x + self.graphic_size.x
    }
}

struct GraphicTimer {
    timer: rc::Text,
    cleaner: rc::RectangleShape,
    position: Vector2u
}

impl GraphicTimer {
    fn new(font: Font, size: &Vector2u, position: &Vector2u) -> GraphicTimer {
        GraphicTimer {
            timer: match rc::Text::new_init("", Rc::new(RefCell::new(font)), 20) {
                Some(t) => t,
                None => fail!("Cannot create GraphicTimer")
            },
            cleaner: match rc::RectangleShape::new_init(&Vector2f{x: size.x as f32 + 1f32, y: size.y as f32 + 1f32}) {
                Some(l) => l,
                None => fail!("Cannot create cleaner for GraphicTimer")
            },
            position: position.clone()
        }.init()
    }

    fn init(mut self) -> GraphicTimer {
        let tmp = self.position.clone();
        self.set_position(&tmp);
        self.cleaner.set_fill_color(&Color::new_RGB(0, 0, 0));
        self.cleaner.set_outline_color(&Color::new_RGB(255, 255, 255));
        self.cleaner.set_outline_thickness(1f32);
        self
    }

    fn set_position(&mut self, position: &Vector2u) {
        self.cleaner.set_position(&Vector2f{x: self.position.x as f32, y: self.position.y as f32});
        self.timer.set_position(&Vector2f{x: self.position.x as f32 + 5f32, y: self.position.y as f32 + 2f32});
    }

    fn draw(&self, win: &mut RenderWindow) {
        win.draw(&self.cleaner);
        win.draw(&self.timer);
    }
}

struct ProgressBar {
    line: rc::RectangleShape,
    graphic_size: Vector2u,
    maximum: uint,
    value: uint,
    real_value: uint,
    cleaner: rc::RectangleShape,
}

impl ProgressBar {
    fn new(color: &Color) -> ProgressBar {
        ProgressBar {
            line: rc::RectangleShape::new().unwrap(),
            graphic_size: Vector2u{x: 0, y: 0},
            maximum: 1,
            value: 0u,
            real_value: 0u,
            cleaner: match rc::RectangleShape::new_init(&Vector2f{x: 0f32, y: 1f32}) {
                Some(l) => l,
                None => fail!("Cannot create border for ProgressBar")
            }
        }.init(color, &Vector2u{x: 0, y: 0})
    }

    fn new_init(size: &Vector2u, position: &Vector2u, color: &Color, maximum: uint) -> ProgressBar {
        ProgressBar {
            line: match rc::RectangleShape::new_init(&Vector2f{x: 0u as f32, y: size.y as f32}) {
                Some(l) => l,
                None => fail!("Cannot create progress bar")
            },
            graphic_size: size.clone(),
            maximum: maximum,
            value: 0u,
            real_value: 0u,
            cleaner: match rc::RectangleShape::new_init(&Vector2f{x: size.x as f32 + 1f32, y: size.y as f32 + 1f32}) {
                Some(l) => l,
                None => fail!("Cannot create border for ProgressBar")
            }
        }.init(color, position)
    }

    fn init(mut self, color: &Color, position: &Vector2u) -> ProgressBar {
        self.set_position(position);
        self.line.set_fill_color(color);
        self.cleaner.set_fill_color(&Color::new_RGB(0, 0, 0));
        self.cleaner.set_outline_color(&Color::new_RGB(255, 255, 255));
        self.cleaner.set_outline_thickness(1f32);
        self
    }

    fn draw(&self, win: &mut RenderWindow) {
        win.draw(&self.cleaner);
        win.draw(&self.line);
    }

    fn set_size(&mut self, size: &Vector2u) {
        self.graphic_size = size.clone();
        self.cleaner.set_size(&Vector2f{x: size.x as f32 + 1f32, y: size.y as f32 + 1f32});
        self.set_progress(self.real_value);
    }

    fn set_position(&mut self, position: &Vector2u) {
        self.line.set_position(&Vector2f{x: position.x as f32, y: position.y as f32});
        self.cleaner.set_position(&Vector2f{x: position.x as f32 - 1f32, y: position.y as f32 - 1f32});
    }

    fn set_progress(&mut self, position: uint) {
        let tmp = if position > self.maximum {
            self.maximum
        } else {
            position
        };
        let new_value = tmp * self.graphic_size.x as uint / self.maximum;

        if new_value != self.value {
            self.value = new_value;
            self.real_value = position;
            self.line.set_size(&Vector2f{x: self.value as f32, y: self.graphic_size.y as f32});
        }
    }

    fn click(&mut self, pos: &Vector2u) {
        let in_order = (pos.x as f32 - self.line.get_position().x) / self.graphic_size.x as f32 * 100f32;

        self.set_progress((in_order * self.maximum as f32 / 100f32) as uint);
    }

    fn is_inside(&self, pos: &Vector2u) -> bool {
        pos.y >= self.line.get_position().y as u32 && pos.y <= self.line.get_position().y as u32 + self.graphic_size.y &&
        pos.x >= self.line.get_position().x as u32 && pos.y <= self.line.get_position().x as u32 + self.graphic_size.x
    }
}

pub struct GraphicHandler {
    font: Font,
    musics: GraphicPlayList,
    timer: GraphicTimer,
    music_bar: ProgressBar,
    volume_bar: ProgressBar,
    playlist: PlayList,
    spectrum: GraphicSpectrum
}

impl GraphicHandler {
    fn init(mut self) -> GraphicHandler {
        self.volume_bar.set_progress(100);
        self.timer.set_position(&Vector2u{x: 513u32, y: self.volume_bar.line.get_position().y as u32});
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
                &Vector2u{x: window.get_size().x - (window.get_size().x - 513u32), y: 0},
                &Vector2u{x: window.get_size().x - 512u32, y: window.get_size().y - 20u32}),
            timer: GraphicTimer::new(font, &Vector2u{x: window.get_size().x - 632u32, y: 24u32},
                                        &Vector2u{x: 513u32, y: window.get_size().y - 35u32}),
            music_bar: ProgressBar::new_init(&Vector2u{x: window.get_size().x, y: 8u32}, &Vector2u{x: 0u32, y: window.get_size().y - 8u32},
                &Color::new_RGB(255, 255, 255), 1u),
            volume_bar: ProgressBar::new_init(&Vector2u{x: 120u32, y: 20u32},
                &Vector2u{x: window.get_size().x - 120u32, y: window.get_size().y - 29u32},
                &Color::new_RGB(255, 25, 25), 100u),
            playlist: playlist,
            spectrum: GraphicSpectrum::new(window.get_size().y as uint - 9u)
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
        self.volume_bar.draw(win);
        self.timer.draw(win);
        self.spectrum.draw(win);
        self.music_bar.draw(win);
        win.display();
    }

    fn main_loop(&mut self, chan: &Channel, old_position: uint, length: u32) -> Option<uint> {
        match chan.is_playing() {
            Ok(b) => {
                if b == true {
                    let position = chan.get_position(FMOD_TIMEUNIT_MS).unwrap();

                    if position != old_position {
                        self.spectrum.update_spectrum(chan.get_spectrum(512u, 0i32, fmod::DSP_FFT_WindowRect).unwrap());
                        self.timer.timer.set_string(format!("{:02u}:{:02u} / {:02u}:{:02u}",
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
                    event::KeyPressed{code, ..} => match code {
                        keyboard::Add => {
                            self.volume_bar.set_progress(self.volume_bar.real_value + 1);
                            chan.set_volume(self.volume_bar.real_value as f32 / 100f32);
                        }
                        keyboard::Substract => {
                            self.volume_bar.set_progress(self.volume_bar.real_value - 1);
                            chan.set_volume(self.volume_bar.real_value as f32 / 100f32);
                        }
                        _ => {}
                    },
                    event::MouseButtonReleased{button, x, y} => match button {
                        mouse::MouseLeft => {
                            if self.music_bar.is_inside(&Vector2u{x: x as u32, y: y as u32}) {
                                self.music_bar.click(&Vector2u{x: x as u32, y: y as u32});
                                chan.set_position(self.music_bar.real_value, FMOD_TIMEUNIT_MS);
                            } else if self.volume_bar.is_inside(&Vector2u{x: x as u32, y: y as u32}) {
                                self.volume_bar.click(&Vector2u{x: x as u32, y: y as u32});
                                chan.set_volume(self.volume_bar.real_value as f32 / 100f32);
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