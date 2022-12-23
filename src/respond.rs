use crate::calc::*;
use raylib::ffi::KeyboardKey::*;
use raylib::prelude::*;

pub fn respond(f : &mut Grid, handle : &RaylibHandle) {
	if handle.is_key_down(KEY_Q) {::std::process::exit(0);}
	if handle.is_key_released(KEY_SPACE) {randomize(f);}

}