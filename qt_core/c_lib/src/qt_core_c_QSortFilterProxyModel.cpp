#include "qt_core_c_QSortFilterProxyModel.h"

QSortFilterProxyModel* qt_core_c_QSortFilterProxyModel_G_dynamic_cast_QSortFilterProxyModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr) {
  return dynamic_cast<QSortFilterProxyModel*>(ptr);
}

QSortFilterProxyModel* qt_core_c_QSortFilterProxyModel_G_dynamic_cast_QSortFilterProxyModel_ptr_QAbstractProxyModel(QAbstractProxyModel* ptr) {
  return dynamic_cast<QSortFilterProxyModel*>(ptr);
}

QSortFilterProxyModel* qt_core_c_QSortFilterProxyModel_G_dynamic_cast_QSortFilterProxyModel_ptr_QObject(QObject* ptr) {
  return dynamic_cast<QSortFilterProxyModel*>(ptr);
}

QAbstractItemModel* qt_core_c_QSortFilterProxyModel_G_static_cast_QAbstractItemModel_ptr(QSortFilterProxyModel* ptr) {
  return static_cast<QAbstractItemModel*>(ptr);
}

QAbstractProxyModel* qt_core_c_QSortFilterProxyModel_G_static_cast_QAbstractProxyModel_ptr(QSortFilterProxyModel* ptr) {
  return static_cast<QAbstractProxyModel*>(ptr);
}

QObject* qt_core_c_QSortFilterProxyModel_G_static_cast_QObject_ptr(QSortFilterProxyModel* ptr) {
  return static_cast<QObject*>(ptr);
}

QSortFilterProxyModel* qt_core_c_QSortFilterProxyModel_G_static_cast_QSortFilterProxyModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr) {
  return static_cast<QSortFilterProxyModel*>(ptr);
}

QSortFilterProxyModel* qt_core_c_QSortFilterProxyModel_G_static_cast_QSortFilterProxyModel_ptr_QAbstractProxyModel(QAbstractProxyModel* ptr) {
  return static_cast<QSortFilterProxyModel*>(ptr);
}

QSortFilterProxyModel* qt_core_c_QSortFilterProxyModel_G_static_cast_QSortFilterProxyModel_ptr_QObject(QObject* ptr) {
  return static_cast<QSortFilterProxyModel*>(ptr);
}

void qt_core_c_QSortFilterProxyModel_buddy_to_output(const QSortFilterProxyModel* this_ptr, const QModelIndex* index, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->buddy(*index));
}

bool qt_core_c_QSortFilterProxyModel_canFetchMore(const QSortFilterProxyModel* this_ptr, const QModelIndex* parent) {
  return this_ptr->canFetchMore(*parent);
}

void qt_core_c_QSortFilterProxyModel_clear(QSortFilterProxyModel* this_ptr) {
  this_ptr->clear();
}

int qt_core_c_QSortFilterProxyModel_columnCount_no_args(const QSortFilterProxyModel* this_ptr) {
  return this_ptr->columnCount();
}

int qt_core_c_QSortFilterProxyModel_columnCount_parent(const QSortFilterProxyModel* this_ptr, const QModelIndex* parent) {
  return this_ptr->columnCount(*parent);
}

void qt_core_c_QSortFilterProxyModel_data_to_output_index(const QSortFilterProxyModel* this_ptr, const QModelIndex* index, QVariant* output) {
  new(output) QVariant(this_ptr->data(*index));
}

void qt_core_c_QSortFilterProxyModel_data_to_output_index_role(const QSortFilterProxyModel* this_ptr, const QModelIndex* index, int role, QVariant* output) {
  new(output) QVariant(this_ptr->data(*index, role));
}

void qt_core_c_QSortFilterProxyModel_delete(QSortFilterProxyModel* this_ptr) {
  delete this_ptr;
}

