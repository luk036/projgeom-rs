#pragma once

#include "pg_plane.hpp"
#include "concepts.hpp"
#include <array>
#include <tuple>

namespace projgeom {

template<SignedIntegral Scalar = int64_t>
constexpr std::array<PgLine<Scalar>, 3> tri_dual(
    const std::array<PgPoint<Scalar>, 3>& triangle) noexcept {
    const auto& point_a = triangle[0];
    const auto& point_b = triangle[1];
    const auto& point_c = triangle[2];
    
    return {point_b.meet(point_c), point_c.meet(point_a), point_a.meet(point_b)};
}

template<SignedIntegral Scalar = int64_t>
constexpr std::array<PgPoint<Scalar>, 3> tri_dual(
    const std::array<PgLine<Scalar>, 3>& trilateral) noexcept {
    const auto& line_a = trilateral[0];
    const auto& line_b = trilateral[1];
    const auto& line_c = trilateral[2];
    
    return {line_b.meet(line_c), line_c.meet(line_a), line_a.meet(line_b)};
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
    const auto& point_a = triangle[0];
    const auto& point_b = triangle[1];
    const auto& point_c = triangle[2];
    
    const auto trilateral = tri_dual(triangle);
    const auto& line_l = trilateral[0];
    const auto& line_m = trilateral[1];
    const auto& line_n = trilateral[2];
    
    // Altitudes are lines through vertices perpendicular to opposite sides
    return {
        Point::line_through(point_a, line_l),  // altitude from A perpendicular to BC
        Point::line_through(point_b, line_m),  // altitude from B perpendicular to CA
        Point::line_through(point_c, line_n)   // altitude from C perpendicular to AB
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