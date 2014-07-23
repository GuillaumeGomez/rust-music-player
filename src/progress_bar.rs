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
use rsfml::graphics::{RenderWindow, Color, RectangleShape, Font};
use graphic_element::GraphicElement;

pub struct ProgressBar {
    line: rc::RectangleShape,
    pub maximum: uint,
    value: uint,
    real_value: uint,
    cleaner: rc::RectangleShape,
    need_to_draw: bool,
    name: String
}

impl ProgressBar {
    fn init(mut self, color: &Color, position: &Vector2f) -> ProgressBar {
        self.set_position(position);
        self.line.set_fill_color(color);
        self.cleaner.set_fill_color(&Color::new_RGB(0, 0, 0));
        self.cleaner.set_outline_color(&Color::new_RGB(255, 255, 255));
        self.cleaner.set_outline_thickness(1f32);
        self
    }

    pub fn set_progress(&mut self, position: uint) {
        let tmp = if position > self.maximum {
            self.maximum
        } else {
            position
        };
        if self.maximum > 0u {
            let new_value = tmp * (self.cleaner.get_size().x as uint - 2u) / self.maximum;

            if new_value != self.value {
                self.need_to_draw = true;
                self.value = new_value;
                self.real_value = position;
                self.line.set_size(&Vector2f{x: self.value as f32, y: self.cleaner.get_size().y as f32 - 2f32});
            }
        }
    }

    pub fn get_real_value(&self) -> uint {
        self.real_value
    }

    pub fn set_maximum(&mut self, maximum: uint) {
        self.maximum = maximum;
    }
}

impl GraphicElement for ProgressBar {
    fn new_init(size: &Vector2f, position: &Vector2f, color: &Color, unused: Option<&Font>) -> ProgressBar {
        ProgressBar {
            line: match rc::RectangleShape::new_init(&Vector2f{x: 0f32, y: size.y}) {
                Some(l) => l,
                None => fail!("Cannot create progress bar")
            },
            maximum: 0u,
            value: 0u,
            real_value: 0u,
            name: String::new(),
            cleaner: match rc::RectangleShape::new_init(&Vector2f{x: size.x as f32 + 1f32, y: size.y as f32 + 1f32}) {
                Some(l) => l,
                None => fail!("Cannot create cleaner for ProgressBar")
            },
            need_to_draw: true
        }.init(color, position)
    }

    fn is_inside(&self, position: &Vector2f) -> bool {
        position.y >= self.line.get_position().y && position.y <= self.line.get_position().y + self.cleaner.get_size().y &&
        position.x >= self.line.get_position().x && position.x <= self.line.get_position().x + self.cleaner.get_size().x
    }

    fn clicked(&mut self, position: &Vector2f) {
        let in_order = (position.x - self.line.get_position().x) / (self.cleaner.get_size().x - 1f32) * 100f32;
        let tmp_maximum = self.maximum;

        self.set_progress((in_order * tmp_maximum as f32 / 100f32) as uint);
    }

    fn draw(&mut self, window: &mut RenderWindow) {
        window.draw(&self.cleaner);
        window.draw(&self.line);
        self.need_to_draw = false;
    }

    fn get_size(&self) -> Vector2f {
        let tmp = self.cleaner.get_size();

        Vector2f{x: tmp.x + 2f32, y: tmp.y + 2f32}
    }

    fn set_size(&mut self, size: &Vector2f) {
        self.need_to_draw = true;
        self.cleaner.set_size(&Vector2f{x: size.x - 2f32, y: size.y - 2f32});
        let tmp_real_value = self.real_value;
        self.set_progress(tmp_real_value);
    }

    fn get_position(&self) -> Vector2f {
        let tmp = self.cleaner.get_position();

        Vector2f{x: tmp.x - 1f32, y: tmp.y - 1f32}
    }

    fn set_position(&mut self, position: &Vector2f) {
        self.need_to_draw = true;
        self.line.set_position(&Vector2f{x: position.x + 2f32, y: position.y + 2f32});
        self.cleaner.set_position(&Vector2f{x: position.x + 1f32, y: position.y + 1f32});
    }

    fn cursor_moved(&mut self, position: &Vector2f) {
    }

    fn get_min_size(&self) -> Vector2f {
        Vector2f{x: 3f32, y: 3f32}
    }

    fn get_max_size(&self) -> Option<Vector2f> {
        None
    }

    fn set_element_name(&mut self, name: &String) {
        self.name = name.clone();
    }

    fn get_element_name<'a>(&'a self) -> &'a String {
        &self.name
    }

    fn mouse_leave(&mut self) {
    }
}