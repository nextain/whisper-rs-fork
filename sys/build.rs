#![allow(clippy::uninlined_format_args)]

extern crate bindgen;
extern crate semver;

use cmake::Config;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();
    // Link C++ standard library
    if let Some(cpp_stdlib) = get_cpp_link_stdlib(&target) {
        println!("cargo:rustc-link-lib=dylib={}", cpp_stdlib);
    }
    // Link macOS Accelerate framework for matrix calculations
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=framework=Accelerate");
        #[cfg(feature = "coreml")]
        {
            println!("cargo:rustc-link-lib=framework=Foundation");
            println!("cargo:rustc-link-lib=framework=CoreML");
        }
        #[cfg(feature = "metal")]
        {
            println!("cargo:rustc-link-lib=framework=Foundation");
            println!("cargo:rustc-link-lib=framework=Metal");
            println!("cargo:rustc-link-lib=framework=MetalKit");
        }
    }

    #[cfg(feature = "coreml")]
    println!("cargo:rustc-link-lib=static=whisper.coreml");

    #[cfg(feature = "openblas")]
    {
        if let Ok(openblas_path) = env::var("OPENBLAS_PATH") {
            println!(
                "cargo::rustc-link-search={}",
                PathBuf::from(openblas_path).join("lib").display()
            );
        }
        if cfg!(windows) {
            println!("cargo:rustc-link-lib=libopenblas");
        } else {
            println!("cargo:rustc-link-lib=openblas");
        }
    }
    // cuda-dynamic: do NOT link CUDA libraries — they are loaded at runtime via dlopen
    #[cfg(all(feature = "cuda", not(feature = "cuda-dynamic")))]
    {
        println!("cargo:rustc-link-lib=cublas");
        println!("cargo:rustc-link-lib=cudart");
        println!("cargo:rustc-link-lib=cublasLt");
        println!("cargo:rustc-link-lib=cuda");
        cfg_if::cfg_if! {
            if #[cfg(target_os = "windows")] {
                let cuda_path = PathBuf::from(env::var("CUDA_PATH").unwrap()).join("lib/x64");
                println!("cargo:rustc-link-search={}", cuda_path.display());
            } else {
                println!("cargo:rustc-link-lib=culibos");
                println!("cargo:rustc-link-search=/usr/local/cuda/lib64");
                println!("cargo:rustc-link-search=/usr/local/cuda/lib64/stubs");
                println!("cargo:rustc-link-search=/opt/cuda/lib64");
                println!("cargo:rustc-link-search=/opt/cuda/lib64/stubs");
            }
        }
    }
    #[cfg(feature = "hipblas")]
    {
        println!("cargo:rustc-link-lib=hipblas");
        println!("cargo:rustc-link-lib=rocblas");
        println!("cargo:rustc-link-lib=amdhip64");

        cfg_if::cfg_if! {
            if #[cfg(target_os = "windows")] {
                panic!("Due to a problem with the last revision of the ROCm 5.7 library, it is not possible to compile the library for the windows environment.\nSee https://github.com/ggerganov/whisper.cpp/issues/2202 for more details.")
            } else {
                println!("cargo:rerun-if-env-changed=HIP_PATH");

                let hip_path = match env::var("HIP_PATH") {
                    Ok(path) =>PathBuf::from(path),
                    Err(_) => PathBuf::from("/opt/rocm"),
                };
                let hip_lib_path = hip_path.join("lib");

                println!("cargo:rustc-link-search={}",hip_lib_path.display());
            }
        }
    }

    #[cfg(feature = "openmp")]
    {
        if target.contains("gnu") {
            println!("cargo:rustc-link-lib=gomp");
        } else if target.contains("apple") {
            println!("cargo:rustc-link-lib=omp");
            println!("cargo:rustc-link-search=/opt/homebrew/opt/libomp/lib");
        }
    }

    println!("cargo:rerun-if-changed=wrapper.h");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let whisper_root = out.join("whisper.cpp");

    if !whisper_root.exists() {
        std::fs::create_dir_all(&whisper_root).unwrap();
        fs_extra::dir::copy("./whisper.cpp", &out, &Default::default()).unwrap_or_else(|e| {
            panic!(
                "Failed to copy whisper sources into {}: {}",
                whisper_root.display(),
                e
            )
        });
    }

    if env::var("WHISPER_DONT_GENERATE_BINDINGS").is_ok() || !has_libclang() {
        if !has_libclang() {
            println!("cargo:warning=libclang not found — using bundled bindings.rs");
        }
        let _: u64 = std::fs::copy("src/bindings.rs", out.join("bindings.rs"))
            .expect("Failed to copy bindings.rs");
    } else {
        // https://github.com/rust-lang/rust-bindgen/issues/2691
        // https://github.com/rust-lang/rust-bindgen/issues/3264
        // https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs/-/issues/779
        let package_msrv = match option_env!("CARGO_PKG_RUST_VERSION") {
            Some(v) if !v.is_empty() => {
                let v = semver::Version::parse(v).expect("Invalid CARGO_PKG_RUST_VERSION");
                bindgen::RustTarget::stable(v.minor, v.patch)
            }
            // as_chunks in utilities.rs requires 1.88+
            _ => bindgen::RustTarget::stable(88, 0),
        }
        .map_err(|v| v.to_string())
        .unwrap();

        let mut bindings = bindgen::Builder::default()
            .rust_edition(bindgen::RustEdition::Edition2021)
            .rust_target(package_msrv)
            .header("wrapper.h");

        #[cfg(feature = "metal")]
        {
            bindings = bindings.header("whisper.cpp/ggml/include/ggml-metal.h");
        }
        #[cfg(feature = "vulkan")]
        {
            bindings = bindings
                .header("whisper.cpp/ggml/include/ggml-vulkan.h")
                .clang_arg("-DGGML_USE_VULKAN=1");
        }

        // Blocklist Linux libc types that cause size assertion failures on Windows.
        // These types come from glibc headers and are not part of the whisper.cpp API.
        bindings = bindings
            .blocklist_type("_G_fpos_t")
            .blocklist_type("_G_fpos64_t")
            .blocklist_type("_IO_FILE")
            .blocklist_type("_IO_lock_t")
            .blocklist_type("_IO_marker")
            .blocklist_type("_IO_codecvt")
            .blocklist_type("_IO_wide_data")
            .blocklist_type("__FILE")
            .blocklist_type("FILE");

        let bindings = bindings
            .clang_arg("-I./whisper.cpp/")
            .clang_arg("-I./whisper.cpp/include")
            .clang_arg("-I./whisper.cpp/ggml/include")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate();

        match bindings {
            Ok(b) => {
                let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
                b.write_to_file(out_path.join("bindings.rs"))
                    .expect("Couldn't write bindings!");
            }
            Err(e) => {
                println!("cargo:warning=Unable to generate bindings: {}", e);
                println!("cargo:warning=Using bundled bindings.rs, which may be out of date");
                // copy src/bindings.rs to OUT_DIR
                std::fs::copy("src/bindings.rs", out.join("bindings.rs"))
                    .expect("Unable to copy bindings.rs");
            }
        }
    };

    // stop if we're on docs.rs
    if env::var("DOCS_RS").is_ok() {
        return;
    }

    let mut config = Config::new(&whisper_root);

    config
        .profile("Release")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("WHISPER_ALL_WARNINGS", "OFF")
        .define("WHISPER_ALL_WARNINGS_3RD_PARTY", "OFF")
        .define("WHISPER_BUILD_TESTS", "OFF")
        .define("WHISPER_BUILD_EXAMPLES", "OFF")
        .very_verbose(true)
        .pic(true);

    if cfg!(target_os = "windows") {
        config.cxxflag("/utf-8");
        println!("cargo:rustc-link-lib=advapi32");
    }

    if cfg!(feature = "coreml") {
        config.define("WHISPER_COREML", "ON");
        config.define("WHISPER_COREML_ALLOW_FALLBACK", "1");
    }

    if cfg!(feature = "cuda") && !cfg!(feature = "cuda-dynamic") {
        // Static CUDA: build ggml-cuda into the main static library
        config.define("GGML_CUDA", "ON");
        config.define("CMAKE_POSITION_INDEPENDENT_CODE", "ON");
        config.define("CMAKE_CUDA_FLAGS", "-Xcompiler=-fPIC");
    }
    // cuda-dynamic: main build is CPU-only; CUDA backend built separately as .so

    if cfg!(feature = "hipblas") {
        config.define("GGML_HIP", "ON");
        config.define("CMAKE_C_COMPILER", "hipcc");
        config.define("CMAKE_CXX_COMPILER", "hipcc");
        println!("cargo:rerun-if-env-changed=AMDGPU_TARGETS");
        if let Ok(gpu_targets) = env::var("AMDGPU_TARGETS") {
            config.define("AMDGPU_TARGETS", gpu_targets);
        }
    }

    if cfg!(feature = "vulkan") {
        config.define("GGML_VULKAN", "ON");
        if cfg!(windows) {
            println!("cargo:rerun-if-env-changed=VULKAN_SDK");
            println!("cargo:rustc-link-lib=vulkan-1");
            let vulkan_path = match env::var("VULKAN_SDK") {
                Ok(path) => PathBuf::from(path),
                Err(_) => panic!(
                    "Please install Vulkan SDK and ensure that VULKAN_SDK env variable is set"
                ),
            };
            let vulkan_lib_path = vulkan_path.join("Lib");
            println!("cargo:rustc-link-search={}", vulkan_lib_path.display());
        } else if cfg!(target_os = "macos") {
            println!("cargo:rerun-if-env-changed=VULKAN_SDK");
            println!("cargo:rustc-link-lib=vulkan");
            let vulkan_path = match env::var("VULKAN_SDK") {
                Ok(path) => PathBuf::from(path),
                Err(_) => panic!(
                    "Please install Vulkan SDK and ensure that VULKAN_SDK env variable is set"
                ),
            };
            let vulkan_lib_path = vulkan_path.join("lib");
            println!("cargo:rustc-link-search={}", vulkan_lib_path.display());
        } else {
            println!("cargo:rustc-link-lib=vulkan");
        }
    }

    if cfg!(feature = "openblas") {
        config.define("GGML_BLAS", "ON");
        config.define("GGML_BLAS_VENDOR", "OpenBLAS");
        if env::var("BLAS_INCLUDE_DIRS").is_err() {
            panic!("BLAS_INCLUDE_DIRS environment variable must be set when using OpenBLAS");
        }
        config.define("BLAS_INCLUDE_DIRS", env::var("BLAS_INCLUDE_DIRS").unwrap());
        println!("cargo:rerun-if-env-changed=BLAS_INCLUDE_DIRS");
    }

    if cfg!(feature = "metal") {
        config.define("GGML_METAL", "ON");
        config.define("GGML_METAL_NDEBUG", "ON");
        config.define("GGML_METAL_EMBED_LIBRARY", "ON");
    } else {
        // Metal is enabled by default, so we need to explicitly disable it
        config.define("GGML_METAL", "OFF");
    }

    if cfg!(debug_assertions) || cfg!(feature = "force-debug") {
        // debug builds are too slow to even remotely be usable,
        // so we build with optimizations even in debug mode
        config.define("CMAKE_BUILD_TYPE", "RelWithDebInfo");
        config.cxxflag("-DWHISPER_DEBUG");
    } else {
        // we're in release mode, explicitly set to release mode
        // see also https://codeberg.org/tazz4843/whisper-rs/issues/226
        config.define("CMAKE_BUILD_TYPE", "Release");
    }

    // Allow passing any WHISPER or CMAKE compile flags
    for (key, value) in env::vars() {
        let is_whisper_flag =
            key.starts_with("WHISPER_") && key != "WHISPER_DONT_GENERATE_BINDINGS";
        let is_ggml_flag = key.starts_with("GGML_");
        let is_cmake_flag = key.starts_with("CMAKE_");

        // cuda-dynamic: prevent GGML_CUDA from leaking into the main (CPU-only) build
        if cfg!(feature = "cuda-dynamic") && key == "GGML_CUDA" {
            continue;
        }

        if is_whisper_flag || is_ggml_flag || is_cmake_flag {
            config.define(&key, &value);
        }
    }

    if cfg!(not(feature = "openmp")) {
        config.define("GGML_OPENMP", "OFF");
    }

    if cfg!(feature = "intel-sycl") {
        config.define("BUILD_SHARED_LIBS", "ON");
        config.define("GGML_SYCL", "ON");
        config.define("GGML_SYCL_TARGET", "INTEL");
        config.define("CMAKE_C_COMPILER", "icx");
        config.define("CMAKE_CXX_COMPILER", "icpx");
    }

    let destination = config.build();

    add_link_search_path(&out.join("build")).unwrap();

    println!("cargo:rustc-link-search=native={}", destination.display());
    if cfg!(feature = "intel-sycl") {
        println!("cargo:rustc-link-lib=whisper");
        println!("cargo:rustc-link-lib=ggml");
        println!("cargo:rustc-link-lib=ggml-base");
        println!("cargo:rustc-link-lib=ggml-cpu");
    } else {
        println!("cargo:rustc-link-lib=static=whisper");
        println!("cargo:rustc-link-lib=static=ggml");
        println!("cargo:rustc-link-lib=static=ggml-base");
        println!("cargo:rustc-link-lib=static=ggml-cpu");
    }
    if cfg!(target_os = "macos") || cfg!(feature = "openblas") {
        println!("cargo:rustc-link-lib=static=ggml-blas");
    }
    if cfg!(feature = "vulkan") {
        if cfg!(feature = "intel-sycl") {
            println!("cargo:rustc-link-lib=ggml-vulkan");
        } else {
            println!("cargo:rustc-link-lib=static=ggml-vulkan");
        }
    }

    if cfg!(feature = "hipblas") {
        println!("cargo:rustc-link-lib=static=ggml-hip");
    }

    if cfg!(feature = "metal") {
        println!("cargo:rustc-link-lib=static=ggml-metal");
    }

    if cfg!(feature = "cuda") && !cfg!(feature = "cuda-dynamic") {
        println!("cargo:rustc-link-lib=static=ggml-cuda");
    }

    if cfg!(feature = "openblas") {
        println!("cargo:rustc-link-lib=static=ggml-blas");
    }

    if cfg!(feature = "intel-sycl") {
        println!("cargo:rustc-link-lib=ggml-sycl");
    }

    // cuda-dynamic: build CUDA backend as a separate shared library (.so/.dll)
    // This runs a second cmake pass to produce libggml-cuda.so (or ggml-cuda.dll on Windows),
    // which gets loaded at runtime via ggml's dlopen/LoadLibrary infrastructure (ggml_backend_load_all).
    // NOTE: Windows support is untested — file search patterns added but not verified on MSVC builds.
    #[cfg(feature = "cuda-dynamic")]
    {
        let cuda_build_dir = out.join("cuda-dynamic-build");
        std::fs::create_dir_all(&cuda_build_dir).unwrap();

        // Configure: build with CUDA enabled, shared libs, and dynamic backend loading
        let mut cmake_args = vec![
            format!("{}", whisper_root.display()),
            "-DCMAKE_BUILD_TYPE=Release".to_string(),
            "-DGGML_CUDA=ON".to_string(),
            "-DBUILD_SHARED_LIBS=ON".to_string(),
            "-DGGML_BACKEND_DL=ON".to_string(),
            "-DGGML_NATIVE=OFF".to_string(),
            "-DCMAKE_POSITION_INDEPENDENT_CODE=ON".to_string(),
            "-DWHISPER_BUILD_TESTS=OFF".to_string(),
            "-DWHISPER_BUILD_EXAMPLES=OFF".to_string(),
            "-DGGML_OPENMP=OFF".to_string(),
        ];

        // Pass through CUDA environment variables
        if let Ok(cuda_path) = env::var("CUDA_PATH") {
            cmake_args.push(format!("-DCMAKE_CUDA_COMPILER={}/bin/nvcc", cuda_path));
        } else if let Ok(cudacxx) = env::var("CUDACXX") {
            cmake_args.push(format!("-DCMAKE_CUDA_COMPILER={}", cudacxx));
        }

        let cmake_status = Command::new("cmake")
            .args(&cmake_args)
            .current_dir(&cuda_build_dir)
            .status()
            .expect("Failed to run cmake for cuda-dynamic build");
        assert!(cmake_status.success(), "cmake configure for cuda-dynamic failed");

        // Build only the ggml-cuda target
        let build_status = Command::new("cmake")
            .args(["--build", ".", "--target", "ggml-cuda", "--config", "Release", "-j"])
            .current_dir(&cuda_build_dir)
            .status()
            .expect("Failed to build cuda-dynamic target");
        assert!(build_status.success(), "cmake build for cuda-dynamic (ggml-cuda) failed");

        // Find the built CUDA backend shared library
        // Linux: libggml-cuda.so, Windows: ggml-cuda.dll
        let is_windows = target.contains("windows");
        let cuda_lib_prefix = if is_windows { "ggml-cuda" } else { "libggml-cuda" };
        let cuda_so = find_file_recursive(&cuda_build_dir, cuda_lib_prefix);
        let cuda_so = cuda_so.unwrap_or_else(|| panic!(
            "Could not find {} after cuda-dynamic build", cuda_lib_prefix
        ));

        // Also find ggml-base shared lib (required by ggml-cuda MODULE at runtime)
        // Linux: libggml-base.so(.version), Windows: ggml-base.dll
        let base_so = if is_windows {
            find_file_recursive(&cuda_build_dir, "ggml-base.dll")
        } else {
            find_file_recursive(&cuda_build_dir, "libggml-base.so.")
                .or_else(|| find_file_recursive(&cuda_build_dir, "libggml-base.so"))
        };

        // Export paths via cargo metadata so consuming crates can bundle these files
        println!("cargo:metadata=cuda_dynamic_so={}", cuda_so.display());
        if let Some(ref base) = base_so {
            println!("cargo:metadata=cuda_dynamic_base_so={}", base.display());
            // Also find versioned symlink targets (Linux only; Windows has no versioned .dll)
            let base_dir = base.parent().unwrap();
            let base_prefix = if is_windows { "ggml-base.dll" } else { "libggml-base.so" };
            for entry in std::fs::read_dir(base_dir).unwrap() {
                let entry = entry.unwrap();
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with(base_prefix) {
                    println!("cargo:metadata=cuda_dynamic_base_lib={}", entry.path().display());
                }
            }
        }

        println!("cargo:warning=cuda-dynamic: CUDA backend built at {}", cuda_so.display());
        if let Some(ref base) = base_so {
            println!("cargo:warning=cuda-dynamic: ggml-base shared lib at {}", base.display());
        }
    }

    println!(
        "cargo:WHISPER_CPP_VERSION={}",
        get_whisper_cpp_version(&whisper_root)
            .expect("Failed to read whisper.cpp CMake config")
            .expect("Could not find whisper.cpp version declaration"),
    );

    // for whatever reason this file is generated during build and triggers cargo complaining
    _ = std::fs::remove_file("bindings/javascript/package.json");
}

