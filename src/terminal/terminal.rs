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
use std::io::{self, Read, Write, Stdin, Stdout};
use std::os::unix::io::{AsRawFd, RawFd};

use std::thread;
use chan;

use libc::{isatty, STDIN_FILENO, STDOUT_FILENO};
use info;
use termios::{Termios, tcsetattr, TCSANOW};

use size::{self, Size};
use error::{self, Error};
use terminal::{Features, Cursor, Screen};
use terminal::resize;
use terminal::event::{self, Event};

#[derive(Debug)]
pub struct Terminal<I: Read = Stdin, O: Write = Stdout> {
	tty:    RawFd,
	input:  Option<I>,
	output: O,

	database: Rc<info::Database>,
	initial:  Termios,
	resizer:  Option<u32>,
}

pub type Default = Terminal<Stdin, Stdout>;

impl Terminal<Stdin, Stdout> {
	/// Open the default terminal.
	pub fn default() -> error::Result<Self> {
		unsafe {
			if isatty(STDIN_FILENO) == 0 || isatty(STDOUT_FILENO) == 0 {
				return Err(Error::NotInteractive);
			}
		}

		Ok(Terminal {
			tty:    STDOUT_FILENO,
			input:  Some(io::stdin()),
			output: io::stdout(),

			database: Rc::new(info::Database::from_env()?),
			initial:  Termios::from_fd(STDOUT_FILENO)?,
			resizer:  None,
		})
	}
}

impl<I: Read + AsRawFd, O: Write + AsRawFd> Terminal<I, O> {
	/// Open a terminal from the given streams.
	pub fn open(input: I, output: O) -> error::Result<Self> {
		unsafe {
			if isatty(input.as_raw_fd()) == 0 || isatty(output.as_raw_fd()) == 0 {
				return Err(Error::NotInteractive);
			}

			let tty = output.as_raw_fd();

			Ok(Terminal {
				tty:    tty,
				input:  Some(input),
				output: output,

				database: Rc::new(info::Database::from_env()?),
				initial:  Termios::from_fd(tty)?,
				resizer:  None,
			})
		}
	}
}

impl<I: Read, O: Write> Terminal<I, O> {
	/// Get the terminal capability database.
	pub fn database(&self) -> &Rc<info::Database> {
		&self.database
	}

	/// Get the terminal size.
	pub fn size(&self) -> Size {
		size::get(self.tty).unwrap()
	}

	/// Access terminal features.
	pub fn features(&mut self) -> Features<I, O> {
		Features::new(self)
	}

	/// Access the cursor.
	pub fn cursor(&mut self) -> Cursor<I, O> {
		Cursor::new(self)
	}

	/// Access the screen.
	pub fn screen(&mut self) -> Screen<I, O> {
		Screen::new(self)
	}
}

impl<I: Read + Send + 'static, O: Write> Terminal<I, O> {
	/// Prepare for events.
	pub fn events(&mut self) -> error::Result<chan::Receiver<Event>> {
		use std::str;

		if let Some(mut input) = self.input.take() {
			let (sender, receiver) = chan::sync(1);
			self.resizer           = Some(resize::register(sender.clone()));

			thread::spawn(move || {
				let mut buffer = [0u8; 256];

				while let Ok(amount) = input.read(&mut buffer) {
					if amount == 0 {
						break;
					}

					if let Ok(value) = str::from_utf8(&buffer[..amount]) {
						sender.send(Event::Input(value.into()));
					}
				}

				sender.send(Event::Close);
			});

			Ok(receiver)
		}
		else {
			Err(io::Error::new(io::ErrorKind::NotConnected, "events() has already been called").into())
		}
	}
}

impl<I: Read, O: Write> AsRef<info::Database> for Terminal<I, O> {
	fn as_ref(&self) -> &info::Database {
		&self.database
	}
}

impl<I: Read, O: Write> AsRawFd for Terminal<I, O> {
	fn as_raw_fd(&self) -> RawFd {
		self.tty
	}
}

impl<I: Read, O: Write> Write for Terminal<I, O> {
	#[inline]
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self.output.write(buf)
	}

	#[inline]
	fn flush(&mut self) -> io::Result<()> {
		self.output.flush()
	}
}

impl<I: Read, O: Write> Read for Terminal<I, O> {
	#[inline]
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		if let Some(input) = self.input.as_mut() {
			input.read(buf)
		}
		else {
			Err(io::Error::new(io::ErrorKind::NotConnected, "events() has been called"))
		}
	}
}

impl<I: Read, O: Write> Drop for Terminal<I, O> {
	fn drop(&mut self) {
		if let Some(id) = self.resizer {
			resize::unregister(id);
		}

		let _ = tcsetattr(self.tty, TCSANOW, &self.initial);
	}
}
