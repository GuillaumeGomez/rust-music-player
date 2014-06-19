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
use rsfml::graphics::{RenderWindow, Color, RectangleShape, Text, Font};
use std::rc::Rc;
use std::cell::RefCell;
use graphic_element::GraphicElement;

pub struct GraphicButton {
    label: rc::Text,
    button: rc::RectangleShape,
    need_to_draw: bool,
    pushed: bool,
    has_mouse: bool,
    name: String
}

impl GraphicButton {
    fn init(mut self, position: &Vector2f) -> GraphicButton {
        self.set_position(position);
        self.button.set_fill_color(&Color::new_RGB(10, 10, 10));
        self.button.set_outline_color(&Color::new_RGB(255, 255, 255));
        self.button.set_outline_thickness(1f32);
        self
    }

    pub fn set_label(&mut self, label: &String) {
        if label != &self.label.get_string() {
            self.label.set_string(label.as_slice());
            let size = self.label.get_local_bounds().width;
            self.label.set_position(&Vector2f{x: (self.button.get_size().x - 1f32 - size as f32) / 2f32 + self.button.get_position().x as f32,
                                              y: self.button.get_position().y + (self.button.get_size().y - 20f32) / 2f32 - 2f32});
            self.need_to_draw = true;
        }
    }

    pub fn is_pushed(&self) -> bool {
        self.pushed
    }

    pub fn set_pushed(&mut self, pushed: bool) {
        if self.pushed != pushed {
            self.pushed = pushed;
            if self.pushed {
                self.button.set_fill_color(&Color::new_RGB(205, 187, 100));
            } else {
                self.button.set_fill_color(&Color::new_RGB(10, 10, 10));
            }
            self.need_to_draw = true;
        }
    }
}

impl GraphicElement for GraphicButton {
    fn new_init(size: &Vector2f, position: &Vector2f, unused: &Color, font: Option<&Font>) -> GraphicButton {
        GraphicButton {
            label: match rc::Text::new_init("", Rc::new(RefCell::new(match font {
                    Some(f) => f.clone(),
                    None => fail!("GraphicButton needs Font")
                })), 20) {
                Some(t) => t,
                None => fail!("Cannot create label for GraphicButton")
            },
            button: match rc::RectangleShape::new_init(&Vector2f{x: size.x as f32 - 2f32, y: size.y as f32 - 2f32}) {
                Some(l) => l,
                None => fail!("Cannot create cleaner for GraphicButton")
            },
            need_to_draw: true,
            pushed: false,
            has_mouse: false,
            name: String::new()
        }.init(position)
    }

    fn cursor_moved(&mut self, position: &Vector2f) {
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

    fn clicked(&mut self, position: &Vector2f) {
        if self.pushed {
            self.set_pushed(false)
        } else {
            self.set_pushed(true)
        }
    }

    fn draw(&mut self, win: &mut RenderWindow) {
        if self.need_to_draw {
            win.draw(&self.button);
            win.draw(&self.label);
            self.need_to_draw = false;
        }
    }

    fn set_position(&mut self, position: &Vector2f) {
        let size = self.label.get_local_bounds().width;

        self.button.set_position(&Vector2f{x: position.x + 1f32, y: position.y + 1f32});
        self.label.set_position(&Vector2f{x: (self.button.get_size().x - 1f32 - size as f32) / 2f32 + self.button.get_position().x,
                                              y: self.button.get_position().y + (self.button.get_size().y - 20f32) / 2f32 - 2f32});
        self.need_to_draw = true;
    }

    fn get_position(&self) -> Vector2f {
        let tmp = self.button.get_position();

        Vector2f{x: tmp.x - 1f32, y: tmp.y - 1f32}
    }

    fn set_size(&mut self, size: &Vector2f) {
        let tmp = self.button.get_position();

        self.button.set_size(&Vector2f{x: size.x as f32 - 2f32, y: size.y as f32 - 2f32});
        self.set_position(&tmp);
    }

    fn get_size(&self) -> Vector2f {
        let tmp = self.button.get_size();

        Vector2f{x: tmp.x + 2f32, y: tmp.y + 2f32}
    }

    fn get_min_size(&self) -> Vector2f {
        Vector2f{x: 100f32, y: 40f32}
    }

    fn get_max_size(&self) -> Option<Vector2f> {
        None
    }

    fn is_inside(&self, pos: &Vector2f) -> bool {
        pos.y >= self.button.get_position().y && pos.y <= self.button.get_position().y + self.button.get_size().y &&
        pos.x >= self.button.get_position().x && pos.x <= self.button.get_position().x + self.button.get_size().x
    }

    fn mouse_leave(&mut self) {
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

    fn set_element_name(&mut self, name: &String) {
        self.name = name.clone();
    }

    fn get_element_name<'a>(&'a self) -> &'a String {
        &self.name
    }
}