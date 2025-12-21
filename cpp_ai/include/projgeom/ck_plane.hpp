#pragma once

#include "pg_plane.hpp"
#include "concepts.hpp"
#include <array>
#include <tuple>

namespace projgeom {

template<SignedIntegral Scalar = int64_t>
constexpr std::array<PgLine<Scalar>, 3> tri_dual(
    const std::array<PgPoint<Scalar>, 3>& triangle) noexcept {
    const auto& a = triangle[0];
    const auto& b = triangle[1];
    const auto& c = triangle[2];
    
    return {b.meet(c), c.meet(a), a.meet(b)};
}

template<SignedIntegral Scalar = int64_t>
constexpr std::array<PgPoint<Scalar>, 3> tri_dual(
    const std::array<PgLine<Scalar>, 3>& trilateral) noexcept {
    const auto& a = trilateral[0];
    const auto& b = trilateral[1];
    const auto& c = trilateral[2];
    
    return {b.meet(c), c.meet(a), a.meet(b)};
}

template<typename Point, typename Line, SignedIntegral Scalar>
requires CayleyKleinPlane<Point, Line, Scalar>
constexpr bool is_perpendicular(const Point& p, const Line& l) {
    // Default implementation for projective plane
    // Specific geometries will override this
    return p.incident(l);
}

template<typename Point, typename Line>
requires CayleyKleinPlane<Point, Line, typename Point::ScalarType>
constexpr std::array<Line, 3> tri_altitude(const std::array<Point, 3>& triangle) {
    const auto& a = triangle[0];
    const auto& b = triangle[1];
    const auto& c = triangle[2];
    
    const auto trilateral = tri_dual(triangle);
    const auto& l = trilateral[0];
    const auto& m = trilateral[1];
    const auto& n = trilateral[2];
    
    // Altitudes are lines through vertices perpendicular to opposite sides
    return {
        Point::line_through(a, l),  // altitude from A perpendicular to BC
        Point::line_through(b, m),  // altitude from B perpendicular to CA
        Point::line_through(c, n)   // altitude from C perpendicular to AB
    };
}

template<typename Point, typename Line>
requires CayleyKleinPlane<Point, Line, typename Point::ScalarType>
constexpr Point orthocenter(const std::array<Point, 3>& triangle) {
    const auto altitudes = tri_altitude<Point, Line>(triangle);
    return altitudes[1].meet(altitudes[2]);
}



// static_assert(CayleyKleinPlane<EllipticPoint<int64_t>, EllipticLine<int64_t>, int64_t>);
// static_assert(CayleyKleinPlane<EllipticLine<int64_t>, EllipticPoint<int64_t>, int64_t>);

} // namespace projgeom