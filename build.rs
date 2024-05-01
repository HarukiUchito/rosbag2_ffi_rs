use cmake;

fn main() {
    let dst = cmake::build("rosbag2_cpp_interface");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=rosbag2_cpp_interface");

    /*
    let bindings = bindgen::Builder::default()
        .header("cmake/library.hpp")
        //.header("/opt/ros/humble/include/rosbag2_cpp/rosbag2_cpp/reader.hpp")
        .detect_include_paths(true)
        .clang_arg("-I/usr/lib/gcc/x86_64-linux-gnu/11/include")
        .clang_arg("-I/opt/ros/humble/include/rclcpp")
        .clang_arg("-I/opt/ros/humble/include/rcl")
        .clang_arg("-I/opt/ros/humble/include/rmw")
        .clang_arg("-I/opt/ros/humble/include/rcutils")
        .clang_arg("-I/opt/ros/humble/include/rosidl_runtime_c")
        .clang_arg("-I/opt/ros/humble/include/rosidl_typesupport_interface")
        .clang_arg("-I/opt/ros/humble/include/rosidl_typesupport_introspection_cpp")
        .clang_arg("-I/opt/ros/humble/include/rosidl_typesupport_cpp")
        .clang_arg("-I/opt/ros/humble/include/rosidl_runtime_cpp")
        .clang_arg("-I/opt/ros/humble/include/rosbag2_cpp")
        .clang_arg("-I/opt/ros/humble/include/rosbag2_storage")
        .clang_arg("-I/opt/ros/humble/include/rcpputils")
        .clang_arg("-I/opt/ros/humble/include/rosbag2_storage")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");*/

    let soil_lib_dir = "/opt/ros/humble/lib";
    println!("cargo:rustc-link-search={}", soil_lib_dir);
    println!("cargo:rustc-link-lib=dylib=rosbag2_cpp");
    println!("cargo:rustc-link-lib=dylib=rosbag2_storage");
}
