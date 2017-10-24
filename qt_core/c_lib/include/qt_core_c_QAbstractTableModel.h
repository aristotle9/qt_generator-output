#ifndef QT_CORE_C_QABSTRACTTABLEMODEL_H
#define QT_CORE_C_QABSTRACTTABLEMODEL_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QAbstractTableModel* qt_core_c_QAbstractTableModel_G_dynamic_cast_QAbstractTableModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr);
QT_CORE_C_EXPORT QAbstractTableModel* qt_core_c_QAbstractTableModel_G_dynamic_cast_QAbstractTableModel_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QAbstractItemModel* qt_core_c_QAbstractTableModel_G_static_cast_QAbstractItemModel_ptr(QAbstractTableModel* ptr);
QT_CORE_C_EXPORT QAbstractTableModel* qt_core_c_QAbstractTableModel_G_static_cast_QAbstractTableModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr);
QT_CORE_C_EXPORT QAbstractTableModel* qt_core_c_QAbstractTableModel_G_static_cast_QAbstractTableModel_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QAbstractTableModel_G_static_cast_QObject_ptr(QAbstractTableModel* ptr);
QT_CORE_C_EXPORT void qt_core_c_QAbstractTableModel_delete(QAbstractTableModel* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractTableModel_dropMimeData(QAbstractTableModel* this_ptr, const QMimeData* data, const Qt::DropAction* action, int row, int column, const QModelIndex* parent);
QT_CORE_C_EXPORT void qt_core_c_QAbstractTableModel_index_to_output_row_column(const QAbstractTableModel* this_ptr, int row, int column, QModelIndex* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractTableModel_index_to_output_row_column_parent(const QAbstractTableModel* this_ptr, int row, int column, const QModelIndex* parent, QModelIndex* output);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QAbstractTableModel_metaObject(const QAbstractTableModel* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAbstractTableModel_sibling_to_output(const QAbstractTableModel* this_ptr, int row, int column, const QModelIndex* idx, QModelIndex* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractTableModel_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractTableModel_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QABSTRACTTABLEMODEL_H
