{
    values = {
        "/data/data/com.termux/files/usr/bin/g++",
        {
            "-fvisibility=hidden",
            "-fvisibility-inlines-hidden",
            "-O3",
            "-std=c++23",
            "-I",
            "build/.objs/projgeom-cpp/linux/arm64/release/include/projgeom/cxx",
            "-include",
            "pch.hpp",
            "-Iinclude",
            "-DNDEBUG"
        }
    },
    files = {
        "src/myck_object.cpp",
        "build/.objs/projgeom-cpp/linux/arm64/release/include/projgeom/cxx/pch.hpp.gch"
    },
    depfiles_format = "gcc",
    depfiles = "build/.objs/projgeom-cpp/linux/arm64/release/src/myck_object.cpp.o:   src/myck_object.cpp   build/.objs/projgeom-cpp/linux/arm64/release/include/projgeom/cxx/pch.hpp   /data/data/com.termux/files/home/github/rs/projgeom-rs/cpp_ai/include/projgeom/pch.hpp   include/projgeom/myck_object.hpp include/projgeom/ck_plane.hpp   include/projgeom/pg_plane.hpp include/projgeom/pg_object.hpp   include/projgeom/concepts.hpp include/projgeom/pch.hpp\
"
}