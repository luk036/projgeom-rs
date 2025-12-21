{
    depfiles_format = "gcc",
    files = {
        "src/pg_object.cpp",
        "build/.objs/projgeom-cpp/linux/arm64/release/include/projgeom/cxx/pch.hpp.gch"
    },
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
    depfiles = "build/.objs/projgeom-cpp/linux/arm64/release/src/pg_object.cpp.o:   src/pg_object.cpp   build/.objs/projgeom-cpp/linux/arm64/release/include/projgeom/cxx/pch.hpp   /data/data/com.termux/files/home/github/rs/projgeom-rs/cpp_ai/include/projgeom/pch.hpp   include/projgeom/pg_object.hpp include/projgeom/concepts.hpp   include/projgeom/pch.hpp\
"
}