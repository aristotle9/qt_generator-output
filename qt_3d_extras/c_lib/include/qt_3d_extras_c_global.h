#ifndef QT_3D_EXTRAS_C_GLOBAL_H
#define QT_3D_EXTRAS_C_GLOBAL_H

// This header includes system headers and declares functions
// required by all regular headers of the library.

// for fixed size integer types
#include <stdint.h>

// placement new statements require this
#include <new>

// original C++ library includes generated by cpp_to_rust
#include "Qt3DExtras"

#include "qt_3d_extras_c_exports.h"

// Calls destructor of `T` class. This template function
// is necessary because it's not possible to use `x->~T()`
// syntax directly if `T` contains `::`.
template<typename T>
void qt_3d_extras_c_call_destructor(T* x) {
    x->~T();
}


#endif // QT_3D_EXTRAS_C_GLOBAL_H
