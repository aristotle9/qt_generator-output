#include "qt_core_c_QItemSelection.h"

QItemSelection* qt_core_c_QItemSelection_G_static_cast_QItemSelection_ptr(QList< QItemSelectionRange >* ptr) {
  return static_cast<QItemSelection*>(ptr);
}

QList< QItemSelectionRange >* qt_core_c_QItemSelection_G_static_cast_QList_QItemSelectionRange_ptr(QItemSelection* ptr) {
  return static_cast<QList< QItemSelectionRange >*>(ptr);
}

void qt_core_c_QItemSelection_constructor_no_args(QItemSelection* output) {
  new(output) QItemSelection();
}

void qt_core_c_QItemSelection_constructor_topLeft_bottomRight(const QModelIndex* topLeft, const QModelIndex* bottomRight, QItemSelection* output) {
  new(output) QItemSelection(*topLeft, *bottomRight);
}

bool qt_core_c_QItemSelection_contains(const QItemSelection* this_ptr, const QModelIndex* index) {
  return this_ptr->contains(*index);
}

void qt_core_c_QItemSelection_destructor(QItemSelection* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QItemSelection_indexes_to_output(const QItemSelection* this_ptr, QList< QModelIndex >* output) {
  new(output) QList< QModelIndex >(this_ptr->indexes());
}

void qt_core_c_QItemSelection_select(QItemSelection* this_ptr, const QModelIndex* topLeft, const QModelIndex* bottomRight) {
  this_ptr->select(*topLeft, *bottomRight);
}

void qt_core_c_QItemSelection_split(const QItemSelectionRange* range, const QItemSelectionRange* other, QItemSelection* result) {
  QItemSelection::split(*range, *other, result);
}

