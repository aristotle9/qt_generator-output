#include "qt_gui_c_QStandardItemModel.h"

QDataStream* qt_gui_c_QStandardItemModel_G_operator_shl(QDataStream* out, const QStandardItem* item) {
  return &operator<<(*out, *item);
}

QDataStream* qt_gui_c_QStandardItemModel_G_operator_shr(QDataStream* in, QStandardItem* item) {
  return &operator>>(*in, *item);
}

QAbstractItemModel* qt_gui_c_QStandardItemModel_G_static_cast_QAbstractItemModel_ptr(QStandardItemModel* ptr) {
  return static_cast<QAbstractItemModel*>(ptr);
}

QObject* qt_gui_c_QStandardItemModel_G_static_cast_QObject_ptr(QStandardItemModel* ptr) {
  return static_cast<QObject*>(ptr);
}

QStandardItemModel* qt_gui_c_QStandardItemModel_G_static_cast_QStandardItemModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr) {
  return static_cast<QStandardItemModel*>(ptr);
}

QStandardItemModel* qt_gui_c_QStandardItemModel_G_static_cast_QStandardItemModel_ptr_QObject(QObject* ptr) {
  return static_cast<QStandardItemModel*>(ptr);
}

void qt_gui_c_QStandardItemModel_appendColumn(QStandardItemModel* this_ptr, const QList< QStandardItem* >* items) {
  this_ptr->appendColumn(*items);
}

void qt_gui_c_QStandardItemModel_appendRow_item(QStandardItemModel* this_ptr, QStandardItem* item) {
  this_ptr->appendRow(item);
}

void qt_gui_c_QStandardItemModel_appendRow_items(QStandardItemModel* this_ptr, const QList< QStandardItem* >* items) {
  this_ptr->appendRow(*items);
}

void qt_gui_c_QStandardItemModel_clear(QStandardItemModel* this_ptr) {
  this_ptr->clear();
}

int qt_gui_c_QStandardItemModel_columnCount_no_args(const QStandardItemModel* this_ptr) {
  return this_ptr->columnCount();
}

int qt_gui_c_QStandardItemModel_columnCount_parent(const QStandardItemModel* this_ptr, const QModelIndex* parent) {
  return this_ptr->columnCount(*parent);
}

void qt_gui_c_QStandardItemModel_data_to_output_index(const QStandardItemModel* this_ptr, const QModelIndex* index, QVariant* output) {
  new(output) QVariant(this_ptr->data(*index));
}

void qt_gui_c_QStandardItemModel_data_to_output_index_role(const QStandardItemModel* this_ptr, const QModelIndex* index, int role, QVariant* output) {
  new(output) QVariant(this_ptr->data(*index, role));
}

void qt_gui_c_QStandardItemModel_delete(QStandardItemModel* this_ptr) {
  delete this_ptr;
}

bool qt_gui_c_QStandardItemModel_dropMimeData(QStandardItemModel* this_ptr, const QMimeData* data, const Qt::DropAction* action, int row, int column, const QModelIndex* parent) {
  return this_ptr->dropMimeData(data, *action, row, column, *parent);
}

bool qt_gui_c_QStandardItemModel_hasChildren_no_args(const QStandardItemModel* this_ptr) {
  return this_ptr->hasChildren();
}

bool qt_gui_c_QStandardItemModel_hasChildren_parent(const QStandardItemModel* this_ptr, const QModelIndex* parent) {
  return this_ptr->hasChildren(*parent);
}

void qt_gui_c_QStandardItemModel_headerData_to_output_section_orientation(const QStandardItemModel* this_ptr, int section, const Qt::Orientation* orientation, QVariant* output) {
  new(output) QVariant(this_ptr->headerData(section, *orientation));
}

void qt_gui_c_QStandardItemModel_headerData_to_output_section_orientation_role(const QStandardItemModel* this_ptr, int section, const Qt::Orientation* orientation, int role, QVariant* output) {
  new(output) QVariant(this_ptr->headerData(section, *orientation, role));
}

