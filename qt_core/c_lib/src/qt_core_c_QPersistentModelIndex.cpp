#include "qt_core_c_QPersistentModelIndex.h"

void qt_core_c_QPersistentModelIndex_child_to_output(const QPersistentModelIndex* this_ptr, int row, int column, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->child(row, column));
}

int qt_core_c_QPersistentModelIndex_column(const QPersistentModelIndex* this_ptr) {
  return this_ptr->column();
}

void qt_core_c_QPersistentModelIndex_constructor_index(const QModelIndex* index, QPersistentModelIndex* output) {
  new(output) QPersistentModelIndex(*index);
}

void qt_core_c_QPersistentModelIndex_constructor_no_args(QPersistentModelIndex* output) {
  new(output) QPersistentModelIndex();
}

void qt_core_c_QPersistentModelIndex_constructor_other(const QPersistentModelIndex* other, QPersistentModelIndex* output) {
  new(output) QPersistentModelIndex(*other);
}

const QModelIndex* qt_core_c_QPersistentModelIndex_convert_to_QModelIndex_ref(const QPersistentModelIndex* this_ptr) {
  return &this_ptr->operator const QModelIndex &();
}

void qt_core_c_QPersistentModelIndex_data_to_output_no_args(const QPersistentModelIndex* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->data());
}

void qt_core_c_QPersistentModelIndex_data_to_output_role(const QPersistentModelIndex* this_ptr, int role, QVariant* output) {
  new(output) QVariant(this_ptr->data(role));
}

void qt_core_c_QPersistentModelIndex_destructor(QPersistentModelIndex* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QPersistentModelIndex_isValid(const QPersistentModelIndex* this_ptr) {
  return this_ptr->isValid();
}

const QAbstractItemModel* qt_core_c_QPersistentModelIndex_model(const QPersistentModelIndex* this_ptr) {
  return this_ptr->model();
}

QPersistentModelIndex* qt_core_c_QPersistentModelIndex_operator_assign_QModelIndex(QPersistentModelIndex* this_ptr, const QModelIndex* other) {
  return &this_ptr->operator=(*other);
}

QPersistentModelIndex* qt_core_c_QPersistentModelIndex_operator_assign_QPersistentModelIndex(QPersistentModelIndex* this_ptr, const QPersistentModelIndex* other) {
  return &this_ptr->operator=(*other);
}

bool qt_core_c_QPersistentModelIndex_operator_eq_QModelIndex(const QPersistentModelIndex* this_ptr, const QModelIndex* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QPersistentModelIndex_operator_eq_QPersistentModelIndex(const QPersistentModelIndex* this_ptr, const QPersistentModelIndex* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QPersistentModelIndex_operator_lt(const QPersistentModelIndex* this_ptr, const QPersistentModelIndex* other) {
  return this_ptr->operator<(*other);
}

bool qt_core_c_QPersistentModelIndex_operator_neq_QModelIndex(const QPersistentModelIndex* this_ptr, const QModelIndex* other) {
  return this_ptr->operator!=(*other);
}

bool qt_core_c_QPersistentModelIndex_operator_neq_QPersistentModelIndex(const QPersistentModelIndex* this_ptr, const QPersistentModelIndex* other) {
  return this_ptr->operator!=(*other);
}

void qt_core_c_QPersistentModelIndex_parent_to_output(const QPersistentModelIndex* this_ptr, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->parent());
}

int qt_core_c_QPersistentModelIndex_row(const QPersistentModelIndex* this_ptr) {
  return this_ptr->row();
}

void qt_core_c_QPersistentModelIndex_sibling_to_output(const QPersistentModelIndex* this_ptr, int row, int column, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->sibling(row, column));
}

void qt_core_c_QPersistentModelIndex_swap(QPersistentModelIndex* this_ptr, QPersistentModelIndex* other) {
  this_ptr->swap(*other);
}

