//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

use std::mem;
use std::os::unix::io::RawFd;

use libc::{ioctl, winsize, TIOCGWINSZ};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Size {
	/// The number of horizontal cells.
	pub columns: u32,

	/// The number of vertical cells.
	pub rows: u32,

	/// The number of horizontal pixels.
	pub width: u32,

	/// The number of vertical pixels.
	pub height: u32,
}

/// Get the terminal size.
pub fn get(fd: RawFd) -> Option<Size> {
	unsafe {
		let mut size: winsize = mem::zeroed();
		
		if ioctl(fd, TIOCGWINSZ, &mut size) != 0 {
			return None;
		}

		Some(Size {
			columns: size.ws_col as u32,
			rows:    size.ws_row as u32,

			width:  size.ws_xpixel as u32,
			height: size.ws_ypixel as u32,
		})
	}
}

impl Size {
	/// The number of horizontal cells.
	pub fn columns(&self) -> u32 {
		self.columns
	}

	/// The number of vertical cells.
	pub fn rows(&self) -> u32 {
		self.rows
	}

	/// The number of horizontal pixels.
	pub fn width(&self) -> u32 {
		self.width
	}

	/// The number of vertical pixels.
	pub fn height(&self) -> u32 {
		self.height
	}

	/// The size of a single cell in pixels.
	pub fn cell(&self) -> (u32, u32) {
		if self.width == 0 || self.height == 0 {
			(0, 0)
		}
		else {
			(self.width / self.columns, self.height / self.rows)
		}
	}
}
