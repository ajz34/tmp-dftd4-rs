cmake_minimum_required(VERSION 3.15)
project(dep-dftd4)

include(ExternalProject)

if (NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE RELWITHDEBINFO)
endif()

set(DFTD4_SRC "" CACHE PATH "User specified directory of source code (dftd4)")
if(NOT DFTD4_SRC AND (NOT $ENV{DFTD4_SRC} EQUAL ""))
    set(DFTD4_SRC $ENV{DFTD4_SRC})
endif()

if(DFTD4_SRC)
    message(STATUS "User specified directory of source code (dftd4): ${DFTD4_SRC}")
else()
    set(DFTD4_SRC https://github.com/dftd4/dftd4.git)
    message(STATUS "Download source code (dftd4): ${DFTD4_SRC}")
endif()

ExternalProject_Add(dftd4
    GIT_REPOSITORY ${DFTD4_SRC}
    GIT_TAG v3.7.0
    PREFIX ${PROJECT_BINARY_DIR}/deps
    CMAKE_ARGS
        -GNinja
        -DCMAKE_BUILD_TYPE=RELEASE
        -DBUILD_SHARED_LIBS=${BUILD_SHARED_LIBS}
        -DCMAKE_INSTALL_PREFIX=${PROJECT_BINARY_DIR}/deps
)

install(DIRECTORY ${PROJECT_BINARY_DIR}/deps/lib DESTINATION ${CMAKE_INSTALL_PREFIX})
