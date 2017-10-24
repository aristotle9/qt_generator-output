#ifndef QT_CORE_C_QSORTFILTERPROXYMODEL_H
#define QT_CORE_C_QSORTFILTERPROXYMODEL_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QSortFilterProxyModel* qt_core_c_QSortFilterProxyModel_G_dynamic_cast_QSortFilterProxyModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr);
QT_CORE_C_EXPORT QSortFilterProxyModel* qt_core_c_QSortFilterProxyModel_G_dynamic_cast_QSortFilterProxyModel_ptr_QAbstractProxyModel(QAbstractProxyModel* ptr);
QT_CORE_C_EXPORT QSortFilterProxyModel* qt_core_c_QSortFilterProxyModel_G_dynamic_cast_QSortFilterProxyModel_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QAbstractItemModel* qt_core_c_QSortFilterProxyModel_G_static_cast_QAbstractItemModel_ptr(QSortFilterProxyModel* ptr);
QT_CORE_C_EXPORT QAbstractProxyModel* qt_core_c_QSortFilterProxyModel_G_static_cast_QAbstractProxyModel_ptr(QSortFilterProxyModel* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QSortFilterProxyModel_G_static_cast_QObject_ptr(QSortFilterProxyModel* ptr);
QT_CORE_C_EXPORT QSortFilterProxyModel* qt_core_c_QSortFilterProxyModel_G_static_cast_QSortFilterProxyModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr);
QT_CORE_C_EXPORT QSortFilterProxyModel* qt_core_c_QSortFilterProxyModel_G_static_cast_QSortFilterProxyModel_ptr_QAbstractProxyModel(QAbstractProxyModel* ptr);
QT_CORE_C_EXPORT QSortFilterProxyModel* qt_core_c_QSortFilterProxyModel_G_static_cast_QSortFilterProxyModel_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_buddy_to_output(const QSortFilterProxyModel* this_ptr, const QModelIndex* index, QModelIndex* output);
QT_CORE_C_EXPORT bool qt_core_c_QSortFilterProxyModel_canFetchMore(const QSortFilterProxyModel* this_ptr, const QModelIndex* parent);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_clear(QSortFilterProxyModel* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QSortFilterProxyModel_columnCount_no_args(const QSortFilterProxyModel* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QSortFilterProxyModel_columnCount_parent(const QSortFilterProxyModel* this_ptr, const QModelIndex* parent);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_data_to_output_index(const QSortFilterProxyModel* this_ptr, const QModelIndex* index, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_data_to_output_index_role(const QSortFilterProxyModel* this_ptr, const QModelIndex* index, int role, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_delete(QSortFilterProxyModel* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QSortFilterProxyModel_dropMimeData(QSortFilterProxyModel* this_ptr, const QMimeData* data, const Qt::DropAction* action, int row, int column, const QModelIndex* parent);
QT_CORE_C_EXPORT bool qt_core_c_QSortFilterProxyModel_dynamicSortFilter(const QSortFilterProxyModel* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_fetchMore(QSortFilterProxyModel* this_ptr, const QModelIndex* parent);
QT_CORE_C_EXPORT int qt_core_c_QSortFilterProxyModel_filterKeyColumn(const QSortFilterProxyModel* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_filterRegExp_to_output(const QSortFilterProxyModel* this_ptr, QRegExp* output);
QT_CORE_C_EXPORT int qt_core_c_QSortFilterProxyModel_filterRole(const QSortFilterProxyModel* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QSortFilterProxyModel_hasChildren_no_args(const QSortFilterProxyModel* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QSortFilterProxyModel_hasChildren_parent(const QSortFilterProxyModel* this_ptr, const QModelIndex* parent);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_headerData_to_output_section_orientation(const QSortFilterProxyModel* this_ptr, int section, const Qt::Orientation* orientation, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_headerData_to_output_section_orientation_role(const QSortFilterProxyModel* this_ptr, int section, const Qt::Orientation* orientation, int role, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_index_to_output_row_column(const QSortFilterProxyModel* this_ptr, int row, int column, QModelIndex* output);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_index_to_output_row_column_parent(const QSortFilterProxyModel* this_ptr, int row, int column, const QModelIndex* parent, QModelIndex* output);
QT_CORE_C_EXPORT bool qt_core_c_QSortFilterProxyModel_insertColumns_column_count(QSortFilterProxyModel* this_ptr, int column, int count);
QT_CORE_C_EXPORT bool qt_core_c_QSortFilterProxyModel_insertColumns_column_count_parent(QSortFilterProxyModel* this_ptr, int column, int count, const QModelIndex* parent);
QT_CORE_C_EXPORT bool qt_core_c_QSortFilterProxyModel_insertRows_row_count(QSortFilterProxyModel* this_ptr, int row, int count);
QT_CORE_C_EXPORT bool qt_core_c_QSortFilterProxyModel_insertRows_row_count_parent(QSortFilterProxyModel* this_ptr, int row, int count, const QModelIndex* parent);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_invalidate(QSortFilterProxyModel* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QSortFilterProxyModel_isSortLocaleAware(const QSortFilterProxyModel* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_mapFromSource_to_output(const QSortFilterProxyModel* this_ptr, const QModelIndex* sourceIndex, QModelIndex* output);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_mapSelectionFromSource_to_output(const QSortFilterProxyModel* this_ptr, const QItemSelection* sourceSelection, QItemSelection* output);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_mapSelectionToSource_to_output(const QSortFilterProxyModel* this_ptr, const QItemSelection* proxySelection, QItemSelection* output);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_mapToSource_to_output(const QSortFilterProxyModel* this_ptr, const QModelIndex* proxyIndex, QModelIndex* output);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QSortFilterProxyModel_metaObject(const QSortFilterProxyModel* this_ptr);
QT_CORE_C_EXPORT QMimeData* qt_core_c_QSortFilterProxyModel_mimeData(const QSortFilterProxyModel* this_ptr, const QList< QModelIndex >* indexes);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_mimeTypes_to_output(const QSortFilterProxyModel* this_ptr, QStringList* output);
QT_CORE_C_EXPORT QSortFilterProxyModel* qt_core_c_QSortFilterProxyModel_new_no_args();
QT_CORE_C_EXPORT QSortFilterProxyModel* qt_core_c_QSortFilterProxyModel_new_parent(QObject* parent);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_parent_to_output(const QSortFilterProxyModel* this_ptr, const QModelIndex* child, QModelIndex* output);
QT_CORE_C_EXPORT bool qt_core_c_QSortFilterProxyModel_removeColumns_column_count(QSortFilterProxyModel* this_ptr, int column, int count);
QT_CORE_C_EXPORT bool qt_core_c_QSortFilterProxyModel_removeColumns_column_count_parent(QSortFilterProxyModel* this_ptr, int column, int count, const QModelIndex* parent);
QT_CORE_C_EXPORT bool qt_core_c_QSortFilterProxyModel_removeRows_row_count(QSortFilterProxyModel* this_ptr, int row, int count);
QT_CORE_C_EXPORT bool qt_core_c_QSortFilterProxyModel_removeRows_row_count_parent(QSortFilterProxyModel* this_ptr, int row, int count, const QModelIndex* parent);
QT_CORE_C_EXPORT int qt_core_c_QSortFilterProxyModel_rowCount_no_args(const QSortFilterProxyModel* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QSortFilterProxyModel_rowCount_parent(const QSortFilterProxyModel* this_ptr, const QModelIndex* parent);
QT_CORE_C_EXPORT bool qt_core_c_QSortFilterProxyModel_setData_index_value(QSortFilterProxyModel* this_ptr, const QModelIndex* index, const QVariant* value);
QT_CORE_C_EXPORT bool qt_core_c_QSortFilterProxyModel_setData_index_value_role(QSortFilterProxyModel* this_ptr, const QModelIndex* index, const QVariant* value, int role);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_setDynamicSortFilter(QSortFilterProxyModel* this_ptr, bool enable);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_setFilterCaseSensitivity(QSortFilterProxyModel* this_ptr, const Qt::CaseSensitivity* cs);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_setFilterFixedString(QSortFilterProxyModel* this_ptr, const QString* pattern);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_setFilterKeyColumn(QSortFilterProxyModel* this_ptr, int column);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_setFilterRegExp_pattern(QSortFilterProxyModel* this_ptr, const QString* pattern);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_setFilterRegExp_regExp(QSortFilterProxyModel* this_ptr, const QRegExp* regExp);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_setFilterRole(QSortFilterProxyModel* this_ptr, int role);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_setFilterWildcard(QSortFilterProxyModel* this_ptr, const QString* pattern);
QT_CORE_C_EXPORT bool qt_core_c_QSortFilterProxyModel_setHeaderData_section_orientation_value(QSortFilterProxyModel* this_ptr, int section, const Qt::Orientation* orientation, const QVariant* value);
QT_CORE_C_EXPORT bool qt_core_c_QSortFilterProxyModel_setHeaderData_section_orientation_value_role(QSortFilterProxyModel* this_ptr, int section, const Qt::Orientation* orientation, const QVariant* value, int role);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_setSortCaseSensitivity(QSortFilterProxyModel* this_ptr, const Qt::CaseSensitivity* cs);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_setSortLocaleAware(QSortFilterProxyModel* this_ptr, bool on);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_setSortRole(QSortFilterProxyModel* this_ptr, int role);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_setSourceModel(QSortFilterProxyModel* this_ptr, QAbstractItemModel* sourceModel);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_sibling_to_output(const QSortFilterProxyModel* this_ptr, int row, int column, const QModelIndex* idx, QModelIndex* output);
QT_CORE_C_EXPORT int qt_core_c_QSortFilterProxyModel_sortColumn(const QSortFilterProxyModel* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QSortFilterProxyModel_sortRole(const QSortFilterProxyModel* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_sort_column(QSortFilterProxyModel* this_ptr, int column);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_sort_column_order(QSortFilterProxyModel* this_ptr, int column, const Qt::SortOrder* order);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_span_to_output(const QSortFilterProxyModel* this_ptr, const QModelIndex* index, QSize* output);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QSortFilterProxyModel_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QSORTFILTERPROXYMODEL_H
