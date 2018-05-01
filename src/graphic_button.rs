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
use sfml::graphics::Shape;
use sfml::graphics::Transformable;
use sfml::graphics::{Color, Font, RectangleShape, RenderTarget, RenderWindow, Text};
use sfml::system::Vector2f;

pub struct GraphicButton<'b> {
    label: Text<'b>,
    button: RectangleShape<'b>,
    need_to_draw: bool,
    pushed: bool,
    has_mouse: bool,
    name: String,
}

impl<'b> GraphicButton<'b> {
    fn init(mut self, position: &Vector2f) -> GraphicButton<'b> {
        self.set_position(position);
        self.button.set_fill_color(&Color::rgb(10, 10, 10));
        self.button.set_outline_color(&Color::rgb(255, 255, 255));
        self.button.set_outline_thickness(1f32);
        self
    }

    pub fn set_label(&mut self, label: &String) {
        if label != &self.label.string() {
            self.label.set_string(&label);
            let size = self.label.local_bounds().width;
            self.label.set_position(Vector2f {
                x: (self.button.size().x - 1f32 - size as f32) / 2f32
                    + self.button.position().x as f32,
                y: self.button.position().y + (self.button.size().y - 20f32) / 2f32 - 2f32,
            });
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
                self.button.set_fill_color(&Color::rgb(205, 187, 100));
            } else {
                self.button.set_fill_color(&Color::rgb(10, 10, 10));
            }
            self.need_to_draw = true;
        }
    }
}

impl<'b> GraphicElement<'b> for GraphicButton<'b> {
    fn new_init(
        size: &Vector2f,
        position: &Vector2f,
        unused: &Color,
        font: Option<&'b Font>,
    ) -> GraphicButton<'b> {
        GraphicButton {
            label: Text::new("", &font.clone().unwrap(), 20),
            button: RectangleShape::with_size(Vector2f {
                x: size.x as f32 - 2f32,
                y: size.y as f32 - 2f32,
            }),
            need_to_draw: true,
            pushed: false,
            has_mouse: false,
            name: String::new(),
        }.init(position)
    }

    fn cursor_moved(&mut self, position: &Vector2f) {
        if !self.has_mouse {
            let tmp = self.button.size();
            let pos = self.button.position();

            self.button.set_outline_thickness(2f32);
            self.button.set_size(Vector2f {
                x: tmp.x - 2f32,
                y: tmp.y - 2f32,
            });
            self.button.set_position(Vector2f {
                x: pos.x + 1f32,
                y: pos.y + 1f32,
            });
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
        win.draw(&self.button);
        win.draw(&self.label);
        self.need_to_draw = false;
    }

    fn set_position(&mut self, position: &Vector2f) {
        let size = self.label.local_bounds().width;

        self.button.set_position(Vector2f {
            x: position.x + 1f32,
            y: position.y + 1f32,
        });
        self.label.set_position(Vector2f {
            x: (self.button.size().x - 1f32 - size as f32) / 2f32 + self.button.position().x,
            y: self.button.position().y + (self.button.size().y - 20f32) / 2f32 - 2f32,
        });
        self.need_to_draw = true;
    }

    fn get_position(&self) -> Vector2f {
        let tmp = self.button.position();

        Vector2f {
            x: tmp.x - 1f32,
            y: tmp.y - 1f32,
        }
    }

    fn set_size(&mut self, size: &Vector2f) {
        let tmp = self.button.position();

        self.button.set_size(Vector2f {
            x: size.x as f32 - 2f32,
            y: size.y as f32 - 2f32,
        });
        self.set_position(&tmp);
    }

    fn get_size(&self) -> Vector2f {
        let tmp = self.button.size();

        Vector2f {
            x: tmp.x + 2f32,
            y: tmp.y + 2f32,
        }
    }

    fn get_min_size(&self) -> Vector2f {
        Vector2f {
            x: 100f32,
            y: 40f32,
        }
    }

    fn get_max_size(&self) -> Option<Vector2f> {
        None
    }

    fn is_inside(&self, pos: &Vector2f) -> bool {
        pos.y >= self.button.position().y
            && pos.y <= self.button.position().y + self.button.size().y
            && pos.x >= self.button.position().x
            && pos.x <= self.button.position().x + self.button.size().x
    }

    fn mouse_leave(&mut self) {
        if self.has_mouse {
            let tmp = self.button.size();
            let pos = self.button.position();

            self.button.set_outline_thickness(1f32);
            self.button.set_size(Vector2f {
                x: tmp.x + 2f32,
                y: tmp.y + 2f32,
            });
            self.button.set_position(Vector2f {
                x: pos.x - 1f32,
                y: pos.y - 1f32,
            });
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
