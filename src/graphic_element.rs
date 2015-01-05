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
use rsfml::graphics::{RenderWindow, Color, Font};
use std::kinds::Sized;

pub trait GraphicElement: Sized {
	fn new_init(size: &Vector2f, position: &Vector2f, color: &Color, additional: Option<&Font>) -> Self;
	fn draw(&mut self, window: &mut RenderWindow);
	fn is_inside(&self, position: &Vector2f) -> bool;
	fn clicked(&mut self, position: &Vector2f);
	fn set_position(&mut self, position: &Vector2f);
	fn get_position(&self) -> Vector2f;
	fn set_size(&mut self, size: &Vector2f);
	fn get_size(&self) -> Vector2f;
	fn get_min_size(&self) -> Vector2f;
	fn get_max_size(&self) -> Option<Vector2f>;
	fn get_element_name<'a>(&'a self) -> &'a String;
	fn set_element_name(&mut self, name: &String);
	fn cursor_moved(&mut self, position: &Vector2f);
	fn mouse_leave(&mut self);
}