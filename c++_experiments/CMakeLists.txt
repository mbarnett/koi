cmake_minimum_required (VERSION 3.1)
project (koi)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake/")
include(Warnings)
include(Targets)

find_package(Boost 1.60 COMPONENTS system filesystem REQUIRED)

set_default_target(Debug)

set(CMAKE_CXX_STANDARD 14)

enable_all_compiler_warnings()

include_directories(${Boost_INCLUDE_DIR} deps/fmt)

add_subdirectory(deps/fmt)

file(GLOB SOURCES "src/*.cpp")

add_executable(koi ${SOURCES})
target_link_libraries(koi ${Boost_LIBRARIES} fmt)
