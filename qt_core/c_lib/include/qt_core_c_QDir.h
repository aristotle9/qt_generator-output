#ifndef QT_CORE_C_QDIR_H
#define QT_CORE_C_QDIR_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QDir_G_operator_shl_to_output(const QDebug* debug, const QDir* dir, QDebug* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_G_swap(QDir* value1, QDir* value2);
QT_CORE_C_EXPORT void qt_core_c_QDir_absoluteFilePath_to_output(const QDir* this_ptr, const QString* fileName, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_absolutePath_to_output(const QDir* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_addResourceSearchPath(const QString* path);
QT_CORE_C_EXPORT void qt_core_c_QDir_addSearchPath(const QString* prefix, const QString* path);
QT_CORE_C_EXPORT void qt_core_c_QDir_canonicalPath_to_output(const QDir* this_ptr, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QDir_cd(QDir* this_ptr, const QString* dirName);
QT_CORE_C_EXPORT bool qt_core_c_QDir_cdUp(QDir* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDir_cleanPath_to_output(const QString* path, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_constructor_arg1(const QDir* arg1, QDir* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_constructor_no_args(QDir* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_constructor_path(const QString* path, QDir* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_constructor_path_nameFilter(const QString* path, const QString* nameFilter, QDir* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_constructor_path_nameFilter_sort(const QString* path, const QString* nameFilter, unsigned int sort, QDir* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_constructor_path_nameFilter_sort_filter(const QString* path, const QString* nameFilter, unsigned int sort, unsigned int filter, QDir* output);
QT_CORE_C_EXPORT unsigned int qt_core_c_QDir_count(const QDir* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDir_currentPath_to_output(QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_current_to_output(QDir* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_destructor(QDir* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDir_dirName_to_output(const QDir* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_drives_to_output(QList< QFileInfo >* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_entryInfoList_to_output_filters(const QDir* this_ptr, unsigned int filters, QList< QFileInfo >* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_entryInfoList_to_output_filters_sort(const QDir* this_ptr, unsigned int filters, unsigned int sort, QList< QFileInfo >* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_entryInfoList_to_output_nameFilters(const QDir* this_ptr, const QStringList* nameFilters, QList< QFileInfo >* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_entryInfoList_to_output_nameFilters_filters(const QDir* this_ptr, const QStringList* nameFilters, unsigned int filters, QList< QFileInfo >* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_entryInfoList_to_output_nameFilters_filters_sort(const QDir* this_ptr, const QStringList* nameFilters, unsigned int filters, unsigned int sort, QList< QFileInfo >* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_entryInfoList_to_output_no_args(const QDir* this_ptr, QList< QFileInfo >* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_entryList_to_output_filters(const QDir* this_ptr, unsigned int filters, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_entryList_to_output_filters_sort(const QDir* this_ptr, unsigned int filters, unsigned int sort, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_entryList_to_output_nameFilters(const QDir* this_ptr, const QStringList* nameFilters, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_entryList_to_output_nameFilters_filters(const QDir* this_ptr, const QStringList* nameFilters, unsigned int filters, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_entryList_to_output_nameFilters_filters_sort(const QDir* this_ptr, const QStringList* nameFilters, unsigned int filters, unsigned int sort, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_entryList_to_output_no_args(const QDir* this_ptr, QStringList* output);
QT_CORE_C_EXPORT bool qt_core_c_QDir_exists_name(const QDir* this_ptr, const QString* name);
QT_CORE_C_EXPORT bool qt_core_c_QDir_exists_no_args(const QDir* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDir_filePath_to_output(const QDir* this_ptr, const QString* fileName, QString* output);
QT_CORE_C_EXPORT unsigned int qt_core_c_QDir_filter(const QDir* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDir_fromNativeSeparators_to_output(const QString* pathName, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_homePath_to_output(QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_home_to_output(QDir* output);
QT_CORE_C_EXPORT bool qt_core_c_QDir_isAbsolute(const QDir* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QDir_isAbsolutePath(const QString* path);
QT_CORE_C_EXPORT bool qt_core_c_QDir_isEmpty_filters(const QDir* this_ptr, unsigned int filters);
QT_CORE_C_EXPORT bool qt_core_c_QDir_isEmpty_no_args(const QDir* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QDir_isReadable(const QDir* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QDir_isRelative(const QDir* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QDir_isRelativePath(const QString* path);
QT_CORE_C_EXPORT bool qt_core_c_QDir_isRoot(const QDir* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDir_listSeparator_to_output(QChar* output);
QT_CORE_C_EXPORT bool qt_core_c_QDir_makeAbsolute(QDir* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QDir_match_filter_fileName(const QString* filter, const QString* fileName);
QT_CORE_C_EXPORT bool qt_core_c_QDir_match_filters_fileName(const QStringList* filters, const QString* fileName);
QT_CORE_C_EXPORT bool qt_core_c_QDir_mkdir(const QDir* this_ptr, const QString* dirName);
QT_CORE_C_EXPORT bool qt_core_c_QDir_mkpath(const QDir* this_ptr, const QString* dirPath);
QT_CORE_C_EXPORT void qt_core_c_QDir_nameFiltersFromString_to_output(const QString* nameFilter, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_nameFilters_to_output(const QDir* this_ptr, QStringList* output);
QT_CORE_C_EXPORT QDir* qt_core_c_QDir_operator_assign_arg1(QDir* this_ptr, const QDir* arg1);
QT_CORE_C_EXPORT QDir* qt_core_c_QDir_operator_assign_path(QDir* this_ptr, const QString* path);
QT_CORE_C_EXPORT bool qt_core_c_QDir_operator_eq(const QDir* this_ptr, const QDir* dir);
QT_CORE_C_EXPORT void qt_core_c_QDir_operator_index_to_output(const QDir* this_ptr, int arg1, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QDir_operator_neq(const QDir* this_ptr, const QDir* dir);
QT_CORE_C_EXPORT void qt_core_c_QDir_path_to_output(const QDir* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_refresh(const QDir* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDir_relativeFilePath_to_output(const QDir* this_ptr, const QString* fileName, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QDir_remove(QDir* this_ptr, const QString* fileName);
QT_CORE_C_EXPORT bool qt_core_c_QDir_removeRecursively(QDir* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QDir_rename(QDir* this_ptr, const QString* oldName, const QString* newName);
QT_CORE_C_EXPORT bool qt_core_c_QDir_rmdir(const QDir* this_ptr, const QString* dirName);
QT_CORE_C_EXPORT bool qt_core_c_QDir_rmpath(const QDir* this_ptr, const QString* dirPath);
QT_CORE_C_EXPORT void qt_core_c_QDir_rootPath_to_output(QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_root_to_output(QDir* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_searchPaths_to_output(const QString* prefix, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_separator_to_output(QChar* output);
QT_CORE_C_EXPORT bool qt_core_c_QDir_setCurrent(const QString* path);
QT_CORE_C_EXPORT void qt_core_c_QDir_setFilter(QDir* this_ptr, unsigned int filter);
QT_CORE_C_EXPORT void qt_core_c_QDir_setNameFilters(QDir* this_ptr, const QStringList* nameFilters);
QT_CORE_C_EXPORT void qt_core_c_QDir_setPath(QDir* this_ptr, const QString* path);
QT_CORE_C_EXPORT void qt_core_c_QDir_setSearchPaths(const QString* prefix, const QStringList* searchPaths);
QT_CORE_C_EXPORT void qt_core_c_QDir_setSorting(QDir* this_ptr, unsigned int sort);
QT_CORE_C_EXPORT unsigned int qt_core_c_QDir_sorting(const QDir* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDir_swap(QDir* this_ptr, QDir* other);
QT_CORE_C_EXPORT void qt_core_c_QDir_tempPath_to_output(QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_temp_to_output(QDir* output);
QT_CORE_C_EXPORT void qt_core_c_QDir_toNativeSeparators_to_output(const QString* pathName, QString* output);

} // extern "C"

#endif // QT_CORE_C_QDIR_H