bool qt_core_c_QSortFilterProxyModel_dropMimeData(QSortFilterProxyModel* this_ptr, const QMimeData* data, const Qt::DropAction* action, int row, int column, const QModelIndex* parent) {
  return this_ptr->dropMimeData(data, *action, row, column, *parent);
}

bool qt_core_c_QSortFilterProxyModel_dynamicSortFilter(const QSortFilterProxyModel* this_ptr) {
  return this_ptr->dynamicSortFilter();
}

void qt_core_c_QSortFilterProxyModel_fetchMore(QSortFilterProxyModel* this_ptr, const QModelIndex* parent) {
  this_ptr->fetchMore(*parent);
}

int qt_core_c_QSortFilterProxyModel_filterKeyColumn(const QSortFilterProxyModel* this_ptr) {
  return this_ptr->filterKeyColumn();
}

void qt_core_c_QSortFilterProxyModel_filterRegExp_to_output(const QSortFilterProxyModel* this_ptr, QRegExp* output) {
  new(output) QRegExp(this_ptr->filterRegExp());
}

int qt_core_c_QSortFilterProxyModel_filterRole(const QSortFilterProxyModel* this_ptr) {
  return this_ptr->filterRole();
}

bool qt_core_c_QSortFilterProxyModel_hasChildren_no_args(const QSortFilterProxyModel* this_ptr) {
  return this_ptr->hasChildren();
}

bool qt_core_c_QSortFilterProxyModel_hasChildren_parent(const QSortFilterProxyModel* this_ptr, const QModelIndex* parent) {
  return this_ptr->hasChildren(*parent);
}

void qt_core_c_QSortFilterProxyModel_headerData_to_output_section_orientation(const QSortFilterProxyModel* this_ptr, int section, const Qt::Orientation* orientation, QVariant* output) {
  new(output) QVariant(this_ptr->headerData(section, *orientation));
}

void qt_core_c_QSortFilterProxyModel_headerData_to_output_section_orientation_role(const QSortFilterProxyModel* this_ptr, int section, const Qt::Orientation* orientation, int role, QVariant* output) {
  new(output) QVariant(this_ptr->headerData(section, *orientation, role));
}

void qt_core_c_QSortFilterProxyModel_index_to_output_row_column(const QSortFilterProxyModel* this_ptr, int row, int column, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->index(row, column));
}

void qt_core_c_QSortFilterProxyModel_index_to_output_row_column_parent(const QSortFilterProxyModel* this_ptr, int row, int column, const QModelIndex* parent, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->index(row, column, *parent));
}

bool qt_core_c_QSortFilterProxyModel_insertColumns_column_count(QSortFilterProxyModel* this_ptr, int column, int count) {
  return this_ptr->insertColumns(column, count);
}

bool qt_core_c_QSortFilterProxyModel_insertColumns_column_count_parent(QSortFilterProxyModel* this_ptr, int column, int count, const QModelIndex* parent) {
  return this_ptr->insertColumns(column, count, *parent);
}

bool qt_core_c_QSortFilterProxyModel_insertRows_row_count(QSortFilterProxyModel* this_ptr, int row, int count) {
  return this_ptr->insertRows(row, count);
}

bool qt_core_c_QSortFilterProxyModel_insertRows_row_count_parent(QSortFilterProxyModel* this_ptr, int row, int count, const QModelIndex* parent) {
  return this_ptr->insertRows(row, count, *parent);
}

void qt_core_c_QSortFilterProxyModel_invalidate(QSortFilterProxyModel* this_ptr) {
  this_ptr->invalidate();
}

bool qt_core_c_QSortFilterProxyModel_isSortLocaleAware(const QSortFilterProxyModel* this_ptr) {
  return this_ptr->isSortLocaleAware();
}

void qt_core_c_QSortFilterProxyModel_mapFromSource_to_output(const QSortFilterProxyModel* this_ptr, const QModelIndex* sourceIndex, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->mapFromSource(*sourceIndex));
}

