#ifndef QT_CORE_C_QSTORAGEINFO_H
#define QT_CORE_C_QSTORAGEINFO_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT bool qt_core_c_QStorageInfo_G_operator_neq(const QStorageInfo* first, const QStorageInfo* second);
QT_CORE_C_EXPORT void qt_core_c_QStorageInfo_G_swap(QStorageInfo* value1, QStorageInfo* value2);
QT_CORE_C_EXPORT int qt_core_c_QStorageInfo_blockSize(const QStorageInfo* this_ptr);
QT_CORE_C_EXPORT qint64 qt_core_c_QStorageInfo_bytesAvailable(const QStorageInfo* this_ptr);
QT_CORE_C_EXPORT qint64 qt_core_c_QStorageInfo_bytesFree(const QStorageInfo* this_ptr);
QT_CORE_C_EXPORT qint64 qt_core_c_QStorageInfo_bytesTotal(const QStorageInfo* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QStorageInfo_constructor_dir(const QDir* dir, QStorageInfo* output);
QT_CORE_C_EXPORT void qt_core_c_QStorageInfo_constructor_no_args(QStorageInfo* output);
QT_CORE_C_EXPORT void qt_core_c_QStorageInfo_constructor_other(const QStorageInfo* other, QStorageInfo* output);
QT_CORE_C_EXPORT void qt_core_c_QStorageInfo_constructor_path(const QString* path, QStorageInfo* output);
QT_CORE_C_EXPORT void qt_core_c_QStorageInfo_destructor(QStorageInfo* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QStorageInfo_device_to_output(const QStorageInfo* this_ptr, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QStorageInfo_displayName_to_output(const QStorageInfo* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QStorageInfo_fileSystemType_to_output(const QStorageInfo* this_ptr, QByteArray* output);
QT_CORE_C_EXPORT bool qt_core_c_QStorageInfo_isReadOnly(const QStorageInfo* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QStorageInfo_isReady(const QStorageInfo* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QStorageInfo_isRoot(const QStorageInfo* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QStorageInfo_isValid(const QStorageInfo* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QStorageInfo_mountedVolumes_to_output(QList< QStorageInfo >* output);
QT_CORE_C_EXPORT void qt_core_c_QStorageInfo_name_to_output(const QStorageInfo* this_ptr, QString* output);
QT_CORE_C_EXPORT QStorageInfo* qt_core_c_QStorageInfo_operator_assign(QStorageInfo* this_ptr, const QStorageInfo* other);
QT_CORE_C_EXPORT void qt_core_c_QStorageInfo_refresh(QStorageInfo* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QStorageInfo_rootPath_to_output(const QStorageInfo* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QStorageInfo_root_to_output(QStorageInfo* output);
QT_CORE_C_EXPORT void qt_core_c_QStorageInfo_setPath(QStorageInfo* this_ptr, const QString* path);
QT_CORE_C_EXPORT void qt_core_c_QStorageInfo_subvolume_to_output(const QStorageInfo* this_ptr, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QStorageInfo_swap(QStorageInfo* this_ptr, QStorageInfo* other);

} // extern "C"

#endif // QT_CORE_C_QSTORAGEINFO_H
