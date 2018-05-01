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
use sfml::graphics::Text;
use sfml::graphics::Transformable;
use sfml::graphics::{CircleShape, Color, Font, RectangleShape, RenderTarget, RenderWindow};
use sfml::system::Vector2f;

pub struct GraphicSoundPosition<'b> {
    circle: CircleShape<'b>,
    center: CircleShape<'b>,
    cross1: RectangleShape<'b>,
    cross2: RectangleShape<'b>,
    cleaner: RectangleShape<'b>,
    text_x: Text<'b>,
    text_y: Text<'b>,
    name: String,
    pub need_to_draw: bool,
    pub x: f32,
    pub y: f32,
    pub limit: f32,
}

impl<'b> GraphicSoundPosition<'b> {
    fn init(mut self, position: &Vector2f) -> GraphicSoundPosition<'b> {
        self.circle.set_fill_color(&Color::rgb(0, 0, 0));
        self.circle.set_outline_color(&Color::rgb(255, 255, 255));
        self.circle.set_outline_thickness(1f32);
        self.cross1.set_rotation(45f32);
        self.cross1.set_fill_color(&Color::rgb(255, 50, 50));
        self.cross2.set_rotation(315f32);
        self.cross2.set_fill_color(&Color::rgb(255, 50, 50));
        self.cleaner.set_fill_color(&Color::rgb(0, 0, 0));
        let size = self.cleaner.size();

        self.set_cross_pos(&Vector2f {
            x: size.x / 2f32 + position.x,
            y: size.y / 2f32 + position.y,
        });
        self.set_position(position);
        self
    }

    fn set_cross_pos(&mut self, position: &Vector2f) {
        self.cross1.set_position(Vector2f {
            x: position.x + 3f32,
            y: position.y - 5f32,
        });
        self.cross2.set_position(Vector2f {
            x: position.x - 5f32,
            y: position.y - 5f32,
        });
        self.need_to_draw = true;
    }

    fn convert_cross_pos(&mut self) {
        let tmp_x = self.x;
        let tmp_y = self.y;
        let tmp_size = self.cleaner.size();
        let tmp_pos = self.cleaner.position();
        let tmp_limit = self.limit;

        self.set_cross_pos(&Vector2f {
            x: (tmp_x + tmp_limit) * tmp_size.x / (tmp_limit * 2f32) + tmp_pos.x,
            y: (tmp_y + tmp_limit) * tmp_size.y / (tmp_limit * 2f32) + tmp_pos.y,
        });
    }

    pub fn reset_cross_pos(&mut self) {
        self.x = 0f32;
        self.y = 0f32;
        let pos = self.cleaner.position();
        let size = self.cleaner.size();

        self.set_cross_pos(&Vector2f {
            x: pos.x + size.x / 2f32,
            y: pos.y + size.y / 2f32,
        });
        self.text_x.set_string(&format!("x: {}", self.x));
        self.text_y.set_string(&format!("y: {}", self.y));
    }
}

impl<'b> GraphicElement<'b> for GraphicSoundPosition<'b> {
    fn new_init(
        size: &Vector2f,
        position: &Vector2f,
        unused: &Color,
        additionnal: Option<&'b Font>,
    ) -> GraphicSoundPosition<'b> {
        let font = match additionnal {
            Some(f) => f,
            None => panic!("Need font paramater for ProgressBar"),
        };
        GraphicSoundPosition {
            circle: CircleShape::new(
                if size.x > size.y {
                    size.y as f32 / 2f32 - 2f32
                } else {
                    size.x as f32 / 2f32 - 2f32
                },
                50u32,
            ),
            center: CircleShape::new(6f32, 15u32),
            cross1: RectangleShape::with_size(Vector2f { x: 2f32, y: 13f32 }),
            cross2: RectangleShape::with_size(Vector2f { x: 2f32, y: 13f32 }),
            cleaner: RectangleShape::with_size(Vector2f {
                x: size.x,
                y: size.y,
            }),
            text_x: Text::new("x: 0", &additionnal.unwrap(), 20),
            text_y: Text::new("y: 0", &additionnal.unwrap(), 20),
            name: String::new(),
            need_to_draw: true,
            x: 0f32,
            y: 0f32,
            limit: 30f32,
        }.init(position)
    }

    fn is_inside(&self, position: &Vector2f) -> bool {
        position.y >= self.cleaner.position().y
            && position.y <= self.cleaner.position().y + self.cleaner.size().y
            && position.x >= self.cleaner.position().x
            && position.x <= self.cleaner.position().x + self.cleaner.size().x
    }

    fn clicked(&mut self, position: &Vector2f) {
        let tmp_x = self.cleaner.position().x + self.cleaner.size().x / 2f32 - position.x;
        let tmp_y = self.cleaner.position().y + self.cleaner.size().y / 2f32 - position.y;
        let res = tmp_x * tmp_x + tmp_y * tmp_y;

        if res.sqrt() <= self.circle.radius() {
            let radius = self.circle.radius() as f32;
            self.x = tmp_x * self.limit / -radius;
            self.y = tmp_y * self.limit / radius;
            self.set_cross_pos(&Vector2f {
                x: position.x,
                y: position.y,
            });
            self.text_x.set_string(&format!("x: {}", self.x));
            self.text_y.set_string(&format!("y: {}", self.y));
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
        let radius = self.circle.radius();
        let center_radius = self.center.radius();
        let old_position = self.cross1.position();

        self.circle.set_position(Vector2f {
            x: position.x + (self.cleaner.size().x - radius * 2f32) / 2f32,
            y: position.y + (self.cleaner.size().y - radius * 2f32) / 2f32,
        });
        self.cleaner.set_position(Vector2f {
            x: position.x,
            y: position.y,
        });
        self.center.set_position(Vector2f {
            x: position.x + (self.cleaner.size().x - center_radius * 2f32) / 2f32,
            y: position.y + (self.cleaner.size().y - center_radius * 2f32) / 2f32,
        });
        self.text_x.set_position(Vector2f {
            x: position.x + 1f32,
            y: position.y,
        });
        self.text_y.set_position(Vector2f {
            x: position.x + 1f32,
            y: position.y + 21f32,
        });
        self.convert_cross_pos();
        self.need_to_draw = true;
    }

    fn get_position(&self) -> Vector2f {
        self.cleaner.position()
    }

    fn get_size(&self) -> Vector2f {
        self.cleaner.size()
    }

    fn set_size(&mut self, size: &Vector2f) {
        if size != &self.cleaner.size() {
            self.need_to_draw = true;
            self.cleaner.set_size(Vector2f {
                x: size.x - 2f32,
                y: size.y - 2f32,
            });
            self.circle.set_radius(if size.x > size.y {
                size.y as f32 / 2f32
            } else {
                size.x as f32 / 2f32
            });
            self.convert_cross_pos();
        }
    }

    fn get_min_size(&self) -> Vector2f {
        Vector2f { x: 20f32, y: 20f32 }
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

    fn cursor_moved(&mut self, position: &Vector2f) {}

    fn mouse_leave(&mut self) {}
}
