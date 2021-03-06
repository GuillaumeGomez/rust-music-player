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

use graphic_element::GraphicElement;
use sfml::graphics::{Color, Font, RectangleShape, RenderTarget, RenderWindow, Text};
use sfml::graphics::{Shape, Transformable};
use sfml::system::Vector2f;

pub struct GraphicPlayList<'a> {
    musics: Vec<String>,
    texts: Vec<Text<'a>>,
    to_draw: usize,
    current: usize,
    hover_element: Option<usize>,
    add_to_view: isize,
    cleaner: RectangleShape<'a>,
    need_to_draw: bool,
    has_mouse: bool,
    font: Font,
    name: String,
}

impl<'b> GraphicPlayList<'b> {
    fn init(mut self, position: &Vector2f) -> GraphicPlayList<'b> {
        self.set_position(position);
        self.set_current(0usize);
        self.cleaner.set_fill_color(&Color::rgb(0, 0, 0));
        self.cleaner.set_outline_color(&Color::rgb(255, 255, 255));
        self.cleaner.set_outline_thickness(1f32);
        self
    }

    pub fn add_music(&mut self, music: String, font: &'b Font) {
        if !self.musics.contains(&music.clone()) {
            self.musics.push(music.clone());
            let pos = if self.texts.len() > 0 {
                match self.texts.last() {
                    Some(f) => f.position(),
                    None => Vector2f {
                        x: self.cleaner.position().x + 4f32,
                        y: self.cleaner.position().y - 22f32,
                    },
                }
            } else {
                Vector2f {
                    x: self.cleaner.position().x + 4f32,
                    y: self.cleaner.position().y - 22f32,
                }
            };

            self.texts
                .push(Text::new(music.split('/').last().unwrap(), font, 20));
            let tmp = self.cleaner.position();
            self.set_position(&tmp);
        }
    }

    pub fn add_musics(&mut self, musics: &Vec<String>, font: &'b Font) {
        // for i in 0..musics.len(){
        // self.add_music(musics[i].clone(), font);
        // }
        for tmp in musics.iter() {
            self.add_music(tmp.clone(), font);
        }
    }

    pub fn set_to_add(&mut self, to_add: isize) {
        let tmp_add = to_add * 22isize;
        let max = (self.texts.len() as isize + 1isize) * 22isize;

        if self.add_to_view != to_add && tmp_add >= 0isize
            && tmp_add + self.to_draw as isize * 22isize < max
            && self.texts.len() as isize * 22isize >= (self.cleaner.size().y as isize - 1)
        {
            let mut pos = self.cleaner.position().y as isize - tmp_add as isize;
            for tmp in self.texts.iter_mut() {
                let x = tmp.position().x;
                tmp.set_position(Vector2f {
                    x: x as f32,
                    y: pos as f32,
                });
                pos += 22isize;
            }
            self.add_to_view = to_add;
            self.need_to_draw = true;
        }
    }

    pub fn set_current(&mut self, current: usize) {
        self.set_current_intern(current, false)
    }

    fn set_current_intern(&mut self, current: usize, by_click: bool) {
        if self.texts.len() > 0 && current != self.current {
            if self.current < self.texts.len() {
                self.texts[self.current].set_fill_color(&Color::rgb(255, 255, 255));
            }
            self.texts[current].set_fill_color(&Color::rgb(255, 125, 25));
            self.current = current;
            self.need_to_draw = true;
            let tmp_to_draw = self.to_draw;

            if by_click == false
                && self.texts.len() as isize * 22isize >= (self.cleaner.size().y as isize - 1)
            {
                if self.current as isize + 2isize >= self.to_draw as isize + self.add_to_view {
                    self.set_to_add(current as isize + 2isize - tmp_to_draw as isize);
                } else if (self.current as isize) < self.add_to_view {
                    self.set_to_add(current as isize);
                }
            }
        }
    }

    pub fn get_current(&self) -> usize {
        self.current
    }

    pub fn get_add_to_view(&self) -> isize {
        self.add_to_view
    }

    pub fn remove_music(&mut self, pos: usize) {
        self.texts.remove(pos);
        let tmp = Vector2f {
            x: self.cleaner.position().x,
            y: self.cleaner.position().y,
        };
        self.set_position(&tmp);
        if self.musics.len() == 0usize || self.texts.len() == 0usize {
            panic!("GraphicPlayList cannot be empty");
        }
        self.need_to_draw = true;
    }

    pub fn repeat(&mut self, pos: usize) {}
}

