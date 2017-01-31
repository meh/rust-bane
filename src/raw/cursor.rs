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

use std::io::Write;

use info::{self, capability as cap};
use error;

pub fn show<T: AsRef<info::Database>, W: Write>(info: T, mut output: W) -> error::Result<()> {
	output.write_all(&expand!(info.as_ref().get::<cap::CursorNormal>().unwrap())?)
		.map_err(Into::into)
}

pub fn hide<T: AsRef<info::Database>, W: Write>(info: T, mut output: W) -> error::Result<()> {
	output.write_all(&expand!(info.as_ref().get::<cap::CursorInvisible>().unwrap())?)
		.map_err(Into::into)
}
