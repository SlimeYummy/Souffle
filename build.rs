use std::error;
use std::process;
use std::fs;
use std::path;

type BoxErr = Box<error::Error>;

fn list_cpp(mod_name: &str) -> Result<Vec<String>, BoxErr> {
    let mod_path = path::Path::new(
        &format!(".\\src\\{}", mod_name)
    );
    if !mod_path.is_dir() {
        return Err(
            format!("Not a mod {}", mod_name)
        );
    }
    let file_name_vec: Vec<String> = Vec::new();
    for iter in try!(fs::read_dir(mod_path)) {
        let file_path = try!(iter).path();
        if !file_path.is_file() {
            continue;
use std::process;
use std::fs;
use std::path;
use std::ffi;

fn list_cpp(mod_name: &str) -> Vec<String> {
    let mod_path_str = format!(".\\src\\{}", mod_name);
    let mod_path = path::Path::new(&mod_path_str);
    if !mod_path.is_dir() {
        panic!("Not a mod. [{}]", mod_name);
    }
    let mut file_name_vec = Vec::<String>::new();
    for entry in fs::read_dir(mod_path).unwrap() {
        if let Ok(entry) = entry {
            let file_path = entry.path();
            if !file_path.is_file() {
                continue;
            }
            let ext_name = file_path.extension().unwrap_or(ffi::OsStr::new(""));
            if ext_name == ".cpp" {
                let file_name_os = file_path.file_stem().unwrap();
                let file_name = String::from(
                    file_name_os.to_str().unwrap()
                );
                file_name_vec.push(file_name);
            }
        }
    }
    return file_name_vec;
}

fn compile_cpp(mod_name: &str, file_name_vec: &[String]) {
    let cpp_name_vec: Vec<String> =
        file_name_vec.iter()
        .map(|file_name|{
            format!("..\\src\\{}\\{}.cpp", mod_name, file_name)
        })
        .collect();
    process::Command::new("cl.exe")
    .current_dir(format!(".\\native\\{}", mod_name))
    .args(&["/c", "\\Ox"])
    .args(&cpp_name_vec)
    .status()
    .expect(&format!("Execute cl.exe err. [{}]", mod_name));
}

fn link_cpp(mod_name: &str, file_name_vec: &[String]) {
    let obj_name_vec: Vec<String> =
        file_name_vec.iter()
        .map(|file_name|{
            format!("{}\\{}.obj", mod_name, file_name)
        })
        .collect();
    process::Command::new("lib.exe")
    .current_dir(".\\native")
    .args(&obj_name_vec)
    .arg(format!("/OUT:\"{}.lib\"", mod_name))
    .status()
    .expect(&format!("Execute lib.exe err. [{}]", mod_name));
}

fn build_cpp(mod_name: &str) {
    let file_base_vec = list_cpp(mod_name);
    compile_cpp(mod_name, &file_base_vec);
    link_cpp(mod_name, &file_base_vec);
}

fn main() {
    fs::create_dir(".\\target").unwrap();
    fs::create_dir(".\\target\\native").unwrap();
    build_cpp("");
    //println!("cargo:rustc-link-search=./target/native/");
}
