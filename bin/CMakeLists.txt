cmake_minimum_required(VERSION 3.5)

project(skdeck VERSION 0.1 LANGUAGES CXX)

set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt5 REQUIRED COMPONENTS Widgets LinguistTools)

set(TS_FILES
    ts/skdeck_en_US.ts
    ts/skdeck_fr_FR.ts
)

set(PROJECT_SOURCES
    main.cpp
    mainwindow.cpp
    mainwindow.h
    mainwindow.ui
    ${TS_FILES}
    ${PROJECT_BINARY_DIR}/translations.qrc
)

qt5_create_translation(QM_FILES ${PROJECT_SOURCE_DIR} ${TS_FILES})
configure_file(translations.qrc ${PROJECT_BINARY_DIR} COPYONLY)

add_executable(skdeck ${PROJECT_SOURCES})
target_link_libraries(skdeck PRIVATE skdeck-lib)
target_link_libraries(skdeck PRIVATE Qt5::Widgets)