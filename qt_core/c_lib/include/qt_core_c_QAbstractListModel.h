#ifndef QT_CORE_C_QABSTRACTLISTMODEL_H
#define QT_CORE_C_QABSTRACTLISTMODEL_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QAbstractListModel* qt_core_c_QAbstractListModel_G_dynamic_cast_QAbstractListModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr);
QT_CORE_C_EXPORT QAbstractListModel* qt_core_c_QAbstractListModel_G_dynamic_cast_QAbstractListModel_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QAbstractItemModel* qt_core_c_QAbstractListModel_G_static_cast_QAbstractItemModel_ptr(QAbstractListModel* ptr);
QT_CORE_C_EXPORT QAbstractListModel* qt_core_c_QAbstractListModel_G_static_cast_QAbstractListModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr);
QT_CORE_C_EXPORT QAbstractListModel* qt_core_c_QAbstractListModel_G_static_cast_QAbstractListModel_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QAbstractListModel_G_static_cast_QObject_ptr(QAbstractListModel* ptr);
QT_CORE_C_EXPORT void qt_core_c_QAbstractListModel_delete(QAbstractListModel* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractListModel_dropMimeData(QAbstractListModel* this_ptr, const QMimeData* data, const Qt::DropAction* action, int row, int column, const QModelIndex* parent);
QT_CORE_C_EXPORT void qt_core_c_QAbstractListModel_index_to_output_row(const QAbstractListModel* this_ptr, int row, QModelIndex* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractListModel_index_to_output_row_column(const QAbstractListModel* this_ptr, int row, int column, QModelIndex* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractListModel_index_to_output_row_column_parent(const QAbstractListModel* this_ptr, int row, int column, const QModelIndex* parent, QModelIndex* output);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QAbstractListModel_metaObject(const QAbstractListModel* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAbstractListModel_sibling_to_output(const QAbstractListModel* this_ptr, int row, int column, const QModelIndex* idx, QModelIndex* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractListModel_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractListModel_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QABSTRACTLISTMODEL_H
