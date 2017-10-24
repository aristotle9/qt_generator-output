#include "qt_core_c_QIdentityProxyModel.h"

QIdentityProxyModel* qt_core_c_QIdentityProxyModel_G_dynamic_cast_QIdentityProxyModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr) {
  return dynamic_cast<QIdentityProxyModel*>(ptr);
}

QIdentityProxyModel* qt_core_c_QIdentityProxyModel_G_dynamic_cast_QIdentityProxyModel_ptr_QAbstractProxyModel(QAbstractProxyModel* ptr) {
  return dynamic_cast<QIdentityProxyModel*>(ptr);
}

QIdentityProxyModel* qt_core_c_QIdentityProxyModel_G_dynamic_cast_QIdentityProxyModel_ptr_QObject(QObject* ptr) {
  return dynamic_cast<QIdentityProxyModel*>(ptr);
}

QAbstractItemModel* qt_core_c_QIdentityProxyModel_G_static_cast_QAbstractItemModel_ptr(QIdentityProxyModel* ptr) {
  return static_cast<QAbstractItemModel*>(ptr);
}

QAbstractProxyModel* qt_core_c_QIdentityProxyModel_G_static_cast_QAbstractProxyModel_ptr(QIdentityProxyModel* ptr) {
  return static_cast<QAbstractProxyModel*>(ptr);
}

QIdentityProxyModel* qt_core_c_QIdentityProxyModel_G_static_cast_QIdentityProxyModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr) {
  return static_cast<QIdentityProxyModel*>(ptr);
}

QIdentityProxyModel* qt_core_c_QIdentityProxyModel_G_static_cast_QIdentityProxyModel_ptr_QAbstractProxyModel(QAbstractProxyModel* ptr) {
  return static_cast<QIdentityProxyModel*>(ptr);
}

QIdentityProxyModel* qt_core_c_QIdentityProxyModel_G_static_cast_QIdentityProxyModel_ptr_QObject(QObject* ptr) {
  return static_cast<QIdentityProxyModel*>(ptr);
}

QObject* qt_core_c_QIdentityProxyModel_G_static_cast_QObject_ptr(QIdentityProxyModel* ptr) {
  return static_cast<QObject*>(ptr);
}

int qt_core_c_QIdentityProxyModel_columnCount_no_args(const QIdentityProxyModel* this_ptr) {
  return this_ptr->columnCount();
}

int qt_core_c_QIdentityProxyModel_columnCount_parent(const QIdentityProxyModel* this_ptr, const QModelIndex* parent) {
  return this_ptr->columnCount(*parent);
}

void qt_core_c_QIdentityProxyModel_delete(QIdentityProxyModel* this_ptr) {
  delete this_ptr;
}

bool qt_core_c_QIdentityProxyModel_dropMimeData(QIdentityProxyModel* this_ptr, const QMimeData* data, const Qt::DropAction* action, int row, int column, const QModelIndex* parent) {
  return this_ptr->dropMimeData(data, *action, row, column, *parent);
}

void qt_core_c_QIdentityProxyModel_headerData_to_output_section_orientation(const QIdentityProxyModel* this_ptr, int section, const Qt::Orientation* orientation, QVariant* output) {
  new(output) QVariant(this_ptr->headerData(section, *orientation));
}

void qt_core_c_QIdentityProxyModel_headerData_to_output_section_orientation_role(const QIdentityProxyModel* this_ptr, int section, const Qt::Orientation* orientation, int role, QVariant* output) {
  new(output) QVariant(this_ptr->headerData(section, *orientation, role));
}

void qt_core_c_QIdentityProxyModel_index_to_output_row_column(const QIdentityProxyModel* this_ptr, int row, int column, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->index(row, column));
}

void qt_core_c_QIdentityProxyModel_index_to_output_row_column_parent(const QIdentityProxyModel* this_ptr, int row, int column, const QModelIndex* parent, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->index(row, column, *parent));
}

bool qt_core_c_QIdentityProxyModel_insertColumns_column_count(QIdentityProxyModel* this_ptr, int column, int count) {
  return this_ptr->insertColumns(column, count);
}

