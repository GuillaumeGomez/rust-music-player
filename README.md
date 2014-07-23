rust-music-player [![Build Status](https://api.travis-ci.org/GuillaumeGomez/rust-music-player.png?branch=master)](https://travis-ci.org/GuillaumeGomez/rust-music-player)
=================

A little music player in rust with rsfml and rfmod.


##Installation

You must install on your computer the rfmod and the rsfml bindings before compiling this project.

rfmod: https://github.com/GuillaumeGomez/rust-fmod

rsfml: https://github.com/jeremyletang/rust-sfml

Then create a `lib` folder where you put the above compiled libraries files to compile with `make` command.

###How to

Here is the list of the binded keyboards keys :
 * ESC : exit the program
 * Up / Down : change the music
 * Add / Subtract : change the music volume
 * Space : pause / unpause current music
 * BackSpace : reset user position (in 3D)
 * Delete : remove the current music

You can also interact with the software like this :
 * you can scroll the playlist
 * you can click on a music to play it
 * you can click on the music progress bar to go to precise position
 * you can click on the volume progress bar to change the music's volume
 * you can click to change your 3D position

 You can have all of these instructions when you launch the player with the "-h" option or the "--help" option :

 ```Shell
 music_player -h
 music_player --help
 ```

 To just start the music player :

 ```Shell
 music_player music1 music2
 ```


##License
	Copyright (c) 2014 Guillaume Gomez

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

	http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.
