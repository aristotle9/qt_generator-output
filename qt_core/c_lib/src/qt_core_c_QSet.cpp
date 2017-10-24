#include "qt_core_c_QSet.h"

int qt_core_c_QSet_QAbstractState_ptr_capacity(const QSet< QAbstractState* >* this_ptr) {
  return this_ptr->capacity();
}

void qt_core_c_QSet_QAbstractState_ptr_clear(QSet< QAbstractState* >* this_ptr) {
  this_ptr->clear();
}

void qt_core_c_QSet_QAbstractState_ptr_constructor(QSet< QAbstractState* >* output) {
  new(output) QSet< QAbstractState* >();
}

bool qt_core_c_QSet_QAbstractState_ptr_contains_set(const QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* set) {
  return this_ptr->contains(*set);
}

bool qt_core_c_QSet_QAbstractState_ptr_contains_value(const QSet< QAbstractState* >* this_ptr, QAbstractState* const * value) {
  return this_ptr->contains(*value);
}

int qt_core_c_QSet_QAbstractState_ptr_count(const QSet< QAbstractState* >* this_ptr) {
  return this_ptr->count();
}

void qt_core_c_QSet_QAbstractState_ptr_destructor(QSet< QAbstractState* >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QSet_QAbstractState_ptr_empty(const QSet< QAbstractState* >* this_ptr) {
  return this_ptr->empty();
}

void qt_core_c_QSet_QAbstractState_ptr_fromList_to_output(const QList< QAbstractState* >* list, QSet< QAbstractState* >* output) {
  new(output) QSet< QAbstractState* >(QSet< QAbstractState* >::fromList(*list));
}

QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_intersect(QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other) {
  return &this_ptr->intersect(*other);
}

bool qt_core_c_QSet_QAbstractState_ptr_intersects(const QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other) {
  return this_ptr->intersects(*other);
}

bool qt_core_c_QSet_QAbstractState_ptr_isEmpty(const QSet< QAbstractState* >* this_ptr) {
  return this_ptr->isEmpty();
}

QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_operator_add_assign_other(QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other) {
  return &this_ptr->operator+=(*other);
}

QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_operator_add_assign_value(QSet< QAbstractState* >* this_ptr, QAbstractState* const * value) {
  return &this_ptr->operator+=(*value);
}

void qt_core_c_QSet_QAbstractState_ptr_operator_add_to_output(const QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other, QSet< QAbstractState* >* output) {
  new(output) QSet< QAbstractState* >(this_ptr->operator+(*other));
}

QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_operator_bit_and_assign_other(QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other) {
  return &this_ptr->operator&=(*other);
}

QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_operator_bit_and_assign_value(QSet< QAbstractState* >* this_ptr, QAbstractState* const * value) {
  return &this_ptr->operator&=(*value);
}

void qt_core_c_QSet_QAbstractState_ptr_operator_bit_and_to_output(const QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other, QSet< QAbstractState* >* output) {
  new(output) QSet< QAbstractState* >(this_ptr->operator&(*other));
}

QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_operator_bit_or_assign_other(QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other) {
  return &this_ptr->operator|=(*other);
}

QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_operator_bit_or_assign_value(QSet< QAbstractState* >* this_ptr, QAbstractState* const * value) {
  return &this_ptr->operator|=(*value);
}

void qt_core_c_QSet_QAbstractState_ptr_operator_bit_or_to_output(const QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other, QSet< QAbstractState* >* output) {
  new(output) QSet< QAbstractState* >(this_ptr->operator|(*other));
}

bool qt_core_c_QSet_QAbstractState_ptr_operator_eq(const QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QSet_QAbstractState_ptr_operator_neq(const QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other) {
  return this_ptr->operator!=(*other);
}

QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_operator_shl(QSet< QAbstractState* >* this_ptr, QAbstractState* const * value) {
  return &this_ptr->operator<<(*value);
}

QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_operator_sub_assign_other(QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other) {
  return &this_ptr->operator-=(*other);
}

QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_operator_sub_assign_value(QSet< QAbstractState* >* this_ptr, QAbstractState* const * value) {
  return &this_ptr->operator-=(*value);
}

void qt_core_c_QSet_QAbstractState_ptr_operator_sub_to_output(const QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other, QSet< QAbstractState* >* output) {
  new(output) QSet< QAbstractState* >(this_ptr->operator-(*other));
}

bool qt_core_c_QSet_QAbstractState_ptr_remove(QSet< QAbstractState* >* this_ptr, QAbstractState* const * value) {
  return this_ptr->remove(*value);
}

void qt_core_c_QSet_QAbstractState_ptr_reserve(QSet< QAbstractState* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QSet_QAbstractState_ptr_size(const QSet< QAbstractState* >* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QSet_QAbstractState_ptr_squeeze(QSet< QAbstractState* >* this_ptr) {
  this_ptr->squeeze();
}

QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_subtract(QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other) {
  return &this_ptr->subtract(*other);
}

void qt_core_c_QSet_QAbstractState_ptr_swap(QSet< QAbstractState* >* this_ptr, QSet< QAbstractState* >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QSet_QAbstractState_ptr_toList_to_output(const QSet< QAbstractState* >* this_ptr, QList< QAbstractState* >* output) {
  new(output) QList< QAbstractState* >(this_ptr->toList());
}

QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_unite(QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other) {
  return &this_ptr->unite(*other);
}

void qt_core_c_QSet_QAbstractState_ptr_values_to_output(const QSet< QAbstractState* >* this_ptr, QList< QAbstractState* >* output) {
  new(output) QList< QAbstractState* >(this_ptr->values());
}

