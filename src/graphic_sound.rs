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
use rsfml::graphics::{RenderWindow, Color, RectangleShape, CircleShape, Font};
use std::rc::Rc;
use std::cell::RefCell;

pub struct GraphicSound {
    circle: rc::CircleShape,
    center: rc::CircleShape,
    cross1: rc::RectangleShape,
    cross2: rc::RectangleShape,
    cleaner: rc::RectangleShape,
    text_x: rc::Text,
    text_y: rc::Text,
    pub need_to_draw: bool,
    pub x: f32,
    pub y: f32
}

impl GraphicSound {
    pub fn new_init(font: &Font, size: &Vector2u, position: &Vector2u) -> GraphicSound {
        GraphicSound {
            circle: match rc::CircleShape::new_init(if size.x > size.y {
                size.y as f32 / 2f32
            } else {
                size.x as f32 / 2f32
            }, 50u) {
                Some(t) => t,
                None => fail!("Cannot create circle for GraphicSound")
            },
            center: match rc::CircleShape::new_init(6f32, 15u) {
                Some(t) => t,
                None => fail!("Cannot create center for GraphicSound")
            },
            cross1: match rc::RectangleShape::new_init(&Vector2f{x: 2f32, y: 13f32}) {
                Some(l) => l,
                None => fail!("Cannot create cross for GraphicSound")
            },
            cross2: match rc::RectangleShape::new_init(&Vector2f{x: 2f32, y: 13f32}) {
                Some(l) => l,
                None => fail!("Cannot create cross for GraphicSound")
            },
            cleaner: match rc::RectangleShape::new_init(&Vector2f{x: size.x as f32, y: size.y as f32}) {
                Some(l) => l,
                None => fail!("Cannot create cleaner for GraphicSound")
            },
            text_x: match rc::Text::new_init("x: 0", Rc::new(RefCell::new(font.clone())), 20) {
                Some(t) => t,
                None => fail!("Cannot create Text")
            },
            text_y: match rc::Text::new_init("y: 0", Rc::new(RefCell::new(font.clone())), 20) {
                Some(t) => t,
                None => fail!("Cannot create Text")
            },
            need_to_draw: true,
            x: 0f32,
            y: 0f32
        }.init(position)
    }

    fn init(mut self, position: &Vector2u) -> GraphicSound {
        self.circle.set_fill_color(&Color::new_RGB(0, 0, 0));
        self.circle.set_outline_color(&Color::new_RGB(255, 255, 255));
        self.circle.set_outline_thickness(1f32);
        self.cross1.set_rotation(45f32);
        self.cross1.set_fill_color(&Color::new_RGB(255, 50, 50));
        self.cross2.set_rotation(315f32);
        self.cross2.set_fill_color(&Color::new_RGB(255, 50, 50));
        self.cleaner.set_fill_color(&Color::new_RGB(0, 0, 0));
        self.set_position(position);
        self
    }

    pub fn draw(&mut self, win: &mut RenderWindow) {
        if self.need_to_draw {
            win.draw(&self.cleaner);
            win.draw(&self.circle);
            win.draw(&self.text_x);
            win.draw(&self.text_y);
            win.draw(&self.center);
            win.draw(&self.cross1);
            win.draw(&self.cross2);
            self.need_to_draw = false;
        }
    }

    pub fn set_position(&mut self, position: &Vector2u) {
        let radius = self.circle.get_radius();
        let center_radius = self.center.get_radius();
        let size = self.cleaner.get_size();

        self.circle.set_position(&Vector2f{x: position.x as f32 + (self.cleaner.get_size().x - radius * 2f32) / 2f32,
            y: position.y as f32 + (self.cleaner.get_size().y - radius * 2f32) / 2f32});
        self.cleaner.set_position(&Vector2f{x: position.x as f32, y: position.y as f32});
        self.center.set_position(&Vector2f{x: position.x as f32 + (self.cleaner.get_size().x - center_radius * 2f32) / 2f32,
            y: position.y as f32 + (self.cleaner.get_size().y - center_radius * 2f32) / 2f32});
        self.text_x.set_position(&Vector2f{x: position.x as f32, y: position.y as f32});
        self.text_y.set_position(&Vector2f{x: position.x as f32, y: position.y as f32 + 21f32});
        self.set_cross_pos(&Vector2f{x: size.x / 2f32 + position.x as f32,
            y: size.y / 2f32 + position.y as f32});
        self.need_to_draw = true;
    }

    pub fn is_inside(&self, pos: &Vector2u) -> bool {
        pos.y as f32 >= self.cleaner.get_position().y && pos.y as f32 <= self.cleaner.get_position().y + self.cleaner.get_size().y &&
        pos.x as f32 >= self.cleaner.get_position().x && pos.x as f32 <= self.cleaner.get_position().x + self.cleaner.get_size().x
    }

    fn set_cross_pos(&mut self, position: &Vector2f) {
        self.cross1.set_position(&Vector2f{x: position.x + 3f32, y: position.y - 5f32});
        self.cross2.set_position(&Vector2f{x: position.x - 5f32, y: position.y - 5f32});
        self.need_to_draw = true;
    }

    pub fn clicked(&mut self, position: &Vector2u) {
        let tmp_x = self.cleaner.get_position().x + self.cleaner.get_size().x / 2f32 - position.x as f32;
        let tmp_y = self.cleaner.get_position().y + self.cleaner.get_size().y / 2f32 - position.y as f32;
        let res = tmp_x * tmp_x + tmp_y * tmp_y;

        if res.sqrt() <= self.circle.get_radius() {
            let radius = self.circle.get_radius() as f32;
            self.x = tmp_x * 100f32 / -radius;
            self.y = tmp_y * 100f32 / radius;
            self.set_cross_pos(&Vector2f{x: position.x as f32, y: position.y as f32});
            self.text_x.set_string(format!("x: {}", self.x).as_slice());
            self.text_y.set_string(format!("y: {}", self.y).as_slice());
        }
    }
}