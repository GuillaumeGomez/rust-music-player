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
use rsfml::graphics::{RenderWindow, Color, RectangleShape, CircleShape, Font, RenderTarget};
use std::rc::Rc;
use std::cell::RefCell;
use graphic_element::GraphicElement;

pub struct GraphicSoundPosition {
    circle: rc::CircleShape,
    center: rc::CircleShape,
    cross1: rc::RectangleShape,
    cross2: rc::RectangleShape,
    cleaner: rc::RectangleShape,
    text_x: rc::Text,
    text_y: rc::Text,
    name: String,
    pub need_to_draw: bool,
    pub x: f32,
    pub y: f32,
    pub limit: f32
}

impl GraphicSoundPosition {
    fn init(mut self, position: &Vector2f) -> GraphicSoundPosition {
        self.circle.set_fill_color(&Color::new_RGB(0, 0, 0));
        self.circle.set_outline_color(&Color::new_RGB(255, 255, 255));
        self.circle.set_outline_thickness(1f32);
        self.cross1.set_rotation(45f32);
        self.cross1.set_fill_color(&Color::new_RGB(255, 50, 50));
        self.cross2.set_rotation(315f32);
        self.cross2.set_fill_color(&Color::new_RGB(255, 50, 50));
        self.cleaner.set_fill_color(&Color::new_RGB(0, 0, 0));
        let size = self.cleaner.get_size();

        self.set_cross_pos(&Vector2f{x: size.x / 2f32 + position.x,
            y: size.y / 2f32 + position.y});
        self.set_position(position);
        self
    }

    fn set_cross_pos(&mut self, position: &Vector2f) {
        self.cross1.set_position(&Vector2f{x: position.x + 3f32, y: position.y - 5f32});
        self.cross2.set_position(&Vector2f{x: position.x - 5f32, y: position.y - 5f32});
        self.need_to_draw = true;
    }

    fn convert_cross_pos(&mut self) {
        let tmp_x = self.x;
        let tmp_y = self.y;
        let tmp_size = self.cleaner.get_size();
        let tmp_pos = self.cleaner.get_position();
        let tmp_limit = self.limit;

        self.set_cross_pos(&Vector2f{x: (tmp_x + tmp_limit) * tmp_size.x / (tmp_limit * 2f32) + tmp_pos.x,
            y: (tmp_y + tmp_limit) * tmp_size.y / (tmp_limit * 2f32) + tmp_pos.y});
    }

    pub fn reset_cross_pos(&mut self) {
        self.x = 0f32;
        self.y = 0f32;
        let pos = self.cleaner.get_position();
        let size = self.cleaner.get_size();

        self.set_cross_pos(&Vector2f{x: pos.x + size.x / 2f32, y: pos.y + size.y / 2f32});
        self.text_x.set_string(format!("x: {}", self.x).as_slice());
        self.text_y.set_string(format!("y: {}", self.y).as_slice());
    }
}

impl GraphicElement for GraphicSoundPosition {
    fn new_init(size: &Vector2f, position: &Vector2f, unused: &Color, additionnal: Option<&Font>) -> GraphicSoundPosition {
        let font = match additionnal {
            Some(f) => f,
            None => fail!("Need font paramater for ProgressBar")
        };
        GraphicSoundPosition {
            circle: match rc::CircleShape::new_init(if size.x > size.y {
                size.y as f32 / 2f32 - 2f32
            } else {
                size.x as f32 / 2f32 - 2f32
            }, 50u) {
                Some(t) => t,
                None => fail!("Cannot create circle for GraphicSoundPosition")
            },
            center: match rc::CircleShape::new_init(6f32, 15u) {
                Some(t) => t,
                None => fail!("Cannot create center for GraphicSoundPosition")
            },
            cross1: match rc::RectangleShape::new_init(&Vector2f{x: 2f32, y: 13f32}) {
                Some(l) => l,
                None => fail!("Cannot create cross for GraphicSoundPosition")
            },
            cross2: match rc::RectangleShape::new_init(&Vector2f{x: 2f32, y: 13f32}) {
                Some(l) => l,
                None => fail!("Cannot create cross for GraphicSoundPosition")
            },
            cleaner: match rc::RectangleShape::new_init(&Vector2f{x: size.x, y: size.y}) {
                Some(l) => l,
                None => fail!("Cannot create cleaner for GraphicSoundPosition")
            },
            text_x: match rc::Text::new_init("x: 0", Rc::new(RefCell::new(font.clone())), 20) {
                Some(t) => t,
                None => fail!("Cannot create Text")
            },
            text_y: match rc::Text::new_init("y: 0", Rc::new(RefCell::new(font.clone())), 20) {
                Some(t) => t,
                None => fail!("Cannot create Text")
            },
            name: String::new(),
            need_to_draw: true,
            x: 0f32,
            y: 0f32,
            limit: 30f32
        }.init(position)
    }

