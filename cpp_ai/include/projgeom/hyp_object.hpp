#pragma once

#include "ck_plane.hpp"

namespace projgeom {

template<SignedIntegral Scalar = int64_t>
constexpr std::array<Scalar, 3> HYP_PERP_COEFFS = {1, 1, -1};

template<SignedIntegral Scalar = int64_t>
struct HyperbolicPoint : PgPoint<Scalar> {
    using Base = PgPoint<Scalar>;
    using Base::Base;
    
    constexpr HyperbolicPoint(const PgPoint<Scalar>& p) : Base(p) {}
    
    constexpr bool is_perpendicular(const PgLine<Scalar>& l) const noexcept {
        const auto weighted_coord = std::array{
            HYP_PERP_COEFFS<Scalar>[0] * this->coord[0],
            HYP_PERP_COEFFS<Scalar>[1] * this->coord[1],
            HYP_PERP_COEFFS<Scalar>[2] * this->coord[2]
        };
        return dot_product(weighted_coord, l.coord) == 0;
    }
    
    static constexpr PgLine<Scalar> line_through(
        const HyperbolicPoint& p, const PgLine<Scalar>& l) noexcept {
        const auto weighted_coord = std::array{
            HYP_PERP_COEFFS<Scalar>[0] * p.coord[0],
            HYP_PERP_COEFFS<Scalar>[1] * p.coord[1],
            HYP_PERP_COEFFS<Scalar>[2] * p.coord[2]
        };
        return PgLine<Scalar>(cross_product(weighted_coord, l.coord));
    }
};

template<SignedIntegral Scalar = int64_t>
struct HyperbolicLine : PgLine<Scalar> {
    using Base = PgLine<Scalar>;
    using Base::Base;
    
    constexpr HyperbolicLine(const PgLine<Scalar>& l) : Base(l) {}
    
    constexpr bool is_perpendicular(const PgPoint<Scalar>& p) const noexcept {
        const auto weighted_coord = std::array{
            HYP_PERP_COEFFS<Scalar>[0] * this->coord[0],
            HYP_PERP_COEFFS<Scalar>[1] * this->coord[1],
            HYP_PERP_COEFFS<Scalar>[2] * this->coord[2]
        };
        return dot_product(weighted_coord, p.coord) == 0;
    }
    
    static constexpr PgPoint<Scalar> point_on(
        const HyperbolicLine& l, const PgPoint<Scalar>& p) noexcept {
        const auto weighted_coord = std::array{
            HYP_PERP_COEFFS<Scalar>[0] * l.coord[0],
            HYP_PERP_COEFFS<Scalar>[1] * l.coord[1],
            HYP_PERP_COEFFS<Scalar>[2] * l.coord[2]
        };
        return PgPoint<Scalar>(cross_product(weighted_coord, p.coord));
    }
};

// static_assert(CayleyKleinPlane<HyperbolicPoint<int64_t>, HyperbolicLine<int64_t>, int64_t>);
// static_assert(CayleyKleinPlane<HyperbolicLine<int64_t>, HyperbolicPoint<int64_t>, int64_t>);

} // namespace projgeom