void qt_core_c_QSortFilterProxyModel_mapSelectionFromSource_to_output(const QSortFilterProxyModel* this_ptr, const QItemSelection* sourceSelection, QItemSelection* output) {
  new(output) QItemSelection(this_ptr->mapSelectionFromSource(*sourceSelection));
}

void qt_core_c_QSortFilterProxyModel_mapSelectionToSource_to_output(const QSortFilterProxyModel* this_ptr, const QItemSelection* proxySelection, QItemSelection* output) {
  new(output) QItemSelection(this_ptr->mapSelectionToSource(*proxySelection));
}

void qt_core_c_QSortFilterProxyModel_mapToSource_to_output(const QSortFilterProxyModel* this_ptr, const QModelIndex* proxyIndex, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->mapToSource(*proxyIndex));
}

const QMetaObject* qt_core_c_QSortFilterProxyModel_metaObject(const QSortFilterProxyModel* this_ptr) {
  return this_ptr->metaObject();
}

QMimeData* qt_core_c_QSortFilterProxyModel_mimeData(const QSortFilterProxyModel* this_ptr, const QList< QModelIndex >* indexes) {
  return this_ptr->mimeData(*indexes);
}

void qt_core_c_QSortFilterProxyModel_mimeTypes_to_output(const QSortFilterProxyModel* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->mimeTypes());
}

QSortFilterProxyModel* qt_core_c_QSortFilterProxyModel_new_no_args() {
  return new QSortFilterProxyModel();
}

QSortFilterProxyModel* qt_core_c_QSortFilterProxyModel_new_parent(QObject* parent) {
  return new QSortFilterProxyModel(parent);
}

void qt_core_c_QSortFilterProxyModel_parent_to_output(const QSortFilterProxyModel* this_ptr, const QModelIndex* child, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->parent(*child));
}

bool qt_core_c_QSortFilterProxyModel_removeColumns_column_count(QSortFilterProxyModel* this_ptr, int column, int count) {
  return this_ptr->removeColumns(column, count);
}

bool qt_core_c_QSortFilterProxyModel_removeColumns_column_count_parent(QSortFilterProxyModel* this_ptr, int column, int count, const QModelIndex* parent) {
  return this_ptr->removeColumns(column, count, *parent);
}

bool qt_core_c_QSortFilterProxyModel_removeRows_row_count(QSortFilterProxyModel* this_ptr, int row, int count) {
  return this_ptr->removeRows(row, count);
}

bool qt_core_c_QSortFilterProxyModel_removeRows_row_count_parent(QSortFilterProxyModel* this_ptr, int row, int count, const QModelIndex* parent) {
  return this_ptr->removeRows(row, count, *parent);
}

int qt_core_c_QSortFilterProxyModel_rowCount_no_args(const QSortFilterProxyModel* this_ptr) {
  return this_ptr->rowCount();
}

int qt_core_c_QSortFilterProxyModel_rowCount_parent(const QSortFilterProxyModel* this_ptr, const QModelIndex* parent) {
  return this_ptr->rowCount(*parent);
}

bool qt_core_c_QSortFilterProxyModel_setData_index_value(QSortFilterProxyModel* this_ptr, const QModelIndex* index, const QVariant* value) {
  return this_ptr->setData(*index, *value);
}

bool qt_core_c_QSortFilterProxyModel_setData_index_value_role(QSortFilterProxyModel* this_ptr, const QModelIndex* index, const QVariant* value, int role) {
  return this_ptr->setData(*index, *value, role);
}

void qt_core_c_QSortFilterProxyModel_setDynamicSortFilter(QSortFilterProxyModel* this_ptr, bool enable) {
  this_ptr->setDynamicSortFilter(enable);
}

void qt_core_c_QSortFilterProxyModel_setFilterCaseSensitivity(QSortFilterProxyModel* this_ptr, const Qt::CaseSensitivity* cs) {
  this_ptr->setFilterCaseSensitivity(*cs);
}

