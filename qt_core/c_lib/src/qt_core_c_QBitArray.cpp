#include "qt_core_c_QBitArray.h"

void qt_core_c_QBitArray_G_operator_bit_and_to_output(const QBitArray* arg1, const QBitArray* arg2, QBitArray* output) {
  new(output) QBitArray(operator&(*arg1, *arg2));
}

void qt_core_c_QBitArray_G_operator_bit_or_to_output(const QBitArray* arg1, const QBitArray* arg2, QBitArray* output) {
  new(output) QBitArray(operator|(*arg1, *arg2));
}

void qt_core_c_QBitArray_G_operator_bit_xor_to_output(const QBitArray* arg1, const QBitArray* arg2, QBitArray* output) {
  new(output) QBitArray(operator^(*arg1, *arg2));
}

void qt_core_c_QBitArray_G_operator_shl_to_output(const QDebug* arg1, const QBitArray* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

void qt_core_c_QBitArray_G_swap(QBitArray* value1, QBitArray* value2) {
  swap(*value1, *value2);
}

bool qt_core_c_QBitArray_at(const QBitArray* this_ptr, int i) {
  return this_ptr->at(i);
}

void qt_core_c_QBitArray_clear(QBitArray* this_ptr) {
  this_ptr->clear();
}

void qt_core_c_QBitArray_clearBit(QBitArray* this_ptr, int i) {
  this_ptr->clearBit(i);
}

void qt_core_c_QBitArray_constructor_no_args(QBitArray* output) {
  new(output) QBitArray();
}

void qt_core_c_QBitArray_constructor_other(const QBitArray* other, QBitArray* output) {
  new(output) QBitArray(*other);
}

void qt_core_c_QBitArray_constructor_size(int size, QBitArray* output) {
  new(output) QBitArray(size);
}

void qt_core_c_QBitArray_constructor_size_val(int size, bool val, QBitArray* output) {
  new(output) QBitArray(size, val);
}

int qt_core_c_QBitArray_count_no_args(const QBitArray* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QBitArray_count_on(const QBitArray* this_ptr, bool on) {
  return this_ptr->count(on);
}

void qt_core_c_QBitArray_destructor(QBitArray* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QBitArray_fill_val(QBitArray* this_ptr, bool val) {
  return this_ptr->fill(val);
}

void qt_core_c_QBitArray_fill_val_first_last(QBitArray* this_ptr, bool val, int first, int last) {
  this_ptr->fill(val, first, last);
}

bool qt_core_c_QBitArray_fill_val_size(QBitArray* this_ptr, bool val, int size) {
  return this_ptr->fill(val, size);
}

bool qt_core_c_QBitArray_isEmpty(const QBitArray* this_ptr) {
  return this_ptr->isEmpty();
}

bool qt_core_c_QBitArray_isNull(const QBitArray* this_ptr) {
  return this_ptr->isNull();
}

QBitArray* qt_core_c_QBitArray_operator_assign(QBitArray* this_ptr, const QBitArray* other) {
  return &this_ptr->operator=(*other);
}

QBitArray* qt_core_c_QBitArray_operator_bit_and_assign(QBitArray* this_ptr, const QBitArray* arg1) {
  return &this_ptr->operator&=(*arg1);
}

void qt_core_c_QBitArray_operator_bit_not_to_output(const QBitArray* this_ptr, QBitArray* output) {
  new(output) QBitArray(this_ptr->operator~());
}

QBitArray* qt_core_c_QBitArray_operator_bit_or_assign(QBitArray* this_ptr, const QBitArray* arg1) {
  return &this_ptr->operator|=(*arg1);
}

QBitArray* qt_core_c_QBitArray_operator_bit_xor_assign(QBitArray* this_ptr, const QBitArray* arg1) {
  return &this_ptr->operator^=(*arg1);
}

bool qt_core_c_QBitArray_operator_eq(const QBitArray* this_ptr, const QBitArray* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QBitArray_operator_index_int(const QBitArray* this_ptr, int i) {
  return this_ptr->operator[](i);
}

void qt_core_c_QBitArray_operator_index_to_output_int(QBitArray* this_ptr, int i, QBitRef* output) {
  new(output) QBitRef(this_ptr->operator[](i));
}

void qt_core_c_QBitArray_operator_index_to_output_unsigned_int(QBitArray* this_ptr, unsigned int i, QBitRef* output) {
  new(output) QBitRef(this_ptr->operator[](i));
}

bool qt_core_c_QBitArray_operator_index_unsigned_int(const QBitArray* this_ptr, unsigned int i) {
  return this_ptr->operator[](i);
}

bool qt_core_c_QBitArray_operator_neq(const QBitArray* this_ptr, const QBitArray* other) {
  return this_ptr->operator!=(*other);
}

void qt_core_c_QBitArray_resize(QBitArray* this_ptr, int size) {
  this_ptr->resize(size);
}

void qt_core_c_QBitArray_setBit_i(QBitArray* this_ptr, int i) {
  this_ptr->setBit(i);
}

void qt_core_c_QBitArray_setBit_i_val(QBitArray* this_ptr, int i, bool val) {
  this_ptr->setBit(i, val);
}

int qt_core_c_QBitArray_size(const QBitArray* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QBitArray_swap(QBitArray* this_ptr, QBitArray* other) {
  this_ptr->swap(*other);
}

bool qt_core_c_QBitArray_testBit(const QBitArray* this_ptr, int i) {
  return this_ptr->testBit(i);
}

bool qt_core_c_QBitArray_toggleBit(QBitArray* this_ptr, int i) {
  return this_ptr->toggleBit(i);
}

void qt_core_c_QBitArray_truncate(QBitArray* this_ptr, int pos) {
  this_ptr->truncate(pos);
}

