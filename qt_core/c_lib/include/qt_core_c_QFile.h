#ifndef QT_CORE_C_QFILE_H
#define QT_CORE_C_QFILE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QFile* qt_core_c_QFile_G_dynamic_cast_QFile_ptr_QFileDevice(QFileDevice* ptr);
QT_CORE_C_EXPORT QFile* qt_core_c_QFile_G_dynamic_cast_QFile_ptr_QIODevice(QIODevice* ptr);
QT_CORE_C_EXPORT QFile* qt_core_c_QFile_G_dynamic_cast_QFile_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QFileDevice* qt_core_c_QFile_G_static_cast_QFileDevice_ptr(QFile* ptr);
QT_CORE_C_EXPORT QFile* qt_core_c_QFile_G_static_cast_QFile_ptr_QFileDevice(QFileDevice* ptr);
QT_CORE_C_EXPORT QFile* qt_core_c_QFile_G_static_cast_QFile_ptr_QIODevice(QIODevice* ptr);
QT_CORE_C_EXPORT QFile* qt_core_c_QFile_G_static_cast_QFile_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QIODevice* qt_core_c_QFile_G_static_cast_QIODevice_ptr(QFile* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QFile_G_static_cast_QObject_ptr(QFile* ptr);
QT_CORE_C_EXPORT bool qt_core_c_QFile_copy_fileName_newName(const QString* fileName, const QString* newName);
QT_CORE_C_EXPORT bool qt_core_c_QFile_copy_newName(QFile* this_ptr, const QString* newName);
QT_CORE_C_EXPORT void qt_core_c_QFile_decodeName_to_output_QByteArray(const QByteArray* localFileName, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QFile_decodeName_to_output_char(const char* localFileName, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QFile_delete(QFile* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QFile_encodeName_to_output(const QString* fileName, QByteArray* output);
QT_CORE_C_EXPORT bool qt_core_c_QFile_exists_fileName(const QString* fileName);
QT_CORE_C_EXPORT bool qt_core_c_QFile_exists_no_args(const QFile* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QFile_fileName_to_output(const QFile* this_ptr, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QFile_link_newName(QFile* this_ptr, const QString* newName);
QT_CORE_C_EXPORT bool qt_core_c_QFile_link_oldname_newName(const QString* oldname, const QString* newName);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QFile_metaObject(const QFile* this_ptr);
QT_CORE_C_EXPORT QFile* qt_core_c_QFile_new_name(const QString* name);
QT_CORE_C_EXPORT QFile* qt_core_c_QFile_new_name_parent(const QString* name, QObject* parent);
QT_CORE_C_EXPORT QFile* qt_core_c_QFile_new_no_args();
QT_CORE_C_EXPORT QFile* qt_core_c_QFile_new_parent(QObject* parent);
QT_CORE_C_EXPORT bool qt_core_c_QFile_open_fd_ioFlags(QFile* this_ptr, int fd, unsigned int ioFlags);
QT_CORE_C_EXPORT bool qt_core_c_QFile_open_fd_ioFlags_handleFlags(QFile* this_ptr, int fd, unsigned int ioFlags, unsigned int handleFlags);
QT_CORE_C_EXPORT bool qt_core_c_QFile_open_flags(QFile* this_ptr, unsigned int flags);
QT_CORE_C_EXPORT unsigned int qt_core_c_QFile_permissions_filename(const QString* filename);
QT_CORE_C_EXPORT unsigned int qt_core_c_QFile_permissions_no_args(const QFile* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QFile_readLink_to_output_fileName(const QString* fileName, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QFile_readLink_to_output_no_args(const QFile* this_ptr, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QFile_remove_fileName(const QString* fileName);
QT_CORE_C_EXPORT bool qt_core_c_QFile_remove_no_args(QFile* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QFile_rename_newName(QFile* this_ptr, const QString* newName);
QT_CORE_C_EXPORT bool qt_core_c_QFile_rename_oldName_newName(const QString* oldName, const QString* newName);
QT_CORE_C_EXPORT bool qt_core_c_QFile_resize_filename_sz(const QString* filename, qint64 sz);
QT_CORE_C_EXPORT bool qt_core_c_QFile_resize_sz(QFile* this_ptr, qint64 sz);
QT_CORE_C_EXPORT void qt_core_c_QFile_setFileName(QFile* this_ptr, const QString* name);
QT_CORE_C_EXPORT bool qt_core_c_QFile_setPermissions_filename_permissionSpec(const QString* filename, unsigned int permissionSpec);
QT_CORE_C_EXPORT bool qt_core_c_QFile_setPermissions_permissionSpec(QFile* this_ptr, unsigned int permissionSpec);
QT_CORE_C_EXPORT qint64 qt_core_c_QFile_size(const QFile* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QFile_symLinkTarget_to_output_fileName(const QString* fileName, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QFile_symLinkTarget_to_output_no_args(const QFile* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QFile_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QFile_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QFILE_H
