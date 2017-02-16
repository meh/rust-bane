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

use std::sync::{Arc, Mutex, MutexGuard};
use std::io::{self, Read, Write, Stdin, Stdout};
use std::os::unix::io::{AsRawFd, RawFd};

use std::thread;
use chan;

use libc::{isatty, STDIN_FILENO, STDOUT_FILENO};
use info;
use termios::{Termios, tcsetattr, TCSANOW};

use size::{self, Size};
use error::{self, Error};
use {Features, Cursor, Erase, Text};
use resize;
use keys::{Key, Keys};

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Event {
	Close,
	Resize,
	Focus(bool),
	Paste(Vec<u8>),
	Key(Key),
}

#[derive(Debug)]
pub struct Terminal<I: Read = Stdin, O: Write = Stdout> {
	tty:    RawFd,
	output: O,

	input:  Option<I>,
	events: Option<chan::Receiver<Event>>,
	keys:   Arc<Mutex<Keys>>,

	database: info::Database,
	context:  info::expand::Context,

	initial: Termios,
	resizer: Option<resize::Handler>,
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

		let info  = info::Database::from_env()?;
		let state = Termios::from_fd(STDOUT_FILENO)?;
		let keys  = Arc::new(Mutex::new(Keys::new(&info)));

		Terminal {
			tty:    STDOUT_FILENO,
			output: io::stdout(),

			input:  Some(io::stdin()),
			events: None,
			keys:   keys,

			database: info,
			context:  info::expand::Context::default(),

			initial: state,
			resizer: None,
		}.init()
	}
}

impl<I: Read + AsRawFd, O: Write + AsRawFd> Terminal<I, O> {
	/// Open a terminal from the given streams.
	pub fn open(input: I, output: O) -> error::Result<Self> {
		unsafe {
			if isatty(input.as_raw_fd()) == 0 || isatty(output.as_raw_fd()) == 0 {
				return Err(Error::NotInteractive);
			}

			let tty   = output.as_raw_fd();
			let info  = info::Database::from_env()?;
			let state = Termios::from_fd(tty)?;
			let keys  = Arc::new(Mutex::new(Keys::new(&info)));

			Terminal {
				tty:    tty,
				output: output,

				input:  Some(input),
				events: None,
				keys:   keys,

				database: info,
				context:  info::expand::Context::default(),

				initial: state,
				resizer: None,
			}.init()
		}
	}
}

impl<I: Read, O: Write> Terminal<I, O> {
	/// Initialize with the default terminal features.
	fn init(mut self) -> error::Result<Self> {
		use control::{self, CSI, DEC};

		// Enable application cursor.
		control::format_to(&mut self.output,
			&DEC::Set(CSI::values(&[DEC::Mode::ApplicationCursor])))?;

		// Enable application keypad.
		control::format_to(&mut self.output,
			&DEC::ApplicationKeypad(true))?;

		// Enable bracketed paste, focus notification, mouse support.
		control::format_to(&mut self.output,
			&CSI::Private(b'h', None, CSI::args(&[2004, 1004, 1006])))?;

		// Commit the changes.
		self.output.flush()?;

		Ok(self)
	}

	/// Reset the default terminal features.
	fn deinit(&mut self) -> error::Result<&mut Self> {
		use control::{self, CSI, DEC};

		// Disable application cursor.
		control::format_to(&mut self.output,
			&DEC::Reset(CSI::values(&[DEC::Mode::ApplicationCursor])))?;

		// Disable application keypad.
		control::format_to(&mut self.output,
			&DEC::ApplicationKeypad(false))?;

		// Disable bracketed paste, focus notification, mouse support.
		control::format_to(&mut self.output,
			&CSI::Private(b'l', None, CSI::args(&[2004, 1004, 1006])))?;

		// Commit the changes.
		self.output.flush()?;

		Ok(self)
	}
}

impl<I: Read, O: Write> Terminal<I, O> {
	/// Run an expansion.
	pub fn expansion<T, F>(&mut self, f: F) -> error::Result<T>
		where F: FnOnce(&info::Database, &mut info::expand::Context, &mut O) -> error::Result<T>
	{
		f(&self.database, &mut self.context, &mut self.output)
	}

	/// Get the key handler.
	pub fn keys(&mut self) -> MutexGuard<Keys> {
		self.keys.lock().unwrap()
	}

	/// Get the terminal capability database.
	pub fn database(&self) -> &info::Database {
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

	/// Access erasion operations.
	pub fn erase(&mut self) -> Erase<I, O> {
		Erase::new(self)
	}

	/// Access text operations.
	pub fn text(&mut self) -> Text<I, O> {
		Text::new(self)
	}
}

impl<I: Read + Send + 'static, O: Write> Terminal<I, O> {
	/// Prepare for events.
	pub fn events(&mut self) -> chan::Receiver<Event> {
		if self.events.is_none() {
			let mut stream = self.input.take().unwrap();
			let     keys   = self.keys.clone();

			let (sender, receiver) = chan::sync(1);
			self.resizer           = Some(resize::register(sender.clone()));

			thread::spawn(move || {
				let mut buffer = [0u8; 4096];
				let mut paste  = None;

				while let Ok(amount) = stream.read(&mut buffer) {
					if amount == 0 {
						break;
					}

					let mut input = &buffer[..amount];

					while !input.is_empty() {
						if paste.is_some() || input.starts_with(b"\x1B[200~") {
							if paste.is_none() {
								input = &input[b"\x1B[200~".len() ..];
							}

							let mut result  = paste.take().unwrap_or(Vec::new());
							let mut count   = 0;
							let mut current = input;

							while !current.is_empty() {
								if !current.starts_with(b"\x1B[201~") {
									count   += 1;
									current  = &current[1..];
								}
								else {
									result.extend_from_slice(&input[.. count]);
									break;
								}
							}

							if current.is_empty() {
								paste = Some(result);
							}
							else {
								input = &current[b"\x1B[201~".len() ..];
								sender.send(Event::Paste(result));
							}
						}
						else if input.starts_with(b"\x1B[I") {
							sender.send(Event::Focus(true));
							input = &input[b"\x1B[I".len() ..];
						}
						else if input.starts_with(b"\x1B[O") {
							sender.send(Event::Focus(false));
							input = &input[b"\x1B[O".len() ..];
						}
						else {
							let (rest, key) = keys.lock().unwrap().find(input);
							input           = rest;

							if let Some(key) = key {
								sender.send(Event::Key(key));
							}
						}
					}
				}

				sender.send(Event::Close);
			});

			self.events = Some(receiver.clone());
		}

		self.events.clone().unwrap()
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
		if let Some(id) = self.resizer.take() {
			resize::unregister(id);
		}

		let _ = self.deinit();
		let _ = tcsetattr(self.tty, TCSANOW, &self.initial);
	}
}
