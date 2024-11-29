use std::ffi::OsString;


pub fn name_to_title(name: OsString) -> String {

    let name = name.into_string().expect("Cannot covert file name to string");

    let name = name.split('.').next().expect("No file name");

    name.to_lowercase().replace("_", " ")
}