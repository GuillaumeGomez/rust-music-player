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
#![allow(unused_variable)]

use rsfml::graphics::rc;
use rsfml::system::vector2::{Vector2f};
use rsfml::graphics::{RenderWindow, Color, Text, Font, RectangleShape, RenderTarget};
use std::rc::Rc;
use std::cell::RefCell;
use graphic_element::GraphicElement;

pub struct GraphicPlayList {
    musics: Vec<String>,
    texts: Vec<rc::Text>,
    to_draw: uint,
    current: uint,
    hover_element: Option<uint>,
    add_to_view: int,
    cleaner: rc::RectangleShape,
    need_to_draw: bool,
    has_mouse: bool,
    font: Font,
    name: String
}

impl GraphicPlayList {
    fn init(mut self, position: &Vector2f) -> GraphicPlayList {
        self.set_position(position);
        self.set_current(0u);
        self.cleaner.set_fill_color(&Color::new_RGB(0, 0, 0));
        self.cleaner.set_outline_color(&Color::new_RGB(255, 255, 255));
        self.cleaner.set_outline_thickness(1f32);
        self
    }

    pub fn add_music(&mut self, music: &String) {
        if !self.musics.contains(music) {
            self.musics.push(music.clone());
            /*let pos = if self.texts.len() > 0 {
                    match self.texts.last() {
                        Some(f) => f.get_position(),
                        None => Vector2f{x: self.cleaner.get_position().x + 4f32, y: self.cleaner.get_position().y - 22f32}
                    }
                } else {
                    Vector2f{x: self.cleaner.get_position().x + 4f32, y: self.cleaner.get_position().y - 22f32}
                };*/
            self.texts.push(match rc::Text::new_init(music.as_slice().split_terminator('/').last().unwrap(),
                Rc::new(RefCell::new(self.font.clone())), 20) {
                Some(t) => t,
                None => fail!("Cannot create Text")
            });
            let tmp = self.cleaner.get_position();
            self.set_position(&tmp);
        }
    }

    pub fn add_musics(&mut self, musics: &Vec<String>) {
        for tmp in musics.iter() {
            self.add_music(tmp)
        }
    }

    pub fn set_to_add(&mut self, to_add: int) {
        let tmp_add = to_add * 22i;
        let max = (self.texts.len() as int + 1i) * 22i;

        if self.add_to_view != to_add && tmp_add >= 0i && tmp_add + self.to_draw as int * 22i < max
            && self.texts.len() as int * 22i >= (self.cleaner.get_size().y as int - 1) {
            let mut pos = self.cleaner.get_position().y as int - tmp_add as int;
            for tmp in self.texts.mut_iter() {
                let x = tmp.get_position().x;
                tmp.set_position(&Vector2f{x: x as f32, y: pos as f32});
                pos += 22i;
            }
            self.add_to_view = to_add;
            self.need_to_draw = true;
        }
    }

    pub fn set_current(&mut self, current: uint) {
        self.set_current_intern(current, false)
    }

    fn set_current_intern(&mut self, current: uint, by_click: bool) {
        if self.texts.len() > 0 && current != self.current {
            if self.current < self.texts.len() {
                self.texts.get_mut(self.current).set_color(&Color::new_RGB(255, 255, 255));
            }
            self.texts.get_mut(current).set_color(&Color::new_RGB(255, 125, 25));
            self.current = current;
            self.need_to_draw = true;
            let tmp_to_draw = self.to_draw;

            if by_click == false && self.texts.len() as int * 22i >= (self.cleaner.get_size().y as int - 1) {
                if self.current as int + 2i >= self.to_draw as int + self.add_to_view {
                    self.set_to_add(current as int + 2i - tmp_to_draw as int);
                } else if (self.current as int) < self.add_to_view {
                    self.set_to_add(current as int);
                }
            }
        }
    }

    pub fn get_current(&self) -> uint {
        self.current
    }

    pub fn get_add_to_view(&self) -> int {
        self.add_to_view
    }

    pub fn remove_music(&mut self, pos: uint) {
        self.texts.remove(pos);
        let tmp = Vector2f{x: self.cleaner.get_position().x, y: self.cleaner.get_position().y};
        self.set_position(&tmp);
        if self.musics.len() == 0u || self.texts.len() == 0u {
            fail!("GraphicPlayList cannot be empty");
        }
        self.need_to_draw = true;
    }
}

impl GraphicElement for GraphicPlayList {
    fn new_init(size: &Vector2f, position: &Vector2f, color: &Color, font: Option<&Font>) -> GraphicPlayList {
        GraphicPlayList {
            musics: Vec::new(),
            texts: Vec::new(),
            to_draw: 0u,
            current: 1u,
            cleaner: match rc::RectangleShape::new_init(&Vector2f{x: size.x - 2f32, y: size.y - 2f32}) {
                Some(l) => l,
                None => fail!("Cannot create cleaner for GraphicPlayList")
            },
            hover_element: None,
            add_to_view: 0i,
            need_to_draw: true,
            has_mouse: false,
            font: match font {
                Some(f) => f.clone(),
                None => fail!("GraphicPlayList needs Font")
            },
            name: String::new()
        }.init(position)
    }

