#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "projgeom-cpp::projgeom-cpp" for configuration "Release"
set_property(TARGET projgeom-cpp::projgeom-cpp APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(projgeom-cpp::projgeom-cpp PROPERTIES
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libprojgeom-cpp.so"
  IMPORTED_SONAME_RELEASE "libprojgeom-cpp.so"
  )

list(APPEND _cmake_import_check_targets projgeom-cpp::projgeom-cpp )
list(APPEND _cmake_import_check_files_for_projgeom-cpp::projgeom-cpp "${_IMPORT_PREFIX}/lib/libprojgeom-cpp.so" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
