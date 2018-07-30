macro(enable_all_compiler_warnings)
  if ("${CMAKE_CXX_COMPILER_ID}" MATCHES ".*Clang" OR "${CMAKE_CXX_COMPILER_ID}" STREQUAL "GNU")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra")
  elseif ("${CMAKE_CXX_COMPILER_ID}" STREQUAL "MSVC")
    set(CMAKE_CXX_WARNING_LEVEL 4)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /W4")
  else()
    message(FATAL_ERROR "Unrecognized CMAKE_CXX_COMPILER_ID \"${CMAKE_CXX_COMPILER_ID}\", could not set Warning flags for your environment")
  endif()
endmacro()