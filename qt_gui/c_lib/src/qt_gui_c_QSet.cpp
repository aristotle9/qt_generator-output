#include "qt_gui_c_QSet.h"

int qt_gui_c_QSet_QByteArray_capacity(const QSet< QByteArray >* this_ptr) {
  return this_ptr->capacity();
}

void qt_gui_c_QSet_QByteArray_clear(QSet< QByteArray >* this_ptr) {
  this_ptr->clear();
}

void qt_gui_c_QSet_QByteArray_constructor(QSet< QByteArray >* output) {
  new(output) QSet< QByteArray >();
}

bool qt_gui_c_QSet_QByteArray_contains_set(const QSet< QByteArray >* this_ptr, const QSet< QByteArray >* set) {
  return this_ptr->contains(*set);
}

bool qt_gui_c_QSet_QByteArray_contains_value(const QSet< QByteArray >* this_ptr, const QByteArray* value) {
  return this_ptr->contains(*value);
}

int qt_gui_c_QSet_QByteArray_count(const QSet< QByteArray >* this_ptr) {
  return this_ptr->count();
}

void qt_gui_c_QSet_QByteArray_destructor(QSet< QByteArray >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QSet_QByteArray_empty(const QSet< QByteArray >* this_ptr) {
  return this_ptr->empty();
}

void qt_gui_c_QSet_QByteArray_fromList_to_output(const QList< QByteArray >* list, QSet< QByteArray >* output) {
  new(output) QSet< QByteArray >(QSet< QByteArray >::fromList(*list));
}

QSet< QByteArray >* qt_gui_c_QSet_QByteArray_intersect(QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other) {
  return &this_ptr->intersect(*other);
}

bool qt_gui_c_QSet_QByteArray_intersects(const QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other) {
  return this_ptr->intersects(*other);
}

bool qt_gui_c_QSet_QByteArray_isEmpty(const QSet< QByteArray >* this_ptr) {
  return this_ptr->isEmpty();
}

QSet< QByteArray >* qt_gui_c_QSet_QByteArray_operator_add_assign_other(QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other) {
  return &this_ptr->operator+=(*other);
}

QSet< QByteArray >* qt_gui_c_QSet_QByteArray_operator_add_assign_value(QSet< QByteArray >* this_ptr, const QByteArray* value) {
  return &this_ptr->operator+=(*value);
}

void qt_gui_c_QSet_QByteArray_operator_add_to_output(const QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other, QSet< QByteArray >* output) {
  new(output) QSet< QByteArray >(this_ptr->operator+(*other));
}

QSet< QByteArray >* qt_gui_c_QSet_QByteArray_operator_bit_and_assign_other(QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other) {
  return &this_ptr->operator&=(*other);
}

QSet< QByteArray >* qt_gui_c_QSet_QByteArray_operator_bit_and_assign_value(QSet< QByteArray >* this_ptr, const QByteArray* value) {
  return &this_ptr->operator&=(*value);
}

void qt_gui_c_QSet_QByteArray_operator_bit_and_to_output(const QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other, QSet< QByteArray >* output) {
  new(output) QSet< QByteArray >(this_ptr->operator&(*other));
}

QSet< QByteArray >* qt_gui_c_QSet_QByteArray_operator_bit_or_assign_other(QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other) {
  return &this_ptr->operator|=(*other);
}

QSet< QByteArray >* qt_gui_c_QSet_QByteArray_operator_bit_or_assign_value(QSet< QByteArray >* this_ptr, const QByteArray* value) {
  return &this_ptr->operator|=(*value);
}

void qt_gui_c_QSet_QByteArray_operator_bit_or_to_output(const QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other, QSet< QByteArray >* output) {
  new(output) QSet< QByteArray >(this_ptr->operator|(*other));
}

bool qt_gui_c_QSet_QByteArray_operator_eq(const QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other) {
  return this_ptr->operator==(*other);
}

bool qt_gui_c_QSet_QByteArray_operator_neq(const QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other) {
  return this_ptr->operator!=(*other);
}

QSet< QByteArray >* qt_gui_c_QSet_QByteArray_operator_shl(QSet< QByteArray >* this_ptr, const QByteArray* value) {
  return &this_ptr->operator<<(*value);
}

QSet< QByteArray >* qt_gui_c_QSet_QByteArray_operator_sub_assign_other(QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other) {
  return &this_ptr->operator-=(*other);
}

QSet< QByteArray >* qt_gui_c_QSet_QByteArray_operator_sub_assign_value(QSet< QByteArray >* this_ptr, const QByteArray* value) {
  return &this_ptr->operator-=(*value);
}

void qt_gui_c_QSet_QByteArray_operator_sub_to_output(const QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other, QSet< QByteArray >* output) {
  new(output) QSet< QByteArray >(this_ptr->operator-(*other));
}

bool qt_gui_c_QSet_QByteArray_remove(QSet< QByteArray >* this_ptr, const QByteArray* value) {
  return this_ptr->remove(*value);
}

void qt_gui_c_QSet_QByteArray_reserve(QSet< QByteArray >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_gui_c_QSet_QByteArray_size(const QSet< QByteArray >* this_ptr) {
  return this_ptr->size();
}

void qt_gui_c_QSet_QByteArray_squeeze(QSet< QByteArray >* this_ptr) {
  this_ptr->squeeze();
}

QSet< QByteArray >* qt_gui_c_QSet_QByteArray_subtract(QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other) {
  return &this_ptr->subtract(*other);
}

void qt_gui_c_QSet_QByteArray_swap(QSet< QByteArray >* this_ptr, QSet< QByteArray >* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QSet_QByteArray_toList_to_output(const QSet< QByteArray >* this_ptr, QList< QByteArray >* output) {
  new(output) QList< QByteArray >(this_ptr->toList());
}

QSet< QByteArray >* qt_gui_c_QSet_QByteArray_unite(QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other) {
  return &this_ptr->unite(*other);
}

void qt_gui_c_QSet_QByteArray_values_to_output(const QSet< QByteArray >* this_ptr, QList< QByteArray >* output) {
  new(output) QList< QByteArray >(this_ptr->values());
}

