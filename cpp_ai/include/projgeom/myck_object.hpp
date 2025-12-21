#pragma once

#include "ck_plane.hpp"

namespace projgeom {

template<SignedIntegral Scalar = int64_t>
constexpr std::array<Scalar, 3> MYCK_POINT_PERP_COEFFS = {-2, 1, -2};
template<SignedIntegral Scalar = int64_t>
constexpr std::array<Scalar, 3> MYCK_LINE_PERP_COEFFS = {-1, 2, -1};

template<SignedIntegral Scalar = int64_t>
struct MyCKPoint : PgPoint<Scalar> {
    using Base = PgPoint<Scalar>;
    using Base::Base;
    
    constexpr MyCKPoint(const PgPoint<Scalar>& p) : Base(p) {}
    
    constexpr bool is_perpendicular(const PgLine<Scalar>& l) const noexcept {
        const auto weighted_coord = std::array{
            MYCK_POINT_PERP_COEFFS<Scalar>[0] * this->coord[0],
            MYCK_POINT_PERP_COEFFS<Scalar>[1] * this->coord[1],
            MYCK_POINT_PERP_COEFFS<Scalar>[2] * this->coord[2]
        };
        return dot_product(weighted_coord, l.coord) == 0;
    }
    
    static constexpr PgLine<Scalar> line_through(
        const MyCKPoint& p, const PgLine<Scalar>& l) noexcept {
        const auto weighted_coord = std::array{
            MYCK_POINT_PERP_COEFFS<Scalar>[0] * p.coord[0],
            MYCK_POINT_PERP_COEFFS<Scalar>[1] * p.coord[1],
            MYCK_POINT_PERP_COEFFS<Scalar>[2] * p.coord[2]
        };
        return PgLine<Scalar>(cross_product(weighted_coord, l.coord));
    }
};

template<SignedIntegral Scalar = int64_t>
struct MyCKLine : PgLine<Scalar> {
    using Base = PgLine<Scalar>;
    using Base::Base;
    
    constexpr MyCKLine(const PgLine<Scalar>& l) : Base(l) {}
    
    constexpr bool is_perpendicular(const PgPoint<Scalar>& p) const noexcept {
        const auto weighted_coord = std::array{
            MYCK_LINE_PERP_COEFFS<Scalar>[0] * this->coord[0],
            MYCK_LINE_PERP_COEFFS<Scalar>[1] * this->coord[1],
            MYCK_LINE_PERP_COEFFS<Scalar>[2] * this->coord[2]
        };
        return dot_product(weighted_coord, p.coord) == 0;
    }
    
    static constexpr PgPoint<Scalar> point_on(
        const MyCKLine& l, const PgPoint<Scalar>& p) noexcept {
        const auto weighted_coord = std::array{
            MYCK_LINE_PERP_COEFFS<Scalar>[0] * l.coord[0],
            MYCK_LINE_PERP_COEFFS<Scalar>[1] * l.coord[1],
            MYCK_LINE_PERP_COEFFS<Scalar>[2] * l.coord[2]
        };
        return PgPoint<Scalar>(cross_product(weighted_coord, p.coord));
    }
};

// static_assert(CayleyKleinPlane<MyCKPoint<int64_t>, MyCKLine<int64_t>, int64_t>);
// static_assert(CayleyKleinPlane<MyCKLine<int64_t>, MyCKPoint<int64_t>, int64_t>);

} // namespace projgeom