{
    depfiles = "build/.objs/projgeom-cpp/linux/arm64/release/include/projgeom/cxx/pch.hpp.gch:   include/projgeom/pch.hpp\
",
    depfiles_format = "gcc",
    files = {
        "include/projgeom/pch.hpp"
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
    }
}