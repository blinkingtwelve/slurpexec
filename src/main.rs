use std::env::args;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io;
use std::path::Path;
use std::process::exit;

use nix::unistd::execv;

const USAGE: &str = r#"Invocation error. Usage example (from a shell):
slurpexec -f /path/to/thefilestdinwillgointo /bin/cat /path/to/thefilestdinwillgointo
"#;

fn cstringify(argv: &[String]) -> Vec<Box<CString>> {
    let mut cstrings: Vec<Box<CString>> = vec![];
    for arg in argv {
        let arg_bytes: &[u8] = &arg.as_bytes();
        let cstring = CString::new(arg_bytes).expect("Not a nice C string");
        cstrings.push(Box::new(cstring));
    }
    return cstrings;
}

fn execv_stringarray(argv: &[String]) {
    let mut argv_c_boxed = cstringify(argv);
    let mut argv_c_ptrs: Vec<&CStr> = vec![];
    for arg in &mut argv_c_boxed {
        argv_c_ptrs.push(&**arg);
    }
    let _ = execv(argv_c_ptrs[0], &argv_c_ptrs);
}

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();
    if args.len() < 3 || args[1] != "-f" {
        eprintln!("{}", USAGE);
        exit(101);
    }
    let stdin = &mut io::stdin();
    let outfile = &mut File::create(&Path::new(&args[2])).unwrap_or_else(|error| {
        eprintln!("Can't open file '{}' for writing: {:?}", &args[2], error);
        exit(error.raw_os_error().unwrap());
    });
    io::copy(stdin, outfile).unwrap_or_else(|error| {
        eprintln!("Can't write to file '{}': {:?}", &args[2], error);
        exit(error.raw_os_error().unwrap());
    });
    execv_stringarray(&args[3..]);
    let last_err = io::Error::last_os_error();
    eprintln!(
        "Execv failed with error {:?}: {}",
        last_err,
        args[3..].join(" ")
    );
    exit(last_err.raw_os_error().unwrap());
}
