#include "qt_core_c_QAbstractTableModel.h"

QAbstractTableModel* qt_core_c_QAbstractTableModel_G_dynamic_cast_QAbstractTableModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr) {
  return dynamic_cast<QAbstractTableModel*>(ptr);
}

QAbstractTableModel* qt_core_c_QAbstractTableModel_G_dynamic_cast_QAbstractTableModel_ptr_QObject(QObject* ptr) {
  return dynamic_cast<QAbstractTableModel*>(ptr);
}

QAbstractItemModel* qt_core_c_QAbstractTableModel_G_static_cast_QAbstractItemModel_ptr(QAbstractTableModel* ptr) {
  return static_cast<QAbstractItemModel*>(ptr);
}

QAbstractTableModel* qt_core_c_QAbstractTableModel_G_static_cast_QAbstractTableModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr) {
  return static_cast<QAbstractTableModel*>(ptr);
}

QAbstractTableModel* qt_core_c_QAbstractTableModel_G_static_cast_QAbstractTableModel_ptr_QObject(QObject* ptr) {
  return static_cast<QAbstractTableModel*>(ptr);
}

QObject* qt_core_c_QAbstractTableModel_G_static_cast_QObject_ptr(QAbstractTableModel* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_core_c_QAbstractTableModel_delete(QAbstractTableModel* this_ptr) {
  delete this_ptr;
}

bool qt_core_c_QAbstractTableModel_dropMimeData(QAbstractTableModel* this_ptr, const QMimeData* data, const Qt::DropAction* action, int row, int column, const QModelIndex* parent) {
  return this_ptr->dropMimeData(data, *action, row, column, *parent);
}

void qt_core_c_QAbstractTableModel_index_to_output_row_column(const QAbstractTableModel* this_ptr, int row, int column, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->index(row, column));
}

void qt_core_c_QAbstractTableModel_index_to_output_row_column_parent(const QAbstractTableModel* this_ptr, int row, int column, const QModelIndex* parent, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->index(row, column, *parent));
}

const QMetaObject* qt_core_c_QAbstractTableModel_metaObject(const QAbstractTableModel* this_ptr) {
  return this_ptr->metaObject();
}

void qt_core_c_QAbstractTableModel_sibling_to_output(const QAbstractTableModel* this_ptr, int row, int column, const QModelIndex* idx, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->sibling(row, column, *idx));
}

void qt_core_c_QAbstractTableModel_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractTableModel::trUtf8(s, c, n));
}

void qt_core_c_QAbstractTableModel_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractTableModel::tr(s, c, n));
}

