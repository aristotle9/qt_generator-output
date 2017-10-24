#ifndef QT_CORE_C_QFILEDEVICE_H
#define QT_CORE_C_QFILEDEVICE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QFileDevice* qt_core_c_QFileDevice_G_dynamic_cast_QFileDevice_ptr_QIODevice(QIODevice* ptr);
QT_CORE_C_EXPORT QFileDevice* qt_core_c_QFileDevice_G_dynamic_cast_QFileDevice_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QFileDevice* qt_core_c_QFileDevice_G_static_cast_QFileDevice_ptr_QIODevice(QIODevice* ptr);
QT_CORE_C_EXPORT QFileDevice* qt_core_c_QFileDevice_G_static_cast_QFileDevice_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QIODevice* qt_core_c_QFileDevice_G_static_cast_QIODevice_ptr(QFileDevice* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QFileDevice_G_static_cast_QObject_ptr(QFileDevice* ptr);
QT_CORE_C_EXPORT bool qt_core_c_QFileDevice_atEnd(const QFileDevice* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QFileDevice_close(QFileDevice* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QFileDevice_delete(QFileDevice* this_ptr);
QT_CORE_C_EXPORT QFileDevice::FileError qt_core_c_QFileDevice_error(const QFileDevice* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QFileDevice_fileName_to_output(const QFileDevice* this_ptr, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QFileDevice_flush(QFileDevice* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QFileDevice_handle(const QFileDevice* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QFileDevice_isSequential(const QFileDevice* this_ptr);
QT_CORE_C_EXPORT unsigned char* qt_core_c_QFileDevice_map_offset_size(QFileDevice* this_ptr, qint64 offset, qint64 size);
QT_CORE_C_EXPORT unsigned char* qt_core_c_QFileDevice_map_offset_size_flags(QFileDevice* this_ptr, qint64 offset, qint64 size, QFileDevice::MemoryMapFlags flags);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QFileDevice_metaObject(const QFileDevice* this_ptr);
QT_CORE_C_EXPORT unsigned int qt_core_c_QFileDevice_permissions(const QFileDevice* this_ptr);
QT_CORE_C_EXPORT qint64 qt_core_c_QFileDevice_pos(const QFileDevice* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QFileDevice_resize(QFileDevice* this_ptr, qint64 sz);
QT_CORE_C_EXPORT bool qt_core_c_QFileDevice_seek(QFileDevice* this_ptr, qint64 offset);
QT_CORE_C_EXPORT bool qt_core_c_QFileDevice_setPermissions(QFileDevice* this_ptr, unsigned int permissionSpec);
QT_CORE_C_EXPORT qint64 qt_core_c_QFileDevice_size(const QFileDevice* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QFileDevice_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QFileDevice_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QFileDevice_unmap(QFileDevice* this_ptr, unsigned char* address);
QT_CORE_C_EXPORT void qt_core_c_QFileDevice_unsetError(QFileDevice* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QFILEDEVICE_H
