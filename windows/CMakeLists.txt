cmake_minimum_required(VERSION 3.10)
set(PROJECT_NAME "dart_escpos_printer")
project(${PROJECT_NAME} LANGUAGES CXX)
include("../cargokit/cmake/cargokit.cmake")
apply_cargokit(${PROJECT_NAME} ../rust dart_escpos_printer "")
set(dart_escpos_printer_bundled_libraries
  "${${PROJECT_NAME}_cargokit_lib}"
  PARENT_SCOPE
)