#include "qt_core_c_QStringListModel.h"

QStringListModel* qt_core_c_QStringListModel_G_dynamic_cast_QStringListModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr) {
  return dynamic_cast<QStringListModel*>(ptr);
}

QStringListModel* qt_core_c_QStringListModel_G_dynamic_cast_QStringListModel_ptr_QAbstractListModel(QAbstractListModel* ptr) {
  return dynamic_cast<QStringListModel*>(ptr);
}

QStringListModel* qt_core_c_QStringListModel_G_dynamic_cast_QStringListModel_ptr_QObject(QObject* ptr) {
  return dynamic_cast<QStringListModel*>(ptr);
}

QAbstractItemModel* qt_core_c_QStringListModel_G_static_cast_QAbstractItemModel_ptr(QStringListModel* ptr) {
  return static_cast<QAbstractItemModel*>(ptr);
}

QAbstractListModel* qt_core_c_QStringListModel_G_static_cast_QAbstractListModel_ptr(QStringListModel* ptr) {
  return static_cast<QAbstractListModel*>(ptr);
}

QObject* qt_core_c_QStringListModel_G_static_cast_QObject_ptr(QStringListModel* ptr) {
  return static_cast<QObject*>(ptr);
}

QStringListModel* qt_core_c_QStringListModel_G_static_cast_QStringListModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr) {
  return static_cast<QStringListModel*>(ptr);
}

QStringListModel* qt_core_c_QStringListModel_G_static_cast_QStringListModel_ptr_QAbstractListModel(QAbstractListModel* ptr) {
  return static_cast<QStringListModel*>(ptr);
}

QStringListModel* qt_core_c_QStringListModel_G_static_cast_QStringListModel_ptr_QObject(QObject* ptr) {
  return static_cast<QStringListModel*>(ptr);
}

void qt_core_c_QStringListModel_data_to_output_index(const QStringListModel* this_ptr, const QModelIndex* index, QVariant* output) {
  new(output) QVariant(this_ptr->data(*index));
}

void qt_core_c_QStringListModel_data_to_output_index_role(const QStringListModel* this_ptr, const QModelIndex* index, int role, QVariant* output) {
  new(output) QVariant(this_ptr->data(*index, role));
}

void qt_core_c_QStringListModel_delete(QStringListModel* this_ptr) {
  delete this_ptr;
}

bool qt_core_c_QStringListModel_insertRows_row_count(QStringListModel* this_ptr, int row, int count) {
  return this_ptr->insertRows(row, count);
}

bool qt_core_c_QStringListModel_insertRows_row_count_parent(QStringListModel* this_ptr, int row, int count, const QModelIndex* parent) {
  return this_ptr->insertRows(row, count, *parent);
}

const QMetaObject* qt_core_c_QStringListModel_metaObject(const QStringListModel* this_ptr) {
  return this_ptr->metaObject();
}

QStringListModel* qt_core_c_QStringListModel_new_no_args() {
  return new QStringListModel();
}

QStringListModel* qt_core_c_QStringListModel_new_parent(QObject* parent) {
  return new QStringListModel(parent);
}

QStringListModel* qt_core_c_QStringListModel_new_strings(const QStringList* strings) {
  return new QStringListModel(*strings);
}

QStringListModel* qt_core_c_QStringListModel_new_strings_parent(const QStringList* strings, QObject* parent) {
  return new QStringListModel(*strings, parent);
}

bool qt_core_c_QStringListModel_removeRows_row_count(QStringListModel* this_ptr, int row, int count) {
  return this_ptr->removeRows(row, count);
}

bool qt_core_c_QStringListModel_removeRows_row_count_parent(QStringListModel* this_ptr, int row, int count, const QModelIndex* parent) {
  return this_ptr->removeRows(row, count, *parent);
}

int qt_core_c_QStringListModel_rowCount_no_args(const QStringListModel* this_ptr) {
  return this_ptr->rowCount();
}

int qt_core_c_QStringListModel_rowCount_parent(const QStringListModel* this_ptr, const QModelIndex* parent) {
  return this_ptr->rowCount(*parent);
}

bool qt_core_c_QStringListModel_setData_index_value(QStringListModel* this_ptr, const QModelIndex* index, const QVariant* value) {
  return this_ptr->setData(*index, *value);
}

bool qt_core_c_QStringListModel_setData_index_value_role(QStringListModel* this_ptr, const QModelIndex* index, const QVariant* value, int role) {
  return this_ptr->setData(*index, *value, role);
}

void qt_core_c_QStringListModel_setStringList(QStringListModel* this_ptr, const QStringList* strings) {
  this_ptr->setStringList(*strings);
}

void qt_core_c_QStringListModel_sibling_to_output(const QStringListModel* this_ptr, int row, int column, const QModelIndex* idx, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->sibling(row, column, *idx));
}

void qt_core_c_QStringListModel_sort_column(QStringListModel* this_ptr, int column) {
  this_ptr->sort(column);
}

void qt_core_c_QStringListModel_sort_column_order(QStringListModel* this_ptr, int column, const Qt::SortOrder* order) {
  this_ptr->sort(column, *order);
}

void qt_core_c_QStringListModel_stringList_to_output(const QStringListModel* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->stringList());
}

void qt_core_c_QStringListModel_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QStringListModel::trUtf8(s, c, n));
}

void qt_core_c_QStringListModel_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QStringListModel::tr(s, c, n));
}

