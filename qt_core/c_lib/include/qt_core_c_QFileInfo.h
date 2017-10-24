#ifndef QT_CORE_C_QFILEINFO_H
#define QT_CORE_C_QFILEINFO_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QFileInfo_G_swap(QFileInfo* value1, QFileInfo* value2);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_absoluteDir_to_output(const QFileInfo* this_ptr, QDir* output);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_absoluteFilePath_to_output(const QFileInfo* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_absolutePath_to_output(const QFileInfo* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_baseName_to_output(const QFileInfo* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_bundleName_to_output(const QFileInfo* this_ptr, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QFileInfo_caching(const QFileInfo* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_canonicalFilePath_to_output(const QFileInfo* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_canonicalPath_to_output(const QFileInfo* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_completeBaseName_to_output(const QFileInfo* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_completeSuffix_to_output(const QFileInfo* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_constructor_QDir_QString(const QDir* dir, const QString* file, QFileInfo* output);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_constructor_QFile(const QFile* file, QFileInfo* output);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_constructor_QFileInfo(const QFileInfo* fileinfo, QFileInfo* output);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_constructor_QString(const QString* file, QFileInfo* output);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_constructor_no_args(QFileInfo* output);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_created_to_output(const QFileInfo* this_ptr, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_destructor(QFileInfo* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_dir_to_output(const QFileInfo* this_ptr, QDir* output);
QT_CORE_C_EXPORT bool qt_core_c_QFileInfo_exists_file(const QString* file);
QT_CORE_C_EXPORT bool qt_core_c_QFileInfo_exists_no_args(const QFileInfo* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_fileName_to_output(const QFileInfo* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_filePath_to_output(const QFileInfo* this_ptr, QString* output);
QT_CORE_C_EXPORT unsigned int qt_core_c_QFileInfo_groupId(const QFileInfo* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_group_to_output(const QFileInfo* this_ptr, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QFileInfo_isAbsolute(const QFileInfo* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QFileInfo_isBundle(const QFileInfo* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QFileInfo_isDir(const QFileInfo* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QFileInfo_isExecutable(const QFileInfo* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QFileInfo_isFile(const QFileInfo* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QFileInfo_isHidden(const QFileInfo* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QFileInfo_isNativePath(const QFileInfo* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QFileInfo_isReadable(const QFileInfo* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QFileInfo_isRelative(const QFileInfo* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QFileInfo_isRoot(const QFileInfo* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QFileInfo_isSymLink(const QFileInfo* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QFileInfo_isWritable(const QFileInfo* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_lastModified_to_output(const QFileInfo* this_ptr, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_lastRead_to_output(const QFileInfo* this_ptr, QDateTime* output);
QT_CORE_C_EXPORT bool qt_core_c_QFileInfo_makeAbsolute(QFileInfo* this_ptr);
QT_CORE_C_EXPORT QFileInfo* qt_core_c_QFileInfo_operator_assign(QFileInfo* this_ptr, const QFileInfo* fileinfo);
QT_CORE_C_EXPORT bool qt_core_c_QFileInfo_operator_eq(const QFileInfo* this_ptr, const QFileInfo* fileinfo);
QT_CORE_C_EXPORT bool qt_core_c_QFileInfo_operator_neq(const QFileInfo* this_ptr, const QFileInfo* fileinfo);
QT_CORE_C_EXPORT unsigned int qt_core_c_QFileInfo_ownerId(const QFileInfo* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_owner_to_output(const QFileInfo* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_path_to_output(const QFileInfo* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_readLink_to_output(const QFileInfo* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_refresh(QFileInfo* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_setCaching(QFileInfo* this_ptr, bool on);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_setFile_QDir_QString(QFileInfo* this_ptr, const QDir* dir, const QString* file);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_setFile_QFile(QFileInfo* this_ptr, const QFile* file);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_setFile_QString(QFileInfo* this_ptr, const QString* file);
QT_CORE_C_EXPORT qint64 qt_core_c_QFileInfo_size(const QFileInfo* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_suffix_to_output(const QFileInfo* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_swap(QFileInfo* this_ptr, QFileInfo* other);
QT_CORE_C_EXPORT void qt_core_c_QFileInfo_symLinkTarget_to_output(const QFileInfo* this_ptr, QString* output);

} // extern "C"

#endif // QT_CORE_C_QFILEINFO_H
