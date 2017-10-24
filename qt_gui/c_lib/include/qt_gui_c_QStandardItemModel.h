#ifndef QT_GUI_C_QSTANDARDITEMMODEL_H
#define QT_GUI_C_QSTANDARDITEMMODEL_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QDataStream* qt_gui_c_QStandardItemModel_G_operator_shl(QDataStream* out, const QStandardItem* item);
QT_GUI_C_EXPORT QDataStream* qt_gui_c_QStandardItemModel_G_operator_shr(QDataStream* in, QStandardItem* item);
QT_GUI_C_EXPORT QAbstractItemModel* qt_gui_c_QStandardItemModel_G_static_cast_QAbstractItemModel_ptr(QStandardItemModel* ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QStandardItemModel_G_static_cast_QObject_ptr(QStandardItemModel* ptr);
QT_GUI_C_EXPORT QStandardItemModel* qt_gui_c_QStandardItemModel_G_static_cast_QStandardItemModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr);
QT_GUI_C_EXPORT QStandardItemModel* qt_gui_c_QStandardItemModel_G_static_cast_QStandardItemModel_ptr_QObject(QObject* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_appendColumn(QStandardItemModel* this_ptr, const QList< QStandardItem* >* items);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_appendRow_item(QStandardItemModel* this_ptr, QStandardItem* item);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_appendRow_items(QStandardItemModel* this_ptr, const QList< QStandardItem* >* items);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_clear(QStandardItemModel* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QStandardItemModel_columnCount_no_args(const QStandardItemModel* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QStandardItemModel_columnCount_parent(const QStandardItemModel* this_ptr, const QModelIndex* parent);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_data_to_output_index(const QStandardItemModel* this_ptr, const QModelIndex* index, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_data_to_output_index_role(const QStandardItemModel* this_ptr, const QModelIndex* index, int role, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_delete(QStandardItemModel* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItemModel_dropMimeData(QStandardItemModel* this_ptr, const QMimeData* data, const Qt::DropAction* action, int row, int column, const QModelIndex* parent);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItemModel_hasChildren_no_args(const QStandardItemModel* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItemModel_hasChildren_parent(const QStandardItemModel* this_ptr, const QModelIndex* parent);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_headerData_to_output_section_orientation(const QStandardItemModel* this_ptr, int section, const Qt::Orientation* orientation, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_headerData_to_output_section_orientation_role(const QStandardItemModel* this_ptr, int section, const Qt::Orientation* orientation, int role, QVariant* output);
QT_GUI_C_EXPORT QStandardItem* qt_gui_c_QStandardItemModel_horizontalHeaderItem(const QStandardItemModel* this_ptr, int column);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_indexFromItem_to_output(const QStandardItemModel* this_ptr, const QStandardItem* item, QModelIndex* output);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_index_to_output_row_column(const QStandardItemModel* this_ptr, int row, int column, QModelIndex* output);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_index_to_output_row_column_parent(const QStandardItemModel* this_ptr, int row, int column, const QModelIndex* parent, QModelIndex* output);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItemModel_insertColumn_column(QStandardItemModel* this_ptr, int column);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_insertColumn_column_items(QStandardItemModel* this_ptr, int column, const QList< QStandardItem* >* items);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItemModel_insertColumn_column_parent(QStandardItemModel* this_ptr, int column, const QModelIndex* parent);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItemModel_insertColumns_column_count(QStandardItemModel* this_ptr, int column, int count);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItemModel_insertColumns_column_count_parent(QStandardItemModel* this_ptr, int column, int count, const QModelIndex* parent);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItemModel_insertRow_row(QStandardItemModel* this_ptr, int row);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_insertRow_row_item(QStandardItemModel* this_ptr, int row, QStandardItem* item);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_insertRow_row_items(QStandardItemModel* this_ptr, int row, const QList< QStandardItem* >* items);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItemModel_insertRow_row_parent(QStandardItemModel* this_ptr, int row, const QModelIndex* parent);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItemModel_insertRows_row_count(QStandardItemModel* this_ptr, int row, int count);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItemModel_insertRows_row_count_parent(QStandardItemModel* this_ptr, int row, int count, const QModelIndex* parent);
QT_GUI_C_EXPORT QStandardItem* qt_gui_c_QStandardItemModel_invisibleRootItem(const QStandardItemModel* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_itemData_to_output(const QStandardItemModel* this_ptr, const QModelIndex* index, QMap< int, QVariant >* output);
QT_GUI_C_EXPORT QStandardItem* qt_gui_c_QStandardItemModel_itemFromIndex(const QStandardItemModel* this_ptr, const QModelIndex* index);
QT_GUI_C_EXPORT const QStandardItem* qt_gui_c_QStandardItemModel_itemPrototype(const QStandardItemModel* this_ptr);
QT_GUI_C_EXPORT QStandardItem* qt_gui_c_QStandardItemModel_item_row(const QStandardItemModel* this_ptr, int row);
QT_GUI_C_EXPORT QStandardItem* qt_gui_c_QStandardItemModel_item_row_column(const QStandardItemModel* this_ptr, int row, int column);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QStandardItemModel_metaObject(const QStandardItemModel* this_ptr);
QT_GUI_C_EXPORT QMimeData* qt_gui_c_QStandardItemModel_mimeData(const QStandardItemModel* this_ptr, const QList< QModelIndex >* indexes);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_mimeTypes_to_output(const QStandardItemModel* this_ptr, QStringList* output);
QT_GUI_C_EXPORT QStandardItemModel* qt_gui_c_QStandardItemModel_new_no_args();
QT_GUI_C_EXPORT QStandardItemModel* qt_gui_c_QStandardItemModel_new_parent(QObject* parent);
QT_GUI_C_EXPORT QStandardItemModel* qt_gui_c_QStandardItemModel_new_rows_columns(int rows, int columns);
QT_GUI_C_EXPORT QStandardItemModel* qt_gui_c_QStandardItemModel_new_rows_columns_parent(int rows, int columns, QObject* parent);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_parent_to_output(const QStandardItemModel* this_ptr, const QModelIndex* child, QModelIndex* output);
QT_GUI_C_EXPORT int qt_gui_c_QStandardItemModel_qt_metacall(QStandardItemModel* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QStandardItemModel_qt_metacast(QStandardItemModel* this_ptr, const char* arg1);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItemModel_removeColumns_column_count(QStandardItemModel* this_ptr, int column, int count);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItemModel_removeColumns_column_count_parent(QStandardItemModel* this_ptr, int column, int count, const QModelIndex* parent);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItemModel_removeRows_row_count(QStandardItemModel* this_ptr, int row, int count);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItemModel_removeRows_row_count_parent(QStandardItemModel* this_ptr, int row, int count, const QModelIndex* parent);
QT_GUI_C_EXPORT int qt_gui_c_QStandardItemModel_rowCount_no_args(const QStandardItemModel* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QStandardItemModel_rowCount_parent(const QStandardItemModel* this_ptr, const QModelIndex* parent);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_setColumnCount(QStandardItemModel* this_ptr, int columns);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItemModel_setData_index_value(QStandardItemModel* this_ptr, const QModelIndex* index, const QVariant* value);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItemModel_setData_index_value_role(QStandardItemModel* this_ptr, const QModelIndex* index, const QVariant* value, int role);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItemModel_setHeaderData_section_orientation_value(QStandardItemModel* this_ptr, int section, const Qt::Orientation* orientation, const QVariant* value);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItemModel_setHeaderData_section_orientation_value_role(QStandardItemModel* this_ptr, int section, const Qt::Orientation* orientation, const QVariant* value, int role);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_setHorizontalHeaderItem(QStandardItemModel* this_ptr, int column, QStandardItem* item);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_setHorizontalHeaderLabels(QStandardItemModel* this_ptr, const QStringList* labels);
QT_GUI_C_EXPORT bool qt_gui_c_QStandardItemModel_setItemData(QStandardItemModel* this_ptr, const QModelIndex* index, const QMap< int, QVariant >* roles);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_setItemPrototype(QStandardItemModel* this_ptr, const QStandardItem* item);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_setItemRoleNames(QStandardItemModel* this_ptr, const QHash< int, QByteArray >* roleNames);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_setItem_row_column_item(QStandardItemModel* this_ptr, int row, int column, QStandardItem* item);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_setItem_row_item(QStandardItemModel* this_ptr, int row, QStandardItem* item);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_setRowCount(QStandardItemModel* this_ptr, int rows);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_setSortRole(QStandardItemModel* this_ptr, int role);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_setVerticalHeaderItem(QStandardItemModel* this_ptr, int row, QStandardItem* item);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_setVerticalHeaderLabels(QStandardItemModel* this_ptr, const QStringList* labels);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_sibling_to_output(const QStandardItemModel* this_ptr, int row, int column, const QModelIndex* idx, QModelIndex* output);
QT_GUI_C_EXPORT int qt_gui_c_QStandardItemModel_sortRole(const QStandardItemModel* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_sort_column(QStandardItemModel* this_ptr, int column);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_sort_column_order(QStandardItemModel* this_ptr, int column, const Qt::SortOrder* order);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_takeColumn_to_output(QStandardItemModel* this_ptr, int column, QList< QStandardItem* >* output);
QT_GUI_C_EXPORT QStandardItem* qt_gui_c_QStandardItemModel_takeHorizontalHeaderItem(QStandardItemModel* this_ptr, int column);
QT_GUI_C_EXPORT QStandardItem* qt_gui_c_QStandardItemModel_takeItem_row(QStandardItemModel* this_ptr, int row);
QT_GUI_C_EXPORT QStandardItem* qt_gui_c_QStandardItemModel_takeItem_row_column(QStandardItemModel* this_ptr, int row, int column);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_takeRow_to_output(QStandardItemModel* this_ptr, int row, QList< QStandardItem* >* output);
QT_GUI_C_EXPORT QStandardItem* qt_gui_c_QStandardItemModel_takeVerticalHeaderItem(QStandardItemModel* this_ptr, int row);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QStandardItemModel_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT QStandardItem* qt_gui_c_QStandardItemModel_verticalHeaderItem(const QStandardItemModel* this_ptr, int row);

} // extern "C"

#endif // QT_GUI_C_QSTANDARDITEMMODEL_H
