#include "qt_core_c_QGenericArgument.h"

void qt_core_c_QGenericArgument_constructor_aName(const char* aName, QGenericArgument* output) {
  new(output) QGenericArgument(aName);
}

void qt_core_c_QGenericArgument_constructor_aName_aData(const char* aName, const void* aData, QGenericArgument* output) {
  new(output) QGenericArgument(aName, aData);
}

void qt_core_c_QGenericArgument_constructor_no_args(QGenericArgument* output) {
  new(output) QGenericArgument();
}

void* qt_core_c_QGenericArgument_data(const QGenericArgument* this_ptr) {
  return this_ptr->data();
}

void qt_core_c_QGenericArgument_destructor(QGenericArgument* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

const char* qt_core_c_QGenericArgument_name(const QGenericArgument* this_ptr) {
  return this_ptr->name();
}