    fn set_position(&mut self, position: &Vector2f) {
        let mut pos = position.y;
        let limit = self.cleaner.get_size().y - 1f32 + position.y;

        self.to_draw = 0;
        self.cleaner.set_position(&Vector2f{x: position.x, y: position.y});
        if self.texts.len() > 0 {
            for tmp in self.texts.mut_iter() {
                tmp.set_position(&Vector2f{x: self.cleaner.get_position().x + 4f32, y: pos});
                if pos < limit {
                    self.to_draw += 1;
                }
                pos += 22f32;
            }
            if self.to_draw > 0 && self.to_draw * 22u > limit as uint + 2u {
                self.to_draw -= 1;
            }
        }
        self.need_to_draw = true;
    }

    fn get_position(&self) -> Vector2f {
        let tmp = self.cleaner.get_position();

        Vector2f{x: tmp.x - 1f32, y: tmp.y - 1f32}
    }

    fn set_size(&mut self, size: &Vector2f) {
        let pos = self.cleaner.get_position();

        self.cleaner.set_size(&Vector2f{x: size.x - 2f32, y: size.y - 2f32});
        self.set_position(&pos);
    }

    fn get_size(&self) -> Vector2f {
        let tmp = self.cleaner.get_size();

        Vector2f{x: tmp.x + 2f32, y: tmp.y + 2f32}
    }

    fn get_min_size(&self) -> Vector2f {
        Vector2f{x: 50f32, y: 50f32}
    }

    fn get_max_size(&self) -> Option<Vector2f> {
        None
    }

    fn cursor_moved(&mut self, position: &Vector2f) {
        let tmp = ((position.y - self.cleaner.get_position().y) / 22f32 + self.add_to_view as f32) as uint;

        self.need_to_draw = true;
        self.has_mouse = true;
        if tmp >= self.texts.len() {
            self.hover_element = None;
            return;
        }
        match self.hover_element {
            Some(s) => {
                if self.current == tmp {
                    self.texts.get_mut(s).set_color(&Color::new_RGB(255, 255, 255));
                    self.hover_element = None;
                } else if s != tmp {
                    self.texts.get_mut(s).set_color(&Color::new_RGB(255, 255, 255));
                    self.hover_element = Some(tmp);
                    self.texts.get_mut(tmp).set_color(&Color::new_RGB(255, 175, 100));
                }
            }
            None => {
                if self.current != tmp {
                    self.hover_element = Some(tmp);
                    self.texts.get_mut(tmp).set_color(&Color::new_RGB(255, 175, 100));
                }
            }
        }
    }

    fn clicked(&mut self, position: &Vector2f) {
        if position.y >= self.cleaner.get_position().y {
            let tmp = ((position.y - self.cleaner.get_position().y) / 22f32 + self.add_to_view as f32) as uint;

            self.need_to_draw = true;
            if tmp < self.texts.len() {
                self.hover_element = match self.hover_element {
                    Some(s) => {
                        self.texts.get_mut(s).set_color(&Color::new_RGB(255, 255, 255));
                        None
                    }
                    None => None
                };
                self.set_current_intern(tmp, true);
            }
        }
    }

    fn mouse_leave(&mut self) {
        if self.has_mouse {
            match self.hover_element {
                Some(s) => {
                    self.texts.get_mut(s).set_color(&Color::new_RGB(255, 255, 255));
                    self.hover_element = None;
                    self.need_to_draw = true;
                }
                None => {}
            }
            self.has_mouse = false;
        }
    }

    fn is_inside(&self, pos: &Vector2f) -> bool {
        pos.y >= self.cleaner.get_position().y && pos.y <= self.cleaner.get_position().y + self.cleaner.get_size().y &&
        pos.x >= self.cleaner.get_position().x && pos.x <= self.cleaner.get_position().x + self.cleaner.get_size().x
    }

    fn draw(&mut self, win: &mut RenderWindow) {
        let mut it = 0i;

        win.draw(&self.cleaner);
        if self.texts.len() > 0 {
            for tmp in self.texts.mut_iter() {
                if it == self.to_draw as int + self.add_to_view {
                    break;
                }
                if it >= self.add_to_view as int {
                    win.draw(tmp);
                }
                it += 1;
            }
        }
        self.need_to_draw = false;
    }

    fn set_element_name(&mut self, name: &String) {
        self.name = name.clone();
    }

    fn get_element_name<'a>(&'a self) -> &'a String {
        &self.name
    }
}