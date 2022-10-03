cmake_minimum_required(VERSION 3.5)

add_subdirectory(corrosion)

project(skdeck-lib VERSION 0.1 LANGUAGES CXX)

set(CMAKE_INCLUDE_CURRENT_DIR ON)
set(CMAKE_AUTOMOC ON)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt5 REQUIRED COMPONENTS Core)
corrosion_import_crate(MANIFEST_PATH rust/Cargo.toml)

set(PROJECT_SOURCES
    crusaderkingsmanager.cpp
    crusaderkingsmanager.h
)

add_library(skdeck-lib STATIC ${PROJECT_SOURCES})
target_link_libraries(skdeck-lib PRIVATE Qt5::Core)
target_link_libraries(skdeck-lib PRIVATE skdeck-lib-rs)