bool qt_core_c_QIdentityProxyModel_insertColumns_column_count_parent(QIdentityProxyModel* this_ptr, int column, int count, const QModelIndex* parent) {
  return this_ptr->insertColumns(column, count, *parent);
}

bool qt_core_c_QIdentityProxyModel_insertRows_row_count(QIdentityProxyModel* this_ptr, int row, int count) {
  return this_ptr->insertRows(row, count);
}

bool qt_core_c_QIdentityProxyModel_insertRows_row_count_parent(QIdentityProxyModel* this_ptr, int row, int count, const QModelIndex* parent) {
  return this_ptr->insertRows(row, count, *parent);
}

void qt_core_c_QIdentityProxyModel_mapFromSource_to_output(const QIdentityProxyModel* this_ptr, const QModelIndex* sourceIndex, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->mapFromSource(*sourceIndex));
}

void qt_core_c_QIdentityProxyModel_mapSelectionFromSource_to_output(const QIdentityProxyModel* this_ptr, const QItemSelection* selection, QItemSelection* output) {
  new(output) QItemSelection(this_ptr->mapSelectionFromSource(*selection));
}

void qt_core_c_QIdentityProxyModel_mapSelectionToSource_to_output(const QIdentityProxyModel* this_ptr, const QItemSelection* selection, QItemSelection* output) {
  new(output) QItemSelection(this_ptr->mapSelectionToSource(*selection));
}

void qt_core_c_QIdentityProxyModel_mapToSource_to_output(const QIdentityProxyModel* this_ptr, const QModelIndex* proxyIndex, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->mapToSource(*proxyIndex));
}

const QMetaObject* qt_core_c_QIdentityProxyModel_metaObject(const QIdentityProxyModel* this_ptr) {
  return this_ptr->metaObject();
}

QIdentityProxyModel* qt_core_c_QIdentityProxyModel_new_no_args() {
  return new QIdentityProxyModel();
}

QIdentityProxyModel* qt_core_c_QIdentityProxyModel_new_parent(QObject* parent) {
  return new QIdentityProxyModel(parent);
}

void qt_core_c_QIdentityProxyModel_parent_to_output(const QIdentityProxyModel* this_ptr, const QModelIndex* child, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->parent(*child));
}

bool qt_core_c_QIdentityProxyModel_removeColumns_column_count(QIdentityProxyModel* this_ptr, int column, int count) {
  return this_ptr->removeColumns(column, count);
}

bool qt_core_c_QIdentityProxyModel_removeColumns_column_count_parent(QIdentityProxyModel* this_ptr, int column, int count, const QModelIndex* parent) {
  return this_ptr->removeColumns(column, count, *parent);
}

bool qt_core_c_QIdentityProxyModel_removeRows_row_count(QIdentityProxyModel* this_ptr, int row, int count) {
  return this_ptr->removeRows(row, count);
}

bool qt_core_c_QIdentityProxyModel_removeRows_row_count_parent(QIdentityProxyModel* this_ptr, int row, int count, const QModelIndex* parent) {
  return this_ptr->removeRows(row, count, *parent);
}

int qt_core_c_QIdentityProxyModel_rowCount_no_args(const QIdentityProxyModel* this_ptr) {
  return this_ptr->rowCount();
}

int qt_core_c_QIdentityProxyModel_rowCount_parent(const QIdentityProxyModel* this_ptr, const QModelIndex* parent) {
  return this_ptr->rowCount(*parent);
}

void qt_core_c_QIdentityProxyModel_setSourceModel(QIdentityProxyModel* this_ptr, QAbstractItemModel* sourceModel) {
  this_ptr->setSourceModel(sourceModel);
}

void qt_core_c_QIdentityProxyModel_sibling_to_output(const QIdentityProxyModel* this_ptr, int row, int column, const QModelIndex* idx, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->sibling(row, column, *idx));
}

void qt_core_c_QIdentityProxyModel_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QIdentityProxyModel::trUtf8(s, c, n));
}

void qt_core_c_QIdentityProxyModel_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QIdentityProxyModel::tr(s, c, n));
}

