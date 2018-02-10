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

extern crate libc;
extern crate crossbeam_channel as channel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;
extern crate fnv;

extern crate termios;

pub extern crate control_code;
pub use control_code as control;

pub extern crate terminfo;
pub use terminfo as info;

#[macro_use]
mod util;

mod error;
pub use error::{Error, Result};

mod resize;
mod size;
pub use size::{get as size, Size};

mod features;
pub use features::Features;

pub mod erase;
pub use erase::Erase;

pub mod text;
pub use text::Text;

pub mod cursor;
pub use cursor::Cursor;

pub mod keys;
pub use keys::{Keys, Key};

pub mod terminal;
pub use terminal::{Terminal, Event};
