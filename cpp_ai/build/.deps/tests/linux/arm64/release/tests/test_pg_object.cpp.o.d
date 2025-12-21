{
    depfiles_format = "gcc",
    files = {
        "tests/test_pg_object.cpp"
    },
    values = {
        "/data/data/com.termux/files/usr/bin/g++",
        {
            "-fvisibility=hidden",
            "-fvisibility-inlines-hidden",
            "-O3",
            "-std=c++23",
            "-Iinclude",
            "-isystem",
            "/data/data/com.termux/files/home/.xmake/packages/d/doctest/2.4.12/79a967e57252433389bbf311efa9ddd4/include",
            "-isystem",
            "/data/data/com.termux/files/home/.xmake/packages/d/doctest/2.4.12/79a967e57252433389bbf311efa9ddd4/include/doctest",
            "-DNDEBUG"
        }
    },
    depfiles = "build/.objs/tests/linux/arm64/release/tests/test_pg_object.cpp.o:   tests/test_pg_object.cpp include/projgeom.hpp   include/projgeom/concepts.hpp include/projgeom/pch.hpp   include/projgeom/pg_object.hpp include/projgeom/pg_plane.hpp   include/projgeom/ck_plane.hpp include/projgeom/ell_object.hpp   include/projgeom/euclid_object.hpp include/projgeom/hyp_object.hpp   include/projgeom/myck_object.hpp include/projgeom/persp_object.hpp\
"
}