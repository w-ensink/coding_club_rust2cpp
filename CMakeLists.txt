cmake_minimum_required(VERSION 3.22)
project(Rust2Cpp VERSION 0.0.1)

add_subdirectory(external/corrosion)
add_subdirectory(external/JUCE)

corrosion_import_crate(MANIFEST_PATH biquad/Cargo.toml)
corrosion_add_cxxbridge(biquad_rs CRATE biquad FILES bindings.rs)

add_subdirectory(plugin)
