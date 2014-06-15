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

pub struct GraphicSpectrum {
    spectrum: Vec<rc::RectangleShape>,
    cleaner: rc::RectangleShape,
    to_update: bool,
    pub need_to_draw: bool
}

impl GraphicSpectrum {
    fn init(mut self, position: &Vector2u) -> GraphicSpectrum {
        let mut it = 0;

        while it < 512 {
            self.spectrum.push(match rc::RectangleShape::new_init(&Vector2f{x: 1f32, y: self.cleaner.get_size().y as f32}) {
                Some(l) => l,
                None => fail!("Cannot create spectrum")
            });
            self.spectrum.get_mut(it).set_fill_color(&Color::new_RGB(50, 100, 30));
            it += 1;
        }
        self.set_position(position);
        self.cleaner.set_fill_color(&Color::new_RGB(0, 0, 0));
        self
    }

    pub fn new(height: uint, position: &Vector2u) -> GraphicSpectrum {
        GraphicSpectrum {
            spectrum: Vec::new(),
            cleaner: match rc::RectangleShape::new_init(&Vector2f{x: 512f32, y: height as f32}) {
                Some(l) => l,
                None => fail!("Cannot create cleaner for GraphicSpectrum")
            },
            to_update: true,
            need_to_draw: true
        }.init(position)
    }

    pub fn set_position(&mut self, position: &Vector2u) {
        let mut it = 0;

        for tmp in self.spectrum.mut_iter() {
            tmp.set_position(&Vector2f{x: it as f32 + position.x as f32, y: self.cleaner.get_size().y as f32 + position.y as f32});
            it += 1;
        }
        self.cleaner.set_position(&Vector2f{x: position.x as f32, y: position.y as f32});
        self.need_to_draw = true;
    }

    pub fn update_spectrum(&mut self, data_left: Vec<f32>, data_right: Vec<f32>) {
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
            self.spectrum.get_mut(it).set_size(&Vector2f{x: 1f32, y: height * tmp});
            it += 1;
        }
        it = 511;
        for t_data in data_right.iter() {
            let mut tmp = *t_data * -15f32;

            if tmp < -1f32 {
                tmp = -1f32;
            }
            self.spectrum.get_mut(it).set_size(&Vector2f{x: 1f32, y: height * tmp});
            it -= 1;
        }
    }

    pub fn draw(&mut self, win: &mut RenderWindow) {
        if self.need_to_draw {
            win.draw(&self.cleaner);
            for tmp in self.spectrum.mut_iter() {
                win.draw(tmp);
            }
            self.need_to_draw = false;
        }
    }

    pub fn is_inside(&self, pos: &Vector2u) -> bool {
        pos.y as f32 >= self.cleaner.get_position().y && pos.y as f32 <= self.cleaner.get_position().y + self.cleaner.get_size().y &&
        pos.x as f32 >= self.cleaner.get_position().x && pos.x as f32 <= self.cleaner.get_position().x + self.cleaner.get_size().x
    }
}