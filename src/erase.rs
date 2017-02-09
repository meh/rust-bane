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
	pub fn new<'b: 'a>(inner: &'b mut Terminal<I, O>) -> Erase<'b, I, O> {
		Erase {
			inner: inner,
		}
	}

	pub fn line(&mut self, value: To) -> error::Result<&mut Self> {
		match value {
			To::Start =>
				expand!(self.inner => ClrBol)?,

			To::End =>
				expand!(self.inner => ClrEol)?,
		}

		Ok(self)
	}

	pub fn screen(&mut self, value: To) -> error::Result<&mut Self> {
		match value {
			To::Start =>
				return Err(error::Error::NotSupported),

			To::End =>
				expand!(self.inner => ClrEos)?,
		}

		Ok(self)
	}
}
