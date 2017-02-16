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

/// Cursor movements.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Travel {
	Up(u32),
	Down(u32),
	Left(u32),
	Right(u32),
	To(Option<u32>, Option<u32>),
}

impl<'a, I: Read + 'a, O: Write + 'a> Cursor<'a, I, O> {
	#[doc(hidden)]
	pub fn new<'b: 'a>(inner: &'b mut Terminal<I, O>) -> Cursor<'b, I, O> {
		Cursor {
			inner: inner,
		}
	}

	/// Make the cursor invisible.
	pub fn invisible(&mut self) -> error::Result<&mut Self> {
		expand!(self.inner => CursorInvisible)?;

		Ok(self)
	}

	/// Make the cursor normal.
	pub fn normal(&mut self) -> error::Result<&mut Self> {
		expand!(self.inner => CursorNormal)?;

		Ok(self)
	}

	/// Make the cursor very visible.
	pub fn visible(&mut self) -> error::Result<&mut Self> {
		expand!(self.inner => CursorVisible)?;

		Ok(self)
	}

	/// Move the cursor.
	pub fn travel(&mut self, value: Travel) -> error::Result<&mut Self> {
		match value {
			Travel::Up(n) if n == 1 =>
				if expand!(? self.inner => CursorUp)? {
					expand!(self.inner => ParmUpCursor; count: 1)?;
				},

			Travel::Up(n) =>
				if expand!(? self.inner => ParmUpCursor; count: n)? {
					for _ in 0 .. n {
						expand!(self.inner => CursorUp)?;
					}
				},

			Travel::Down(n) if n == 1 =>
				if expand!(? self.inner => CursorDown)? {
					expand!(self.inner => ParmDownCursor; count: 1)?;
				},

			Travel::Down(n) =>
				if expand!(? self.inner => ParmDownCursor; count: n)? {
					for _ in 0 .. n {
						expand!(self.inner => CursorDown)?;
					}
				},

			Travel::Left(n) if n == 1 =>
				if expand!(? self.inner => CursorLeft)? {
					expand!(self.inner => ParmLeftCursor; count: 1)?;
				},

			Travel::Left(n) =>
				if expand!(? self.inner => ParmLeftCursor; count: n)? {
					for _ in 0 .. n {
						expand!(self.inner => CursorLeft)?;
					}
				},

			Travel::Right(n) if n == 1 =>
				if expand!(? self.inner => CursorRight)? {
					expand!(self.inner => ParmRightCursor; count: 1)?;
				},

			Travel::Right(n) =>
				if expand!(? self.inner => ParmRightCursor; count: n)? {
					for _ in 0 .. n {
						expand!(self.inner => CursorRight)?;
					}
				},

			Travel::To(Some(x), Some(y)) =>
				if expand!(? self.inner => CursorAddress; x: x, y: y)? {
					expand!(self.inner => RowAddress; y: y)?;
					expand!(self.inner => ColumnAddress; x: x)?;
				},

			Travel::To(Some(x), None) =>
				expand!(self.inner => ColumnAddress; x: x)?,

			Travel::To(None, Some(y)) =>
				expand!(self.inner => RowAddress; y: y)?,

			Travel::To(None, None) =>
				(),
		}

		Ok(self)
	}
}
