#include "qt_core_c_QGenericReturnArgument.h"

QGenericArgument* qt_core_c_QGenericReturnArgument_G_static_cast_QGenericArgument_ptr(QGenericReturnArgument* ptr) {
  return static_cast<QGenericArgument*>(ptr);
}

QGenericReturnArgument* qt_core_c_QGenericReturnArgument_G_static_cast_QGenericReturnArgument_ptr(QGenericArgument* ptr) {
  return static_cast<QGenericReturnArgument*>(ptr);
}

void qt_core_c_QGenericReturnArgument_constructor_aName(const char* aName, QGenericReturnArgument* output) {
  new(output) QGenericReturnArgument(aName);
}

void qt_core_c_QGenericReturnArgument_constructor_aName_aData(const char* aName, void* aData, QGenericReturnArgument* output) {
  new(output) QGenericReturnArgument(aName, aData);
}

void qt_core_c_QGenericReturnArgument_constructor_no_args(QGenericReturnArgument* output) {
  new(output) QGenericReturnArgument();
}

void qt_core_c_QGenericReturnArgument_destructor(QGenericReturnArgument* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

