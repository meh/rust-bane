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

use std::io::{Read, Write};

use error;
use terminal::Terminal;

#[derive(Debug)]
pub struct Cursor<'a, I: Read + 'a, O: Write + 'a> {
	inner: &'a mut Terminal<I, O>
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Travel {
	Up(u32),
	Down(u32),
	Left(u32),
	Right(u32),
	To(Option<u32>, Option<u32>),
}

impl<'a, I: Read + 'a, O: Write + 'a> Cursor<'a, I, O> {
	pub fn new<'b: 'a>(inner: &'b mut Terminal<I, O>) -> Cursor<'b, I, O> {
		Cursor {
			inner: inner,
		}
	}

	pub fn invisible(&mut self) -> error::Result<&mut Self> {
		expand!(self.inner => CursorInvisible)?;

		Ok(self)
	}

	pub fn normal(&mut self) -> error::Result<&mut Self> {
		expand!(self.inner => CursorNormal)?;

		Ok(self)
	}

	pub fn visible(&mut self) -> error::Result<&mut Self> {
		expand!(self.inner => CursorVisible)?;

		Ok(self)
	}

	pub fn travel(&mut self, value: Travel) -> error::Result<&mut Self> {
		match value {
			Travel::Up(n) if n == 1 =>
				if expand!(self.inner => CursorUp).is_err() {
					expand!(self.inner => ParmUpCursor; 1)?;
				},

			Travel::Up(n) =>
				if expand!(self.inner => ParmUpCursor; n).is_err() {
					for _ in 0 .. n {
						expand!(self.inner => CursorUp)?;
					}
				},

			Travel::Down(n) if n == 1 =>
				if expand!(self.inner => CursorDown).is_err() {
					expand!(self.inner => ParmDownCursor; 1)?;
				},

			Travel::Down(n) =>
				if expand!(self.inner => ParmDownCursor; n).is_err() {
					for _ in 0 .. n {
						expand!(self.inner => CursorDown)?;
					}
				},

			Travel::Left(n) if n == 1 =>
				if expand!(self.inner => CursorLeft).is_err() {
					expand!(self.inner => ParmLeftCursor; 1)?;
				},

			Travel::Left(n) =>
				if expand!(self.inner => ParmLeftCursor; n).is_err() {
					for _ in 0 .. n {
						expand!(self.inner => CursorLeft)?;
					}
				},

			Travel::Right(n) if n == 1 =>
				if expand!(self.inner => CursorRight).is_err() {
					expand!(self.inner => ParmRightCursor; 1)?;
				},

			Travel::Right(n) =>
				if expand!(self.inner => ParmRightCursor; n).is_err() {
					for _ in 0 .. n {
						expand!(self.inner => CursorRight)?;
					}
				},

			Travel::To(Some(x), Some(y)) =>
				if expand!(self.inner => CursorAddress; y, x).is_err() {
					expand!(self.inner => RowAddress; y)?;
					expand!(self.inner => ColumnAddress; x)?;
				},

			Travel::To(Some(x), None) =>
				expand!(self.inner => ColumnAddress; x)?,

			Travel::To(None, Some(y)) =>
				expand!(self.inner => RowAddress; y)?,

			Travel::To(None, None) =>
				(),
		}

		Ok(self)
	}
}
