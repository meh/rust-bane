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

use std::collections::{BTreeMap, HashMap};
use std::hash::BuildHasherDefault;
use fnv::FnvHasher;
use std::str;

use info::{self, capability as cap};

#[derive(Debug)]
pub struct Keys(BTreeMap<usize, HashMap<Vec<u8>, Key, BuildHasherDefault<FnvHasher>>>);

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Key {
	pub modifier: Modifier,
	pub value:    Value,
}

bitflags! {
	pub flags Modifier: u8 {
		const NONE  = 0,
		const ALT   = 1 << 0,
		const CTRL  = 1 << 1,
		const LOGO  = 1 << 2,
		const SHIFT = 1 << 3,
	}
}

impl Default for Modifier {
	fn default() -> Self {
		Modifier::empty()
	}
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Value {
	Escape,
	Enter,

	Down,
	Up,
	Left,
	Right,

	PageUp,
	PageDown,

	BackSpace,
	BackTab,
	Tab,
	Delete,
	Insert,
	Home,
	End,
	Begin,

	F(u8),
	Char(char),
}

pub use self::Value::*;

impl Keys {
	pub fn new(info: &info::Database) -> Self {
		let mut map = BTreeMap::default();

		// Load terminfo bindings.
		{
			macro_rules! insert {
				($name:ident => $($key:tt)*) => (
					if let Some(cap) = info.get::<cap::$name>() {
						let value: &[u8] = cap.as_ref();

						map.entry(value.len()).or_insert(HashMap::default())
							.entry(value.into()).or_insert(Key {
								modifier: Modifier::empty(),
								value:    Value::$($key)*
							});
					}
				)
			}

			insert!(KeyEnter => Enter);
			insert!(CarriageReturn => Enter);

			insert!(KeyDown => Down);
			insert!(KeyUp => Up);
			insert!(KeyLeft => Left);
			insert!(KeyRight => Right);

			insert!(KeyNpage => PageDown);
			insert!(KeyPpage => PageUp);

			insert!(KeyBackspace => BackSpace);
			insert!(KeyBtab => BackTab);
			insert!(Tab => Tab);

			insert!(KeyF1  => F(1));
			insert!(KeyF2  => F(2));
			insert!(KeyF3  => F(3));
			insert!(KeyF4  => F(4));
			insert!(KeyF5  => F(5));
			insert!(KeyF6  => F(6));
			insert!(KeyF7  => F(7));
			insert!(KeyF8  => F(8));
			insert!(KeyF9  => F(9));
			insert!(KeyF10 => F(10));
			insert!(KeyF11 => F(11));
			insert!(KeyF12 => F(12));
			insert!(KeyF13 => F(13));
			insert!(KeyF14 => F(14));
			insert!(KeyF15 => F(15));
			insert!(KeyF16 => F(16));
			insert!(KeyF17 => F(17));
			insert!(KeyF18 => F(18));
			insert!(KeyF19 => F(19));
			insert!(KeyF20 => F(20));
			insert!(KeyF21 => F(21));
			insert!(KeyF22 => F(22));
			insert!(KeyF23 => F(23));
			insert!(KeyF24 => F(24));
			insert!(KeyF25 => F(25));
			insert!(KeyF26 => F(26));
			insert!(KeyF27 => F(27));
			insert!(KeyF28 => F(28));
			insert!(KeyF29 => F(29));
			insert!(KeyF30 => F(30));
			insert!(KeyF31 => F(31));
			insert!(KeyF32 => F(32));
			insert!(KeyF33 => F(33));
			insert!(KeyF34 => F(34));
			insert!(KeyF35 => F(35));
			insert!(KeyF36 => F(36));
			insert!(KeyF37 => F(37));
			insert!(KeyF38 => F(38));
			insert!(KeyF39 => F(39));
			insert!(KeyF40 => F(40));
			insert!(KeyF41 => F(41));
			insert!(KeyF42 => F(42));
			insert!(KeyF43 => F(43));
			insert!(KeyF44 => F(44));
			insert!(KeyF45 => F(45));
			insert!(KeyF46 => F(46));
			insert!(KeyF47 => F(47));
			insert!(KeyF48 => F(48));
			insert!(KeyF49 => F(49));
			insert!(KeyF50 => F(50));
			insert!(KeyF51 => F(51));
			insert!(KeyF52 => F(52));
			insert!(KeyF53 => F(53));
			insert!(KeyF54 => F(54));
			insert!(KeyF55 => F(55));
			insert!(KeyF56 => F(56));
			insert!(KeyF57 => F(57));
			insert!(KeyF58 => F(58));
			insert!(KeyF59 => F(59));
			insert!(KeyF60 => F(60));
			insert!(KeyF61 => F(61));
			insert!(KeyF62 => F(62));
			insert!(KeyF63 => F(63));
		}

		// Load default bindings.
		{
			macro_rules! insert {
				($string:expr => $value:expr) => (
					insert!($string => $value; NONE);
				);

				($string:expr => $value:expr; $($mods:ident)|+) => (
					map.entry($string.len()).or_insert(HashMap::default())
						.entry($string.to_vec()).or_insert(Key {
							modifier: $($mods)|+,
							value:    $value,
						});
				);
			}

			insert!(b"\x1B[Z" => Tab; SHIFT);

			insert!(b"\x1B\x7F" => BackSpace; ALT);
			insert!(b"\x7F"     => BackSpace);

			insert!(b"\x1B\r\n" => Enter; ALT);
			insert!(b"\x1B\r"   => Enter; ALT);
			insert!(b"\x1B\n"   => Enter; ALT);
			insert!(b"\r\n"     => Enter);
			insert!(b"\r"       => Enter);
			insert!(b"\n"       => Enter);

			insert!(b"\x1B[3;5~" => Delete; CTRL);
			insert!(b"\x1B[3;2~" => Delete; SHIFT);
			insert!(b"\x1B[3~"   => Delete);

			insert!(b"\x1B[2;5~" => Insert; CTRL);
			insert!(b"\x1B[2;2~" => Insert; SHIFT);
			insert!(b"\x1B[2~"   => Insert);

			insert!(b"\x1B[1;2H" => Home; SHIFT);
			insert!(b"\x1B[H"    => Home);

			insert!(b"\x1B[1;5F" => End; CTRL);
			insert!(b"\x1B[1;2F" => End; SHIFT);
			insert!(b"\x1B[8~"   => End);

			insert!(b"\x1B[E" => Begin);

			insert!(b"\x1B[5;5~" => PageUp; CTRL);
			insert!(b"\x1B[5;2~" => PageUp; SHIFT);
			insert!(b"\x1B[5~"   => PageUp);

			insert!(b"\x1B[6;5~" => PageDown; CTRL);
			insert!(b"\x1B[6;2~" => PageDown; SHIFT);
			insert!(b"\x1B[6~"   => PageDown);

			insert!(b"\x1B[1;5A" => Up; CTRL);
			insert!(b"\x1B[1;3A" => Up; ALT);
			insert!(b"\x1B[1;2A" => Up; SHIFT);
			insert!(b"\x1BBOA"   => Up);

			insert!(b"\x1B[1;5B" => Down; CTRL);
			insert!(b"\x1B[1;3B" => Down; ALT);
			insert!(b"\x1B[1;2B" => Down; SHIFT);
			insert!(b"\x1BBOB"   => Down);

			insert!(b"\x1B[1;5C" => Right; CTRL);
			insert!(b"\x1B[1;3C" => Right; ALT);
			insert!(b"\x1B[1;2C" => Right; SHIFT);
			insert!(b"\x1BBOC"   => Right);

			insert!(b"\x1B[1;5D" => Left; CTRL);
			insert!(b"\x1B[1;3D" => Left; ALT);
			insert!(b"\x1B[1;2D" => Left; SHIFT);
			insert!(b"\x1BBOD"   => Left);

			insert!(b"\x1B[1;5P" => F(1); CTRL);
			insert!(b"\x1B[1;3P" => F(1); ALT);
			insert!(b"\x1B[1;6P" => F(1); LOGO);
			insert!(b"\x1B[1;2P" => F(1); SHIFT);
			insert!(b"\x1BOP"    => F(1));

			insert!(b"\x1B[1;5Q" => F(2); CTRL);
			insert!(b"\x1B[1;3Q" => F(2); ALT);
			insert!(b"\x1B[1;6Q" => F(2); LOGO);
			insert!(b"\x1B[1;2Q" => F(2); SHIFT);
			insert!(b"\x1BOQ"    => F(2));

			insert!(b"\x1B[1;5R" => F(3); CTRL);
			insert!(b"\x1B[1;3R" => F(3); ALT);
			insert!(b"\x1B[1;6R" => F(3); LOGO);
			insert!(b"\x1B[1;2R" => F(3); SHIFT);
			insert!(b"\x1BOR"    => F(3));

			insert!(b"\x1B[1;5S" => F(4); CTRL);
			insert!(b"\x1B[1;3S" => F(4); ALT);
			insert!(b"\x1B[1;6S" => F(4); LOGO);
			insert!(b"\x1B[1;2S" => F(4); SHIFT);
			insert!(b"\x1BOS"    => F(4));

			insert!(b"\x1B[15;5~" => F(5); CTRL);
			insert!(b"\x1B[15;3~" => F(5); ALT);
			insert!(b"\x1B[15;6~" => F(5); LOGO);
			insert!(b"\x1B[15;2~" => F(5); SHIFT);
			insert!(b"\x1B[15~"   => F(5));

			insert!(b"\x1B[17;5~" => F(6); CTRL);
			insert!(b"\x1B[17;3~" => F(6); ALT);
			insert!(b"\x1B[17;6~" => F(6); LOGO);
			insert!(b"\x1B[17;2~" => F(6); SHIFT);
			insert!(b"\x1B[17~"   => F(6));

			insert!(b"\x1B[18;5~" => F(7); CTRL);
			insert!(b"\x1B[18;3~" => F(7); ALT);
			insert!(b"\x1B[18;6~" => F(7); LOGO);
			insert!(b"\x1B[18;2~" => F(7); SHIFT);
			insert!(b"\x1B[18~"   => F(7));

			insert!(b"\x1B[19;5~" => F(8); CTRL);
			insert!(b"\x1B[19;3~" => F(8); ALT);
			insert!(b"\x1B[19;6~" => F(8); LOGO);
			insert!(b"\x1B[19;2~" => F(8); SHIFT);
			insert!(b"\x1B[19~"   => F(8));

			insert!(b"\x1B[20;5~" => F(9); CTRL);
			insert!(b"\x1B[20;3~" => F(9); ALT);
			insert!(b"\x1B[20;6~" => F(9); LOGO);
			insert!(b"\x1B[20;2~" => F(9); SHIFT);
			insert!(b"\x1B[20~"   => F(9));

			insert!(b"\x1B[21;5~" => F(10); CTRL);
			insert!(b"\x1B[21;3~" => F(10); ALT);
			insert!(b"\x1B[21;6~" => F(10); LOGO);
			insert!(b"\x1B[21;2~" => F(10); SHIFT);
			insert!(b"\x1B[21~"   => F(10));

			insert!(b"\x1B[23;5~" => F(11); CTRL);
			insert!(b"\x1B[23;3~" => F(11); ALT);
			insert!(b"\x1B[23;6~" => F(11); LOGO);
			insert!(b"\x1B[23;2~" => F(11); SHIFT);
			insert!(b"\x1B[23~"   => F(11));

			insert!(b"\x1B[24;5~" => F(12); CTRL);
			insert!(b"\x1B[24;3~" => F(12); ALT);
			insert!(b"\x1B[24;6~" => F(12); LOGO);
			insert!(b"\x1B[24;2~" => F(12); SHIFT);
			insert!(b"\x1B[24~"   => F(12));

			insert!(b"\x1B[1;2P"  => F(13));
			insert!(b"\x1B[1;2Q"  => F(14));
			insert!(b"\x1B[1;2R"  => F(15));
			insert!(b"\x1B[1;2S"  => F(16));
			insert!(b"\x1B[15;2~" => F(17));
			insert!(b"\x1B[17;2~" => F(18));
			insert!(b"\x1B[18;2~" => F(19));
			insert!(b"\x1B[19;2~" => F(20));
			insert!(b"\x1B[20;2~" => F(21));
			insert!(b"\x1B[21;2~" => F(22));
			insert!(b"\x1B[23;2~" => F(23));
			insert!(b"\x1B[24;2~" => F(24));
			insert!(b"\x1B[1;5P"  => F(25));
			insert!(b"\x1B[1;5Q"  => F(26));
			insert!(b"\x1B[1;5R"  => F(27));
			insert!(b"\x1B[1;5S"  => F(28));
			insert!(b"\x1B[15;5~" => F(29));
			insert!(b"\x1B[17;5~" => F(30));
			insert!(b"\x1B[18;5~" => F(31));
			insert!(b"\x1B[19;5~" => F(32));
			insert!(b"\x1B[20;5~" => F(33));
			insert!(b"\x1B[21;5~" => F(34));
			insert!(b"\x1B[23;5~" => F(35));
		}

		Keys(map)
	}

