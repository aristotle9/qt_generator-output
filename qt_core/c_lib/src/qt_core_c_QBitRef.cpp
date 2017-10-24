#include "qt_core_c_QBitRef.h"

bool qt_core_c_QBitRef_convert_to_bool(const QBitRef* this_ptr) {
  return this_ptr->operator bool();
}

void qt_core_c_QBitRef_destructor(QBitRef* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

QBitRef* qt_core_c_QBitRef_operator_assign_QBitRef(QBitRef* this_ptr, const QBitRef* val) {
  return &this_ptr->operator=(*val);
}

QBitRef* qt_core_c_QBitRef_operator_assign_bool(QBitRef* this_ptr, bool val) {
  return &this_ptr->operator=(val);
}

bool qt_core_c_QBitRef_operator_not(const QBitRef* this_ptr) {
  return this_ptr->operator!();
}

