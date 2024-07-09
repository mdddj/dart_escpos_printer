# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

project(dart_escpos_printer_library VERSION 0.0.1 LANGUAGES C)

add_library(dart_escpos_printer SHARED
  "dart_escpos_printer.c"
)

set_target_properties(dart_escpos_printer PROPERTIES
  PUBLIC_HEADER dart_escpos_printer.h
  OUTPUT_NAME "dart_escpos_printer"
)

target_compile_definitions(dart_escpos_printer PUBLIC DART_SHARED_LIB)