	pub fn bind<T: Into<Vec<u8>>>(&mut self, value: T, key: Key) -> &mut Self {
		let value = value.into();

		if !value.is_empty() {
			self.0.entry(value.len()).or_insert(HashMap::default())
				.insert(value, key);
		}

		self
	}

	pub fn unbind<T: AsRef<[u8]>>(&mut self, value: T) -> &mut Self {
		let value = value.as_ref();

		if let Some(mut map) = self.0.get_mut(&value.len()) {
			map.remove(value);
		}

		self
	}

	pub fn find<'a>(&self, mut input: &'a [u8]) -> (&'a [u8], Option<Key>) {
		// Check if it's a defined key.
		for (&length, map) in self.0.iter().rev() {
			if length > input.len() {
				continue;
			}

			if let Some(key) = map.get(&input[..length]) {
				return (&input[length..], Some(*key));
			}
		}

		// Check if it's a single escape press.
		if input == &[0x1B] {
			return (&input[1..], Some(Key {
				modifier: Modifier::empty(),
				value:    Escape,
			}));
		}

		let mut mods = Modifier::empty();

		if input[0] == 0x1B {
			mods.insert(ALT);
			input = &input[1..];
		}

		// Check if it's a control character.
		if input[0] & 0b011_00000 == 0 {
			return (&input[1..], Some(Key {
				modifier: mods | CTRL,
				value:    Char((input[0] | 0b010_00000) as char),
			}));
		}

		// Check if it's a unicode character.
		const WIDTH: [u8; 256] = [
			1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
			1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x1F
			1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
			1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x3F
			1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
			1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x5F
			1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
			1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x7F
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x9F
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xBF
			0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
			2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // 0xDF
			3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // 0xEF
			4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xFF
		];

		let length = WIDTH[input[0] as usize] as usize;

		if length >= input.len() {
			if let Ok(string) = str::from_utf8(&input[..length]) {
				return (&input[length..], Some(Key {
					modifier: mods,
					value:    Char(string.chars().next().unwrap())
				}));
			}
		}

		(&input[1..], None)
	}
}
