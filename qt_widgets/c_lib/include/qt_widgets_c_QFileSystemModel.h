#ifndef QT_WIDGETS_C_QFILESYSTEMMODEL_H
#define QT_WIDGETS_C_QFILESYSTEMMODEL_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QAbstractItemModel* qt_widgets_c_QFileSystemModel_G_static_cast_QAbstractItemModel_ptr(QFileSystemModel* ptr);
QT_WIDGETS_C_EXPORT QFileSystemModel* qt_widgets_c_QFileSystemModel_G_static_cast_QFileSystemModel_ptr_QAbstractItemModel(QAbstractItemModel* ptr);
QT_WIDGETS_C_EXPORT QFileSystemModel* qt_widgets_c_QFileSystemModel_G_static_cast_QFileSystemModel_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QFileSystemModel_G_static_cast_QObject_ptr(QFileSystemModel* ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QFileSystemModel_canFetchMore(const QFileSystemModel* this_ptr, const QModelIndex* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QFileSystemModel_columnCount_no_args(const QFileSystemModel* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QFileSystemModel_columnCount_parent(const QFileSystemModel* this_ptr, const QModelIndex* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_data_to_output_index(const QFileSystemModel* this_ptr, const QModelIndex* index, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_data_to_output_index_role(const QFileSystemModel* this_ptr, const QModelIndex* index, int role, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_delete(QFileSystemModel* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QFileSystemModel_dropMimeData(QFileSystemModel* this_ptr, const QMimeData* data, const Qt::DropAction* action, int row, int column, const QModelIndex* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_fetchMore(QFileSystemModel* this_ptr, const QModelIndex* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_fileIcon_to_output(const QFileSystemModel* this_ptr, const QModelIndex* index, QIcon* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_fileInfo_to_output(const QFileSystemModel* this_ptr, const QModelIndex* index, QFileInfo* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_fileName_to_output(const QFileSystemModel* this_ptr, const QModelIndex* index, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_filePath_to_output(const QFileSystemModel* this_ptr, const QModelIndex* index, QString* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QFileSystemModel_hasChildren_no_args(const QFileSystemModel* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QFileSystemModel_hasChildren_parent(const QFileSystemModel* this_ptr, const QModelIndex* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_headerData_to_output_section_orientation(const QFileSystemModel* this_ptr, int section, const Qt::Orientation* orientation, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_headerData_to_output_section_orientation_role(const QFileSystemModel* this_ptr, int section, const Qt::Orientation* orientation, int role, QVariant* output);
QT_WIDGETS_C_EXPORT QFileIconProvider* qt_widgets_c_QFileSystemModel_iconProvider(const QFileSystemModel* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_index_to_output_path(const QFileSystemModel* this_ptr, const QString* path, QModelIndex* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_index_to_output_path_column(const QFileSystemModel* this_ptr, const QString* path, int column, QModelIndex* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_index_to_output_row_column(const QFileSystemModel* this_ptr, int row, int column, QModelIndex* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_index_to_output_row_column_parent(const QFileSystemModel* this_ptr, int row, int column, const QModelIndex* parent, QModelIndex* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QFileSystemModel_isDir(const QFileSystemModel* this_ptr, const QModelIndex* index);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QFileSystemModel_isReadOnly(const QFileSystemModel* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_lastModified_to_output(const QFileSystemModel* this_ptr, const QModelIndex* index, QDateTime* output);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QFileSystemModel_metaObject(const QFileSystemModel* this_ptr);
QT_WIDGETS_C_EXPORT QMimeData* qt_widgets_c_QFileSystemModel_mimeData(const QFileSystemModel* this_ptr, const QList< QModelIndex >* indexes);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_mimeTypes_to_output(const QFileSystemModel* this_ptr, QStringList* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_mkdir_to_output(QFileSystemModel* this_ptr, const QModelIndex* parent, const QString* name, QModelIndex* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_myComputer_to_output_no_args(const QFileSystemModel* this_ptr, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_myComputer_to_output_role(const QFileSystemModel* this_ptr, int role, QVariant* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QFileSystemModel_nameFilterDisables(const QFileSystemModel* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_nameFilters_to_output(const QFileSystemModel* this_ptr, QStringList* output);
QT_WIDGETS_C_EXPORT QFileSystemModel* qt_widgets_c_QFileSystemModel_new_no_args();
QT_WIDGETS_C_EXPORT QFileSystemModel* qt_widgets_c_QFileSystemModel_new_parent(QObject* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_parent_to_output(const QFileSystemModel* this_ptr, const QModelIndex* child, QModelIndex* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QFileSystemModel_qt_metacall(QFileSystemModel* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QFileSystemModel_qt_metacast(QFileSystemModel* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QFileSystemModel_remove(QFileSystemModel* this_ptr, const QModelIndex* index);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QFileSystemModel_resolveSymlinks(const QFileSystemModel* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QFileSystemModel_rmdir(QFileSystemModel* this_ptr, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_rootDirectory_to_output(const QFileSystemModel* this_ptr, QDir* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_rootPath_to_output(const QFileSystemModel* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QFileSystemModel_rowCount_no_args(const QFileSystemModel* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QFileSystemModel_rowCount_parent(const QFileSystemModel* this_ptr, const QModelIndex* parent);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QFileSystemModel_setData_index_value(QFileSystemModel* this_ptr, const QModelIndex* index, const QVariant* value);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QFileSystemModel_setData_index_value_role(QFileSystemModel* this_ptr, const QModelIndex* index, const QVariant* value, int role);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_setIconProvider(QFileSystemModel* this_ptr, QFileIconProvider* provider);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_setNameFilterDisables(QFileSystemModel* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_setNameFilters(QFileSystemModel* this_ptr, const QStringList* filters);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_setReadOnly(QFileSystemModel* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_setResolveSymlinks(QFileSystemModel* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_setRootPath_to_output(QFileSystemModel* this_ptr, const QString* path, QModelIndex* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_sibling_to_output(const QFileSystemModel* this_ptr, int row, int column, const QModelIndex* idx, QModelIndex* output);
QT_WIDGETS_C_EXPORT qint64 qt_widgets_c_QFileSystemModel_size(const QFileSystemModel* this_ptr, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_sort_column(QFileSystemModel* this_ptr, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_sort_column_order(QFileSystemModel* this_ptr, int column, const Qt::SortOrder* order);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileSystemModel_type_to_output(const QFileSystemModel* this_ptr, const QModelIndex* index, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QFILESYSTEMMODEL_H
