#include "qt_core_c_QAbstractProxyModel.h"

QAbstractProxyModel* qt_core_c_QAbstractProxyModel_G_dynamic_cast_QAbstractProxyModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr) {
  return dynamic_cast<QAbstractProxyModel*>(ptr);
}

QAbstractProxyModel* qt_core_c_QAbstractProxyModel_G_dynamic_cast_QAbstractProxyModel_ptr_QObject(QObject* ptr) {
  return dynamic_cast<QAbstractProxyModel*>(ptr);
}

QAbstractItemModel* qt_core_c_QAbstractProxyModel_G_static_cast_QAbstractItemModel_ptr(QAbstractProxyModel* ptr) {
  return static_cast<QAbstractItemModel*>(ptr);
}

QAbstractProxyModel* qt_core_c_QAbstractProxyModel_G_static_cast_QAbstractProxyModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr) {
  return static_cast<QAbstractProxyModel*>(ptr);
}

QAbstractProxyModel* qt_core_c_QAbstractProxyModel_G_static_cast_QAbstractProxyModel_ptr_QObject(QObject* ptr) {
  return static_cast<QAbstractProxyModel*>(ptr);
}

QObject* qt_core_c_QAbstractProxyModel_G_static_cast_QObject_ptr(QAbstractProxyModel* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_core_c_QAbstractProxyModel_buddy_to_output(const QAbstractProxyModel* this_ptr, const QModelIndex* index, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->buddy(*index));
}

bool qt_core_c_QAbstractProxyModel_canDropMimeData(const QAbstractProxyModel* this_ptr, const QMimeData* data, const Qt::DropAction* action, int row, int column, const QModelIndex* parent) {
  return this_ptr->canDropMimeData(data, *action, row, column, *parent);
}

bool qt_core_c_QAbstractProxyModel_canFetchMore(const QAbstractProxyModel* this_ptr, const QModelIndex* parent) {
  return this_ptr->canFetchMore(*parent);
}

void qt_core_c_QAbstractProxyModel_data_to_output_proxyIndex(const QAbstractProxyModel* this_ptr, const QModelIndex* proxyIndex, QVariant* output) {
  new(output) QVariant(this_ptr->data(*proxyIndex));
}

void qt_core_c_QAbstractProxyModel_data_to_output_proxyIndex_role(const QAbstractProxyModel* this_ptr, const QModelIndex* proxyIndex, int role, QVariant* output) {
  new(output) QVariant(this_ptr->data(*proxyIndex, role));
}

void qt_core_c_QAbstractProxyModel_delete(QAbstractProxyModel* this_ptr) {
  delete this_ptr;
}

bool qt_core_c_QAbstractProxyModel_dropMimeData(QAbstractProxyModel* this_ptr, const QMimeData* data, const Qt::DropAction* action, int row, int column, const QModelIndex* parent) {
  return this_ptr->dropMimeData(data, *action, row, column, *parent);
}

void qt_core_c_QAbstractProxyModel_fetchMore(QAbstractProxyModel* this_ptr, const QModelIndex* parent) {
  this_ptr->fetchMore(*parent);
}

bool qt_core_c_QAbstractProxyModel_hasChildren_no_args(const QAbstractProxyModel* this_ptr) {
  return this_ptr->hasChildren();
}

bool qt_core_c_QAbstractProxyModel_hasChildren_parent(const QAbstractProxyModel* this_ptr, const QModelIndex* parent) {
  return this_ptr->hasChildren(*parent);
}

void qt_core_c_QAbstractProxyModel_headerData_to_output_section_orientation(const QAbstractProxyModel* this_ptr, int section, const Qt::Orientation* orientation, QVariant* output) {
  new(output) QVariant(this_ptr->headerData(section, *orientation));
}

void qt_core_c_QAbstractProxyModel_headerData_to_output_section_orientation_role(const QAbstractProxyModel* this_ptr, int section, const Qt::Orientation* orientation, int role, QVariant* output) {
  new(output) QVariant(this_ptr->headerData(section, *orientation, role));
}

void qt_core_c_QAbstractProxyModel_itemData_to_output(const QAbstractProxyModel* this_ptr, const QModelIndex* index, QMap< int, QVariant >* output) {
  new(output) QMap< int, QVariant >(this_ptr->itemData(*index));
}

