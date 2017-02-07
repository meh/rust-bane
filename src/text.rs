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

use info::{self, capability as cap};
use error;
use terminal::Terminal;

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
	info:  Rc<info::Database>,
}

impl<'a, I: Read + 'a, O: Write + 'a> Text<'a, I, O> {
	pub fn new<'b: 'a>(inner: &'b mut Terminal<I, O>) -> Text<'b, I, O> {
		Text {
			info:  inner.database().clone(),
			inner: inner,
		}
	}

	pub fn default(&mut self) -> error::Result<&mut Self> {
		expand!(&mut self.inner, cap!(self.info => ExitAttributeMode)?)?;

		Ok(self)
	}

	pub fn weight(&mut self, value: Weight) -> error::Result<&mut Self> {
		match value {
			Weight::Default => {
				self.inner.write(b"\x1B[22m")?;
			}

			Weight::Bold => {
				if let Ok(cap) = cap!(self.info => EnterBoldMode) {
					expand!(&mut self.inner, cap)?;
				}
				else {
					self.inner.write(b"\x1B[1m")?;
				}
			}

			Weight::Faint => {
				if let Ok(cap) = cap!(self.info => EnterDimMode) {
					expand!(&mut self.inner, cap)?;
				}
				else {
					self.inner.write(b"\x1B[2m")?;
				}
			}
		}

		Ok(self)
	}

	pub fn reverse(&mut self, value: bool) -> error::Result<&mut Self> {
		if value {
			if let Ok(cap) = cap!(self.info => EnterReverseMode) {
				expand!(&mut self.inner, cap)?;
			}
			else {
				self.inner.write(b"\x1B[7m")?;
			}
		}
		else {
			self.inner.write(b"\x1B[27m")?;
		}

		Ok(self)
	}

	pub fn blink(&mut self, value: bool) -> error::Result<&mut Self> {
		if value {
			if let Ok(cap) = cap!(self.info => EnterBlinkMode) {
				expand!(&mut self.inner, cap)?;
			}
			else {
				self.inner.write(b"\x1B[5m")?;
			}
		}
		else {
			self.inner.write(b"\x1B[25m")?;
		}

		Ok(self)
	}

	pub fn invisible(&mut self, value: bool) -> error::Result<&mut Self> {
		if value {
			if let Ok(cap) = cap!(self.info => EnterSecureMode) {
				expand!(&mut self.inner, cap)?;
			}
			else {
				self.inner.write(b"\x1B[8m")?;
			}
		}
		else {
			self.inner.write(b"\x1B[28m")?;
		}

		Ok(self)
	}

	pub fn alternative(&mut self, value: bool) -> error::Result<&mut Self> {
		if value {
			expand!(&mut self.inner, cap!(self.info => EnterAltCharsetMode)?)?;
		}
		else {
			expand!(&mut self.inner, cap!(self.info => ExitAltCharsetMode)?)?;
		}

		Ok(self)
	}

	pub fn italic(&mut self, value: bool) -> error::Result<&mut Self> {
		if value {
			expand!(&mut self.inner, cap!(self.info => EnterItalicsMode)?)?;
		}
		else {
			expand!(&mut self.inner, cap!(self.info => ExitItalicsMode)?)?;
		}

		Ok(self)
	}

	pub fn standout(&mut self, value: bool) -> error::Result<&mut Self> {
		if value {
			expand!(&mut self.inner, cap!(self.info => EnterStandoutMode)?)?;
		}
		else {
			expand!(&mut self.inner, cap!(self.info => ExitStandoutMode)?)?;
		}

		Ok(self)
	}

	pub fn underline(&mut self, value: bool) -> error::Result<&mut Self> {
		if value {
			expand!(&mut self.inner, cap!(self.info => EnterUnderlineMode)?)?;
		}
		else {
			expand!(&mut self.inner, cap!(self.info => ExitUnderlineMode)?)?;
		}

		Ok(self)
	}

	pub fn struck(&mut self, value: bool) -> error::Result<&mut Self> {
		if value {
			self.inner.write(b"\x1B[9m")?;
		}
		else {
			self.inner.write(b"\x1B[29m")?;
		}

		Ok(self)
	}

	pub fn foreground<T: Into<Color>>(&mut self, value: T) -> error::Result<&mut Self> {
		match value.into() {
			Color::Default => {
				self.inner.write(b"\x1B[39m")?;
			}

			Color::Transparent => {
				self.inner.write(b"\x1B[38;1m")?;
			}

			Color::Index(id) if id < 8 => {
				if let Ok(cap) = cap!(self.info => SetAForeground) {
					expand!(&mut self.inner, cap; id)?
				}
				else if let Ok(cap) = cap!(self.info => SetForeground) {
					expand!(&mut self.inner, cap; id)?
				}
				else {
					match cap!(self.info => MaxColors) {
						Ok(cap::MaxColors(n)) if n >= 8 => {
							self.inner.write(format!("\x1B[3{}m", id).as_ref())?;
						}

						_ =>
							return Err(error::Error::NotSupported)
					}
				}
			}

			Color::Index(id) if id < 16 => {
				match cap!(self.info => MaxColors) {
					Ok(cap::MaxColors(n)) if n >= 16 =>
						if let Ok(cap) = cap!(self.info => SetAForeground) {
							expand!(&mut self.inner, cap; id)?;
						}
						else {
							self.inner.write(format!("\x1B[9{}m", id - 8).as_ref())?;
						},

					_ =>
						return Err(error::Error::NotSupported)
				}
			}

			Color::Index(id) => {
				match cap!(self.info => MaxColors) {
					Ok(cap::MaxColors(n)) if n >= 256 =>
						if let Ok(cap) = cap!(self.info => SetAForeground) {
							expand!(&mut self.inner, cap; id)?;
						}
						else {
							self.inner.write(format!("\x1B[38;5;{}m", id).as_ref())?;
						},

					_ =>
						return Err(error::Error::NotSupported)
				}
			}

			Color::Rgb(r, g, b) => {
				if let Ok(cap) = cap!(self.info => SetTrueColorForeground) {
					expand!(&mut self.inner, cap; r, g, b)?;
				}
				else {
					self.inner.write(format!("\x1B[38;2;{};{};{}m", r, g, b).as_ref())?;
				}
			}

			_ =>
				return Err(error::Error::NotSupported)
		}

		Ok(self)
	}

	pub fn background<T: Into<Color>>(&mut self, value: T) -> error::Result<&mut Self> {
		match value.into() {
			Color::Default => {
				self.inner.write(b"\x1B[49m")?;
			}

			Color::Transparent => {
				self.inner.write(b"\x1B[48;1m")?;
			}

			Color::Index(id) if id < 8 => {
				if let Ok(cap) = cap!(self.info => SetABackground) {
					expand!(&mut self.inner, cap; id)?
				}
				else if let Ok(cap) = cap!(self.info => SetBackground) {
					expand!(&mut self.inner, cap; id)?
				}
				else {
					match cap!(self.info => MaxColors) {
						Ok(cap::MaxColors(n)) if n >= 8 => {
							self.inner.write(format!("\x1B[4{}m", id).as_ref())?;
						}

						_ =>
							return Err(error::Error::NotSupported)
					}
				}

			}

			Color::Index(id) if id < 16 => {
				match cap!(self.info => MaxColors) {
					Ok(cap::MaxColors(n)) if n >= 16 =>
						if let Ok(cap) = cap!(self.info => SetABackground) {
							expand!(&mut self.inner, cap; id)?;
						}
						else {
							self.inner.write(format!("\x1B[10{}m", id - 8).as_ref())?;
						},

					_ =>
						return Err(error::Error::NotSupported)
				}
			}

			Color::Index(id) => {
				match cap!(self.info => MaxColors) {
					Ok(cap::MaxColors(n)) if n >= 256 =>
						if let Ok(cap) = cap!(self.info => SetABackground) {
							expand!(&mut self.inner, cap; id)?;
						}
						else {
							self.inner.write(format!("\x1B[48;5;{}m", id).as_ref())?;
						},

					_ =>
						return Err(error::Error::NotSupported)
				}
			}

			Color::Rgb(r, g, b) => {
				if let Ok(cap) = cap!(self.info => SetTrueColorBackground) {
					expand!(&mut self.inner, cap; r, g, b)?;
				}
				else {
					self.inner.write(format!("\x1B[48;2;{};{};{}m", r, g, b).as_ref())?;
				}
			}

			_ =>
				return Err(error::Error::NotSupported)
		}

		Ok(self)
	}

	pub fn write<T: AsRef<str>>(&mut self, value: T) -> error::Result<&mut Self> {
		self.inner.write(value.as_ref().as_ref())?;

		Ok(self)
	}
}
