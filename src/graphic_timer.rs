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
use rsfml::graphics::{RenderWindow, Color, Text, Font, RectangleShape};
use std::rc::Rc;
use std::cell::RefCell;

pub struct GraphicTimer {
    timer: rc::Text,
    cleaner: rc::RectangleShape,
    position: Vector2u
}

impl GraphicTimer {
    pub fn new(font: Font, size: &Vector2u, position: &Vector2u) -> GraphicTimer {
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

    pub fn set_position(&mut self, position: &Vector2u) {
        self.cleaner.set_position(&Vector2f{x: position.x as f32, y: position.y as f32});
        self.timer.set_position(&Vector2f{x: position.x as f32 + 5f32, y: position.y as f32 + 1f32});
        self.position = position.clone();
    }

    pub fn update_display(&mut self, position: uint, length: uint) {
        let st = String::from_str(format!("{:02u}:{:02u} / {:02u}:{:02u}",
            position / 1000 / 60, position / 1000 % 60, length / 1000 / 60, length / 1000 % 60).as_slice());

        if st != self.timer.get_string() {
            self.timer.set_string(st.as_slice());
            let size = self.timer.get_local_bounds().width;
            let y = self.timer.get_position().y;
            self.timer.set_position(&Vector2f{x: (self.cleaner.get_size().x - 1f32 - size as f32) / 2f32 + self.position.x as f32,
                                              y: y});
        }
    }

    pub fn draw(&self, win: &mut RenderWindow) {
        win.draw(&self.cleaner);
        win.draw(&self.timer);
    }
}