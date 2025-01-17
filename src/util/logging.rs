use std::{fmt::Display, io::Write};

#[macro_export]
macro_rules! info {
    ($x:expr $(, $more:expr)*) => {{
    	use ::colored::Colorize;
        println!("{}{}", "| Info | ".bright_cyan(), format!($x, $($more),*));
    }}
}

#[macro_export]
macro_rules! fail {
    ($x:expr $(, $more:expr)*) => {{
    	use ::colored::Colorize;
        eprintln!("{}{}", "| Fail | ".bright_red(), format!($x, $($more),*));
    }}
}

#[macro_export]
macro_rules! fatal {
    ($x:expr $(, $more:expr)*) => {{
        use ::colored::Colorize;
        eprintln!("{}{}", "| Fail | ".bright_red(), format!($x, $($more),*));
        std::process::exit(1);
    }}
}

#[macro_export]
macro_rules! warn {
    ($x:expr $(, $more:expr)*) => {{
    	use ::colored::Colorize;
        eprintln!("{}{}", "| Warn | ".bright_yellow(), format!($x, $($more),*));
    }}
}

#[macro_export]
macro_rules! done {
    ($x:expr $(, $more:expr)*) => {{
    	use ::colored::Colorize;
        println!("{}{}", "| Done | ".bright_green(), format!($x, $($more),*));
    }}
}

#[macro_export]
macro_rules! confirm {
    ($x:expr $(, $more:expr)*) => {
        $crate::logging::ask_confirm(&format!($x, $($more),*), false)
    };
}

#[macro_export]
macro_rules! geode_assert {
	($cond:expr, $x:expr $(, $more:expr)*) => {
		if !$cond {
			use crate::fatal;
			fatal!($x $(, $more)*);
		}
	}
}

pub fn ask_confirm(text: &String, default: bool) -> bool {
	use ::colored::Colorize;
	// print question
	print!(
		"{}{} {} ",
		"| Okay | ".bright_purple(),
		text,
		if default { "(Y/n)" } else { "(N/y)" }
	);
	std::io::stdout().flush().unwrap();
	let mut yes = String::new();
	match std::io::stdin().read_line(&mut yes) {
		Ok(_) => match yes.trim().to_lowercase().as_str() {
			"yes" => true,
			"ye" => true,
			"y" => true,
			"no" => false,
			"n" => false,
			_ => default,
		},
		Err(_) => default,
	}
}

pub trait NiceUnwrap<T> {
	fn nice_unwrap<S: Display>(self, text: S) -> T;
}

impl<T, E: Display> NiceUnwrap<T> for Result<T, E> {
	fn nice_unwrap<S: Display>(self, text: S) -> T {
		self.unwrap_or_else(|e| fatal!("{}: {}", text, e))
	}
}

impl<T> NiceUnwrap<T> for Option<T> {
	fn nice_unwrap<S: Display>(self, text: S) -> T {
		self.unwrap_or_else(|| fatal!("{}", text))
	}
}
