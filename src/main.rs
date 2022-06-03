#![allow(warnings)]

// PACKAGES
use std::fs;
use std::fs::metadata;

extern crate bytesize;
use bytesize::ByteSize;

// MODULES
mod dialog;
pub use crate::dialog::dialog_fn;


fn folder_files(folder_path: &str) {
	let paths = fs::read_dir(folder_path).unwrap();

	for path in paths {
		let md = metadata(path.as_ref().unwrap().path()).unwrap();

		let f_type = match fs::metadata(path.as_ref().unwrap().path()) {
			Ok(m) => m,
			Err(e) => panic!("There was a problem opening the file: {:?}", e),
		};

		println!("Name: {:?}, Dir: {}", path.as_ref().unwrap().file_name(), md.is_dir());
		//println!("Type: {:?}\n", f_type);
		println!("{:?}\n", ByteSize::kb(md.len()));
	}
}

fn main() {
	let x = dialog_fn::folder_dialog().unwrap();
	folder_files(&x);
}
