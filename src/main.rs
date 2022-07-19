#![allow(warnings)]

// PACKAGES
use std::fs;
use std::fs::metadata;

extern crate bytesize;
use bytesize::ByteSize;

// MODULES
mod dialog;
pub use crate::dialog::dialog_fn;

use fltk::{
	app,
	button::Button,
	frame::Frame,
	prelude::*,
	window::Window,
	enums::{Align, Color, Event, FrameType, Key, Shortcut}
};
use fltk_theme::{widget_themes, WidgetTheme, ThemeType};

#[derive(Debug)]
struct Data {
	name: String,
	data_type: String,
	size: i64,
}

fn folder_files(folder_path: &str) -> Vec<Data> {
	let paths = fs::read_dir(folder_path).unwrap();
	let mut base = vec![];

	for path in paths {
		let md = metadata(path.as_ref().unwrap().path()).unwrap();

		let f_type = match fs::metadata(path.as_ref().unwrap().path()) {
			Ok(m) => m,
			Err(e) => panic!("There was a problem opening the file: {:?}", e),
		};

		//println!("Name: {:?}, Dir: {}", path.as_ref().unwrap().file_name(), md.is_dir());
		//println!("Type: {:?}\n", f_type);
		//println!("{:?}\n", ByteSize::kb(md.len()));
		//println!("|--- {:?}", path.as_ref().unwrap().file_name());

		if md.is_dir() {
			let y = path.as_ref().unwrap().file_name();

			base.push(Data {
				name: y.to_os_string().into_string().unwrap(),
				data_type: "folder".to_string(),
				size: 0
			});

		} else {
			let y = path.as_ref().unwrap().file_name();

			base.push(Data {
				name: y.to_os_string().into_string().unwrap(),
				data_type: "file".to_string(),
				size: 0
			});
		}
	}

	return base
}

fn main() {
	let app = app::App::default();
	let mut wind = Window::default().with_size(230, 180).with_label("Win App");
	
	let mut frame_one = Frame::default().with_size(200, 40).center_of(&wind);
	let mut but_one = Button::new(10, 10, 100, 40, "Open Folder");
	let mut but_two = Button::new(120, 10, 100, 40, "Visualize");

	// -- STYLES --
	let widget_theme = WidgetTheme::new(ThemeType::Aero);
	widget_theme.apply();
	app::set_background_color(0x42, 0x42, 0x42);

	frame_one.set_frame(FrameType::EngravedBox);
	frame_one.set_color(Color::White);
	frame_one.set_label_color(Color::Black);
	// -- STYLES --

	wind.end();
	wind.show();

	but_one.set_callback(move |_| {
		let x = dialog_fn::folder_dialog().unwrap();

		let result = folder_files(&x);

		println!("{:?}", result);

		frame_one.set_label("Opened !");
	});

	but_two.set_callback(move |_| {});

	app.run().unwrap();
}
