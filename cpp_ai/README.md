# Projective Geometry in C++23

A C++23 port of the projective geometry library from Rust, featuring modern C++ concepts and support for multiple build systems.

## Features

- **C++23 with Concepts**: Modern type-safe interfaces using C++20/23 concepts
- **Multiple Geometries**: Support for projective, elliptic, hyperbolic, Euclidean, and other geometries
- **Build Systems**: Support for both CMake and xmake
- **Testing**: Comprehensive doctest-based test suite
- **Header-Only Option**: Can be used as header-only library

## Project Structure

```
cpp_ai/
├── include/projgeom/     # Public headers
│   ├── concepts.hpp      # C++23 concepts
│   ├── pg_object.hpp     # Basic vector operations
│   ├── pg_plane.hpp      # Projective plane core
│   ├── ck_plane.hpp      # Cayley-Klein plane
│   ├── ell_object.hpp    # Elliptic geometry
│   ├── euclid_object.hpp # Euclidean geometry
│   ├── hyp_object.hpp    # Hyperbolic geometry
│   ├── myck_object.hpp   # MyCK geometry
│   ├── persp_object.hpp  # Perspective geometry
│   └── pch.hpp           # Precompiled header
├── src/                  # Implementation files
├── tests/                # Test suite
├── CMakeLists.txt        # CMake build system
└── xmake.lua            # xmake build system
```

## Quick Start

### Using CMake

```bash
mkdir build && cd build
cmake .. -DCMAKE_CXX_STANDARD=23
cmake --build .
ctest # Run tests
```

### Using xmake

```bash
xmake
xmake run tests # Build and run tests
```

### Header-Only Usage

```cpp
#include <projgeom.hpp>

using namespace projgeom;

PgPoint p({1, 2, 3});
PgLine l({1, 1, -1});

if (p.incident(l)) {
    // Point is on line
}
```

## Geometry Types

- `PgPoint`, `PgLine`: Basic projective geometry
- `EllipticPoint`, `EllipticLine`: Elliptic geometry
- `HyperbolicPoint`, `HyperbolicLine`: Hyperbolic geometry  
- `EuclidPoint`, `EuclidLine`: Euclidean geometry
- `MyCKPoint`, `MyCKLine`: MyCK geometry
- `PerspPoint`, `PerspLine`: Perspective geometry

## Concepts

The library uses C++23 concepts for type safety:

- `ProjectivePlanePrimitive<Point, Line>`: Basic incidence and meet operations
- `ProjectivePlane<Point, Line, Scalar>`: Projective plane with parametrization
- `CayleyKleinPlane<Point, Line, Scalar>`: Cayley-Klein plane with perpendicularity

## Examples

```cpp
#include <projgeom.hpp>

// Basic projective operations
PgPoint p1({1, 3, 2});
PgPoint p2({-2, 1, -1});
PgLine l = p1.meet(p2); // Line joining two points

// Check incidence
bool on_line = p1.incident(l);

// Euclidean geometry
EuclidPoint ep1({0, 0, 1});
EuclidPoint ep2({2, 4, 1});
EuclidPoint midpoint = ep1.midpoint(ep2);

// Triangle orthocenter
std::array triangle = {ep1, ep2, EuclidPoint({1, 3, 1})};
EuclidPoint ortho = orthocenter(triangle);
```

## Testing

The test suite uses doctest and can be run with:

```bash
# CMake
cd build && ctest

# xmake
xmake run tests
```

## Dependencies

- C++23 compiler (GCC 13+, Clang 16+, MSVC 19.30+)
- doctest (automatically fetched by CMake, or via xmake package)

## License

MIT License (same as original Rust project)