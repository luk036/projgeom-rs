#include "projgeom/myck_object.hpp"

namespace projgeom {

// Explicit template instantiations
template struct MyCKPoint<int64_t>;
template struct MyCKLine<int64_t>;

} // namespace projgeom