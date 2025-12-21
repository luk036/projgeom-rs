cmake_minimum_required(VERSION 3.20)
project(projgeom-cpp VERSION 0.1.3 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

option(BUILD_SHARED_LIBS "Build shared library" ON)
option(BUILD_TESTING "Build tests" ON)

# Add doctest if available
find_package(doctest QUIET)
if(NOT doctest_FOUND)
    message(STATUS "doctest not found, downloading...")
    include(FetchContent)
    FetchContent_Declare(
        doctest
        GIT_REPOSITORY https://github.com/doctest/doctest.git
        GIT_TAG v2.4.11
    )
    FetchContent_MakeAvailable(doctest)
endif()

# Library
add_library(projgeom-cpp
    src/pg_object.cpp
    src/pg_plane.cpp
    src/ck_plane.cpp
    src/ell_object.cpp
    src/euclid_object.cpp
    src/hyp_object.cpp
    src/myck_object.cpp
    src/persp_object.cpp
)

target_include_directories(projgeom-cpp PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include>
)

target_compile_features(projgeom-cpp PUBLIC cxx_std_23)

# Tests
if(BUILD_TESTING)
    add_executable(tests
        tests/main.cpp
        tests/test_pg_object.cpp
        tests/test_pg_plane.cpp
        tests/test_ck_plane.cpp
        tests/test_geometries.cpp
    )
    
    target_link_libraries(tests PRIVATE projgeom-cpp doctest::doctest)
    
    add_test(NAME projgeom-tests COMMAND tests)
endif()

# Install
install(TARGETS projgeom-cpp
    EXPORT projgeom-cpp-targets
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
    RUNTIME DESTINATION bin
    INCLUDES DESTINATION include
)

install(DIRECTORY include/projgeom
    DESTINATION include
)

install(EXPORT projgeom-cpp-targets
    FILE projgeom-cpp-config.cmake
    NAMESPACE projgeom-cpp::
    DESTINATION lib/cmake/projgeom-cpp
)