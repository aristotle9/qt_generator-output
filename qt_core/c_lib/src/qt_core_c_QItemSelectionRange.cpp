#include "qt_core_c_QItemSelectionRange.h"

int qt_core_c_QItemSelectionRange_bottom(const QItemSelectionRange* this_ptr) {
  return this_ptr->bottom();
}

const QPersistentModelIndex* qt_core_c_QItemSelectionRange_bottomRight(const QItemSelectionRange* this_ptr) {
  return &this_ptr->bottomRight();
}

void qt_core_c_QItemSelectionRange_constructor_index(const QModelIndex* index, QItemSelectionRange* output) {
  new(output) QItemSelectionRange(*index);
}

void qt_core_c_QItemSelectionRange_constructor_no_args(QItemSelectionRange* output) {
  new(output) QItemSelectionRange();
}

void qt_core_c_QItemSelectionRange_constructor_other(const QItemSelectionRange* other, QItemSelectionRange* output) {
  new(output) QItemSelectionRange(*other);
}

void qt_core_c_QItemSelectionRange_constructor_topL_bottomR(const QModelIndex* topL, const QModelIndex* bottomR, QItemSelectionRange* output) {
  new(output) QItemSelectionRange(*topL, *bottomR);
}

bool qt_core_c_QItemSelectionRange_contains_index(const QItemSelectionRange* this_ptr, const QModelIndex* index) {
  return this_ptr->contains(*index);
}

bool qt_core_c_QItemSelectionRange_contains_row_column_parentIndex(const QItemSelectionRange* this_ptr, int row, int column, const QModelIndex* parentIndex) {
  return this_ptr->contains(row, column, *parentIndex);
}

void qt_core_c_QItemSelectionRange_destructor(QItemSelectionRange* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

int qt_core_c_QItemSelectionRange_height(const QItemSelectionRange* this_ptr) {
  return this_ptr->height();
}

void qt_core_c_QItemSelectionRange_indexes_to_output(const QItemSelectionRange* this_ptr, QList< QModelIndex >* output) {
  new(output) QList< QModelIndex >(this_ptr->indexes());
}

void qt_core_c_QItemSelectionRange_intersected_to_output(const QItemSelectionRange* this_ptr, const QItemSelectionRange* other, QItemSelectionRange* output) {
  new(output) QItemSelectionRange(this_ptr->intersected(*other));
}

bool qt_core_c_QItemSelectionRange_intersects(const QItemSelectionRange* this_ptr, const QItemSelectionRange* other) {
  return this_ptr->intersects(*other);
}

bool qt_core_c_QItemSelectionRange_isEmpty(const QItemSelectionRange* this_ptr) {
  return this_ptr->isEmpty();
}

bool qt_core_c_QItemSelectionRange_isValid(const QItemSelectionRange* this_ptr) {
  return this_ptr->isValid();
}

int qt_core_c_QItemSelectionRange_left(const QItemSelectionRange* this_ptr) {
  return this_ptr->left();
}

const QAbstractItemModel* qt_core_c_QItemSelectionRange_model(const QItemSelectionRange* this_ptr) {
  return this_ptr->model();
}

QItemSelectionRange* qt_core_c_QItemSelectionRange_operator_assign(QItemSelectionRange* this_ptr, const QItemSelectionRange* other) {
  return &this_ptr->operator=(*other);
}

bool qt_core_c_QItemSelectionRange_operator_eq(const QItemSelectionRange* this_ptr, const QItemSelectionRange* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QItemSelectionRange_operator_lt(const QItemSelectionRange* this_ptr, const QItemSelectionRange* other) {
  return this_ptr->operator<(*other);
}

bool qt_core_c_QItemSelectionRange_operator_neq(const QItemSelectionRange* this_ptr, const QItemSelectionRange* other) {
  return this_ptr->operator!=(*other);
}

void qt_core_c_QItemSelectionRange_parent_to_output(const QItemSelectionRange* this_ptr, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->parent());
}

int qt_core_c_QItemSelectionRange_right(const QItemSelectionRange* this_ptr) {
  return this_ptr->right();
}

void qt_core_c_QItemSelectionRange_swap(QItemSelectionRange* this_ptr, QItemSelectionRange* other) {
  this_ptr->swap(*other);
}

int qt_core_c_QItemSelectionRange_top(const QItemSelectionRange* this_ptr) {
  return this_ptr->top();
}

const QPersistentModelIndex* qt_core_c_QItemSelectionRange_topLeft(const QItemSelectionRange* this_ptr) {
  return &this_ptr->topLeft();
}

int qt_core_c_QItemSelectionRange_width(const QItemSelectionRange* this_ptr) {
  return this_ptr->width();
}

