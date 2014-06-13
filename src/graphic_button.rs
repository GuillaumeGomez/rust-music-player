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
use rsfml::graphics::{RenderWindow, Color, RectangleShape, Text, Font};
use std::rc::Rc;
use std::cell::RefCell;

pub struct GraphicButton {
    label: rc::Text,
    button: rc::RectangleShape,
    need_to_draw: bool,
    pushed: bool,
    has_mouse: bool
}

impl GraphicButton {
    pub fn new_init(font: &Font, size: &Vector2u, position: &Vector2u, label: &String) -> GraphicButton {
        GraphicButton {
            label: match rc::Text::new_init(label.as_slice(), Rc::new(RefCell::new(font.clone())), 20) {
                Some(t) => t,
                None => fail!("Cannot create label for GraphicButton")
            },
            button: match rc::RectangleShape::new_init(&Vector2f{x: size.x as f32 - 2f32, y: size.y as f32 - 2f32}) {
                Some(l) => l,
                None => fail!("Cannot create cleaner for GraphicButton")
            },
            need_to_draw: true,
            pushed: false,
            has_mouse: false
        }.init(position)
    }

    fn init(mut self, position: &Vector2u) -> GraphicButton {
        self.set_position(position);
        self.button.set_fill_color(&Color::new_RGB(10, 10, 10));
        self.button.set_outline_color(&Color::new_RGB(255, 255, 255));
        self.button.set_outline_thickness(1f32);
        self
    }

    pub fn draw(&mut self, win: &mut RenderWindow) {
        if self.need_to_draw {
            win.draw(&self.button);
            win.draw(&self.label);
            self.need_to_draw = false;
        }
    }

    pub fn set_position(&mut self, position: &Vector2u) {
        let size = self.label.get_local_bounds().width;

        self.button.set_position(&Vector2f{x: position.x as f32 + 1f32, y: position.y as f32 + 1f32});
        self.label.set_position(&Vector2f{x: (self.button.get_size().x - 1f32 - size as f32) / 2f32 + self.button.get_position().x as f32,
                                              y: self.button.get_position().y + (self.button.get_size().y - 20f32) / 2f32 - 2f32});
        self.need_to_draw = true;
    }

    pub fn is_inside(&self, pos: &Vector2u) -> bool {
        pos.y as f32 >= self.button.get_position().y && pos.y as f32 <= self.button.get_position().y + self.button.get_size().y &&
        pos.x as f32 >= self.button.get_position().x && pos.x as f32 <= self.button.get_position().x + self.button.get_size().x
    }

    pub fn mouse_leave(&mut self) {
        if self.has_mouse {
            let tmp = self.button.get_size();
            let pos = self.button.get_position();

            self.button.set_outline_thickness(1f32);
            self.button.set_size(&Vector2f{x: tmp.x + 2f32, y: tmp.y + 2f32});
            self.button.set_position(&Vector2f{x: pos.x - 1f32, y: pos.y - 1f32});
            self.need_to_draw = true;
            self.has_mouse = false;
        }
    }

    pub fn cursor_moved(&mut self, position: &Vector2u) {
        if !self.has_mouse {
            let tmp = self.button.get_size();
            let pos = self.button.get_position();

            self.button.set_outline_thickness(2f32);
            self.button.set_size(&Vector2f{x: tmp.x - 2f32, y: tmp.y - 2f32});
            self.button.set_position(&Vector2f{x: pos.x + 1f32, y: pos.y + 1f32});
            self.need_to_draw = true;
            self.has_mouse = true;
        }
    }

    pub fn clicked(&mut self, position: &Vector2u) {
        if self.pushed {
            self.pushed = false;
            self.button.set_fill_color(&Color::new_RGB(10, 10, 10));
            self.need_to_draw = true;
        } else {
            self.pushed = true;
            self.button.set_fill_color(&Color::new_RGB(205, 187, 100));
            self.need_to_draw = true;
        }
    }
}