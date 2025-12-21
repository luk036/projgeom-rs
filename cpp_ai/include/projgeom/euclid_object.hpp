#pragma once

#include "ck_plane.hpp"

namespace projgeom {

template<SignedIntegral Scalar = int64_t>
constexpr PgLine<Scalar> EUCLID_L_INF = PgLine<Scalar>({0, 0, 1});

template<SignedIntegral Scalar = int64_t>
struct EuclidPoint : PgPoint<Scalar> {
    using Base = PgPoint<Scalar>;
    using Base::Base;
    
    constexpr EuclidPoint(const PgPoint<Scalar>& p) : Base(p) {}
    
    constexpr bool is_perpendicular(const PgLine<Scalar>& l) const noexcept {
        // In Euclidean geometry, perpendicular to a line means parallel to L_INF
        return l == EUCLID_L_INF<Scalar>;
    }
    
    static constexpr PgLine<Scalar> line_through(
        const EuclidPoint& p, const PgLine<Scalar>& l) noexcept {
        // Line through p parallel to l (perpendicular in Euclidean sense)
        return EUCLID_L_INF<Scalar>;
    }
    
    constexpr EuclidPoint midpoint(const EuclidPoint& other) const noexcept {
        return EuclidPoint(this->parametrize(other.coord[2], other, this->coord[2]));
    }
};

template<SignedIntegral Scalar = int64_t>
struct EuclidLine : PgLine<Scalar> {
    using Base = PgLine<Scalar>;
    using Base::Base;
    
    constexpr EuclidLine(const PgLine<Scalar>& l) : Base(l) {}
    
    constexpr bool is_perpendicular(const PgPoint<Scalar>& p) const noexcept {
        // In Euclidean geometry, perpendicular to a point means the point is at infinity
        return p.coord[2] == 0;
    }
    
    static constexpr PgPoint<Scalar> point_on(
        const EuclidLine& l, const PgPoint<Scalar>& p) noexcept {
        // Point on l with direction of p (parallel point)
        return PgPoint<Scalar>({l.coord[0], l.coord[1], 0});
    }
    
    constexpr bool is_parallel(const EuclidLine& other) const noexcept {
        return this->coord[0] * other.coord[1] == this->coord[1] * other.coord[0];
    }
    
    constexpr bool is_perpendicular(const EuclidLine& other) const noexcept {
        return dot1(std::array{this->coord[0], this->coord[1]},
                   std::array{other.coord[0], other.coord[1]}) == 0;
    }
    
    constexpr EuclidLine perp() const noexcept {
        // Perpendicular line (rotate normal vector by 90 degrees)
        return EuclidLine(std::array{-this->coord[1], this->coord[0], this->coord[2]});
    }
    
    /*
    constexpr EuclidLine altitude(const EuclidPoint<Scalar>& pt_a) const noexcept {
        return this->perp().meet(pt_a); // FIXME: This has type errors
    }
    */
};

/*
template<SignedIntegral Scalar>
constexpr std::array<EuclidLine<Scalar>, 3> tri_altitude(
    const std::array<EuclidPoint<Scalar>, 3>& triangle) noexcept {
    const auto& a_1 = triangle[0];
    const auto& a_2 = triangle[1];
    const auto& a_3 = triangle[2];
    
    const auto trilateral = tri_dual(triangle);
    const auto& l_1 = trilateral[0];
    const auto& l_2 = trilateral[1];
    const auto& l_3 = trilateral[2];
    
    return {
        l_1.altitude(a_1),
        l_2.altitude(a_2),
        l_3.altitude(a_3)
    };
}
*/

/*
template<SignedIntegral Scalar>
constexpr EuclidPoint<Scalar> orthocenter(
    const std::array<EuclidPoint<Scalar>, 3>& triangle) noexcept {
    const auto& a_1 = triangle[0];
    const auto& a_2 = triangle[1];
    const auto& a_3 = triangle[2];
    
    const auto t_1 = a_2.meet(a_3).altitude(a_1);
    const auto t_2 = a_3.meet(a_1).altitude(a_2);
    return t_1.meet(t_2);
}
*/

// static_assert(CayleyKleinPlane<EuclidPoint<int64_t>, EuclidLine<int64_t>, int64_t>);
// static_assert(CayleyKleinPlane<EuclidLine<int64_t>, EuclidPoint<int64_t>, int64_t>);

} // namespace projgeom