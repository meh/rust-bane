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

use info::capability as cap;
use error;
use terminal::Terminal;

/// The font weight.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Weight {
	Default,
	Bold,
	Faint,
}

impl Default for Weight {
	fn default() -> Self {
		Weight::Default
	}
}

/// The font color.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Color {
	Default,
	Transparent,
	Index(u8),
	Cmy(u8, u8, u8),
	Cmyk(u8, u8, u8, u8),
	Rgb(u8, u8, u8),
}

impl Default for Color {
	fn default() -> Self {
		Color::Default
	}
}

#[derive(Debug)]
pub struct Text<'a, I: Read + 'a, O: Write + 'a> {
	inner: &'a mut Terminal<I, O>,
}

impl<'a, I: Read + 'a, O: Write + 'a> Text<'a, I, O> {
	#[doc(hidden)]
	pub fn new<'b: 'a>(inner: &'b mut Terminal<I, O>) -> Text<'b, I, O> {
		Text {
			inner: inner,
		}
	}

	/// Reset all attributes.
	pub fn default(&mut self) -> error::Result<&mut Self> {
		expand!(self.inner => ExitAttributeMode)?;

		Ok(self)
	}

	/// Change the font weight.
	pub fn weight(&mut self, value: Weight) -> error::Result<&mut Self> {
		match value {
			Weight::Default => {
				self.inner.write(b"\x1B[22m")?;
			}

			Weight::Bold => {
				if expand!(? self.inner => EnterBoldMode)? {
					self.inner.write(b"\x1B[1m")?;
				}
			}

			Weight::Faint => {
				if expand!(? self.inner => EnterDimMode)? {
					self.inner.write(b"\x1B[2m")?;
				}
			}
		}

		Ok(self)
	}

	/// Change reverse mode.
	pub fn reverse(&mut self, value: bool) -> error::Result<&mut Self> {
		if value {
			if expand!(? self.inner => EnterReverseMode)? {
				self.inner.write(b"\x1B[7m")?;
			}
		}
		else {
			self.inner.write(b"\x1B[27m")?;
		}

		Ok(self)
	}

	/// Change blink mode.
	pub fn blink(&mut self, value: bool) -> error::Result<&mut Self> {
		if value {
			if expand!(? self.inner => EnterBlinkMode)? {
				self.inner.write(b"\x1B[5m")?;
			}
		}
		else {
			self.inner.write(b"\x1B[25m")?;
		}

		Ok(self)
	}

	/// Change invisible mode.
	pub fn invisible(&mut self, value: bool) -> error::Result<&mut Self> {
		if value {
			if expand!(? self.inner => EnterSecureMode)? {
				self.inner.write(b"\x1B[8m")?;
			}
		}
		else {
			self.inner.write(b"\x1B[28m")?;
		}

		Ok(self)
	}

	/// Change the charset.
	pub fn alternative(&mut self, value: bool) -> error::Result<&mut Self> {
		if value {
			expand!(self.inner => EnterAltCharsetMode)?;
		}
		else {
			expand!(self.inner => ExitAltCharsetMode)?;
		}

		Ok(self)
	}

	/// Change italic mode.
	pub fn italic(&mut self, value: bool) -> error::Result<&mut Self> {
		if value {
			expand!(self.inner => EnterItalicsMode)?;
		}
		else {
			expand!(self.inner => ExitItalicsMode)?;
		}

		Ok(self)
	}

	/// Change standout mode.
	pub fn standout(&mut self, value: bool) -> error::Result<&mut Self> {
		if value {
			expand!(self.inner => EnterStandoutMode)?;
		}
		else {
			expand!(self.inner => ExitStandoutMode)?;
		}

		Ok(self)
	}

	/// Change underline mode.
	pub fn underline(&mut self, value: bool) -> error::Result<&mut Self> {
		if value {
			expand!(self.inner => EnterUnderlineMode)?;
		}
		else {
			expand!(self.inner => ExitUnderlineMode)?;
		}

		Ok(self)
	}

	/// Change strike-through mode.
	pub fn struck(&mut self, value: bool) -> error::Result<&mut Self> {
		if value {
			self.inner.write(b"\x1B[9m")?;
		}
		else {
			self.inner.write(b"\x1B[29m")?;
		}

		Ok(self)
	}

	/// Change the foreground color.
	pub fn foreground<T: Into<Color>>(&mut self, value: T) -> error::Result<&mut Self> {
		match value.into() {
			Color::Default => {
				self.inner.write(b"\x1B[39m")?;
			}

			Color::Transparent => {
				self.inner.write(b"\x1B[38;1m")?;
			}

			Color::Index(id) if id < 8 => {
				if expand!(? self.inner => SetAForeground; color: id)? {
					if expand!(? self.inner => SetForeground; color: id)? {
						match cap!(self.inner.database() => MaxColors) {
							Ok(cap::MaxColors(n)) if n >= 8 => {
								write!(self.inner, "\x1B[3{}m", id)?;
							}

							_ =>
								return Err(error::Error::NotSupported)
						}
					}
				}
			}

			Color::Index(id) if id < 16 => {
				match cap!(self.inner.database() => MaxColors) {
					Ok(cap::MaxColors(n)) if n >= 16 =>
						if expand!(? self.inner => SetAForeground; color: id)? {
							write!(self.inner, "\x1B[9{}m", id - 8)?;
						},

					_ =>
						return Err(error::Error::NotSupported)
				}
			}

			Color::Index(id) => {
				match cap!(self.inner.database() => MaxColors) {
					Ok(cap::MaxColors(n)) if n >= 256 =>
						if expand!(? self.inner => SetAForeground; color: id)? {
							write!(self.inner, "\x1B[38;5;{}m", id)?;
						},

					_ =>
						return Err(error::Error::NotSupported)
				}
			}

			Color::Rgb(r, g, b) => {
				if expand!(? self.inner => SetTrueColorForeground; r: r, g: g, b: b)? {
					write!(self.inner, "\x1B[38;2;{};{};{}m", r, g, b)?;
				}
			}

			_ =>
				return Err(error::Error::NotSupported)
		}

		Ok(self)
	}

	/// Change the background color.
	pub fn background<T: Into<Color>>(&mut self, value: T) -> error::Result<&mut Self> {
		match value.into() {
			Color::Default => {
				write!(self.inner, "\x1B[49m")?;
			}

			Color::Transparent => {
				write!(self.inner, "\x1B[48;1m")?;
			}

			Color::Index(id) if id < 8 => {
				if expand!(? self.inner => SetABackground; color: id)? {
					if expand!(? self.inner => SetBackground; color: id)? {
						match cap!(self.inner.database() => MaxColors) {
							Ok(cap::MaxColors(n)) if n >= 8 => {
								write!(self.inner, "\x1B[4{}m", id)?;
							}

							_ =>
								return Err(error::Error::NotSupported)
						}
					}
				}
			}

			Color::Index(id) if id < 16 => {
				match cap!(self.inner.database() => MaxColors) {
					Ok(cap::MaxColors(n)) if n >= 16 =>
						if expand!(? self.inner => SetABackground; color: id)? {
							write!(self.inner, "\x1B[10{}m", id - 8)?;
						},

					_ =>
						return Err(error::Error::NotSupported)
				}
			}

			Color::Index(id) => {
				match cap!(self.inner.database() => MaxColors) {
					Ok(cap::MaxColors(n)) if n >= 256 =>
						if expand!(? self.inner => SetABackground; color: id)? {
							write!(self.inner, "\x1B[48;5;{}m", id)?;
						},

					_ =>
						return Err(error::Error::NotSupported)
				}
			}

			Color::Rgb(r, g, b) => {
				if expand!(? self.inner => SetTrueColorBackground; r: r, g: g, b: b)? {
					write!(self.inner, "\x1B[48;2;{};{};{}m", r, g, b)?;
				}
			}

			_ =>
				return Err(error::Error::NotSupported)
		}

		Ok(self)
	}

	/// Write the given text.
	pub fn write<T: AsRef<str>>(&mut self, value: T) -> error::Result<&mut Self> {
		self.inner.write(value.as_ref().as_ref())?;

		Ok(self)
	}
}
