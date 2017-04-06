use std::env;
use std::ffi::OsStr;
use std::process::Command;
use std::str;

fn rbconfig(key: &str) -> Vec<u8> {
    let ruby = match env::var_os("RUBY") {
        Some(val) => val.to_os_string(),
        None => OsStr::new("ruby").to_os_string(),
    };
    let config = Command::new(ruby)
        .arg("-e")
        .arg(format!("print RbConfig::CONFIG['{}']", key))
        .output()
        .unwrap_or_else(|e| panic!("ruby not found: {}", e));

    config.stdout
}

fn use_static() {
    let ruby_libs = rbconfig("LIBS");
    let libs = String::from_utf8_lossy(&ruby_libs);

    // Ruby gives back the libs in the form: `-lpthread -lgmp`
    // Cargo wants them as: `-l pthread -l gmp`
    let transformed_lib_args = libs.replace("-l", "-l ");

    println!("cargo:rustc-flags={}", transformed_lib_args);
}

fn use_dylib() {
    let libruby_so = rbconfig("RUBY_SO_NAME");

    println!("cargo:rustc-link-lib=dylib={}",
             String::from_utf8_lossy(&libruby_so));
}

fn main() {
    let libdir = rbconfig("libdir");
    let libruby_shared = rbconfig("ENABLE_SHARED");

    if env::var_os("RUBY_STATIC").is_some() {
        use_static()
    } else {
        match str::from_utf8(&libruby_shared).expect("RbConfig value not UTF-8!") {
            "no" => use_static(),
            "yes" => use_dylib(),
            _ => {
                let msg = "Error! Couldn't find a valid value for \
                RbConfig::CONFIG['ENABLE_SHARED']. \
                This may mean that your ruby's build config is corrupted. \
                Possible solution: build a new Ruby with the `--enable-shared` configure opt.";
                panic!(msg)
            }
        }
    }

    println!("cargo:rustc-link-search={}",
             String::from_utf8_lossy(&libdir));
}