QStandardItem* qt_gui_c_QStandardItemModel_horizontalHeaderItem(const QStandardItemModel* this_ptr, int column) {
  return this_ptr->horizontalHeaderItem(column);
}

void qt_gui_c_QStandardItemModel_indexFromItem_to_output(const QStandardItemModel* this_ptr, const QStandardItem* item, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->indexFromItem(item));
}

void qt_gui_c_QStandardItemModel_index_to_output_row_column(const QStandardItemModel* this_ptr, int row, int column, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->index(row, column));
}

void qt_gui_c_QStandardItemModel_index_to_output_row_column_parent(const QStandardItemModel* this_ptr, int row, int column, const QModelIndex* parent, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->index(row, column, *parent));
}

bool qt_gui_c_QStandardItemModel_insertColumn_column(QStandardItemModel* this_ptr, int column) {
  return this_ptr->insertColumn(column);
}

void qt_gui_c_QStandardItemModel_insertColumn_column_items(QStandardItemModel* this_ptr, int column, const QList< QStandardItem* >* items) {
  this_ptr->insertColumn(column, *items);
}

bool qt_gui_c_QStandardItemModel_insertColumn_column_parent(QStandardItemModel* this_ptr, int column, const QModelIndex* parent) {
  return this_ptr->insertColumn(column, *parent);
}

bool qt_gui_c_QStandardItemModel_insertColumns_column_count(QStandardItemModel* this_ptr, int column, int count) {
  return this_ptr->insertColumns(column, count);
}

bool qt_gui_c_QStandardItemModel_insertColumns_column_count_parent(QStandardItemModel* this_ptr, int column, int count, const QModelIndex* parent) {
  return this_ptr->insertColumns(column, count, *parent);
}

bool qt_gui_c_QStandardItemModel_insertRow_row(QStandardItemModel* this_ptr, int row) {
  return this_ptr->insertRow(row);
}

void qt_gui_c_QStandardItemModel_insertRow_row_item(QStandardItemModel* this_ptr, int row, QStandardItem* item) {
  this_ptr->insertRow(row, item);
}

void qt_gui_c_QStandardItemModel_insertRow_row_items(QStandardItemModel* this_ptr, int row, const QList< QStandardItem* >* items) {
  this_ptr->insertRow(row, *items);
}

bool qt_gui_c_QStandardItemModel_insertRow_row_parent(QStandardItemModel* this_ptr, int row, const QModelIndex* parent) {
  return this_ptr->insertRow(row, *parent);
}

bool qt_gui_c_QStandardItemModel_insertRows_row_count(QStandardItemModel* this_ptr, int row, int count) {
  return this_ptr->insertRows(row, count);
}

bool qt_gui_c_QStandardItemModel_insertRows_row_count_parent(QStandardItemModel* this_ptr, int row, int count, const QModelIndex* parent) {
  return this_ptr->insertRows(row, count, *parent);
}

QStandardItem* qt_gui_c_QStandardItemModel_invisibleRootItem(const QStandardItemModel* this_ptr) {
  return this_ptr->invisibleRootItem();
}

void qt_gui_c_QStandardItemModel_itemData_to_output(const QStandardItemModel* this_ptr, const QModelIndex* index, QMap< int, QVariant >* output) {
  new(output) QMap< int, QVariant >(this_ptr->itemData(*index));
}

QStandardItem* qt_gui_c_QStandardItemModel_itemFromIndex(const QStandardItemModel* this_ptr, const QModelIndex* index) {
  return this_ptr->itemFromIndex(*index);
}

const QStandardItem* qt_gui_c_QStandardItemModel_itemPrototype(const QStandardItemModel* this_ptr) {
  return this_ptr->itemPrototype();
}

QStandardItem* qt_gui_c_QStandardItemModel_item_row(const QStandardItemModel* this_ptr, int row) {
  return this_ptr->item(row);
}

QStandardItem* qt_gui_c_QStandardItemModel_item_row_column(const QStandardItemModel* this_ptr, int row, int column) {
  return this_ptr->item(row, column);
}

const QMetaObject* qt_gui_c_QStandardItemModel_metaObject(const QStandardItemModel* this_ptr) {
  return this_ptr->metaObject();
}

