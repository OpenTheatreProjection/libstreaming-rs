use std::env;
use std::path::PathBuf;

#[cfg(feature = "enable_nvenc")]
fn compile_nvenc(){
    println!("cargo:rerun-if-changed=c/nvEncodeAPI.h");
    println!("cargo:rustc-link-lib-search=/opt/cuda/lib");
    println!("cargo:rustc-link-lib=nvidia-encode");
    println!("cargo:rustc-link-lib=cuda");
    let bindings = bindgen::Builder::default()
        .header("c/nvEncodeAPI.h")
        .clang_arg("-I/opt/cuda/include")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("nvenc.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(feature = "enable_ffmpeg")]
fn compile_ffmpeg(){
    println!("cargo:rerun-if-changed=c/ffmpeg.h");
    println!("cargo:rustc-link-lib-search=native=./lib");
    println!("cargo:rustc-link-search=native=./lib");
    println!("cargo:rustc-link-lib=avcodec");
    println!("cargo:rustc-link-lib=avdevice");
    println!("cargo:rustc-link-lib=avutil");
    println!("cargo:rustc-link-lib=avformat");
    println!("cargo:rustc-link-lib=avfilter");
    println!("cargo:rustc-link-lib=swscale");
    println!("cargo:rustc-link-lib=swresample");
    println!("cargo:rustc-link-lib=postproc");
    let bindings = bindgen::Builder::default()
        .header("c/ffmpeg.h")
        .clang_arg("-I.")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("ffmpeg.rs"))
        .expect("Couldn't write bindings!");
}


fn main(){
    #[cfg(feature = "enable_nvenc")]
    compile_nvenc();

    #[cfg(feature = "enable_ffmpeg")]
    compile_ffmpeg();
}