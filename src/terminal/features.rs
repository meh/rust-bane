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
use std::os::unix::io::AsRawFd;
use termios::{Termios, tcsetattr};
use termios::{ICANON, ECHO, TCSANOW, IEXTEN, ISIG, VMIN, VTIME};

use error;
use terminal::Terminal;

#[derive(Debug)]
pub struct Features<'a, I: Read + 'a, O: Write + 'a> {
	inner: &'a mut Terminal<I, O>,
	state: Termios,
}

impl<'a, I: Read + 'a, O: Write + 'a> Features<'a, I, O> {
	pub fn new<'b: 'a>(inner: &'b mut Terminal<I, O>) -> Features<'b, I, O> {
		Features {
			state: Termios::from_fd(inner.as_raw_fd()).unwrap(),
			inner: inner,
		}
	}

	pub fn echo(&mut self, value: bool) -> error::Result<&mut Self> {
		if value {
			self.state.c_lflag |= ECHO;
		}
		else {
			self.state.c_lflag &= !ECHO;
		}

		tcsetattr(self.inner.as_raw_fd(), TCSANOW, &self.state)?;

		Ok(self)
	}

	/// Change the terminal's raw mode.
	pub fn raw(&mut self, value: bool) -> error::Result<&mut Self> {
		if value {
			self.state.c_lflag     &= !(ICANON | ISIG | IEXTEN);
			self.state.c_cc[VMIN]   = 1;
			self.state.c_cc[VTIME]  = 0;
		}
		else {
			self.state.c_lflag |= ISIG | ICANON | IEXTEN;
		}

		tcsetattr(self.inner.as_raw_fd(), TCSANOW, &self.state)?;

		Ok(self)
	}
}
