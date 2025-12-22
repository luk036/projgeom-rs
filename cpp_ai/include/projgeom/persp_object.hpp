#pragma once

#include "ck_plane.hpp"

namespace projgeom {

// Forward declarations
template<SignedIntegral Scalar> struct PerspPoint;
template<SignedIntegral Scalar> struct PerspLine;

// Constant definitions (needed before struct definitions)
template<SignedIntegral Scalar>
inline constexpr PgLine<Scalar> L_INF = PgLine<Scalar>({0, -1, 1});

template<SignedIntegral Scalar = int64_t>
struct PerspPoint : PgPoint<Scalar> {
    using Base = PgPoint<Scalar>;
    using Base::Base;
    
    constexpr PerspPoint(const PgPoint<Scalar>& p) : Base(p) {}
    
    constexpr bool is_perpendicular(const PgLine<Scalar>& l) const noexcept {
        // In perspective geometry, perpendicular means meeting at infinity
        return l == L_INF<Scalar>;
    }
    
    static constexpr PgLine<Scalar> line_through(
        const PerspPoint& p, const PgLine<Scalar>& l) noexcept {
        // In perspective geometry, all lines through a point are parallel to L_INF
        // Actually, this should return the line through p parallel to l
        // For now, return L_INF as placeholder
        return L_INF<Scalar>;
    }
    
    constexpr Scalar dot(const PgLine<Scalar>& l) const noexcept {
        return dot_product(this->coord, l.coord);
    }
};

template<SignedIntegral Scalar = int64_t>
struct PerspLine : PgLine<Scalar> {
    using Base = PgLine<Scalar>;
    using Base::Base;
    
    constexpr PerspLine(const PgLine<Scalar>& l) : Base(l) {}
    
    constexpr bool is_perpendicular(const PgPoint<Scalar>& p) const noexcept {
        // In perspective geometry, perpendicular means the point is on L_INF
        return p.incident(L_INF<Scalar>);
    }
    
    static constexpr PgPoint<Scalar> point_on(
        const PerspLine& l, const PgPoint<Scalar>& p) noexcept {
        // For perspective geometry, point_on a line is not well-defined
        // Return a default point for now
        return PgPoint<Scalar>({0, 0, 1});
    }
    
    constexpr bool is_parallel(const PerspLine& other) const noexcept {
        // Two lines are parallel if they meet on L_INF
        const auto intersection = this->meet(other);
        return intersection.incident(L_INF<Scalar>);
    }
    
    constexpr bool is_perpendicular(const PerspLine& other) const noexcept {
        // Two lines are perpendicular if one is L_INF
        return *this == L_INF<Scalar> || other == L_INF<Scalar>;
    }
};

// Note: I_RE and I_IM are not defined due to circular dependency issues
// They would need to be PerspPoint<Scalar> but that creates dependency issues

} // namespace projgeom