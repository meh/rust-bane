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
use std::mem;

use channel;
use libc::{sigaction, SIGWINCH, c_int, sighandler_t};

use terminal::Event;

lazy_static! {
	static ref SUBSCRIBERS: Mutex<(u32, HashMap<u32, channel::Sender<Event>>)> = {
		Mutex::new((0, HashMap::new()))
	};
}

unsafe extern "C" fn handler(num: c_int) {
	if num != SIGWINCH {
		return;
	}

	if let Ok(guard) = SUBSCRIBERS.try_lock() {
		for subscriber in guard.1.values() {
			subscriber.send(Event::Resize).unwrap();
		}
	}
}

/// A registered resize handler.
#[derive(Debug)]
pub struct Handler(u32);

/// Register a new resize handler.
pub fn register(sender: channel::Sender<Event>) -> Handler {
	let mut guard = SUBSCRIBERS.lock().unwrap();
	let     id    = guard.0 + 1;

	if guard.1.is_empty() {
		unsafe {
			let mut old: sigaction = mem::zeroed();
			let mut new: sigaction = mem::zeroed();
			new.sa_sigaction = handler as sighandler_t;

			sigaction(SIGWINCH, &new, &mut old);
		}
	}

	guard.0 = id;
	guard.1.insert(id, sender);

	Handler(id)
}

/// Unregister the given handler.
pub fn unregister(id: Handler) {
	SUBSCRIBERS.lock().unwrap().1.remove(&id.0);
}
