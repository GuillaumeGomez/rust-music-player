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
use rsfml::graphics::{RenderWindow, Color, Text, Font, RectangleShape};
use std::rc::Rc;
use std::cell::RefCell;
use graphic_element::GraphicElement;

pub struct GraphicTimer {
    timer: rc::Text,
    cleaner: rc::RectangleShape,
    need_to_draw: bool,
    name: String
}

impl GraphicTimer {
    fn init(mut self, position: &Vector2f) -> GraphicTimer {
        self.set_position(position);
        self.cleaner.set_fill_color(&Color::new_RGB(0, 0, 0));
        self.cleaner.set_outline_color(&Color::new_RGB(255, 255, 255));
        self.cleaner.set_outline_thickness(1f32);
        self
    }

    pub fn update_display(&mut self, position: uint, length: uint) {
        let st = String::from_str(format!("{:02u}:{:02u} / {:02u}:{:02u}",
            position / 1000 / 60, position / 1000 % 60, length / 1000 / 60, length / 1000 % 60).as_slice());

        if st != self.timer.get_string() {
            self.need_to_draw = true;
            self.timer.set_string(st.as_slice());
            let size = self.timer.get_local_bounds().width;
            let y = self.timer.get_position().y;
            self.timer.set_position(&Vector2f{x: (self.cleaner.get_size().x - 1f32 - size as f32) / 2f32 + self.cleaner.get_position().x,
                                              y: y});
        }
    }
}

impl GraphicElement for GraphicTimer {
    fn new_init(size: &Vector2f, position: &Vector2f, unused: &Color, font: Option<&Font>) -> GraphicTimer {
        GraphicTimer {
            timer: match rc::Text::new_init("", Rc::new(RefCell::new(match font {
                    Some(f) => f.clone(),
                    None => fail!("GraphicTimer needs Font")
                })), 20) {
                Some(t) => t,
                None => fail!("Cannot create GraphicTimer")
            },
            cleaner: match rc::RectangleShape::new_init(&Vector2f{x: size.x as f32 - 2f32, y: size.y as f32 - 2f32}) {
                Some(l) => l,
                None => fail!("Cannot create cleaner for GraphicTimer")
            },
            need_to_draw: true,
            name: String::new()
        }.init(position)
    }

    fn set_position(&mut self, position: &Vector2f) {
        let size = self.timer.get_local_bounds().width;

        self.cleaner.set_position(&Vector2f{x: position.x + 1f32, y: position.y + 1f32});
        self.timer.set_position(&Vector2f{x: (self.cleaner.get_size().x - 1f32 - size as f32) / 2f32 + self.cleaner.get_position().x,
                                              y: (self.cleaner.get_size().y - 20f32) / 2f32 + self.cleaner.get_position().y - 2f32});
        self.need_to_draw = true;
    }

    fn get_position(&self) -> Vector2f {
        let tmp = self.cleaner.get_position();

        Vector2f{x: tmp.x - 1f32, y: tmp.y - 1f32}
    }

    fn set_size(&mut self, size: &Vector2f) {
        let tmp = self.cleaner.get_position();

        self.cleaner.set_size(&Vector2f{x: size.x - 2f32, y: size.y - 2f32});
        self.set_position(&tmp);
    }

    fn get_size(&self) -> Vector2f {
        let tmp = self.cleaner.get_size();

        Vector2f{x: tmp.x + 2f32, y: tmp.y + 2f32}
    }

    fn draw(&mut self, win: &mut RenderWindow) {
        if self.need_to_draw {
            win.draw(&self.cleaner);
            win.draw(&self.timer);
            self.need_to_draw = false;
        }
    }

    fn get_min_size(&self) -> Vector2f {
        Vector2f{x: 100f32, y: 40f32}
    }

    fn get_max_size(&self) -> Option<Vector2f> {
        Some(Vector2f{x: 200f32, y: 40f32})
    }

    fn is_inside(&self, pos: &Vector2f) -> bool {
        pos.y >= self.cleaner.get_position().y && pos.y <= self.cleaner.get_position().y + self.cleaner.get_size().y &&
        pos.x >= self.cleaner.get_position().x && pos.x <= self.cleaner.get_position().x + self.cleaner.get_size().x
    }

    fn cursor_moved(&mut self, position: &Vector2f) {
    }

    fn clicked(&mut self, position: &Vector2f) {
    }

    fn mouse_leave(&mut self) {
    }

    fn set_element_name(&mut self, name: &String) {
        self.name = name.clone();
    }

    fn get_element_name<'a>(&'a self) -> &'a String {
        &self.name
    }
}