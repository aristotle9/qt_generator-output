#include "qt_core_c_QItemSelectionModel.h"

QItemSelectionModel* qt_core_c_QItemSelectionModel_G_dynamic_cast_QItemSelectionModel_ptr(QObject* ptr) {
  return dynamic_cast<QItemSelectionModel*>(ptr);
}

void qt_core_c_QItemSelectionModel_G_operator_shl_to_output(const QDebug* arg1, const QItemSelectionRange* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

unsigned int qt_core_c_QItemSelectionModel_G_qHash(const QItemSelectionRange* arg1) {
  return qHash(*arg1);
}

QItemSelectionModel* qt_core_c_QItemSelectionModel_G_static_cast_QItemSelectionModel_ptr(QObject* ptr) {
  return static_cast<QItemSelectionModel*>(ptr);
}

QObject* qt_core_c_QItemSelectionModel_G_static_cast_QObject_ptr(QItemSelectionModel* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_core_c_QItemSelectionModel_G_swap(QItemSelection* value1, QItemSelection* value2) {
  swap(*value1, *value2);
}

void qt_core_c_QItemSelectionModel_clear(QItemSelectionModel* this_ptr) {
  this_ptr->clear();
}

void qt_core_c_QItemSelectionModel_clearCurrentIndex(QItemSelectionModel* this_ptr) {
  this_ptr->clearCurrentIndex();
}

void qt_core_c_QItemSelectionModel_clearSelection(QItemSelectionModel* this_ptr) {
  this_ptr->clearSelection();
}

bool qt_core_c_QItemSelectionModel_columnIntersectsSelection(const QItemSelectionModel* this_ptr, int column, const QModelIndex* parent) {
  return this_ptr->columnIntersectsSelection(column, *parent);
}

void qt_core_c_QItemSelectionModel_currentIndex_to_output(const QItemSelectionModel* this_ptr, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->currentIndex());
}

void qt_core_c_QItemSelectionModel_delete(QItemSelectionModel* this_ptr) {
  delete this_ptr;
}

bool qt_core_c_QItemSelectionModel_hasSelection(const QItemSelectionModel* this_ptr) {
  return this_ptr->hasSelection();
}

bool qt_core_c_QItemSelectionModel_isColumnSelected(const QItemSelectionModel* this_ptr, int column, const QModelIndex* parent) {
  return this_ptr->isColumnSelected(column, *parent);
}

bool qt_core_c_QItemSelectionModel_isRowSelected(const QItemSelectionModel* this_ptr, int row, const QModelIndex* parent) {
  return this_ptr->isRowSelected(row, *parent);
}

bool qt_core_c_QItemSelectionModel_isSelected(const QItemSelectionModel* this_ptr, const QModelIndex* index) {
  return this_ptr->isSelected(*index);
}

const QMetaObject* qt_core_c_QItemSelectionModel_metaObject(const QItemSelectionModel* this_ptr) {
  return this_ptr->metaObject();
}

QAbstractItemModel* qt_core_c_QItemSelectionModel_model(QItemSelectionModel* this_ptr) {
  return this_ptr->model();
}

const QAbstractItemModel* qt_core_c_QItemSelectionModel_model_const(const QItemSelectionModel* this_ptr) {
  return this_ptr->model();
}

QItemSelectionModel* qt_core_c_QItemSelectionModel_new_model(QAbstractItemModel* model) {
  return new QItemSelectionModel(model);
}

QItemSelectionModel* qt_core_c_QItemSelectionModel_new_model_parent(QAbstractItemModel* model, QObject* parent) {
  return new QItemSelectionModel(model, parent);
}

QItemSelectionModel* qt_core_c_QItemSelectionModel_new_no_args() {
  return new QItemSelectionModel();
}

void qt_core_c_QItemSelectionModel_reset(QItemSelectionModel* this_ptr) {
  this_ptr->reset();
}

bool qt_core_c_QItemSelectionModel_rowIntersectsSelection(const QItemSelectionModel* this_ptr, int row, const QModelIndex* parent) {
  return this_ptr->rowIntersectsSelection(row, *parent);
}

void qt_core_c_QItemSelectionModel_selectedColumns_to_output_no_args(const QItemSelectionModel* this_ptr, QList< QModelIndex >* output) {
  new(output) QList< QModelIndex >(this_ptr->selectedColumns());
}

void qt_core_c_QItemSelectionModel_selectedColumns_to_output_row(const QItemSelectionModel* this_ptr, int row, QList< QModelIndex >* output) {
  new(output) QList< QModelIndex >(this_ptr->selectedColumns(row));
}

void qt_core_c_QItemSelectionModel_selectedIndexes_to_output(const QItemSelectionModel* this_ptr, QList< QModelIndex >* output) {
  new(output) QList< QModelIndex >(this_ptr->selectedIndexes());
}

void qt_core_c_QItemSelectionModel_selectedRows_to_output_column(const QItemSelectionModel* this_ptr, int column, QList< QModelIndex >* output) {
  new(output) QList< QModelIndex >(this_ptr->selectedRows(column));
}

void qt_core_c_QItemSelectionModel_selectedRows_to_output_no_args(const QItemSelectionModel* this_ptr, QList< QModelIndex >* output) {
  new(output) QList< QModelIndex >(this_ptr->selectedRows());
}

void qt_core_c_QItemSelectionModel_selection_to_output(const QItemSelectionModel* this_ptr, QItemSelection* output) {
  new(output) QItemSelection(this_ptr->selection());
}

void qt_core_c_QItemSelectionModel_setModel(QItemSelectionModel* this_ptr, QAbstractItemModel* model) {
  this_ptr->setModel(model);
}

void qt_core_c_QItemSelectionModel_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QItemSelectionModel::trUtf8(s, c, n));
}

void qt_core_c_QItemSelectionModel_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QItemSelectionModel::tr(s, c, n));
}

