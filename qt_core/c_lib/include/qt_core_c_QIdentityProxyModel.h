#ifndef QT_CORE_C_QIDENTITYPROXYMODEL_H
#define QT_CORE_C_QIDENTITYPROXYMODEL_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QIdentityProxyModel* qt_core_c_QIdentityProxyModel_G_dynamic_cast_QIdentityProxyModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr);
QT_CORE_C_EXPORT QIdentityProxyModel* qt_core_c_QIdentityProxyModel_G_dynamic_cast_QIdentityProxyModel_ptr_QAbstractProxyModel(QAbstractProxyModel* ptr);
QT_CORE_C_EXPORT QIdentityProxyModel* qt_core_c_QIdentityProxyModel_G_dynamic_cast_QIdentityProxyModel_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QAbstractItemModel* qt_core_c_QIdentityProxyModel_G_static_cast_QAbstractItemModel_ptr(QIdentityProxyModel* ptr);
QT_CORE_C_EXPORT QAbstractProxyModel* qt_core_c_QIdentityProxyModel_G_static_cast_QAbstractProxyModel_ptr(QIdentityProxyModel* ptr);
QT_CORE_C_EXPORT QIdentityProxyModel* qt_core_c_QIdentityProxyModel_G_static_cast_QIdentityProxyModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr);
QT_CORE_C_EXPORT QIdentityProxyModel* qt_core_c_QIdentityProxyModel_G_static_cast_QIdentityProxyModel_ptr_QAbstractProxyModel(QAbstractProxyModel* ptr);
QT_CORE_C_EXPORT QIdentityProxyModel* qt_core_c_QIdentityProxyModel_G_static_cast_QIdentityProxyModel_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QIdentityProxyModel_G_static_cast_QObject_ptr(QIdentityProxyModel* ptr);
QT_CORE_C_EXPORT int qt_core_c_QIdentityProxyModel_columnCount_no_args(const QIdentityProxyModel* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QIdentityProxyModel_columnCount_parent(const QIdentityProxyModel* this_ptr, const QModelIndex* parent);
QT_CORE_C_EXPORT void qt_core_c_QIdentityProxyModel_delete(QIdentityProxyModel* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QIdentityProxyModel_dropMimeData(QIdentityProxyModel* this_ptr, const QMimeData* data, const Qt::DropAction* action, int row, int column, const QModelIndex* parent);
QT_CORE_C_EXPORT void qt_core_c_QIdentityProxyModel_headerData_to_output_section_orientation(const QIdentityProxyModel* this_ptr, int section, const Qt::Orientation* orientation, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QIdentityProxyModel_headerData_to_output_section_orientation_role(const QIdentityProxyModel* this_ptr, int section, const Qt::Orientation* orientation, int role, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QIdentityProxyModel_index_to_output_row_column(const QIdentityProxyModel* this_ptr, int row, int column, QModelIndex* output);
QT_CORE_C_EXPORT void qt_core_c_QIdentityProxyModel_index_to_output_row_column_parent(const QIdentityProxyModel* this_ptr, int row, int column, const QModelIndex* parent, QModelIndex* output);
QT_CORE_C_EXPORT bool qt_core_c_QIdentityProxyModel_insertColumns_column_count(QIdentityProxyModel* this_ptr, int column, int count);
QT_CORE_C_EXPORT bool qt_core_c_QIdentityProxyModel_insertColumns_column_count_parent(QIdentityProxyModel* this_ptr, int column, int count, const QModelIndex* parent);
QT_CORE_C_EXPORT bool qt_core_c_QIdentityProxyModel_insertRows_row_count(QIdentityProxyModel* this_ptr, int row, int count);
QT_CORE_C_EXPORT bool qt_core_c_QIdentityProxyModel_insertRows_row_count_parent(QIdentityProxyModel* this_ptr, int row, int count, const QModelIndex* parent);
QT_CORE_C_EXPORT void qt_core_c_QIdentityProxyModel_mapFromSource_to_output(const QIdentityProxyModel* this_ptr, const QModelIndex* sourceIndex, QModelIndex* output);
QT_CORE_C_EXPORT void qt_core_c_QIdentityProxyModel_mapSelectionFromSource_to_output(const QIdentityProxyModel* this_ptr, const QItemSelection* selection, QItemSelection* output);
QT_CORE_C_EXPORT void qt_core_c_QIdentityProxyModel_mapSelectionToSource_to_output(const QIdentityProxyModel* this_ptr, const QItemSelection* selection, QItemSelection* output);
QT_CORE_C_EXPORT void qt_core_c_QIdentityProxyModel_mapToSource_to_output(const QIdentityProxyModel* this_ptr, const QModelIndex* proxyIndex, QModelIndex* output);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QIdentityProxyModel_metaObject(const QIdentityProxyModel* this_ptr);
QT_CORE_C_EXPORT QIdentityProxyModel* qt_core_c_QIdentityProxyModel_new_no_args();
QT_CORE_C_EXPORT QIdentityProxyModel* qt_core_c_QIdentityProxyModel_new_parent(QObject* parent);
QT_CORE_C_EXPORT void qt_core_c_QIdentityProxyModel_parent_to_output(const QIdentityProxyModel* this_ptr, const QModelIndex* child, QModelIndex* output);
QT_CORE_C_EXPORT bool qt_core_c_QIdentityProxyModel_removeColumns_column_count(QIdentityProxyModel* this_ptr, int column, int count);
QT_CORE_C_EXPORT bool qt_core_c_QIdentityProxyModel_removeColumns_column_count_parent(QIdentityProxyModel* this_ptr, int column, int count, const QModelIndex* parent);
QT_CORE_C_EXPORT bool qt_core_c_QIdentityProxyModel_removeRows_row_count(QIdentityProxyModel* this_ptr, int row, int count);
QT_CORE_C_EXPORT bool qt_core_c_QIdentityProxyModel_removeRows_row_count_parent(QIdentityProxyModel* this_ptr, int row, int count, const QModelIndex* parent);
QT_CORE_C_EXPORT int qt_core_c_QIdentityProxyModel_rowCount_no_args(const QIdentityProxyModel* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QIdentityProxyModel_rowCount_parent(const QIdentityProxyModel* this_ptr, const QModelIndex* parent);
QT_CORE_C_EXPORT void qt_core_c_QIdentityProxyModel_setSourceModel(QIdentityProxyModel* this_ptr, QAbstractItemModel* sourceModel);
QT_CORE_C_EXPORT void qt_core_c_QIdentityProxyModel_sibling_to_output(const QIdentityProxyModel* this_ptr, int row, int column, const QModelIndex* idx, QModelIndex* output);
QT_CORE_C_EXPORT void qt_core_c_QIdentityProxyModel_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QIdentityProxyModel_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QIDENTITYPROXYMODEL_H
