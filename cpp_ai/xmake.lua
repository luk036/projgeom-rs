add_rules("mode.debug", "mode.release")

set_project("projgeom-cpp")
set_version("0.1.3")

set_languages("c++23")

add_requires("doctest")

target("projgeom-cpp")
    set_kind("$(kind)")
    add_headerfiles("include/(projgeom/**.hpp)")
    add_files("src/**.cpp")
    add_includedirs("include", {public = true})
    set_pcxxheader("include/projgeom/pch.hpp")
    
    if is_plat("windows") then
        add_cxxflags("/execution-charset:utf-8", "/source-charset:utf-8")
    end

target("tests")
    set_kind("binary")
    add_deps("projgeom-cpp")
    add_files("tests/**.cpp")
    add_packages("doctest")
    add_includedirs("include")
    
    after_build(function (target)
        os.exec(target:targetfile())
    end)