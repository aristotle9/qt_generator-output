#ifndef QT_CORE_C_QSTRINGLISTMODEL_H
#define QT_CORE_C_QSTRINGLISTMODEL_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QStringListModel* qt_core_c_QStringListModel_G_dynamic_cast_QStringListModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr);
QT_CORE_C_EXPORT QStringListModel* qt_core_c_QStringListModel_G_dynamic_cast_QStringListModel_ptr_QAbstractListModel(QAbstractListModel* ptr);
QT_CORE_C_EXPORT QStringListModel* qt_core_c_QStringListModel_G_dynamic_cast_QStringListModel_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QAbstractItemModel* qt_core_c_QStringListModel_G_static_cast_QAbstractItemModel_ptr(QStringListModel* ptr);
QT_CORE_C_EXPORT QAbstractListModel* qt_core_c_QStringListModel_G_static_cast_QAbstractListModel_ptr(QStringListModel* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QStringListModel_G_static_cast_QObject_ptr(QStringListModel* ptr);
QT_CORE_C_EXPORT QStringListModel* qt_core_c_QStringListModel_G_static_cast_QStringListModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr);
QT_CORE_C_EXPORT QStringListModel* qt_core_c_QStringListModel_G_static_cast_QStringListModel_ptr_QAbstractListModel(QAbstractListModel* ptr);
QT_CORE_C_EXPORT QStringListModel* qt_core_c_QStringListModel_G_static_cast_QStringListModel_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT void qt_core_c_QStringListModel_data_to_output_index(const QStringListModel* this_ptr, const QModelIndex* index, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QStringListModel_data_to_output_index_role(const QStringListModel* this_ptr, const QModelIndex* index, int role, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QStringListModel_delete(QStringListModel* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QStringListModel_insertRows_row_count(QStringListModel* this_ptr, int row, int count);
QT_CORE_C_EXPORT bool qt_core_c_QStringListModel_insertRows_row_count_parent(QStringListModel* this_ptr, int row, int count, const QModelIndex* parent);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QStringListModel_metaObject(const QStringListModel* this_ptr);
QT_CORE_C_EXPORT QStringListModel* qt_core_c_QStringListModel_new_no_args();
QT_CORE_C_EXPORT QStringListModel* qt_core_c_QStringListModel_new_parent(QObject* parent);
QT_CORE_C_EXPORT QStringListModel* qt_core_c_QStringListModel_new_strings(const QStringList* strings);
QT_CORE_C_EXPORT QStringListModel* qt_core_c_QStringListModel_new_strings_parent(const QStringList* strings, QObject* parent);
QT_CORE_C_EXPORT bool qt_core_c_QStringListModel_removeRows_row_count(QStringListModel* this_ptr, int row, int count);
QT_CORE_C_EXPORT bool qt_core_c_QStringListModel_removeRows_row_count_parent(QStringListModel* this_ptr, int row, int count, const QModelIndex* parent);
QT_CORE_C_EXPORT int qt_core_c_QStringListModel_rowCount_no_args(const QStringListModel* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QStringListModel_rowCount_parent(const QStringListModel* this_ptr, const QModelIndex* parent);
QT_CORE_C_EXPORT bool qt_core_c_QStringListModel_setData_index_value(QStringListModel* this_ptr, const QModelIndex* index, const QVariant* value);
QT_CORE_C_EXPORT bool qt_core_c_QStringListModel_setData_index_value_role(QStringListModel* this_ptr, const QModelIndex* index, const QVariant* value, int role);
QT_CORE_C_EXPORT void qt_core_c_QStringListModel_setStringList(QStringListModel* this_ptr, const QStringList* strings);
QT_CORE_C_EXPORT void qt_core_c_QStringListModel_sibling_to_output(const QStringListModel* this_ptr, int row, int column, const QModelIndex* idx, QModelIndex* output);
QT_CORE_C_EXPORT void qt_core_c_QStringListModel_sort_column(QStringListModel* this_ptr, int column);
QT_CORE_C_EXPORT void qt_core_c_QStringListModel_sort_column_order(QStringListModel* this_ptr, int column, const Qt::SortOrder* order);
QT_CORE_C_EXPORT void qt_core_c_QStringListModel_stringList_to_output(const QStringListModel* this_ptr, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QStringListModel_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QStringListModel_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QSTRINGLISTMODEL_H
