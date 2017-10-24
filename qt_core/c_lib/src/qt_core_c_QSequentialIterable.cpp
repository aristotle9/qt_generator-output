#include "qt_core_c_QSequentialIterable.h"

void qt_core_c_QSequentialIterable_at_to_output(const QSequentialIterable* this_ptr, int idx, QVariant* output) {
  new(output) QVariant(this_ptr->at(idx));
}

void qt_core_c_QSequentialIterable_begin_to_output(const QSequentialIterable* this_ptr, QSequentialIterable::const_iterator* output) {
  new(output) QSequentialIterable::const_iterator(this_ptr->begin());
}

bool qt_core_c_QSequentialIterable_canReverseIterate(const QSequentialIterable* this_ptr) {
  return this_ptr->canReverseIterate();
}

void qt_core_c_QSequentialIterable_const_iterator_constructor(const QSequentialIterable::const_iterator* other, QSequentialIterable::const_iterator* output) {
  new(output) QSequentialIterable::const_iterator(*other);
}

void qt_core_c_QSequentialIterable_const_iterator_destructor(QSequentialIterable::const_iterator* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

QSequentialIterable::const_iterator* qt_core_c_QSequentialIterable_const_iterator_operator_add_assign(QSequentialIterable::const_iterator* this_ptr, int j) {
  return &this_ptr->operator+=(j);
}

void qt_core_c_QSequentialIterable_const_iterator_operator_add_to_output(const QSequentialIterable::const_iterator* this_ptr, int j, QSequentialIterable::const_iterator* output) {
  new(output) QSequentialIterable::const_iterator(this_ptr->operator+(j));
}

QSequentialIterable::const_iterator* qt_core_c_QSequentialIterable_const_iterator_operator_assign(QSequentialIterable::const_iterator* this_ptr, const QSequentialIterable::const_iterator* other) {
  return &this_ptr->operator=(*other);
}

QSequentialIterable::const_iterator* qt_core_c_QSequentialIterable_const_iterator_operator_dec(QSequentialIterable::const_iterator* this_ptr) {
  return &this_ptr->operator--();
}

void qt_core_c_QSequentialIterable_const_iterator_operator_dec_postfix_to_output(QSequentialIterable::const_iterator* this_ptr, int arg1, QSequentialIterable::const_iterator* output) {
  new(output) QSequentialIterable::const_iterator(this_ptr->operator--(arg1));
}

bool qt_core_c_QSequentialIterable_const_iterator_operator_eq(const QSequentialIterable::const_iterator* this_ptr, const QSequentialIterable::const_iterator* o) {
  return this_ptr->operator==(*o);
}

QSequentialIterable::const_iterator* qt_core_c_QSequentialIterable_const_iterator_operator_inc(QSequentialIterable::const_iterator* this_ptr) {
  return &this_ptr->operator++();
}

void qt_core_c_QSequentialIterable_const_iterator_operator_inc_postfix_to_output(QSequentialIterable::const_iterator* this_ptr, int arg1, QSequentialIterable::const_iterator* output) {
  new(output) QSequentialIterable::const_iterator(this_ptr->operator++(arg1));
}

void qt_core_c_QSequentialIterable_const_iterator_operator_indirection_to_output(const QSequentialIterable::const_iterator* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator*());
}

bool qt_core_c_QSequentialIterable_const_iterator_operator_neq(const QSequentialIterable::const_iterator* this_ptr, const QSequentialIterable::const_iterator* o) {
  return this_ptr->operator!=(*o);
}

QSequentialIterable::const_iterator* qt_core_c_QSequentialIterable_const_iterator_operator_sub_assign(QSequentialIterable::const_iterator* this_ptr, int j) {
  return &this_ptr->operator-=(j);
}

void qt_core_c_QSequentialIterable_const_iterator_operator_sub_to_output(const QSequentialIterable::const_iterator* this_ptr, int j, QSequentialIterable::const_iterator* output) {
  new(output) QSequentialIterable::const_iterator(this_ptr->operator-(j));
}

void qt_core_c_QSequentialIterable_destructor(QSequentialIterable* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QSequentialIterable_end_to_output(const QSequentialIterable* this_ptr, QSequentialIterable::const_iterator* output) {
  new(output) QSequentialIterable::const_iterator(this_ptr->end());
}

int qt_core_c_QSequentialIterable_size(const QSequentialIterable* this_ptr) {
  return this_ptr->size();
}

