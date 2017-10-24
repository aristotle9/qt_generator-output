#include "qt_core_c_QAbstractItemModel.h"

QAbstractItemModel* qt_core_c_QAbstractItemModel_G_dynamic_cast_QAbstractItemModel_ptr(QObject* ptr) {
  return dynamic_cast<QAbstractItemModel*>(ptr);
}

void qt_core_c_QAbstractItemModel_G_operator_shl_to_output(const QDebug* arg1, const QModelIndex* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

unsigned int qt_core_c_QAbstractItemModel_G_qHash_QModelIndex(const QModelIndex* index) {
  return qHash(*index);
}

unsigned int qt_core_c_QAbstractItemModel_G_qHash_QPersistentModelIndex(const QPersistentModelIndex* index) {
  return qHash(*index);
}

unsigned int qt_core_c_QAbstractItemModel_G_qHash_QPersistentModelIndex_unsigned_int(const QPersistentModelIndex* index, unsigned int seed) {
  return qHash(*index, seed);
}

QAbstractItemModel* qt_core_c_QAbstractItemModel_G_static_cast_QAbstractItemModel_ptr(QObject* ptr) {
  return static_cast<QAbstractItemModel*>(ptr);
}

QObject* qt_core_c_QAbstractItemModel_G_static_cast_QObject_ptr(QAbstractItemModel* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_core_c_QAbstractItemModel_G_swap(QPersistentModelIndex* value1, QPersistentModelIndex* value2) {
  swap(*value1, *value2);
}

void qt_core_c_QAbstractItemModel_buddy_to_output(const QAbstractItemModel* this_ptr, const QModelIndex* index, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->buddy(*index));
}

bool qt_core_c_QAbstractItemModel_canDropMimeData(const QAbstractItemModel* this_ptr, const QMimeData* data, const Qt::DropAction* action, int row, int column, const QModelIndex* parent) {
  return this_ptr->canDropMimeData(data, *action, row, column, *parent);
}

bool qt_core_c_QAbstractItemModel_canFetchMore(const QAbstractItemModel* this_ptr, const QModelIndex* parent) {
  return this_ptr->canFetchMore(*parent);
}

int qt_core_c_QAbstractItemModel_columnCount_no_args(const QAbstractItemModel* this_ptr) {
  return this_ptr->columnCount();
}

int qt_core_c_QAbstractItemModel_columnCount_parent(const QAbstractItemModel* this_ptr, const QModelIndex* parent) {
  return this_ptr->columnCount(*parent);
}

void qt_core_c_QAbstractItemModel_data_to_output_index(const QAbstractItemModel* this_ptr, const QModelIndex* index, QVariant* output) {
  new(output) QVariant(this_ptr->data(*index));
}

void qt_core_c_QAbstractItemModel_data_to_output_index_role(const QAbstractItemModel* this_ptr, const QModelIndex* index, int role, QVariant* output) {
  new(output) QVariant(this_ptr->data(*index, role));
}

void qt_core_c_QAbstractItemModel_delete(QAbstractItemModel* this_ptr) {
  delete this_ptr;
}

bool qt_core_c_QAbstractItemModel_dropMimeData(QAbstractItemModel* this_ptr, const QMimeData* data, const Qt::DropAction* action, int row, int column, const QModelIndex* parent) {
  return this_ptr->dropMimeData(data, *action, row, column, *parent);
}

void qt_core_c_QAbstractItemModel_fetchMore(QAbstractItemModel* this_ptr, const QModelIndex* parent) {
  this_ptr->fetchMore(*parent);
}

bool qt_core_c_QAbstractItemModel_hasChildren_no_args(const QAbstractItemModel* this_ptr) {
  return this_ptr->hasChildren();
}

bool qt_core_c_QAbstractItemModel_hasChildren_parent(const QAbstractItemModel* this_ptr, const QModelIndex* parent) {
  return this_ptr->hasChildren(*parent);
}

bool qt_core_c_QAbstractItemModel_hasIndex_row_column(const QAbstractItemModel* this_ptr, int row, int column) {
  return this_ptr->hasIndex(row, column);
}

bool qt_core_c_QAbstractItemModel_hasIndex_row_column_parent(const QAbstractItemModel* this_ptr, int row, int column, const QModelIndex* parent) {
  return this_ptr->hasIndex(row, column, *parent);
}

void qt_core_c_QAbstractItemModel_headerData_to_output_section_orientation(const QAbstractItemModel* this_ptr, int section, const Qt::Orientation* orientation, QVariant* output) {
  new(output) QVariant(this_ptr->headerData(section, *orientation));
}

void qt_core_c_QAbstractItemModel_headerData_to_output_section_orientation_role(const QAbstractItemModel* this_ptr, int section, const Qt::Orientation* orientation, int role, QVariant* output) {
  new(output) QVariant(this_ptr->headerData(section, *orientation, role));
}

void qt_core_c_QAbstractItemModel_index_to_output_row_column(const QAbstractItemModel* this_ptr, int row, int column, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->index(row, column));
}

