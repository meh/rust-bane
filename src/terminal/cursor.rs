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

use std::rc::Rc;
use std::io::{Read, Write};

use info;
use error;
use terminal::Terminal;

#[derive(Debug)]
pub struct Cursor<'a, I: Read + 'a, O: Write + 'a> {
	inner: &'a mut Terminal<I, O>,
	info:  Rc<info::Database>,
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
			info:  inner.database().clone(),
			inner: inner,
		}
	}

	pub fn invisible(&mut self) -> error::Result<&mut Self> {
		expand!(&mut self.inner, cap!(self.info => CursorInvisible)?)?;

		Ok(self)
	}

	pub fn visible(&mut self) -> error::Result<&mut Self> {
		expand!(&mut self.inner, cap!(self.info => CursorNormal)?)?;

		Ok(self)
	}

	pub fn travel(&mut self, value: Travel) -> error::Result<&mut Self> {
		match value {
			Travel::Up(n) if n == 1 =>
				if let Ok(cap) = cap!(self.info => CursorUp) {
					expand!(&mut self.inner, cap)?;
				}
				else {
					expand!(&mut self.inner, cap!(self.info => ParmUpCursor)?; 1)?;
				},

			Travel::Up(n) =>
				if let Ok(cap) = cap!(self.info => ParmUpCursor) {
					expand!(&mut self.inner, cap; n)?;
				}
				else {
					for _ in 0 .. n {
						expand!(&mut self.inner, cap!(self.info => CursorUp)?)?;
					}
				},

			Travel::Down(n) if n == 1 =>
				if let Ok(cap) = cap!(self.info => CursorDown) {
					expand!(&mut self.inner, cap)?;
				}
				else {
					expand!(&mut self.inner, cap!(self.info => ParmDownCursor)?; 1)?;
				},

			Travel::Down(n) =>
				if let Ok(cap) = cap!(self.info => ParmDownCursor) {
					expand!(&mut self.inner, cap; n)?;
				}
				else {
					for _ in 0 .. n {
						expand!(&mut self.inner, cap!(self.info => CursorDown)?)?;
					}
				},

			Travel::Left(n) if n == 1 =>
				if let Ok(cap) = cap!(self.info => CursorLeft) {
					expand!(&mut self.inner, cap)?;
				}
				else {
					expand!(&mut self.inner, cap!(self.info => ParmLeftCursor)?; 1)?;
				},

			Travel::Left(n) =>
				if let Ok(cap) = cap!(self.info => ParmLeftCursor) {
					expand!(&mut self.inner, cap; n)?;
				}
				else {
					for _ in 0 .. n {
						expand!(&mut self.inner, cap!(self.info => CursorLeft)?)?;
					}
				},

			Travel::Right(n) if n == 1 =>
				if let Ok(cap) = cap!(self.info => CursorRight) {
					expand!(&mut self.inner, cap)?;
				}
				else {
					expand!(&mut self.inner, cap!(self.info => ParmRightCursor)?; 1)?;
				},

			Travel::Right(n) =>
				if let Ok(cap) = cap!(self.info => ParmRightCursor) {
					expand!(&mut self.inner, cap; n)?;
				}
				else {
					for _ in 0 .. n {
						expand!(&mut self.inner, cap!(self.info => CursorRight)?)?;
					}
				},

			Travel::To(Some(x), Some(y)) =>
				if let Ok(cap) = cap!(self.info => CursorAddress) {
					expand!(&mut self.inner, cap; y, x)?;
				}
				else {
					expand!(&mut self.inner, cap!(self.info => ColumnAddress)?; x)?;
					expand!(&mut self.inner, cap!(self.info => RowAddress)?; y)?;
				},

			Travel::To(Some(x), None) =>
				expand!(&mut self.inner, cap!(self.info => ColumnAddress)?; x)?,

			Travel::To(None, Some(y)) =>
				expand!(&mut self.inner, cap!(self.info => RowAddress)?; y)?,

			Travel::To(None, None) =>
				(),
		}

		Ok(self)
	}
}
