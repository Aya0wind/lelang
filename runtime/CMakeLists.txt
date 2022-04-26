cmake_minimum_required(VERSION 3.22)
project(runtime C)

set(CMAKE_C_STANDARD 11)

add_library(runtime STATIC library.c)
