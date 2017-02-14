extern crate bane;
use bane::{Event, Key, keys};

fn main() {
	let mut term   = bane::Terminal::default().unwrap();
	let     events = term.events();

	term.features().raw(true).unwrap().echo(false).unwrap();

	for event in events {
		if let Event::Key(Key { value: keys::Char('q'), .. }) = event {
			break;
		}
		else {
			println!("{:?}", event);
		}
	}
}
