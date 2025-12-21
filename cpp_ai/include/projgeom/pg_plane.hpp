#pragma once

#include "pg_object.hpp"
#include "concepts.hpp"
#include <array>
#include <concepts>
#include <cassert>

namespace projgeom {

template<typename Point, typename Line>
requires ProjectivePlanePrimitive<Point, Line>
constexpr void check_axiom(const Point& pt_p, const Point& pt_q, const Line& ln_l) {
    auto ln_m = pt_p.meet(pt_q);
    assert(ln_m == pt_q.meet(pt_p));
    assert(ln_m.incident(pt_p));
    assert(ln_m.incident(pt_q));
    
    auto pt_r = ln_l.meet(ln_m);
    assert(pt_r.incident(ln_l));
    assert(pt_r.incident(ln_m));
}

template<SignedIntegral Scalar = int64_t>
constexpr bool coincident(
    const PgPoint<Scalar>& pt_p,
    const PgPoint<Scalar>& pt_q,
    const PgPoint<Scalar>& pt_r) noexcept {
    const auto& coord_a = pt_p.coord;
    const auto& coord_b = pt_q.coord;
    const auto& coord_c = pt_r.coord;
    
    const auto cross_ab = cross_product(coord_a, coord_b);
    return dot_product(cross_ab, coord_c) == 0;
}

template<SignedIntegral Scalar = int64_t>
constexpr bool coincident(
    const PgLine<Scalar>& ln_l,
    const PgLine<Scalar>& ln_m,
    const PgLine<Scalar>& ln_n) noexcept {
    const auto& coord_a = ln_l.coord;
    const auto& coord_b = ln_m.coord;
    const auto& coord_c = ln_n.coord;
    
    const auto cross_ab = cross_product(coord_a, coord_b);
    return dot_product(cross_ab, coord_c) == 0;
}

template<SignedIntegral Scalar = int64_t>
constexpr PgPoint<Scalar> harm_conj(
    const PgPoint<Scalar>& pt_p,
    const PgPoint<Scalar>& pt_q,
    const PgPoint<Scalar>& pt_r) noexcept {
    const auto& coord_a = pt_p.coord;
    const auto& coord_b = pt_q.coord;
    const auto& coord_c = pt_r.coord;
    
    const auto dot_ab = dot_product(coord_a, coord_b);
    const auto dot_ac = dot_product(coord_a, coord_c);
    const auto dot_bc = dot_product(coord_b, coord_c);
    
    const Scalar lambda = 2 * dot_bc * dot_ab - dot_ac * dot_ab;
    const Scalar mu = 2 * dot_ac * dot_ab - dot_bc * dot_ab;
    
    return PgPoint<Scalar>(plucker_operation(lambda, coord_a, mu, coord_b));
}

template<SignedIntegral Scalar = int64_t>
constexpr PgLine<Scalar> harm_conj(
    const PgLine<Scalar>& ln_l,
    const PgLine<Scalar>& ln_m,
    const PgLine<Scalar>& ln_n) noexcept {
    const auto& coord_a = ln_l.coord;
    const auto& coord_b = ln_m.coord;
    const auto& coord_c = ln_n.coord;
    
    const auto dot_ab = dot_product(coord_a, coord_b);
    const auto dot_ac = dot_product(coord_a, coord_c);
    const auto dot_bc = dot_product(coord_b, coord_c);
    
    const Scalar lambda = 2 * dot_bc * dot_ab - dot_ac * dot_ab;
    const Scalar mu = 2 * dot_ac * dot_ab - dot_bc * dot_ab;
    
    return PgLine<Scalar>(plucker_operation(lambda, coord_a, mu, coord_b));
}

template<SignedIntegral Scalar>
constexpr bool PgPoint<Scalar>::incident(const PgLine<Scalar>& line) const noexcept {
    return dot_product(coord, line.coord) == 0;
}

template<SignedIntegral Scalar>
constexpr PgLine<Scalar> PgPoint<Scalar>::meet(const PgPoint<Scalar>& other) const noexcept {
    return PgLine<Scalar>(cross_product(coord, other.coord));
}

template<SignedIntegral Scalar>
constexpr PgPoint<Scalar> PgPoint<Scalar>::parametrize(
    Scalar lambda, const PgPoint<Scalar>& other, Scalar mu) const noexcept {
    return PgPoint<Scalar>(plucker_operation(lambda, coord, mu, other.coord));
}

template<SignedIntegral Scalar>
constexpr bool PgLine<Scalar>::incident(const PgPoint<Scalar>& point) const noexcept {
    return dot_product(coord, point.coord) == 0;
}

template<SignedIntegral Scalar>
constexpr PgPoint<Scalar> PgLine<Scalar>::meet(const PgLine<Scalar>& other) const noexcept {
    return PgPoint<Scalar>(cross_product(coord, other.coord));
}

template<SignedIntegral Scalar>
constexpr PgLine<Scalar> PgLine<Scalar>::parametrize(
    Scalar lambda, const PgLine<Scalar>& other, Scalar mu) const noexcept {
    return PgLine<Scalar>(plucker_operation(lambda, coord, mu, other.coord));
}

static_assert(ProjectivePlanePrimitive<PgPoint<int64_t>, PgLine<int64_t>>);
static_assert(ProjectivePlanePrimitive<PgLine<int64_t>, PgPoint<int64_t>>);

} // namespace projgeom