void qt_core_c_QAbstractItemModel_index_to_output_row_column_parent(const QAbstractItemModel* this_ptr, int row, int column, const QModelIndex* parent, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->index(row, column, *parent));
}

bool qt_core_c_QAbstractItemModel_insertColumn_column(QAbstractItemModel* this_ptr, int column) {
  return this_ptr->insertColumn(column);
}

bool qt_core_c_QAbstractItemModel_insertColumn_column_parent(QAbstractItemModel* this_ptr, int column, const QModelIndex* parent) {
  return this_ptr->insertColumn(column, *parent);
}

bool qt_core_c_QAbstractItemModel_insertColumns_column_count(QAbstractItemModel* this_ptr, int column, int count) {
  return this_ptr->insertColumns(column, count);
}

bool qt_core_c_QAbstractItemModel_insertColumns_column_count_parent(QAbstractItemModel* this_ptr, int column, int count, const QModelIndex* parent) {
  return this_ptr->insertColumns(column, count, *parent);
}

bool qt_core_c_QAbstractItemModel_insertRow_row(QAbstractItemModel* this_ptr, int row) {
  return this_ptr->insertRow(row);
}

bool qt_core_c_QAbstractItemModel_insertRow_row_parent(QAbstractItemModel* this_ptr, int row, const QModelIndex* parent) {
  return this_ptr->insertRow(row, *parent);
}

bool qt_core_c_QAbstractItemModel_insertRows_row_count(QAbstractItemModel* this_ptr, int row, int count) {
  return this_ptr->insertRows(row, count);
}

bool qt_core_c_QAbstractItemModel_insertRows_row_count_parent(QAbstractItemModel* this_ptr, int row, int count, const QModelIndex* parent) {
  return this_ptr->insertRows(row, count, *parent);
}

void qt_core_c_QAbstractItemModel_itemData_to_output(const QAbstractItemModel* this_ptr, const QModelIndex* index, QMap< int, QVariant >* output) {
  new(output) QMap< int, QVariant >(this_ptr->itemData(*index));
}

const QMetaObject* qt_core_c_QAbstractItemModel_metaObject(const QAbstractItemModel* this_ptr) {
  return this_ptr->metaObject();
}

QMimeData* qt_core_c_QAbstractItemModel_mimeData(const QAbstractItemModel* this_ptr, const QList< QModelIndex >* indexes) {
  return this_ptr->mimeData(*indexes);
}

void qt_core_c_QAbstractItemModel_mimeTypes_to_output(const QAbstractItemModel* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->mimeTypes());
}

bool qt_core_c_QAbstractItemModel_moveColumn(QAbstractItemModel* this_ptr, const QModelIndex* sourceParent, int sourceColumn, const QModelIndex* destinationParent, int destinationChild) {
  return this_ptr->moveColumn(*sourceParent, sourceColumn, *destinationParent, destinationChild);
}

bool qt_core_c_QAbstractItemModel_moveColumns(QAbstractItemModel* this_ptr, const QModelIndex* sourceParent, int sourceColumn, int count, const QModelIndex* destinationParent, int destinationChild) {
  return this_ptr->moveColumns(*sourceParent, sourceColumn, count, *destinationParent, destinationChild);
}

bool qt_core_c_QAbstractItemModel_moveRow(QAbstractItemModel* this_ptr, const QModelIndex* sourceParent, int sourceRow, const QModelIndex* destinationParent, int destinationChild) {
  return this_ptr->moveRow(*sourceParent, sourceRow, *destinationParent, destinationChild);
}

bool qt_core_c_QAbstractItemModel_moveRows(QAbstractItemModel* this_ptr, const QModelIndex* sourceParent, int sourceRow, int count, const QModelIndex* destinationParent, int destinationChild) {
  return this_ptr->moveRows(*sourceParent, sourceRow, count, *destinationParent, destinationChild);
}

void qt_core_c_QAbstractItemModel_parent_to_output(const QAbstractItemModel* this_ptr, const QModelIndex* child, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->parent(*child));
}

