use std::env::args;
use std::ffi::{CStr, CString};
use std::process::exit;
use std::io;
use std::fs::File;
use std::path::Path;

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
    let outfile = &mut File::create(&Path::new(&args[2]))?;
    io::copy(stdin, outfile)?;
    execv_stringarray(&args[3..]);
    panic!("Exec failed: {}", args[3..].join(" "));
}
