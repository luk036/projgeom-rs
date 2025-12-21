#pragma once

#include "ck_plane.hpp"

namespace projgeom {

template<SignedIntegral Scalar = int64_t>
struct EllipticPoint : PgPoint<Scalar> {
    using Base = PgPoint<Scalar>;
    using Base::Base;
    
    constexpr EllipticPoint(const PgPoint<Scalar>& p) : Base(p) {}
    
    constexpr bool is_perpendicular(const PgLine<Scalar>& l) const noexcept {
        return dot_product(this->coord, l.coord) == 0;
    }
    
    static constexpr PgLine<Scalar> line_through(
        const EllipticPoint& p, const PgLine<Scalar>& l) noexcept {
        return PgLine<Scalar>(cross_product(p.coord, l.coord));
    }
};

template<SignedIntegral Scalar = int64_t>
struct EllipticLine : PgLine<Scalar> {
    using Base = PgLine<Scalar>;
    using Base::Base;
    
    constexpr EllipticLine(const PgLine<Scalar>& l) : Base(l) {}
    
    constexpr bool is_perpendicular(const PgPoint<Scalar>& p) const noexcept {
        return dot_product(this->coord, p.coord) == 0;
    }
    
    static constexpr PgPoint<Scalar> point_on(
        const EllipticLine& l, const PgPoint<Scalar>& p) noexcept {
        return PgPoint<Scalar>(cross_product(l.coord, p.coord));
    }
};

// static_assert(CayleyKleinPlane<EllipticPoint<int64_t>, EllipticLine<int64_t>, int64_t>);
// static_assert(CayleyKleinPlane<EllipticLine<int64_t>, EllipticPoint<int64_t>, int64_t>);

} // namespace projgeom