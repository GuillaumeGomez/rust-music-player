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
#![allow(unused_variables)]

use sfml::graphics::rc;
use sfml::system::vector2::{Vector2f};
use sfml::graphics::{RenderWindow, Color, RectangleShape, Font, RenderTarget};
use graphic_element::GraphicElement;

pub struct GraphicSpectrum {
    spectrum: Vec<rc::RectangleShape>,
    cleaner: rc::RectangleShape,
    to_update: bool,
    pub need_to_draw: bool,
    name: String
}

impl GraphicSpectrum {
    fn init(mut self, position: &Vector2f, color: &Color) -> GraphicSpectrum {
        let mut it = 0;

        while it < 512 {
            self.spectrum.push(match rc::RectangleShape::new_init(&Vector2f{x: 1f32, y: self.cleaner.get_size().y}) {
                Some(l) => l,
                None => panic!("Cannot create spectrum")
            });
            self.spectrum[it].set_fill_color(color);
            it += 1;
        }
        self.set_position(position);
        self.cleaner.set_fill_color(&Color::new_rgb(0, 0, 0));
        self
    }

    pub fn update_spectrum(&mut self, data_left: &Vec<f32>, data_right: &Vec<f32>) {
        if !self.to_update {
            self.to_update = true;
            return;
        }
        let mut it = 0;
        let height = self.cleaner.get_size().y;

        self.need_to_draw = true;
        self.to_update = false;
        for t_data in data_left.iter() {
            let mut tmp = *t_data * -15f32;

            if tmp < -1f32 {
                tmp = -1f32;
            }
            self.spectrum[it].set_size(&Vector2f{x: 1f32, y: height * tmp});
            it += 1;
        }
        it = 511;
        for t_data in data_right.iter() {
            let mut tmp = *t_data * -15f32;

            if tmp < -1f32 {
                tmp = -1f32;
            }
            self.spectrum[it].set_size(&Vector2f{x: 1f32, y: height * tmp});
            it -= 1;
        }
    }
}

impl GraphicElement for GraphicSpectrum {
    fn new_init(size: &Vector2f, position: &Vector2f, color: &Color, additionnal: Option<&Font>) -> GraphicSpectrum {
        GraphicSpectrum {
            spectrum: Vec::new(),
            cleaner: match rc::RectangleShape::new_init(&Vector2f{x: 512f32, y: size.y}) {
                Some(l) => l,
                None => panic!("Cannot create cleaner for GraphicSpectrum")
            },
            to_update: true,
            need_to_draw: true,
            name: String::new()
        }.init(position, color)
    }

    fn set_position(&mut self, position: &Vector2f) {
        let mut it = 0usize;

        for tmp in self.spectrum.iter_mut() {
            tmp.set_position(&Vector2f{x: it as f32 + position.x, y: self.cleaner.get_size().y + position.y});
            it += 1;
        }
        self.cleaner.set_position(&Vector2f{x: position.x, y: position.y});
        self.need_to_draw = true;
    }

    fn get_position(&self) -> Vector2f {
        self.cleaner.get_position()
    }

    fn set_size(&mut self, size: &Vector2f) {
        self.cleaner.set_size(&Vector2f{x: 512f32, y: size.y});
        self.need_to_draw = true;
    }

    fn get_size(&self) -> Vector2f {
        self.cleaner.get_size()
    }

    fn draw(&mut self, win: &mut RenderWindow) {
        win.draw(&self.cleaner);
        for tmp in self.spectrum.iter_mut() {
            win.draw(tmp);
        }
        self.need_to_draw = false;
    }

    fn is_inside(&self, pos: &Vector2f) -> bool {
        pos.y >= self.cleaner.get_position().y && pos.y <= self.cleaner.get_position().y + self.cleaner.get_size().y &&
        pos.x >= self.cleaner.get_position().x && pos.x <= self.cleaner.get_position().x + self.cleaner.get_size().x
    }

    fn get_min_size(&self) -> Vector2f {
        Vector2f{x: 20f32, y: 20f32}
    }

    fn get_max_size(&self) -> Option<Vector2f> {
        Some(Vector2f{x: 512f32, y: 100000000f32})
    }

    fn set_element_name(&mut self, name: &String) {
        self.name = name.clone();
    }

    fn get_element_name<'a>(&'a self) -> &'a String {
        &self.name
    }

    fn cursor_moved(&mut self, position: &Vector2f) {
    }

    fn clicked(&mut self, position: &Vector2f) {
    }

    fn mouse_leave(&mut self) {
    }
}