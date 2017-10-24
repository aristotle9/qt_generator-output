#include "qt_core_c_QAbstractListModel.h"

QAbstractListModel* qt_core_c_QAbstractListModel_G_dynamic_cast_QAbstractListModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr) {
  return dynamic_cast<QAbstractListModel*>(ptr);
}

QAbstractListModel* qt_core_c_QAbstractListModel_G_dynamic_cast_QAbstractListModel_ptr_QObject(QObject* ptr) {
  return dynamic_cast<QAbstractListModel*>(ptr);
}

QAbstractItemModel* qt_core_c_QAbstractListModel_G_static_cast_QAbstractItemModel_ptr(QAbstractListModel* ptr) {
  return static_cast<QAbstractItemModel*>(ptr);
}

QAbstractListModel* qt_core_c_QAbstractListModel_G_static_cast_QAbstractListModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr) {
  return static_cast<QAbstractListModel*>(ptr);
}

QAbstractListModel* qt_core_c_QAbstractListModel_G_static_cast_QAbstractListModel_ptr_QObject(QObject* ptr) {
  return static_cast<QAbstractListModel*>(ptr);
}

QObject* qt_core_c_QAbstractListModel_G_static_cast_QObject_ptr(QAbstractListModel* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_core_c_QAbstractListModel_delete(QAbstractListModel* this_ptr) {
  delete this_ptr;
}

bool qt_core_c_QAbstractListModel_dropMimeData(QAbstractListModel* this_ptr, const QMimeData* data, const Qt::DropAction* action, int row, int column, const QModelIndex* parent) {
  return this_ptr->dropMimeData(data, *action, row, column, *parent);
}

void qt_core_c_QAbstractListModel_index_to_output_row(const QAbstractListModel* this_ptr, int row, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->index(row));
}

void qt_core_c_QAbstractListModel_index_to_output_row_column(const QAbstractListModel* this_ptr, int row, int column, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->index(row, column));
}

void qt_core_c_QAbstractListModel_index_to_output_row_column_parent(const QAbstractListModel* this_ptr, int row, int column, const QModelIndex* parent, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->index(row, column, *parent));
}

const QMetaObject* qt_core_c_QAbstractListModel_metaObject(const QAbstractListModel* this_ptr) {
  return this_ptr->metaObject();
}

void qt_core_c_QAbstractListModel_sibling_to_output(const QAbstractListModel* this_ptr, int row, int column, const QModelIndex* idx, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->sibling(row, column, *idx));
}

void qt_core_c_QAbstractListModel_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractListModel::trUtf8(s, c, n));
}

void qt_core_c_QAbstractListModel_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractListModel::tr(s, c, n));
}

