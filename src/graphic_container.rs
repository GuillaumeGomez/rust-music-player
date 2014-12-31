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

use rsfml::system::vector2::{Vector2f};
use rsfml::window::{event, keyboard, mouse};
use rsfml::graphics::{RenderWindow, Color, Font, RenderTarget};
use rfmod;
use playlist::PlayList;
use graphic_timer::GraphicTimer;
use graphic_spectrum::GraphicSpectrum;
use graphic_playlist::GraphicPlayList;
use progress_bar::ProgressBar;
use graphic_button::GraphicButton;
use graphic_sound_position::GraphicSoundPosition;
use graphic_element::GraphicElement;
use std::default::Default;

pub struct GraphicContainer {
    pub buttons: Vec<GraphicButtons>,
    pub widgets: Vec<GraphicElement>,
    surface: rc::RectangleShape,
    current_tab: uint
}

impl GraphicContainer {
    fn init(mut self, position: &Vector2f) -> GraphicContainer {
        self.set_position(position);
        self
    }

    pub fn get_current_tab(&self) -> uint {
        self.current_tab
    }

    pub add_tab(&self, label: &str, widget: GraphicElement) {
    	let new_width = self.get_size().x / buttons.len() as f32 + 1f32;
	let mut pos = 0f32 + self.get_position().x;
        let mut x_changed = false;
        let mut y_changed = false;

        if widget.get_size().x > self.get_size().x {
            self.set_size(&Vector2f{x: widget.get_size().x, y: self.get_size().y);
            x_changed = true;
        }
        if widget.get_size().y > self.get_size().y - 25f32 {
            self.set_size(&Vector2f{x: self.get_size().x, y: widget.get_size().y + 25f32);
            y_changed = true;
        }
	for it in buttons.mut_iter() {
	    it.set_position(&Vector2f{x: pos, y: self.get_position().y});
	    it.set_size(&Vector2f{x: new_width, y: it.get_size().y});
	    pos += new_width;
	}
        if x_changed || y_changed {
            let new_size = Vector2f {
                x: if x_changed { widget.get_size().x } else { self.get_size().x },
                y: if y_changed { widget.get_size().y } else { self.get_size().y - 25f32 }
            };
            for it in widgets.mut_iter() {
                it.set_size(&new_size);
            }
            self.set_size(&Vector2f { x: new_size.x, y: new_size.y + 25f32 };
        }
    	let mut tmp = GraphicButton::new_init(&Vector2f{x: self.get_size().x / buttons.len() as f32 + 1f32, y: 25f32},
	    &Vector2f{x: pos, y: self.get_position().y});

	tmp.set_label(label);
	buttons.push(tmp);
	widgets.push(widget);
    }
}

impl GraphicElement for GraphicContainer {
    fn new_init(size: &Vector2f, position: &Vector2f, unused: &Color, font: Option<&Font>) -> GraphicContainer {
        GraphicContainer {
            widgets: Vec::new(),
            buttons: Vec::new(),
            current_tab: 0,
            surface: match rc::RectangleShape::new_init(&Vector2f{x: 40f32, y: 26f32}) {
                Some(l) => l,
                None => panic!("Cannot create GraphicContainer")
            }
        }
    }

    fn cursor_moved(&mut self, position: &Vector2f) {
        for it in self.buttons.mut_iter() {
            if it.is_inside(position) {
                it.cursor_moved(position);
            } else {
                it.mouse_leave();
            }
        }
        for it in self.widgets.mut_iter() {
            if it.is_inside(position) {
                it.cursor_moved(position);
            } else {
                it.mouse_leave();
            }
        }
    }

    fn clicked(&mut self, position: &Vector2f) {
        if position.y <= self.get_position().y + 25f32 {
            let mut pos = 0u;

            for it in self.buttons.mut_iter() {
                if it.is_inside(position) {
                    if it.is_pushed() {
                        return;
                    }
                    self.buttons[current_tab].clicked(position);
                    it.clicked(position);
                    current_tab = pos;
                    self.widgets[current_tab].need_to_draw = true;
                }
                pos += 1;
            }
        } else {
            self.widgets[current_tab].clicked(position);
        }
    }

    fn draw(&mut self, win: &mut RenderWindow) {
        for it in buttons.mut_iter() {
            win.draw(it);
        }
        win.draw(&self.widgets[current_tab]);
    }

    fn set_position(&mut self, position: &Vector2f) {
        if position.x != self.get_position().x || position.y != self.get_position().y {
            let mut pos = Vector2f { x: position.x, y: position.y };
            let decal = self.get_size().x / self.buttons.len() as f32;

            self.surface.set_position(position);
            for it in self.buttons.mut_iter() {
                it.set_position(pos);
                pos.x += decal;
            }
            pos.x = position.x;
            pos.y += 25f32;
            for it in self.widgets.mut_iter() {
                it.set_position(pos);
            }
        }
    }

    fn get_position(&self) -> Vector2f {
        self.surface.get_position()
    }

    fn set_size(&mut self, size: &Vector2f) {
        if size.x != self.get_size().x && size.y != self.get_size().y {
            for it in self.widgets.mut_iter() {
                if size < it.get_min_size() || size > it.get_max_size() {
                    return;
                }
            }
            let new_width = size.x / self.buttons.len() as f32;

            for it in self.buttons.mut_iter() {
                it.set_size(&Vector2f{x: new_width, y: 25f32});
            }
	    for it in self.widgets.mut_iter() {
                it.set_size(&Vector2f{x: new_width, y: size.y - 25f32});
            }
            self.surface.set_size(size);
        }
    }

    fn get_size(&self) -> Vector2f {
        self.surface.get_size();
    }

    fn get_min_size(&self) -> Vector2f {
        Vector2f{x: 40f32, y: 26f32}
    }

    fn get_max_size(&self) -> Option<Vector2f> {
        None
    }

    fn is_inside(&self, pos: &Vector2f) -> bool {
        pos.y >= self.surface.get_position().y && pos.y <= self.surface.get_position().y + self.surface.get_size().y &&
        pos.x >= self.surface.get_position().x && pos.x <= self.surface.get_position().x + self.surface.get_size().x
    }

    fn mouse_leave(&mut self) {
        self.widgets[self.current_tab].mouse_leave();
        for it in self.buttons.mut_iter() {
            it.mouse_leave();
        }
    }

    fn set_element_name(&mut self, name: &String) {
        self.name = name.clone();
    }

    fn get_element_name<'a>(&'a self) -> &'a String {
        &self.name
    }
}