QMimeData* qt_gui_c_QStandardItemModel_mimeData(const QStandardItemModel* this_ptr, const QList< QModelIndex >* indexes) {
  return this_ptr->mimeData(*indexes);
}

void qt_gui_c_QStandardItemModel_mimeTypes_to_output(const QStandardItemModel* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->mimeTypes());
}

QStandardItemModel* qt_gui_c_QStandardItemModel_new_no_args() {
  return new QStandardItemModel();
}

QStandardItemModel* qt_gui_c_QStandardItemModel_new_parent(QObject* parent) {
  return new QStandardItemModel(parent);
}

QStandardItemModel* qt_gui_c_QStandardItemModel_new_rows_columns(int rows, int columns) {
  return new QStandardItemModel(rows, columns);
}

QStandardItemModel* qt_gui_c_QStandardItemModel_new_rows_columns_parent(int rows, int columns, QObject* parent) {
  return new QStandardItemModel(rows, columns, parent);
}

void qt_gui_c_QStandardItemModel_parent_to_output(const QStandardItemModel* this_ptr, const QModelIndex* child, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->parent(*child));
}

int qt_gui_c_QStandardItemModel_qt_metacall(QStandardItemModel* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QStandardItemModel_qt_metacast(QStandardItemModel* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

bool qt_gui_c_QStandardItemModel_removeColumns_column_count(QStandardItemModel* this_ptr, int column, int count) {
  return this_ptr->removeColumns(column, count);
}

bool qt_gui_c_QStandardItemModel_removeColumns_column_count_parent(QStandardItemModel* this_ptr, int column, int count, const QModelIndex* parent) {
  return this_ptr->removeColumns(column, count, *parent);
}

bool qt_gui_c_QStandardItemModel_removeRows_row_count(QStandardItemModel* this_ptr, int row, int count) {
  return this_ptr->removeRows(row, count);
}

bool qt_gui_c_QStandardItemModel_removeRows_row_count_parent(QStandardItemModel* this_ptr, int row, int count, const QModelIndex* parent) {
  return this_ptr->removeRows(row, count, *parent);
}

int qt_gui_c_QStandardItemModel_rowCount_no_args(const QStandardItemModel* this_ptr) {
  return this_ptr->rowCount();
}

int qt_gui_c_QStandardItemModel_rowCount_parent(const QStandardItemModel* this_ptr, const QModelIndex* parent) {
  return this_ptr->rowCount(*parent);
}

void qt_gui_c_QStandardItemModel_setColumnCount(QStandardItemModel* this_ptr, int columns) {
  this_ptr->setColumnCount(columns);
}

bool qt_gui_c_QStandardItemModel_setData_index_value(QStandardItemModel* this_ptr, const QModelIndex* index, const QVariant* value) {
  return this_ptr->setData(*index, *value);
}

bool qt_gui_c_QStandardItemModel_setData_index_value_role(QStandardItemModel* this_ptr, const QModelIndex* index, const QVariant* value, int role) {
  return this_ptr->setData(*index, *value, role);
}

bool qt_gui_c_QStandardItemModel_setHeaderData_section_orientation_value(QStandardItemModel* this_ptr, int section, const Qt::Orientation* orientation, const QVariant* value) {
  return this_ptr->setHeaderData(section, *orientation, *value);
}

bool qt_gui_c_QStandardItemModel_setHeaderData_section_orientation_value_role(QStandardItemModel* this_ptr, int section, const Qt::Orientation* orientation, const QVariant* value, int role) {
  return this_ptr->setHeaderData(section, *orientation, *value, role);
}

void qt_gui_c_QStandardItemModel_setHorizontalHeaderItem(QStandardItemModel* this_ptr, int column, QStandardItem* item) {
  this_ptr->setHorizontalHeaderItem(column, item);
}

void qt_gui_c_QStandardItemModel_setHorizontalHeaderLabels(QStandardItemModel* this_ptr, const QStringList* labels) {
  this_ptr->setHorizontalHeaderLabels(*labels);
}

bool qt_gui_c_QStandardItemModel_setItemData(QStandardItemModel* this_ptr, const QModelIndex* index, const QMap< int, QVariant >* roles) {
  return this_ptr->setItemData(*index, *roles);
}

void qt_gui_c_QStandardItemModel_setItemPrototype(QStandardItemModel* this_ptr, const QStandardItem* item) {
  this_ptr->setItemPrototype(item);
}

void qt_gui_c_QStandardItemModel_setItemRoleNames(QStandardItemModel* this_ptr, const QHash< int, QByteArray >* roleNames) {
  this_ptr->setItemRoleNames(*roleNames);
}

void qt_gui_c_QStandardItemModel_setItem_row_column_item(QStandardItemModel* this_ptr, int row, int column, QStandardItem* item) {
  this_ptr->setItem(row, column, item);
}

void qt_gui_c_QStandardItemModel_setItem_row_item(QStandardItemModel* this_ptr, int row, QStandardItem* item) {
  this_ptr->setItem(row, item);
}

void qt_gui_c_QStandardItemModel_setRowCount(QStandardItemModel* this_ptr, int rows) {
  this_ptr->setRowCount(rows);
}

void qt_gui_c_QStandardItemModel_setSortRole(QStandardItemModel* this_ptr, int role) {
  this_ptr->setSortRole(role);
}

void qt_gui_c_QStandardItemModel_setVerticalHeaderItem(QStandardItemModel* this_ptr, int row, QStandardItem* item) {
  this_ptr->setVerticalHeaderItem(row, item);
}

void qt_gui_c_QStandardItemModel_setVerticalHeaderLabels(QStandardItemModel* this_ptr, const QStringList* labels) {
  this_ptr->setVerticalHeaderLabels(*labels);
}

void qt_gui_c_QStandardItemModel_sibling_to_output(const QStandardItemModel* this_ptr, int row, int column, const QModelIndex* idx, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->sibling(row, column, *idx));
}

int qt_gui_c_QStandardItemModel_sortRole(const QStandardItemModel* this_ptr) {
  return this_ptr->sortRole();
}

void qt_gui_c_QStandardItemModel_sort_column(QStandardItemModel* this_ptr, int column) {
  this_ptr->sort(column);
}

void qt_gui_c_QStandardItemModel_sort_column_order(QStandardItemModel* this_ptr, int column, const Qt::SortOrder* order) {
  this_ptr->sort(column, *order);
}

void qt_gui_c_QStandardItemModel_takeColumn_to_output(QStandardItemModel* this_ptr, int column, QList< QStandardItem* >* output) {
  new(output) QList< QStandardItem* >(this_ptr->takeColumn(column));
}

QStandardItem* qt_gui_c_QStandardItemModel_takeHorizontalHeaderItem(QStandardItemModel* this_ptr, int column) {
  return this_ptr->takeHorizontalHeaderItem(column);
}

QStandardItem* qt_gui_c_QStandardItemModel_takeItem_row(QStandardItemModel* this_ptr, int row) {
  return this_ptr->takeItem(row);
}

QStandardItem* qt_gui_c_QStandardItemModel_takeItem_row_column(QStandardItemModel* this_ptr, int row, int column) {
  return this_ptr->takeItem(row, column);
}

void qt_gui_c_QStandardItemModel_takeRow_to_output(QStandardItemModel* this_ptr, int row, QList< QStandardItem* >* output) {
  new(output) QList< QStandardItem* >(this_ptr->takeRow(row));
}

QStandardItem* qt_gui_c_QStandardItemModel_takeVerticalHeaderItem(QStandardItemModel* this_ptr, int row) {
  return this_ptr->takeVerticalHeaderItem(row);
}

void qt_gui_c_QStandardItemModel_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QStandardItemModel::trUtf8(s, c, n));
}

void qt_gui_c_QStandardItemModel_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QStandardItemModel::tr(s, c, n));
}

QStandardItem* qt_gui_c_QStandardItemModel_verticalHeaderItem(const QStandardItemModel* this_ptr, int row) {
  return this_ptr->verticalHeaderItem(row);
}

