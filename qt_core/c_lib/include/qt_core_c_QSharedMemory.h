#ifndef QT_CORE_C_QSHAREDMEMORY_H
#define QT_CORE_C_QSHAREDMEMORY_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QSharedMemory* qt_core_c_QSharedMemory_G_dynamic_cast_QSharedMemory_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QSharedMemory_G_static_cast_QObject_ptr(QSharedMemory* ptr);
QT_CORE_C_EXPORT QSharedMemory* qt_core_c_QSharedMemory_G_static_cast_QSharedMemory_ptr(QObject* ptr);
QT_CORE_C_EXPORT bool qt_core_c_QSharedMemory_attach_mode(QSharedMemory* this_ptr, QSharedMemory::AccessMode mode);
QT_CORE_C_EXPORT bool qt_core_c_QSharedMemory_attach_no_args(QSharedMemory* this_ptr);
QT_CORE_C_EXPORT const void* qt_core_c_QSharedMemory_constData(const QSharedMemory* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QSharedMemory_create_size(QSharedMemory* this_ptr, int size);
QT_CORE_C_EXPORT bool qt_core_c_QSharedMemory_create_size_mode(QSharedMemory* this_ptr, int size, QSharedMemory::AccessMode mode);
QT_CORE_C_EXPORT void* qt_core_c_QSharedMemory_data(QSharedMemory* this_ptr);
QT_CORE_C_EXPORT const void* qt_core_c_QSharedMemory_data_const(const QSharedMemory* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSharedMemory_delete(QSharedMemory* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QSharedMemory_detach(QSharedMemory* this_ptr);
QT_CORE_C_EXPORT QSharedMemory::SharedMemoryError qt_core_c_QSharedMemory_error(const QSharedMemory* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSharedMemory_errorString_to_output(const QSharedMemory* this_ptr, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QSharedMemory_isAttached(const QSharedMemory* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSharedMemory_key_to_output(const QSharedMemory* this_ptr, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QSharedMemory_lock(QSharedMemory* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QSharedMemory_metaObject(const QSharedMemory* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSharedMemory_nativeKey_to_output(const QSharedMemory* this_ptr, QString* output);
QT_CORE_C_EXPORT QSharedMemory* qt_core_c_QSharedMemory_new_key(const QString* key);
QT_CORE_C_EXPORT QSharedMemory* qt_core_c_QSharedMemory_new_key_parent(const QString* key, QObject* parent);
QT_CORE_C_EXPORT QSharedMemory* qt_core_c_QSharedMemory_new_no_args();
QT_CORE_C_EXPORT QSharedMemory* qt_core_c_QSharedMemory_new_parent(QObject* parent);
QT_CORE_C_EXPORT void qt_core_c_QSharedMemory_setKey(QSharedMemory* this_ptr, const QString* key);
QT_CORE_C_EXPORT void qt_core_c_QSharedMemory_setNativeKey(QSharedMemory* this_ptr, const QString* key);
QT_CORE_C_EXPORT int qt_core_c_QSharedMemory_size(const QSharedMemory* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSharedMemory_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QSharedMemory_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QSharedMemory_unlock(QSharedMemory* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QSHAREDMEMORY_H
