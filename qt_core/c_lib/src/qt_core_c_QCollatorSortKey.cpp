#include "qt_core_c_QCollatorSortKey.h"

int qt_core_c_QCollatorSortKey_compare(const QCollatorSortKey* this_ptr, const QCollatorSortKey* key) {
  return this_ptr->compare(*key);
}

void qt_core_c_QCollatorSortKey_constructor(const QCollatorSortKey* other, QCollatorSortKey* output) {
  new(output) QCollatorSortKey(*other);
}

void qt_core_c_QCollatorSortKey_destructor(QCollatorSortKey* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

QCollatorSortKey* qt_core_c_QCollatorSortKey_operator_assign(QCollatorSortKey* this_ptr, const QCollatorSortKey* other) {
  return &this_ptr->operator=(*other);
}

void qt_core_c_QCollatorSortKey_swap(QCollatorSortKey* this_ptr, QCollatorSortKey* other) {
  this_ptr->swap(*other);
}

