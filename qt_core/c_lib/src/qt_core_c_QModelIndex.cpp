#include "qt_core_c_QModelIndex.h"

void qt_core_c_QModelIndex_child_to_output(const QModelIndex* this_ptr, int row, int column, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->child(row, column));
}

int qt_core_c_QModelIndex_column(const QModelIndex* this_ptr) {
  return this_ptr->column();
}

void qt_core_c_QModelIndex_constructor(QModelIndex* output) {
  new(output) QModelIndex();
}

void qt_core_c_QModelIndex_data_to_output_no_args(const QModelIndex* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->data());
}

void qt_core_c_QModelIndex_data_to_output_role(const QModelIndex* this_ptr, int role, QVariant* output) {
  new(output) QVariant(this_ptr->data(role));
}

void qt_core_c_QModelIndex_destructor(QModelIndex* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

quintptr qt_core_c_QModelIndex_internalId(const QModelIndex* this_ptr) {
  return this_ptr->internalId();
}

void* qt_core_c_QModelIndex_internalPointer(const QModelIndex* this_ptr) {
  return this_ptr->internalPointer();
}

bool qt_core_c_QModelIndex_isValid(const QModelIndex* this_ptr) {
  return this_ptr->isValid();
}

const QAbstractItemModel* qt_core_c_QModelIndex_model(const QModelIndex* this_ptr) {
  return this_ptr->model();
}

bool qt_core_c_QModelIndex_operator_eq(const QModelIndex* this_ptr, const QModelIndex* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QModelIndex_operator_lt(const QModelIndex* this_ptr, const QModelIndex* other) {
  return this_ptr->operator<(*other);
}

bool qt_core_c_QModelIndex_operator_neq(const QModelIndex* this_ptr, const QModelIndex* other) {
  return this_ptr->operator!=(*other);
}

void qt_core_c_QModelIndex_parent_to_output(const QModelIndex* this_ptr, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->parent());
}

int qt_core_c_QModelIndex_row(const QModelIndex* this_ptr) {
  return this_ptr->row();
}

void qt_core_c_QModelIndex_sibling_to_output(const QModelIndex* this_ptr, int row, int column, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->sibling(row, column));
}

