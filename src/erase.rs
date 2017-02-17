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

/// Edge to erase to.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum To {
	Start,
	End,
}

#[derive(Debug)]
pub struct Erase<'a, I: Read + 'a, O: Write + 'a> {
	inner: &'a mut Terminal<I, O>,
}

impl<'a, I: Read + 'a, O: Write + 'a> Erase<'a, I, O> {
	#[doc(hidden)]
	pub fn new<'b: 'a>(inner: &'b mut Terminal<I, O>) -> Erase<'b, I, O> {
		Erase {
			inner: inner,
		}
	}

	/// Erase the line.
	pub fn line<T: Into<Option<To>>>(&mut self, value: T) -> error::Result<&mut Self> {
		match value.into() {
			None => {
				expand!(self.inner => ClrBol)?;
				expand!(self.inner => ClrEol)?;
			}

			Some(To::Start) =>
				expand!(self.inner => ClrBol)?,

			Some(To::End) =>
				expand!(self.inner => ClrEol)?,
		}

		Ok(self)
	}

	/// Erase the display.
	pub fn screen<T: Into<Option<To>>>(&mut self, value: T) -> error::Result<&mut Self> {
		match value.into() {
			None =>
				expand!(self.inner => ClearScreen)?,

			Some(To::End) =>
				expand!(self.inner => ClrEos)?,

			Some(To::Start) =>
				return Err(error::Error::NotSupported),
		}

		Ok(self)
	}
}
