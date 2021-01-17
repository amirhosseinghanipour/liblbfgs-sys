use std::fs;

fn fail_on_empty_directory(name: &str) {
    if fs::read_dir(name).unwrap().count() == 0 {
        println!(
            "The `{}` directory is empty, did you forget to pull the submodules?",
            name
        );
        println!("Try `git submodule update --init --recursive`");
        panic!();
    }
}

fn build() {
    // let dst = cmake::Config::new("").build_target("lbfgs").build();
    let dst = cmake::Config::new("").build();

    // fs::create_dir_all(dst.join("include")).unwrap();
    // fs::copy("src/liblbfgs/include/lbfgs.h", dst.join("include/lbfgs.h")).unwrap();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=lbfgs");
    println!("cargo:root={}", dst.to_str().unwrap());
    println!("cargo:include={}/include", dst.to_str().unwrap());
}

fn main() {
    fail_on_empty_directory("src/liblbfgs");
    build();
}
