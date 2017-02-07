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
pub struct Screen<'a, I: Read + 'a, O: Write + 'a> {
	inner: &'a mut Terminal<I, O>,
	info:  Rc<info::Database>,
}

impl<'a, I: Read + 'a, O: Write + 'a> Screen<'a, I, O> {
	pub fn new<'b: 'a>(inner: &'b mut Terminal<I, O>) -> Screen<'b, I, O> {
		Screen {
			info:  inner.database().clone(),
			inner: inner,
		}
	}

	pub fn clear(&mut self) -> error::Result<&mut Self> {
		expand!(&mut self.inner, cap!(self.info => ClearScreen)?)?;

		Ok(self)
	}
}
