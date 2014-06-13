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
use rsfml::graphics::{RenderWindow, Color, RectangleShape};

pub struct ProgressBar {
    line: rc::RectangleShape,
    pub maximum: uint,
    value: uint,
    real_value: uint,
    border: rc::RectangleShape,
    need_to_draw: bool
}

impl ProgressBar {
    pub fn new(color: &Color) -> ProgressBar {
        ProgressBar {
            line: rc::RectangleShape::new().unwrap(),
            maximum: 1,
            value: 0u,
            real_value: 0u,
            border: match rc::RectangleShape::new_init(&Vector2f{x: 0f32, y: 1f32}) {
                Some(l) => l,
                None => fail!("Cannot create border for ProgressBar")
            },
            need_to_draw: true
        }.init(color, &Vector2u{x: 0, y: 0})
    }

    pub fn new_init(size: &Vector2u, position: &Vector2u, color: &Color, maximum: uint) -> ProgressBar {
        ProgressBar {
            line: match rc::RectangleShape::new_init(&Vector2f{x: 0u as f32, y: size.y as f32}) {
                Some(l) => l,
                None => fail!("Cannot create progress bar")
            },
            maximum: maximum,
            value: 0u,
            real_value: 0u,
            border: match rc::RectangleShape::new_init(&Vector2f{x: size.x as f32 + 1f32, y: size.y as f32 + 1f32}) {
                Some(l) => l,
                None => fail!("Cannot create border for ProgressBar")
            },
            need_to_draw: true
        }.init(color, position)
    }

    fn init(mut self, color: &Color, position: &Vector2u) -> ProgressBar {
        self.set_position(position);
        self.line.set_fill_color(color);
        self.border.set_fill_color(&Color::new_RGB(0, 0, 0));
        self.border.set_outline_color(&Color::new_RGB(255, 255, 255));
        self.border.set_outline_thickness(1f32);
        self
    }

    pub fn draw(&mut self, win: &mut RenderWindow) {
        if self.need_to_draw {
            win.draw(&self.border);
            win.draw(&self.line);
            self.need_to_draw = false;
        }
    }

    pub fn set_size(&mut self, size: &Vector2u) {
        self.need_to_draw = true;
        self.border.set_size(&Vector2f{x: size.x as f32 + 1f32, y: size.y as f32 + 1f32});
        self.set_progress(self.real_value);
    }

    pub fn set_position(&mut self, position: &Vector2u) {
        self.need_to_draw = true;
        self.line.set_position(&Vector2f{x: position.x as f32, y: position.y as f32});
        self.border.set_position(&Vector2f{x: position.x as f32, y: position.y as f32});
    }

    pub fn set_progress(&mut self, position: uint) {
        let tmp = if position > self.maximum {
            self.maximum
        } else {
            position
        };
        let new_value = tmp * (self.border.get_size().x as uint - 1u) / self.maximum;

        if new_value != self.value {
            self.need_to_draw = true;
            self.value = new_value;
            self.real_value = position;
            self.line.set_size(&Vector2f{x: self.value as f32, y: self.border.get_size().y as f32 - 1f32});
        }
    }

    pub fn clicked(&mut self, pos: &Vector2u) {
        let in_order = (pos.x as f32 - self.line.get_position().x) / (self.border.get_size().x - 1f32) * 100f32;

        self.set_progress((in_order * self.maximum as f32 / 100f32) as uint);
    }

    pub fn is_inside(&self, pos: &Vector2u) -> bool {
        pos.y >= self.line.get_position().y as u32 && pos.y <= self.line.get_position().y as u32 + self.border.get_size().y as u32 - 1u32 &&
        pos.x >= self.line.get_position().x as u32 && pos.x <= self.line.get_position().x as u32 + self.border.get_size().x as u32 - 1u32
    }

    pub fn get_real_value(&self) -> uint {
        self.real_value
    }
}