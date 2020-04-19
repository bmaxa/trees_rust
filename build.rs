// build.rs

use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
/*    Command::new("gcc").args(&["src/hello.c", "-c", "-fPIC", "-o"])
                       .arg(&format!("{}/hello.o", out_dir))
                       .status().unwrap();
*/
    Command::new("fasm").arg("src/rdtsc.asm").arg(out_dir.clone()+"/rdtsc.o").status().unwrap();
    Command::new("ar").args(&["crus", "librdtsc.a", "rdtsc.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=rdtsc");
    println!("cargo:rustc-link-lib=dylib=atomic");
}
