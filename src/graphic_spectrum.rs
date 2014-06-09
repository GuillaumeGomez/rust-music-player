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
    height: uint,
    cleaner: rc::RectangleShape
}

impl GraphicSpectrum {
    fn init(mut self) -> GraphicSpectrum {
        let mut it = 0;

        while it < 512 {
            self.spectrum.push(match rc::RectangleShape::new_init(&Vector2f{x: 1f32, y: self.height as f32}) {
                Some(l) => l,
                None => fail!("Cannot create spectrum")
            });
            self.spectrum.get_mut(it).set_fill_color(&Color::new_RGB(50, 100, 30));
            self.spectrum.get_mut(it).set_position(&Vector2f{x: it as f32, y: self.height as f32});
            it += 1;
        }
        self.cleaner.set_fill_color(&Color::new_RGB(0, 0, 0));
        self
    }

    pub fn new(height: uint) -> GraphicSpectrum {
        GraphicSpectrum {
            spectrum: Vec::new(),
            height: height,
            cleaner: match rc::RectangleShape::new_init(&Vector2f{x: 512f32, y: height as f32}) {
                Some(l) => l,
                None => fail!("Cannot create cleaner for GraphicSpectrum")
            },
        }.init()
    }

    pub fn update_spectrum(&mut self, data: Vec<f32>) {
        let mut it = 0u;

        for tmp in data.iter() {
            self.spectrum.get_mut(it).set_size(&Vector2f{x: 1f32, y: self.height as f32 * *tmp * -20f32});
            it += 1;
        }
    }

    pub fn draw(&mut self, win: &mut RenderWindow) {
        win.draw(&self.cleaner);
        for tmp in self.spectrum.mut_iter() {
            win.draw(tmp);
        }
    }

    pub fn is_inside(&self, pos: &Vector2u) -> bool {
        pos.y <= self.height as u32 && pos.x <= 512
    }
}