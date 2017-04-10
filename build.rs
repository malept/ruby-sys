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

fn use_libdir() {
    let libdir = rbconfig("libdir");
    println!("cargo:rustc-link-search={}",
             String::from_utf8_lossy(&libdir));
}

fn transform_lib_args(rbconfig_key: &str, replacement: &str) -> String {
    let rbconfig_value = rbconfig(rbconfig_key);
    let libs = String::from_utf8_lossy(&rbconfig_value);

    libs.replace("-l", replacement)
}

fn use_static() {
    let ruby_target_os = rbconfig("target_os");
    let target_os = str::from_utf8(&ruby_target_os).expect("RbConfig value not UTF-8!");

    if target_os == "mingw32" {
        use_libdir();
        println!("cargo:rustc-link-lib=static={}", transform_lib_args("LIBRUBYARG_STATIC", ""));
    } else {
        // Ruby gives back the libs in the form: `-lpthread -lgmp`
        // Cargo wants them as: `-l pthread -l gmp`
        println!("cargo:rustc-flags={}", transform_lib_args("LIBS", "-l "));
    }
}

fn use_dylib() {
    use_libdir();
    let libruby_so = rbconfig("RUBY_SO_NAME");
    println!("cargo:rustc-link-lib=dylib={}",
             String::from_utf8_lossy(&libruby_so));
}

fn main() {
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
}
