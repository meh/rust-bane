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

use std::collections::HashMap;
use std::sync::Mutex;

use chan;
use libc::{signal, SIGWINCH, c_int, sighandler_t};

use terminal::Event;

lazy_static! {
	static ref SUBSCRIBERS: Mutex<(u32, HashMap<u32, chan::Sender<Event>>)> = {
		Mutex::new((0, HashMap::new()))
	};
}

unsafe extern "C" fn handler(num: c_int) {
	if num != SIGWINCH {
		return;
	}

	if let Ok(guard) = SUBSCRIBERS.try_lock() {
		for subscriber in guard.1.values() {
			subscriber.send(Event::Resize);
		}
	}
}

pub fn register(sender: chan::Sender<Event>) -> u32 {
	let mut guard = SUBSCRIBERS.lock().unwrap();
	let     id    = guard.0 + 1;

	if guard.1.is_empty() {
		unsafe {
			signal(SIGWINCH, handler as sighandler_t);
		}
	}

	guard.0 = id;
	guard.1.insert(id, sender);

	id
}

pub fn unregister(id: u32) {
	SUBSCRIBERS.lock().unwrap().1.remove(&id);
}
