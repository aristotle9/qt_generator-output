#include "qt_core_c_QAssociativeIterable.h"

void qt_core_c_QAssociativeIterable_begin_to_output(const QAssociativeIterable* this_ptr, QAssociativeIterable::const_iterator* output) {
  new(output) QAssociativeIterable::const_iterator(this_ptr->begin());
}

void qt_core_c_QAssociativeIterable_const_iterator_constructor(const QAssociativeIterable::const_iterator* other, QAssociativeIterable::const_iterator* output) {
  new(output) QAssociativeIterable::const_iterator(*other);
}

void qt_core_c_QAssociativeIterable_const_iterator_destructor(QAssociativeIterable::const_iterator* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QAssociativeIterable_const_iterator_key_to_output(const QAssociativeIterable::const_iterator* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->key());
}

QAssociativeIterable::const_iterator* qt_core_c_QAssociativeIterable_const_iterator_operator_add_assign(QAssociativeIterable::const_iterator* this_ptr, int j) {
  return &this_ptr->operator+=(j);
}

void qt_core_c_QAssociativeIterable_const_iterator_operator_add_to_output(const QAssociativeIterable::const_iterator* this_ptr, int j, QAssociativeIterable::const_iterator* output) {
  new(output) QAssociativeIterable::const_iterator(this_ptr->operator+(j));
}

QAssociativeIterable::const_iterator* qt_core_c_QAssociativeIterable_const_iterator_operator_assign(QAssociativeIterable::const_iterator* this_ptr, const QAssociativeIterable::const_iterator* other) {
  return &this_ptr->operator=(*other);
}

QAssociativeIterable::const_iterator* qt_core_c_QAssociativeIterable_const_iterator_operator_dec(QAssociativeIterable::const_iterator* this_ptr) {
  return &this_ptr->operator--();
}

void qt_core_c_QAssociativeIterable_const_iterator_operator_dec_postfix_to_output(QAssociativeIterable::const_iterator* this_ptr, int arg1, QAssociativeIterable::const_iterator* output) {
  new(output) QAssociativeIterable::const_iterator(this_ptr->operator--(arg1));
}

bool qt_core_c_QAssociativeIterable_const_iterator_operator_eq(const QAssociativeIterable::const_iterator* this_ptr, const QAssociativeIterable::const_iterator* o) {
  return this_ptr->operator==(*o);
}

QAssociativeIterable::const_iterator* qt_core_c_QAssociativeIterable_const_iterator_operator_inc(QAssociativeIterable::const_iterator* this_ptr) {
  return &this_ptr->operator++();
}

void qt_core_c_QAssociativeIterable_const_iterator_operator_inc_postfix_to_output(QAssociativeIterable::const_iterator* this_ptr, int arg1, QAssociativeIterable::const_iterator* output) {
  new(output) QAssociativeIterable::const_iterator(this_ptr->operator++(arg1));
}

void qt_core_c_QAssociativeIterable_const_iterator_operator_indirection_to_output(const QAssociativeIterable::const_iterator* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator*());
}

bool qt_core_c_QAssociativeIterable_const_iterator_operator_neq(const QAssociativeIterable::const_iterator* this_ptr, const QAssociativeIterable::const_iterator* o) {
  return this_ptr->operator!=(*o);
}

QAssociativeIterable::const_iterator* qt_core_c_QAssociativeIterable_const_iterator_operator_sub_assign(QAssociativeIterable::const_iterator* this_ptr, int j) {
  return &this_ptr->operator-=(j);
}

void qt_core_c_QAssociativeIterable_const_iterator_operator_sub_to_output(const QAssociativeIterable::const_iterator* this_ptr, int j, QAssociativeIterable::const_iterator* output) {
  new(output) QAssociativeIterable::const_iterator(this_ptr->operator-(j));
}

void qt_core_c_QAssociativeIterable_const_iterator_value_to_output(const QAssociativeIterable::const_iterator* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->value());
}

void qt_core_c_QAssociativeIterable_destructor(QAssociativeIterable* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QAssociativeIterable_end_to_output(const QAssociativeIterable* this_ptr, QAssociativeIterable::const_iterator* output) {
  new(output) QAssociativeIterable::const_iterator(this_ptr->end());
}

void qt_core_c_QAssociativeIterable_find_to_output(const QAssociativeIterable* this_ptr, const QVariant* key, QAssociativeIterable::const_iterator* output) {
  new(output) QAssociativeIterable::const_iterator(this_ptr->find(*key));
}

int qt_core_c_QAssociativeIterable_size(const QAssociativeIterable* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QAssociativeIterable_value_to_output(const QAssociativeIterable* this_ptr, const QVariant* key, QVariant* output) {
  new(output) QVariant(this_ptr->value(*key));
}

