#![allow(warnings)]

// PACKAGES
use std::fs;
use thiserror::Error;
use native_dialog::{FileDialog, MessageDialog, MessageType};


#[derive(Error, Debug)]
pub enum ErrorBox {
	#[error("system error or I/O failure")]
	IoFailure(#[from] std::io::Error),

	#[error("the implementation returns malformed strings")]
	InvalidString(#[from] std::string::FromUtf8Error),

	#[error("failed to parse the string returned from implementation")]
	UnexpectedOutput(&'static str),

	#[error("cannot find any dialog implementation (kdialog/zenity)")]
	NoImplementation,

	#[error("the implementation reports error")]
	ImplementationError(String),
}

type FnResult = Result<String, ErrorBox>;
fn folder_dialog() -> FnResult {
	let path = FileDialog::new()
		.set_location("~/Desktop")
		.show_open_single_dir()
		.unwrap();

	let path = match path {
		Some(path) => path,
		None => return Err(ErrorBox::NoImplementation),
	};

	let yes = MessageDialog::new()
		.set_type(MessageType::Info)
		.set_title("Do you want to open the file?")
		.set_text(&format!("{:#?}", path))
		.show_confirm()
		.unwrap();

	let newpath = path.into_os_string().into_string().unwrap();

	Ok(newpath)
}

fn folder_files(folder_path: &str) {
	let paths = fs::read_dir(folder_path).unwrap();

	for path in paths {
		println!("Name: {}", path.unwrap().path().display())
	}
}

fn main() {
	let x = folder_dialog().unwrap();
	folder_files(&x);
}
