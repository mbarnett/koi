include(Colorize)

macro(set_default_target _target)
  if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE "${_target}")
  endif(NOT CMAKE_BUILD_TYPE)

  message(STATUS "Creating ${BoldRed}${CMAKE_BUILD_TYPE}${ColorReset} target...")
endmacro()