    fn is_inside(&self, position: &Vector2f) -> bool {
        position.y >= self.cleaner.get_position().y && position.y <= self.cleaner.get_position().y + self.cleaner.get_size().y &&
        position.x >= self.cleaner.get_position().x && position.x <= self.cleaner.get_position().x + self.cleaner.get_size().x
    }

    fn clicked(&mut self, position: &Vector2f) {
        let tmp_x = self.cleaner.get_position().x + self.cleaner.get_size().x / 2f32 - position.x;
        let tmp_y = self.cleaner.get_position().y + self.cleaner.get_size().y / 2f32 - position.y;
        let res = tmp_x * tmp_x + tmp_y * tmp_y;

        if res.sqrt() <= self.circle.get_radius() {
            let radius = self.circle.get_radius() as f32;
            self.x = tmp_x * self.limit / -radius;
            self.y = tmp_y * self.limit / radius;
            self.set_cross_pos(&Vector2f{x: position.x, y: position.y});
            self.text_x.set_string(format!("x: {}", self.x).as_slice());
            self.text_y.set_string(format!("y: {}", self.y).as_slice());
        }
    }

    fn draw(&mut self, win: &mut RenderWindow) {
        win.draw(&self.cleaner);
        win.draw(&self.circle);
        win.draw(&self.text_x);
        win.draw(&self.text_y);
        win.draw(&self.center);
        win.draw(&self.cross1);
        win.draw(&self.cross2);
        self.need_to_draw = false;
    }

    fn set_position(&mut self, position: &Vector2f) {
        let radius = self.circle.get_radius();
        let center_radius = self.center.get_radius();
        let old_position = self.cross1.get_position();

        self.circle.set_position(&Vector2f{x: position.x + (self.cleaner.get_size().x - radius * 2f32) / 2f32,
            y: position.y + (self.cleaner.get_size().y - radius * 2f32) / 2f32});
        self.cleaner.set_position(&Vector2f{x: position.x, y: position.y});
        self.center.set_position(&Vector2f{x: position.x + (self.cleaner.get_size().x - center_radius * 2f32) / 2f32,
            y: position.y + (self.cleaner.get_size().y - center_radius * 2f32) / 2f32});
        self.text_x.set_position(&Vector2f{x: position.x + 1f32, y: position.y});
        self.text_y.set_position(&Vector2f{x: position.x + 1f32, y: position.y + 21f32});
        self.convert_cross_pos();
        self.need_to_draw = true;
    }

    fn get_position(&self) -> Vector2f {
        self.cleaner.get_position()
    }

    fn get_size(&self) -> Vector2f {
        self.cleaner.get_size()
    }

    fn set_size(&mut self, size: &Vector2f) {
        if size != &self.cleaner.get_size() {
            self.need_to_draw = true;
            self.cleaner.set_size(&Vector2f{x: size.x - 2f32, y: size.y - 2f32});
            self.circle.set_radius(if size.x > size.y {
                size.y as f32 / 2f32
            } else {
                size.x as f32 / 2f32
            });
            self.convert_cross_pos();
        }
    }

    fn get_min_size(&self) -> Vector2f {
        Vector2f{x: 20f32, y: 20f32}
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

    fn cursor_moved(&mut self, position: &Vector2f) {
    }

    fn mouse_leave(&mut self) {
    }
}