void qt_core_c_QAbstractProxyModel_mapFromSource_to_output(const QAbstractProxyModel* this_ptr, const QModelIndex* sourceIndex, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->mapFromSource(*sourceIndex));
}

void qt_core_c_QAbstractProxyModel_mapSelectionFromSource_to_output(const QAbstractProxyModel* this_ptr, const QItemSelection* selection, QItemSelection* output) {
  new(output) QItemSelection(this_ptr->mapSelectionFromSource(*selection));
}

void qt_core_c_QAbstractProxyModel_mapSelectionToSource_to_output(const QAbstractProxyModel* this_ptr, const QItemSelection* selection, QItemSelection* output) {
  new(output) QItemSelection(this_ptr->mapSelectionToSource(*selection));
}

void qt_core_c_QAbstractProxyModel_mapToSource_to_output(const QAbstractProxyModel* this_ptr, const QModelIndex* proxyIndex, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->mapToSource(*proxyIndex));
}

const QMetaObject* qt_core_c_QAbstractProxyModel_metaObject(const QAbstractProxyModel* this_ptr) {
  return this_ptr->metaObject();
}

QMimeData* qt_core_c_QAbstractProxyModel_mimeData(const QAbstractProxyModel* this_ptr, const QList< QModelIndex >* indexes) {
  return this_ptr->mimeData(*indexes);
}

void qt_core_c_QAbstractProxyModel_mimeTypes_to_output(const QAbstractProxyModel* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->mimeTypes());
}

void qt_core_c_QAbstractProxyModel_revert(QAbstractProxyModel* this_ptr) {
  this_ptr->revert();
}

bool qt_core_c_QAbstractProxyModel_setData_index_value(QAbstractProxyModel* this_ptr, const QModelIndex* index, const QVariant* value) {
  return this_ptr->setData(*index, *value);
}

bool qt_core_c_QAbstractProxyModel_setData_index_value_role(QAbstractProxyModel* this_ptr, const QModelIndex* index, const QVariant* value, int role) {
  return this_ptr->setData(*index, *value, role);
}

bool qt_core_c_QAbstractProxyModel_setHeaderData_section_orientation_value(QAbstractProxyModel* this_ptr, int section, const Qt::Orientation* orientation, const QVariant* value) {
  return this_ptr->setHeaderData(section, *orientation, *value);
}

bool qt_core_c_QAbstractProxyModel_setHeaderData_section_orientation_value_role(QAbstractProxyModel* this_ptr, int section, const Qt::Orientation* orientation, const QVariant* value, int role) {
  return this_ptr->setHeaderData(section, *orientation, *value, role);
}

bool qt_core_c_QAbstractProxyModel_setItemData(QAbstractProxyModel* this_ptr, const QModelIndex* index, const QMap< int, QVariant >* roles) {
  return this_ptr->setItemData(*index, *roles);
}

void qt_core_c_QAbstractProxyModel_setSourceModel(QAbstractProxyModel* this_ptr, QAbstractItemModel* sourceModel) {
  this_ptr->setSourceModel(sourceModel);
}

void qt_core_c_QAbstractProxyModel_sibling_to_output(const QAbstractProxyModel* this_ptr, int row, int column, const QModelIndex* idx, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->sibling(row, column, *idx));
}

void qt_core_c_QAbstractProxyModel_sort_column(QAbstractProxyModel* this_ptr, int column) {
  this_ptr->sort(column);
}

void qt_core_c_QAbstractProxyModel_sort_column_order(QAbstractProxyModel* this_ptr, int column, const Qt::SortOrder* order) {
  this_ptr->sort(column, *order);
}

QAbstractItemModel* qt_core_c_QAbstractProxyModel_sourceModel(const QAbstractProxyModel* this_ptr) {
  return this_ptr->sourceModel();
}

void qt_core_c_QAbstractProxyModel_span_to_output(const QAbstractProxyModel* this_ptr, const QModelIndex* index, QSize* output) {
  new(output) QSize(this_ptr->span(*index));
}

bool qt_core_c_QAbstractProxyModel_submit(QAbstractProxyModel* this_ptr) {
  return this_ptr->submit();
}

void qt_core_c_QAbstractProxyModel_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractProxyModel::trUtf8(s, c, n));
}

void qt_core_c_QAbstractProxyModel_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractProxyModel::tr(s, c, n));
}