void qt_core_c_QSortFilterProxyModel_setFilterFixedString(QSortFilterProxyModel* this_ptr, const QString* pattern) {
  this_ptr->setFilterFixedString(*pattern);
}

void qt_core_c_QSortFilterProxyModel_setFilterKeyColumn(QSortFilterProxyModel* this_ptr, int column) {
  this_ptr->setFilterKeyColumn(column);
}

void qt_core_c_QSortFilterProxyModel_setFilterRegExp_pattern(QSortFilterProxyModel* this_ptr, const QString* pattern) {
  this_ptr->setFilterRegExp(*pattern);
}

void qt_core_c_QSortFilterProxyModel_setFilterRegExp_regExp(QSortFilterProxyModel* this_ptr, const QRegExp* regExp) {
  this_ptr->setFilterRegExp(*regExp);
}

void qt_core_c_QSortFilterProxyModel_setFilterRole(QSortFilterProxyModel* this_ptr, int role) {
  this_ptr->setFilterRole(role);
}

void qt_core_c_QSortFilterProxyModel_setFilterWildcard(QSortFilterProxyModel* this_ptr, const QString* pattern) {
  this_ptr->setFilterWildcard(*pattern);
}

bool qt_core_c_QSortFilterProxyModel_setHeaderData_section_orientation_value(QSortFilterProxyModel* this_ptr, int section, const Qt::Orientation* orientation, const QVariant* value) {
  return this_ptr->setHeaderData(section, *orientation, *value);
}

bool qt_core_c_QSortFilterProxyModel_setHeaderData_section_orientation_value_role(QSortFilterProxyModel* this_ptr, int section, const Qt::Orientation* orientation, const QVariant* value, int role) {
  return this_ptr->setHeaderData(section, *orientation, *value, role);
}

void qt_core_c_QSortFilterProxyModel_setSortCaseSensitivity(QSortFilterProxyModel* this_ptr, const Qt::CaseSensitivity* cs) {
  this_ptr->setSortCaseSensitivity(*cs);
}

void qt_core_c_QSortFilterProxyModel_setSortLocaleAware(QSortFilterProxyModel* this_ptr, bool on) {
  this_ptr->setSortLocaleAware(on);
}

void qt_core_c_QSortFilterProxyModel_setSortRole(QSortFilterProxyModel* this_ptr, int role) {
  this_ptr->setSortRole(role);
}

void qt_core_c_QSortFilterProxyModel_setSourceModel(QSortFilterProxyModel* this_ptr, QAbstractItemModel* sourceModel) {
  this_ptr->setSourceModel(sourceModel);
}

void qt_core_c_QSortFilterProxyModel_sibling_to_output(const QSortFilterProxyModel* this_ptr, int row, int column, const QModelIndex* idx, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->sibling(row, column, *idx));
}

int qt_core_c_QSortFilterProxyModel_sortColumn(const QSortFilterProxyModel* this_ptr) {
  return this_ptr->sortColumn();
}

int qt_core_c_QSortFilterProxyModel_sortRole(const QSortFilterProxyModel* this_ptr) {
  return this_ptr->sortRole();
}

void qt_core_c_QSortFilterProxyModel_sort_column(QSortFilterProxyModel* this_ptr, int column) {
  this_ptr->sort(column);
}

void qt_core_c_QSortFilterProxyModel_sort_column_order(QSortFilterProxyModel* this_ptr, int column, const Qt::SortOrder* order) {
  this_ptr->sort(column, *order);
}

void qt_core_c_QSortFilterProxyModel_span_to_output(const QSortFilterProxyModel* this_ptr, const QModelIndex* index, QSize* output) {
  new(output) QSize(this_ptr->span(*index));
}

void qt_core_c_QSortFilterProxyModel_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSortFilterProxyModel::trUtf8(s, c, n));
}

void qt_core_c_QSortFilterProxyModel_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSortFilterProxyModel::tr(s, c, n));
}