// From https://github.com/alexcrichton/cc-rs/blob/fba7feded71ee4f63cfe885673ead6d7b4f2f454/src/lib.rs#L2462
fn get_cpp_link_stdlib(target: &str) -> Option<&'static str> {
    if target.contains("msvc") {
        None
    } else if target.contains("apple") || target.contains("freebsd") || target.contains("openbsd") {
        Some("c++")
    } else if target.contains("android") {
        Some("c++_shared")
    } else {
        Some("stdc++")
    }
}

fn add_link_search_path(dir: &std::path::Path) -> std::io::Result<()> {
    if dir.is_dir() {
        println!("cargo:rustc-link-search={}", dir.display());
        for entry in std::fs::read_dir(dir)? {
            add_link_search_path(&entry?.path())?;
        }
    }
    Ok(())
}

fn find_file_recursive(dir: &std::path::Path, name_prefix: &str) -> Option<PathBuf> {
    if !dir.is_dir() {
        return None;
    }
    for entry in std::fs::read_dir(dir).ok()? {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.is_file() {
            if let Some(fname) = path.file_name().and_then(|n| n.to_str()) {
                if fname.starts_with(name_prefix) && !fname.ends_with(".o") {
                    return Some(path);
                }
            }
        } else if path.is_dir() {
            if let Some(found) = find_file_recursive(&path, name_prefix) {
                return Some(found);
            }
        }
    }
    None
}

