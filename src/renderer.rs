use crossterm::{
	terminal::size,
	execute,
	cursor
};
use std::io;
use std::io::{stdout, Write};
use nalgebra::SMatrix;

use crate::current_piece::CurrentObject;

pub fn inject_buffers(
	playfield_buffer : &mut SMatrix<u8, 12, 19>,
	piecepreview_buffer : &mut SMatrix<u8, 6, 6>,
	obj : &CurrentObject, map : SMatrix<u8, 10, 18>,
	level : u8, score : u32, lines : u32) {

	playfield(playfield_buffer, &map);
	if obj.exists {
		player_object(playfield_buffer, obj);
	}
}

pub fn player_object(buffer : &mut SMatrix<u8, 12, 19>, player_obj : &CurrentObject) {

	// Set positions
	
	let x : i8 = player_obj.cx as i8;
	let x1 : i8 = player_obj.cx as i8 + player_obj.x1;
	let x2 : i8 = player_obj.cx as i8 + player_obj.x2;
	let x3 : i8 = player_obj.cx as i8 + player_obj.x3;
	let y : i8 = player_obj.cy as i8;
	let y1 : i8 = player_obj.cy as i8 + player_obj.y1;
	let y2 : i8 = player_obj.cy as i8 + player_obj.y2;
	let y3 : i8 = player_obj.cy as i8 + player_obj.y3;
	
	// Set object positions in buffer
	if check_out_of_bounds(x, y) {
		buffer[(1+x as usize, y as usize)] = player_obj.otype+1;
	}
	if check_out_of_bounds(x1, y1) {
		buffer[(1+x1 as usize, y1 as usize)] = player_obj.otype+1;
	}
	if check_out_of_bounds(x2, y2) {
		buffer[(1+x2 as usize, y2 as usize)] = player_obj.otype+1;
	}
	if check_out_of_bounds(x3, y3) {
		buffer[(1+x3 as usize, y3 as usize)] = player_obj.otype+1;
	}
}

pub fn check_out_of_bounds(x : i8, y : i8) -> bool {
	if x > -1 && x < 12 &&
		y > -1 && y < 19 {

		return true;
	}

	false
}

pub fn playfield(buffer : &mut SMatrix<u8, 12, 19>, map : &SMatrix<u8, 10, 18>) {
	// Write map (with x_offset of 1) into buffer
	for x in 0..10 {
		for y in 0..18 {
			buffer[(x+1, y)] = map[(x, y)];
		}
	}
}

pub fn border(buffer : &mut SMatrix<u8, 12, 19>) {
	// Walls
	for y in 0..19 {
		buffer[(0,  y)] = 7;
		buffer[(11, y)] = 7;
	}

	// Floor
	for x in 1..11 {
		buffer[(x, 18)] = 7;
	}
}

pub fn render_buffer(buffer : &SMatrix<u8, 12, 19>, x_offset : u8, y_offset : u8) -> io::Result<()> {

	let mut stdout = stdout();
	
	for y in 0..19 {
		execute!(stdout, cursor::MoveTo(x_offset as u16, y as u16 + y_offset as u16)).unwrap();
		for x in 0..12 {
			write!(stdout, "\x1b[38;5;{}m██", buffer[(x, y)]).unwrap(); // Reads colored pixel from buffer
		}
	}
	Ok(())
}
