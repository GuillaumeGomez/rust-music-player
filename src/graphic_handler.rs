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

use rsfml::system::vector2::{Vector2u};
use rsfml::window::{event, keyboard, mouse};
use rsfml::graphics::{RenderWindow, Color, Font};
use rfmod::enums::*;
use rfmod::*;
use playlist::PlayList;
use graphic_timer::GraphicTimer;
use graphic_spectrum::GraphicSpectrum;
use graphic_playlist::GraphicPlayList;
use progress_bar::ProgressBar;

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
                &Vector2u{x: window.get_size().x - 512u32, y: window.get_size().y - 35u32}),
            timer: GraphicTimer::new(font, &Vector2u{x: window.get_size().x - 633u32, y: 24u32},
                                        &Vector2u{x: window.get_size().x - (window.get_size().x - 634u32), y: window.get_size().y - 35u32}),
            music_bar: ProgressBar::new_init(&Vector2u{x: window.get_size().x, y: 8u32}, &Vector2u{x: 0u32, y: window.get_size().y - 8u32},
                &Color::new_RGB(255, 255, 255), 1u),
            volume_bar: ProgressBar::new_init(&Vector2u{x: 120u32, y: 20u32},
                &Vector2u{x: 513u32, y: window.get_size().y - 30u32},
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
                        self.timer.update_display(position, length as uint);
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
                        keyboard::Space => {
                            chan.set_paused(!chan.get_paused().unwrap());
                        }
                        _ => {}
                    },
                    event::KeyPressed{code, ..} => match code {
                        keyboard::Add => {
                            let tmp = self.volume_bar.get_real_value();
                            self.volume_bar.set_progress(tmp + 1);
                            chan.set_volume(self.volume_bar.get_real_value() as f32 / 100f32);
                        }
                        keyboard::Subtract => {
                            let tmp = self.volume_bar.get_real_value();
                            self.volume_bar.set_progress(tmp - 1);
                            chan.set_volume(self.volume_bar.get_real_value() as f32 / 100f32);
                        }
                        _ => {}
                    },
                    event::MouseButtonReleased{button, x, y} => match button {
                        mouse::MouseLeft => {
                            if self.music_bar.is_inside(&Vector2u{x: x as u32, y: y as u32}) {
                                self.music_bar.click(&Vector2u{x: x as u32, y: y as u32});
                                chan.set_position(self.music_bar.get_real_value(), FMOD_TIMEUNIT_MS);
                            } else if self.volume_bar.is_inside(&Vector2u{x: x as u32, y: y as u32}) {
                                self.volume_bar.click(&Vector2u{x: x as u32, y: y as u32});
                                chan.set_volume(self.volume_bar.get_real_value() as f32 / 100f32);
                            } else if self.musics.is_inside(&Vector2u{x: x as u32, y: y as u32}) {
                                if self.musics.click(y) {
                                    self.playlist.set_actual(self.musics.get_current());

                                    let tmp_s = self.playlist.get_current();

                                    sound = match self.set_music(fmod, tmp_s) {
                                        Ok(s) => s,
                                        Err(e) => fail!("Error : {}", e)
                                    };
                                    chan = match sound.play() {
                                        Ok(c) => c,
                                        Err(e) => fail!("sound.play : {}", e)
                                    };
                                }
                            }
                        },
                        _ => {}
                    },
                    event::MouseWheelMoved{delta, ..} => {
                        let tmp = self.musics.get_add_to_view();
                        self.musics.set_to_add(tmp + delta);
                    },
                    event::MouseMoved{x, y} => {
                        if self.musics.is_inside(&Vector2u{x: x as u32, y: y as u32}) {
                            self.musics.cursor_moved(y);
                        } else {
                            self.musics.mouse_leave();
                        }
                    }
                    event::NoEvent => break,
                    _ => {}
                }
            }

            let new_position = match self.main_loop(&chan, old_position, length) {
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

            if old_position != new_position {
                old_position = new_position;
                self.update(window);
            }
        }
    }
}