#ifndef QT_CORE_C_QABSTRACTPROXYMODEL_H
#define QT_CORE_C_QABSTRACTPROXYMODEL_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QAbstractProxyModel* qt_core_c_QAbstractProxyModel_G_dynamic_cast_QAbstractProxyModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr);
QT_CORE_C_EXPORT QAbstractProxyModel* qt_core_c_QAbstractProxyModel_G_dynamic_cast_QAbstractProxyModel_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QAbstractItemModel* qt_core_c_QAbstractProxyModel_G_static_cast_QAbstractItemModel_ptr(QAbstractProxyModel* ptr);
QT_CORE_C_EXPORT QAbstractProxyModel* qt_core_c_QAbstractProxyModel_G_static_cast_QAbstractProxyModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr);
QT_CORE_C_EXPORT QAbstractProxyModel* qt_core_c_QAbstractProxyModel_G_static_cast_QAbstractProxyModel_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QAbstractProxyModel_G_static_cast_QObject_ptr(QAbstractProxyModel* ptr);
QT_CORE_C_EXPORT void qt_core_c_QAbstractProxyModel_buddy_to_output(const QAbstractProxyModel* this_ptr, const QModelIndex* index, QModelIndex* output);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractProxyModel_canDropMimeData(const QAbstractProxyModel* this_ptr, const QMimeData* data, const Qt::DropAction* action, int row, int column, const QModelIndex* parent);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractProxyModel_canFetchMore(const QAbstractProxyModel* this_ptr, const QModelIndex* parent);
QT_CORE_C_EXPORT void qt_core_c_QAbstractProxyModel_data_to_output_proxyIndex(const QAbstractProxyModel* this_ptr, const QModelIndex* proxyIndex, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractProxyModel_data_to_output_proxyIndex_role(const QAbstractProxyModel* this_ptr, const QModelIndex* proxyIndex, int role, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractProxyModel_delete(QAbstractProxyModel* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractProxyModel_dropMimeData(QAbstractProxyModel* this_ptr, const QMimeData* data, const Qt::DropAction* action, int row, int column, const QModelIndex* parent);
QT_CORE_C_EXPORT void qt_core_c_QAbstractProxyModel_fetchMore(QAbstractProxyModel* this_ptr, const QModelIndex* parent);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractProxyModel_hasChildren_no_args(const QAbstractProxyModel* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractProxyModel_hasChildren_parent(const QAbstractProxyModel* this_ptr, const QModelIndex* parent);
QT_CORE_C_EXPORT void qt_core_c_QAbstractProxyModel_headerData_to_output_section_orientation(const QAbstractProxyModel* this_ptr, int section, const Qt::Orientation* orientation, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractProxyModel_headerData_to_output_section_orientation_role(const QAbstractProxyModel* this_ptr, int section, const Qt::Orientation* orientation, int role, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractProxyModel_itemData_to_output(const QAbstractProxyModel* this_ptr, const QModelIndex* index, QMap< int, QVariant >* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractProxyModel_mapFromSource_to_output(const QAbstractProxyModel* this_ptr, const QModelIndex* sourceIndex, QModelIndex* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractProxyModel_mapSelectionFromSource_to_output(const QAbstractProxyModel* this_ptr, const QItemSelection* selection, QItemSelection* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractProxyModel_mapSelectionToSource_to_output(const QAbstractProxyModel* this_ptr, const QItemSelection* selection, QItemSelection* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractProxyModel_mapToSource_to_output(const QAbstractProxyModel* this_ptr, const QModelIndex* proxyIndex, QModelIndex* output);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QAbstractProxyModel_metaObject(const QAbstractProxyModel* this_ptr);
QT_CORE_C_EXPORT QMimeData* qt_core_c_QAbstractProxyModel_mimeData(const QAbstractProxyModel* this_ptr, const QList< QModelIndex >* indexes);
QT_CORE_C_EXPORT void qt_core_c_QAbstractProxyModel_mimeTypes_to_output(const QAbstractProxyModel* this_ptr, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractProxyModel_revert(QAbstractProxyModel* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractProxyModel_setData_index_value(QAbstractProxyModel* this_ptr, const QModelIndex* index, const QVariant* value);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractProxyModel_setData_index_value_role(QAbstractProxyModel* this_ptr, const QModelIndex* index, const QVariant* value, int role);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractProxyModel_setHeaderData_section_orientation_value(QAbstractProxyModel* this_ptr, int section, const Qt::Orientation* orientation, const QVariant* value);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractProxyModel_setHeaderData_section_orientation_value_role(QAbstractProxyModel* this_ptr, int section, const Qt::Orientation* orientation, const QVariant* value, int role);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractProxyModel_setItemData(QAbstractProxyModel* this_ptr, const QModelIndex* index, const QMap< int, QVariant >* roles);
QT_CORE_C_EXPORT void qt_core_c_QAbstractProxyModel_setSourceModel(QAbstractProxyModel* this_ptr, QAbstractItemModel* sourceModel);
QT_CORE_C_EXPORT void qt_core_c_QAbstractProxyModel_sibling_to_output(const QAbstractProxyModel* this_ptr, int row, int column, const QModelIndex* idx, QModelIndex* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractProxyModel_sort_column(QAbstractProxyModel* this_ptr, int column);
QT_CORE_C_EXPORT void qt_core_c_QAbstractProxyModel_sort_column_order(QAbstractProxyModel* this_ptr, int column, const Qt::SortOrder* order);
QT_CORE_C_EXPORT QAbstractItemModel* qt_core_c_QAbstractProxyModel_sourceModel(const QAbstractProxyModel* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAbstractProxyModel_span_to_output(const QAbstractProxyModel* this_ptr, const QModelIndex* index, QSize* output);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractProxyModel_submit(QAbstractProxyModel* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAbstractProxyModel_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractProxyModel_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QABSTRACTPROXYMODEL_H
