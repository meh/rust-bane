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

macro_rules! cap {
	($db:expr => $name:ident) => (
		$db.get::<$crate::info::capability::$name>().ok_or($crate::Error::NotSupported)
	);
}

macro_rules! expand {
	(? $term:expr => $capability:ident) => (
		expand!(? $term => $capability;)
	);

	(? $term:expr => $capability:ident; $($name:ident : $value:expr),*) => (
		$term.expansion(|info, context, output| {
			if let Ok(cap) = cap!(info => $capability) {
				cap.expand()$(.$name($value))*.with(context).to(output)?;
				Ok(false)
			}
			else {
				Ok(true)
			}
		})
	);

	($term:expr => $capability:ident) => (
		expand!($term => $capability;)
	);

	($term:expr => $capability:ident; $($name:ident : $value:expr),*) => (
		$term.expansion(|info, context, output| {
			cap!(info => $capability)?.expand()$(.$name($value))*.with(context).to(output)?;
			Ok(())
		})
	);
}
