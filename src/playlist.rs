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
use std::old_io::fs::PathExtensions;

pub struct PlayList {
    musics: Vec<String>,
    actual: usize
}

impl PlayList {
    pub fn to_vec(&self) -> Vec<String> {
        self.musics.clone()
    }

    pub fn new() -> PlayList {
        PlayList {
            musics: Vec::new(),
            actual: 0us
        }
    }

    fn init(mut self) -> PlayList {
        let mut tmp = Vec::new();

        for it in self.musics.iter() {
            let fd = Path::new(it.as_slice());

            if fd.is_file() {
                tmp.push(it.clone());
            }
        }
        self.musics = tmp.clone();
        self
    }

    pub fn from_vec(vec: &Vec<String>) -> PlayList {
        PlayList {
            musics: vec.clone(),
            actual: 0us
        }.init()
    }

    pub fn set_actual(&mut self, actual: usize) {
        self.actual = if self.musics.len() <= actual {
            self.musics.len() - 1
        } else {
            actual
        };

        if self.musics.len() == 0 {
            self.actual = 0;
        }
    }

    pub fn get_next(&mut self) -> String {
        self.actual = if self.musics.len() == 0 {
            0us
        } else if self.actual >= self.musics.len() - 1 {
            0us
        } else {
            self.actual + 1
        };

        self.musics[self.actual].clone()
    }

    pub fn get_prev(&mut self) -> String {
        self.actual = if self.actual == 0 {
            if self.musics.len() > 0 {
                self.musics.len() - 1
            } else {
                0us
            }
        } else {
            self.actual - 1
        };

        self.musics[self.actual].clone()
    }

    pub fn add_music(&mut self, music: String) {
        if !self.musics.contains(&music) {
            self.musics.push(music)
        }
    }

    pub fn add_musics(&mut self, musics: Vec<String>) {
        for tmp in musics.iter() {
            self.add_music(tmp.clone())
        }
    }

    pub fn start(&self) -> String {
        self.musics[0].clone()
    }

    pub fn remove_current(&mut self) {
        if self.musics.len() > 0 {
            self.musics.remove(self.actual);
            if self.musics.len() == 0 {
                self.actual = 0;
            } else if self.musics.len() <= self.actual {
                self.actual = self.musics.len() - 1
            }
        }
    }

    pub fn get_current(&self) -> String {
        self.musics[self.actual].clone()
    }

    pub fn get_nb_musics(&self) -> usize {
        self.musics.len()
    }

    pub fn get_pos(&self) -> usize {
        self.actual
    }
}