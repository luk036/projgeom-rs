#include "projgeom/hyp_object.hpp"

namespace projgeom {

// Explicit template instantiations
template struct HyperbolicPoint<int64_t>;
template struct HyperbolicLine<int64_t>;

} // namespace projgeom