#include "projgeom/pg_plane.hpp"

namespace projgeom {

// Explicit template instantiations
template void check_axiom<PgPoint<int64_t>, PgLine<int64_t>>(const PgPoint<int64_t>&, const PgPoint<int64_t>&, const PgLine<int64_t>&);

template bool coincident<int64_t>(const PgPoint<int64_t>&, const PgPoint<int64_t>&, const PgPoint<int64_t>&);
template bool coincident<int64_t>(const PgLine<int64_t>&, const PgLine<int64_t>&, const PgLine<int64_t>&);

template PgPoint<int64_t> harm_conj<int64_t>(const PgPoint<int64_t>&, const PgPoint<int64_t>&, const PgPoint<int64_t>&);
template PgLine<int64_t> harm_conj<int64_t>(const PgLine<int64_t>&, const PgLine<int64_t>&, const PgLine<int64_t>&);

// Explicit member function instantiations
template bool PgPoint<int64_t>::incident(const PgLine<int64_t>&) const;
template PgLine<int64_t> PgPoint<int64_t>::meet(const PgPoint<int64_t>&) const;
template PgPoint<int64_t> PgPoint<int64_t>::parametrize(int64_t, const PgPoint<int64_t>&, int64_t) const;

template bool PgLine<int64_t>::incident(const PgPoint<int64_t>&) const;
template PgPoint<int64_t> PgLine<int64_t>::meet(const PgLine<int64_t>&) const;
template PgLine<int64_t> PgLine<int64_t>::parametrize(int64_t, const PgLine<int64_t>&, int64_t) const;

} // namespace projgeom