/// Check if libclang is available without panicking.
/// bindgen panics inside Builder::generate() if libclang is missing.
/// We detect this by checking common paths and env vars.
fn has_libclang() -> bool {
    // 1. LIBCLANG_PATH explicitly set
    if env::var("LIBCLANG_PATH").is_ok() {
        return true;
    }
    // 2. LLVM_CONFIG_PATH set
    if env::var("LLVM_CONFIG_PATH").is_ok() {
        return true;
    }
    // 3. Try to find clang/llvm-config in PATH
    if Command::new("llvm-config").arg("--version").output().is_ok() {
        return true;
    }
    // 4. Check well-known LLVM install paths
    let candidates: Vec<PathBuf> = if cfg!(windows) {
        // Windows: LLVM typically installed via winget/choco
        let program_files = env::var("ProgramFiles").unwrap_or_else(|_| "C:\\Program Files".into());
        vec![
            PathBuf::from(&program_files).join("LLVM\\bin\\libclang.dll"),
            PathBuf::from("C:\\Program Files\\LLVM\\bin\\libclang.dll"),
        ]
    } else {
        // Linux: libclang-dev package
        vec![
            PathBuf::from("/usr/lib/llvm-18/lib/libclang.so"),
            PathBuf::from("/usr/lib/llvm-17/lib/libclang.so"),
            PathBuf::from("/usr/lib/llvm-16/lib/libclang.so"),
            PathBuf::from("/usr/lib/llvm-14/lib/libclang.so"),
            PathBuf::from("/usr/lib/libclang.so"),
        ]
    };
    candidates.iter().any(|p| p.exists())
}

fn get_whisper_cpp_version(whisper_root: &std::path::Path) -> std::io::Result<Option<String>> {
    let cmake_lists = BufReader::new(File::open(whisper_root.join("CMakeLists.txt"))?);

    for line in cmake_lists.lines() {
        let line = line?;

        if let Some(suffix) = line.strip_prefix(r#"project("whisper.cpp" VERSION "#) {
            let whisper_cpp_version = suffix.trim_end_matches(')');
            return Ok(Some(whisper_cpp_version.into()));
        }
    }

    Ok(None)
}
