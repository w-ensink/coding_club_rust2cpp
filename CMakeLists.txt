cmake_minimum_required(VERSION 3.22)
project(Rust2Cpp VERSION 0.0.1)

add_subdirectory(external/corrosion)
add_subdirectory(external/JUCE)

add_subdirectory(plugin)
