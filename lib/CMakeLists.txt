cmake_minimum_required(VERSION 3.5)

add_subdirectory(corrosion)

project(skdeck-lib VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(CMAKE_AUTOMOC ON)

find_package(Qt5 REQUIRED COMPONENTS Core)
corrosion_import_crate(MANIFEST_PATH rust/Cargo.toml)

set(PROJECT_SOURCES
    include/ck3/backuploader.h
    include/ck3/loader.h
    include/ck3/model.h
    include/ck3/save.h
    include/ck3/saveloader.h
    ck3/backuploader.cpp
    ck3/loader.cpp
    ck3/model.cpp
    ck3/rssavenotifier.cpp
    ck3/rssavenotifier.h
    ck3/save.cpp
    ck3/saveloader.cpp
)


add_library(skdeck-lib STATIC ${PROJECT_SOURCES})
target_include_directories(skdeck-lib PUBLIC include)
target_link_libraries(skdeck-lib PRIVATE Qt5::Core)
target_link_libraries(skdeck-lib PRIVATE skdeck-lib-rs)