impl<'b> GraphicElement<'b> for GraphicPlayList<'b> {
    fn new_init(
        size: &Vector2f,
        position: &Vector2f,
        color: &Color,
        font: Option<&Font>,
    ) -> GraphicPlayList<'b> {
        GraphicPlayList {
            musics: Vec::new(),
            texts: Vec::new(),
            to_draw: 0usize,
            current: 1usize,
            cleaner: RectangleShape::with_size(Vector2f {
                x: size.x - 2f32,
                y: size.y - 2f32,
            }),
            hover_element: None,
            add_to_view: 0isize,
            need_to_draw: true,
            has_mouse: false,
            font: match font {
                Some(f) => f.clone(),
                None => panic!("GraphicPlayList needs Font"),
            },
            name: String::new(),
        }.init(position)
    }

    fn set_position(&mut self, position: &Vector2f) {
        let mut pos = position.y;
        let limit = self.cleaner.size().y - 1f32 + position.y;

        self.to_draw = 0;
        self.cleaner.set_position(Vector2f {
            x: position.x,
            y: position.y,
        });
        if self.texts.len() > 0 {
            for tmp in self.texts.iter_mut() {
                tmp.set_position(Vector2f {
                    x: self.cleaner.position().x + 4f32,
                    y: pos,
                });
                if pos < limit {
                    self.to_draw += 1;
                }
                pos += 22f32;
            }
            if self.to_draw > 0 && self.to_draw * 22usize > limit as usize + 2usize {
                self.to_draw -= 1;
            }
        }
        self.need_to_draw = true;
    }

    fn get_position(&self) -> Vector2f {
        let tmp = self.cleaner.position();

        Vector2f {
            x: tmp.x - 1f32,
            y: tmp.y - 1f32,
        }
    }

    fn set_size(&mut self, size: &Vector2f) {
        let pos = self.cleaner.position();

        self.cleaner.set_size(Vector2f {
            x: size.x - 2f32,
            y: size.y - 2f32,
        });
        self.set_position(&pos);
    }

    fn get_size(&self) -> Vector2f {
        let tmp = self.cleaner.size();

        Vector2f {
            x: tmp.x + 2f32,
            y: tmp.y + 2f32,
        }
    }

    fn get_min_size(&self) -> Vector2f {
        Vector2f { x: 50f32, y: 50f32 }
    }

    fn get_max_size(&self) -> Option<Vector2f> {
        None
    }

    fn cursor_moved(&mut self, position: &Vector2f) {
        let tmp =
            ((position.y - self.cleaner.position().y) / 22f32 + self.add_to_view as f32) as usize;

        self.need_to_draw = true;
        self.has_mouse = true;
        if tmp >= self.texts.len() {
            self.hover_element = None;
            return;
        }
        match self.hover_element {
            Some(s) => {
                if self.current == tmp {
                    self.texts[s].set_fill_color(&Color::rgb(255, 255, 255));
                    self.hover_element = None;
                } else if s != tmp {
                    self.texts[s].set_fill_color(&Color::rgb(255, 255, 255));
                    self.hover_element = Some(tmp);
                    self.texts[tmp].set_fill_color(&Color::rgb(255, 175, 100));
                }
            }
            None => {
                if self.current != tmp {
                    self.hover_element = Some(tmp);
                    self.texts[tmp].set_fill_color(&Color::rgb(255, 175, 100));
                }
            }
        }
    }

    fn clicked(&mut self, position: &Vector2f) {
        if position.y >= self.cleaner.position().y {
            let tmp = ((position.y - self.cleaner.position().y) / 22f32 + self.add_to_view as f32)
                as usize;

            self.need_to_draw = true;
            if tmp < self.texts.len() {
                self.hover_element = match self.hover_element {
                    Some(s) => {
                        self.texts[s].set_fill_color(&Color::rgb(255, 255, 255));
                        None
                    }
                    None => None,
                };
                self.set_current_intern(tmp, true);
            }
        }
    }

    fn mouse_leave(&mut self) {
        if self.has_mouse {
            match self.hover_element {
                Some(s) => {
                    self.texts[s].set_fill_color(&Color::rgb(255, 255, 255));
                    self.hover_element = None;
                    self.need_to_draw = true;
                }
                None => {}
            }
            self.has_mouse = false;
        }
    }

    fn is_inside(&self, pos: &Vector2f) -> bool {
        pos.y >= self.cleaner.position().y
            && pos.y <= self.cleaner.position().y + self.cleaner.size().y
            && pos.x >= self.cleaner.position().x
            && pos.x <= self.cleaner.position().x + self.cleaner.size().x
    }

    fn draw(&mut self, win: &mut RenderWindow) {
        let mut it = 0isize;

        win.draw(&self.cleaner);
        if self.texts.len() > 0 {
            for tmp in self.texts.iter_mut() {
                if it == self.to_draw as isize + self.add_to_view {
                    break;
                }
                if it >= self.add_to_view as isize {
                    win.draw(tmp);
                }
                it += 1;
            }
        }
        self.need_to_draw = false;
    }

    fn set_element_name(&mut self, name: &String) {
        self.name = name.clone();
    }

    fn get_element_name<'a>(&'a self) -> &'a String {
        &self.name
    }
}
