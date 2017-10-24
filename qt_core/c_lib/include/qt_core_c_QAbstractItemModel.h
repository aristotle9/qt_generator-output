#ifndef QT_CORE_C_QABSTRACTITEMMODEL_H
#define QT_CORE_C_QABSTRACTITEMMODEL_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QAbstractItemModel* qt_core_c_QAbstractItemModel_G_dynamic_cast_QAbstractItemModel_ptr(QObject* ptr);
QT_CORE_C_EXPORT void qt_core_c_QAbstractItemModel_G_operator_shl_to_output(const QDebug* arg1, const QModelIndex* arg2, QDebug* output);
QT_CORE_C_EXPORT unsigned int qt_core_c_QAbstractItemModel_G_qHash_QModelIndex(const QModelIndex* index);
QT_CORE_C_EXPORT unsigned int qt_core_c_QAbstractItemModel_G_qHash_QPersistentModelIndex(const QPersistentModelIndex* index);
QT_CORE_C_EXPORT unsigned int qt_core_c_QAbstractItemModel_G_qHash_QPersistentModelIndex_unsigned_int(const QPersistentModelIndex* index, unsigned int seed);
QT_CORE_C_EXPORT QAbstractItemModel* qt_core_c_QAbstractItemModel_G_static_cast_QAbstractItemModel_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QAbstractItemModel_G_static_cast_QObject_ptr(QAbstractItemModel* ptr);
QT_CORE_C_EXPORT void qt_core_c_QAbstractItemModel_G_swap(QPersistentModelIndex* value1, QPersistentModelIndex* value2);
QT_CORE_C_EXPORT void qt_core_c_QAbstractItemModel_buddy_to_output(const QAbstractItemModel* this_ptr, const QModelIndex* index, QModelIndex* output);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_canDropMimeData(const QAbstractItemModel* this_ptr, const QMimeData* data, const Qt::DropAction* action, int row, int column, const QModelIndex* parent);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_canFetchMore(const QAbstractItemModel* this_ptr, const QModelIndex* parent);
QT_CORE_C_EXPORT int qt_core_c_QAbstractItemModel_columnCount_no_args(const QAbstractItemModel* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QAbstractItemModel_columnCount_parent(const QAbstractItemModel* this_ptr, const QModelIndex* parent);
QT_CORE_C_EXPORT void qt_core_c_QAbstractItemModel_data_to_output_index(const QAbstractItemModel* this_ptr, const QModelIndex* index, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractItemModel_data_to_output_index_role(const QAbstractItemModel* this_ptr, const QModelIndex* index, int role, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractItemModel_delete(QAbstractItemModel* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_dropMimeData(QAbstractItemModel* this_ptr, const QMimeData* data, const Qt::DropAction* action, int row, int column, const QModelIndex* parent);
QT_CORE_C_EXPORT void qt_core_c_QAbstractItemModel_fetchMore(QAbstractItemModel* this_ptr, const QModelIndex* parent);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_hasChildren_no_args(const QAbstractItemModel* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_hasChildren_parent(const QAbstractItemModel* this_ptr, const QModelIndex* parent);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_hasIndex_row_column(const QAbstractItemModel* this_ptr, int row, int column);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_hasIndex_row_column_parent(const QAbstractItemModel* this_ptr, int row, int column, const QModelIndex* parent);
QT_CORE_C_EXPORT void qt_core_c_QAbstractItemModel_headerData_to_output_section_orientation(const QAbstractItemModel* this_ptr, int section, const Qt::Orientation* orientation, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractItemModel_headerData_to_output_section_orientation_role(const QAbstractItemModel* this_ptr, int section, const Qt::Orientation* orientation, int role, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractItemModel_index_to_output_row_column(const QAbstractItemModel* this_ptr, int row, int column, QModelIndex* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractItemModel_index_to_output_row_column_parent(const QAbstractItemModel* this_ptr, int row, int column, const QModelIndex* parent, QModelIndex* output);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_insertColumn_column(QAbstractItemModel* this_ptr, int column);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_insertColumn_column_parent(QAbstractItemModel* this_ptr, int column, const QModelIndex* parent);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_insertColumns_column_count(QAbstractItemModel* this_ptr, int column, int count);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_insertColumns_column_count_parent(QAbstractItemModel* this_ptr, int column, int count, const QModelIndex* parent);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_insertRow_row(QAbstractItemModel* this_ptr, int row);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_insertRow_row_parent(QAbstractItemModel* this_ptr, int row, const QModelIndex* parent);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_insertRows_row_count(QAbstractItemModel* this_ptr, int row, int count);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_insertRows_row_count_parent(QAbstractItemModel* this_ptr, int row, int count, const QModelIndex* parent);
QT_CORE_C_EXPORT void qt_core_c_QAbstractItemModel_itemData_to_output(const QAbstractItemModel* this_ptr, const QModelIndex* index, QMap< int, QVariant >* output);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QAbstractItemModel_metaObject(const QAbstractItemModel* this_ptr);
QT_CORE_C_EXPORT QMimeData* qt_core_c_QAbstractItemModel_mimeData(const QAbstractItemModel* this_ptr, const QList< QModelIndex >* indexes);
QT_CORE_C_EXPORT void qt_core_c_QAbstractItemModel_mimeTypes_to_output(const QAbstractItemModel* this_ptr, QStringList* output);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_moveColumn(QAbstractItemModel* this_ptr, const QModelIndex* sourceParent, int sourceColumn, const QModelIndex* destinationParent, int destinationChild);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_moveColumns(QAbstractItemModel* this_ptr, const QModelIndex* sourceParent, int sourceColumn, int count, const QModelIndex* destinationParent, int destinationChild);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_moveRow(QAbstractItemModel* this_ptr, const QModelIndex* sourceParent, int sourceRow, const QModelIndex* destinationParent, int destinationChild);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_moveRows(QAbstractItemModel* this_ptr, const QModelIndex* sourceParent, int sourceRow, int count, const QModelIndex* destinationParent, int destinationChild);
QT_CORE_C_EXPORT void qt_core_c_QAbstractItemModel_parent_to_output(const QAbstractItemModel* this_ptr, const QModelIndex* child, QModelIndex* output);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_removeColumn_column(QAbstractItemModel* this_ptr, int column);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_removeColumn_column_parent(QAbstractItemModel* this_ptr, int column, const QModelIndex* parent);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_removeColumns_column_count(QAbstractItemModel* this_ptr, int column, int count);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_removeColumns_column_count_parent(QAbstractItemModel* this_ptr, int column, int count, const QModelIndex* parent);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_removeRow_row(QAbstractItemModel* this_ptr, int row);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_removeRow_row_parent(QAbstractItemModel* this_ptr, int row, const QModelIndex* parent);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_removeRows_row_count(QAbstractItemModel* this_ptr, int row, int count);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_removeRows_row_count_parent(QAbstractItemModel* this_ptr, int row, int count, const QModelIndex* parent);
QT_CORE_C_EXPORT void qt_core_c_QAbstractItemModel_revert(QAbstractItemModel* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAbstractItemModel_roleNames_to_output(const QAbstractItemModel* this_ptr, QHash< int, QByteArray >* output);
QT_CORE_C_EXPORT int qt_core_c_QAbstractItemModel_rowCount_no_args(const QAbstractItemModel* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QAbstractItemModel_rowCount_parent(const QAbstractItemModel* this_ptr, const QModelIndex* parent);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_setData_index_value(QAbstractItemModel* this_ptr, const QModelIndex* index, const QVariant* value);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_setData_index_value_role(QAbstractItemModel* this_ptr, const QModelIndex* index, const QVariant* value, int role);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_setHeaderData_section_orientation_value(QAbstractItemModel* this_ptr, int section, const Qt::Orientation* orientation, const QVariant* value);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_setHeaderData_section_orientation_value_role(QAbstractItemModel* this_ptr, int section, const Qt::Orientation* orientation, const QVariant* value, int role);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_setItemData(QAbstractItemModel* this_ptr, const QModelIndex* index, const QMap< int, QVariant >* roles);
QT_CORE_C_EXPORT void qt_core_c_QAbstractItemModel_sibling_to_output(const QAbstractItemModel* this_ptr, int row, int column, const QModelIndex* idx, QModelIndex* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractItemModel_sort_column(QAbstractItemModel* this_ptr, int column);
QT_CORE_C_EXPORT void qt_core_c_QAbstractItemModel_sort_column_order(QAbstractItemModel* this_ptr, int column, const Qt::SortOrder* order);
QT_CORE_C_EXPORT void qt_core_c_QAbstractItemModel_span_to_output(const QAbstractItemModel* this_ptr, const QModelIndex* index, QSize* output);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractItemModel_submit(QAbstractItemModel* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAbstractItemModel_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractItemModel_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QABSTRACTITEMMODEL_H