bool qt_core_c_QAbstractItemModel_removeColumn_column(QAbstractItemModel* this_ptr, int column) {
  return this_ptr->removeColumn(column);
}

bool qt_core_c_QAbstractItemModel_removeColumn_column_parent(QAbstractItemModel* this_ptr, int column, const QModelIndex* parent) {
  return this_ptr->removeColumn(column, *parent);
}

bool qt_core_c_QAbstractItemModel_removeColumns_column_count(QAbstractItemModel* this_ptr, int column, int count) {
  return this_ptr->removeColumns(column, count);
}

bool qt_core_c_QAbstractItemModel_removeColumns_column_count_parent(QAbstractItemModel* this_ptr, int column, int count, const QModelIndex* parent) {
  return this_ptr->removeColumns(column, count, *parent);
}

bool qt_core_c_QAbstractItemModel_removeRow_row(QAbstractItemModel* this_ptr, int row) {
  return this_ptr->removeRow(row);
}

bool qt_core_c_QAbstractItemModel_removeRow_row_parent(QAbstractItemModel* this_ptr, int row, const QModelIndex* parent) {
  return this_ptr->removeRow(row, *parent);
}

bool qt_core_c_QAbstractItemModel_removeRows_row_count(QAbstractItemModel* this_ptr, int row, int count) {
  return this_ptr->removeRows(row, count);
}

bool qt_core_c_QAbstractItemModel_removeRows_row_count_parent(QAbstractItemModel* this_ptr, int row, int count, const QModelIndex* parent) {
  return this_ptr->removeRows(row, count, *parent);
}

void qt_core_c_QAbstractItemModel_revert(QAbstractItemModel* this_ptr) {
  this_ptr->revert();
}

void qt_core_c_QAbstractItemModel_roleNames_to_output(const QAbstractItemModel* this_ptr, QHash< int, QByteArray >* output) {
  new(output) QHash< int, QByteArray >(this_ptr->roleNames());
}

int qt_core_c_QAbstractItemModel_rowCount_no_args(const QAbstractItemModel* this_ptr) {
  return this_ptr->rowCount();
}

int qt_core_c_QAbstractItemModel_rowCount_parent(const QAbstractItemModel* this_ptr, const QModelIndex* parent) {
  return this_ptr->rowCount(*parent);
}

bool qt_core_c_QAbstractItemModel_setData_index_value(QAbstractItemModel* this_ptr, const QModelIndex* index, const QVariant* value) {
  return this_ptr->setData(*index, *value);
}

bool qt_core_c_QAbstractItemModel_setData_index_value_role(QAbstractItemModel* this_ptr, const QModelIndex* index, const QVariant* value, int role) {
  return this_ptr->setData(*index, *value, role);
}

bool qt_core_c_QAbstractItemModel_setHeaderData_section_orientation_value(QAbstractItemModel* this_ptr, int section, const Qt::Orientation* orientation, const QVariant* value) {
  return this_ptr->setHeaderData(section, *orientation, *value);
}

bool qt_core_c_QAbstractItemModel_setHeaderData_section_orientation_value_role(QAbstractItemModel* this_ptr, int section, const Qt::Orientation* orientation, const QVariant* value, int role) {
  return this_ptr->setHeaderData(section, *orientation, *value, role);
}

bool qt_core_c_QAbstractItemModel_setItemData(QAbstractItemModel* this_ptr, const QModelIndex* index, const QMap< int, QVariant >* roles) {
  return this_ptr->setItemData(*index, *roles);
}

void qt_core_c_QAbstractItemModel_sibling_to_output(const QAbstractItemModel* this_ptr, int row, int column, const QModelIndex* idx, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->sibling(row, column, *idx));
}

void qt_core_c_QAbstractItemModel_sort_column(QAbstractItemModel* this_ptr, int column) {
  this_ptr->sort(column);
}

void qt_core_c_QAbstractItemModel_sort_column_order(QAbstractItemModel* this_ptr, int column, const Qt::SortOrder* order) {
  this_ptr->sort(column, *order);
}

void qt_core_c_QAbstractItemModel_span_to_output(const QAbstractItemModel* this_ptr, const QModelIndex* index, QSize* output) {
  new(output) QSize(this_ptr->span(*index));
}

bool qt_core_c_QAbstractItemModel_submit(QAbstractItemModel* this_ptr) {
  return this_ptr->submit();
}

void qt_core_c_QAbstractItemModel_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractItemModel::trUtf8(s, c, n));
}

void qt_core_c_QAbstractItemModel_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractItemModel::tr(s, c, n));
}

