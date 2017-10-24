#ifndef QT_WIDGETS_C_QDIRMODEL_H
#define QT_WIDGETS_C_QDIRMODEL_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QAbstractItemModel* qt_widgets_c_QDirModel_G_static_cast_QAbstractItemModel_ptr(QDirModel* ptr);
QT_WIDGETS_C_EXPORT QDirModel* qt_widgets_c_QDirModel_G_static_cast_QDirModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr);
QT_WIDGETS_C_EXPORT QDirModel* qt_widgets_c_QDirModel_G_static_cast_QDirModel_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QDirModel_G_static_cast_QObject_ptr(QDirModel* ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDirModel_columnCount_no_args(const QDirModel* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDirModel_columnCount_parent(const QDirModel* this_ptr, const QModelIndex* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_data_to_output_index(const QDirModel* this_ptr, const QModelIndex* index, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_data_to_output_index_role(const QDirModel* this_ptr, const QModelIndex* index, int role, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_delete(QDirModel* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QDirModel_dropMimeData(QDirModel* this_ptr, const QMimeData* data, const Qt::DropAction* action, int row, int column, const QModelIndex* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_fileIcon_to_output(const QDirModel* this_ptr, const QModelIndex* index, QIcon* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_fileInfo_to_output(const QDirModel* this_ptr, const QModelIndex* index, QFileInfo* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_fileName_to_output(const QDirModel* this_ptr, const QModelIndex* index, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_filePath_to_output(const QDirModel* this_ptr, const QModelIndex* index, QString* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QDirModel_hasChildren_index(const QDirModel* this_ptr, const QModelIndex* index);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QDirModel_hasChildren_no_args(const QDirModel* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_headerData_to_output_section_orientation(const QDirModel* this_ptr, int section, const Qt::Orientation* orientation, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_headerData_to_output_section_orientation_role(const QDirModel* this_ptr, int section, const Qt::Orientation* orientation, int role, QVariant* output);
QT_WIDGETS_C_EXPORT QFileIconProvider* qt_widgets_c_QDirModel_iconProvider(const QDirModel* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_index_to_output_path(const QDirModel* this_ptr, const QString* path, QModelIndex* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_index_to_output_path_column(const QDirModel* this_ptr, const QString* path, int column, QModelIndex* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_index_to_output_row_column(const QDirModel* this_ptr, int row, int column, QModelIndex* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_index_to_output_row_column_parent(const QDirModel* this_ptr, int row, int column, const QModelIndex* parent, QModelIndex* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QDirModel_isDir(const QDirModel* this_ptr, const QModelIndex* index);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QDirModel_isReadOnly(const QDirModel* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QDirModel_lazyChildCount(const QDirModel* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QDirModel_metaObject(const QDirModel* this_ptr);
QT_WIDGETS_C_EXPORT QMimeData* qt_widgets_c_QDirModel_mimeData(const QDirModel* this_ptr, const QList< QModelIndex >* indexes);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_mimeTypes_to_output(const QDirModel* this_ptr, QStringList* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_mkdir_to_output(QDirModel* this_ptr, const QModelIndex* parent, const QString* name, QModelIndex* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_nameFilters_to_output(const QDirModel* this_ptr, QStringList* output);
QT_WIDGETS_C_EXPORT QDirModel* qt_widgets_c_QDirModel_new_no_args();
QT_WIDGETS_C_EXPORT QDirModel* qt_widgets_c_QDirModel_new_parent(QObject* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_parent_to_output(const QDirModel* this_ptr, const QModelIndex* child, QModelIndex* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDirModel_qt_metacall(QDirModel* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QDirModel_qt_metacast(QDirModel* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_refresh_no_args(QDirModel* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_refresh_parent(QDirModel* this_ptr, const QModelIndex* parent);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QDirModel_remove(QDirModel* this_ptr, const QModelIndex* index);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QDirModel_resolveSymlinks(const QDirModel* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QDirModel_rmdir(QDirModel* this_ptr, const QModelIndex* index);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDirModel_rowCount_no_args(const QDirModel* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDirModel_rowCount_parent(const QDirModel* this_ptr, const QModelIndex* parent);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QDirModel_setData_index_value(QDirModel* this_ptr, const QModelIndex* index, const QVariant* value);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QDirModel_setData_index_value_role(QDirModel* this_ptr, const QModelIndex* index, const QVariant* value, int role);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_setIconProvider(QDirModel* this_ptr, QFileIconProvider* provider);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_setLazyChildCount(QDirModel* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_setNameFilters(QDirModel* this_ptr, const QStringList* filters);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_setReadOnly(QDirModel* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_setResolveSymlinks(QDirModel* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_sort_column(QDirModel* this_ptr, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_sort_column_order(QDirModel* this_ptr, int column, const Qt::SortOrder* order);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDirModel_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QDIRMODEL_H
