#ifndef QT_CORE_C_QBUFFER_H
#define QT_CORE_C_QBUFFER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QBuffer* qt_core_c_QBuffer_G_dynamic_cast_QBuffer_ptr_QIODevice(QIODevice* ptr);
QT_CORE_C_EXPORT QBuffer* qt_core_c_QBuffer_G_dynamic_cast_QBuffer_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QBuffer* qt_core_c_QBuffer_G_static_cast_QBuffer_ptr_QIODevice(QIODevice* ptr);
QT_CORE_C_EXPORT QBuffer* qt_core_c_QBuffer_G_static_cast_QBuffer_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QIODevice* qt_core_c_QBuffer_G_static_cast_QIODevice_ptr(QBuffer* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QBuffer_G_static_cast_QObject_ptr(QBuffer* ptr);
QT_CORE_C_EXPORT bool qt_core_c_QBuffer_atEnd(const QBuffer* this_ptr);
QT_CORE_C_EXPORT QByteArray* qt_core_c_QBuffer_buffer(QBuffer* this_ptr);
QT_CORE_C_EXPORT const QByteArray* qt_core_c_QBuffer_buffer_const(const QBuffer* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QBuffer_canReadLine(const QBuffer* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QBuffer_close(QBuffer* this_ptr);
QT_CORE_C_EXPORT const QByteArray* qt_core_c_QBuffer_data(const QBuffer* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QBuffer_delete(QBuffer* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QBuffer_metaObject(const QBuffer* this_ptr);
QT_CORE_C_EXPORT QBuffer* qt_core_c_QBuffer_new_buf(QByteArray* buf);
QT_CORE_C_EXPORT QBuffer* qt_core_c_QBuffer_new_buf_parent(QByteArray* buf, QObject* parent);
QT_CORE_C_EXPORT QBuffer* qt_core_c_QBuffer_new_no_args();
QT_CORE_C_EXPORT QBuffer* qt_core_c_QBuffer_new_parent(QObject* parent);
QT_CORE_C_EXPORT bool qt_core_c_QBuffer_open(QBuffer* this_ptr, unsigned int openMode);
QT_CORE_C_EXPORT qint64 qt_core_c_QBuffer_pos(const QBuffer* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QBuffer_seek(QBuffer* this_ptr, qint64 off);
QT_CORE_C_EXPORT void qt_core_c_QBuffer_setBuffer(QBuffer* this_ptr, QByteArray* a);
QT_CORE_C_EXPORT void qt_core_c_QBuffer_setData_data(QBuffer* this_ptr, const QByteArray* data);
QT_CORE_C_EXPORT void qt_core_c_QBuffer_setData_data_len(QBuffer* this_ptr, const char* data, int len);
QT_CORE_C_EXPORT qint64 qt_core_c_QBuffer_size(const QBuffer* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QBuffer_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QBuffer_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QBUFFER_H
