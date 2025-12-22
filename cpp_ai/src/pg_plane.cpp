#include "projgeom/pg_plane.hpp"

namespace projgeom {

// Explicit template instantiations
template void check_axiom<PgPoint<int64_t>, PgLine<int64_t>>(const PgPoint<int64_t>&, const PgPoint<int64_t>&, const PgLine<int64_t>&);

template bool coincident<int64_t>(const PgPoint<int64_t>&, const PgPoint<int64_t>&, const PgPoint<int64_t>&);
template bool coincident<int64_t>(const PgLine<int64_t>&, const PgLine<int64_t>&, const PgLine<int64_t>&);

template PgPoint<int64_t> harm_conj<int64_t>(const PgPoint<int64_t>&, const PgPoint<int64_t>&, const PgPoint<int64_t>&);
template PgLine<int64_t> harm_conj<int64_t>(const PgLine<int64_t>&, const PgLine<int64_t>&, const PgLine<int64_t>&);



} // namespace projgeom