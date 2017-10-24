#ifndef QT_CORE_C_QIODEVICE_H
#define QT_CORE_C_QIODEVICE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QIODevice* qt_core_c_QIODevice_G_dynamic_cast_QIODevice_ptr(QObject* ptr);
QT_CORE_C_EXPORT QIODevice* qt_core_c_QIODevice_G_static_cast_QIODevice_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QIODevice_G_static_cast_QObject_ptr(QIODevice* ptr);
QT_CORE_C_EXPORT bool qt_core_c_QIODevice_atEnd(const QIODevice* this_ptr);
QT_CORE_C_EXPORT qint64 qt_core_c_QIODevice_bytesAvailable(const QIODevice* this_ptr);
QT_CORE_C_EXPORT qint64 qt_core_c_QIODevice_bytesToWrite(const QIODevice* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QIODevice_canReadLine(const QIODevice* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QIODevice_close(QIODevice* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QIODevice_commitTransaction(QIODevice* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QIODevice_currentReadChannel(const QIODevice* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QIODevice_currentWriteChannel(const QIODevice* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QIODevice_delete(QIODevice* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QIODevice_errorString_to_output(const QIODevice* this_ptr, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QIODevice_getChar(QIODevice* this_ptr, char* c);
QT_CORE_C_EXPORT bool qt_core_c_QIODevice_isOpen(const QIODevice* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QIODevice_isReadable(const QIODevice* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QIODevice_isSequential(const QIODevice* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QIODevice_isTextModeEnabled(const QIODevice* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QIODevice_isTransactionStarted(const QIODevice* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QIODevice_isWritable(const QIODevice* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QIODevice_metaObject(const QIODevice* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QIODevice_open(QIODevice* this_ptr, unsigned int mode);
QT_CORE_C_EXPORT unsigned int qt_core_c_QIODevice_openMode(const QIODevice* this_ptr);
QT_CORE_C_EXPORT qint64 qt_core_c_QIODevice_peek(QIODevice* this_ptr, char* data, qint64 maxlen);
QT_CORE_C_EXPORT void qt_core_c_QIODevice_peek_to_output(QIODevice* this_ptr, qint64 maxlen, QByteArray* output);
QT_CORE_C_EXPORT qint64 qt_core_c_QIODevice_pos(const QIODevice* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QIODevice_putChar(QIODevice* this_ptr, char c);
QT_CORE_C_EXPORT qint64 qt_core_c_QIODevice_read(QIODevice* this_ptr, char* data, qint64 maxlen);
QT_CORE_C_EXPORT void qt_core_c_QIODevice_readAll_to_output(QIODevice* this_ptr, QByteArray* output);
QT_CORE_C_EXPORT int qt_core_c_QIODevice_readChannelCount(const QIODevice* this_ptr);
QT_CORE_C_EXPORT qint64 qt_core_c_QIODevice_readLine(QIODevice* this_ptr, char* data, qint64 maxlen);
QT_CORE_C_EXPORT void qt_core_c_QIODevice_readLine_to_output_maxlen(QIODevice* this_ptr, qint64 maxlen, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QIODevice_readLine_to_output_no_args(QIODevice* this_ptr, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QIODevice_read_to_output(QIODevice* this_ptr, qint64 maxlen, QByteArray* output);
QT_CORE_C_EXPORT bool qt_core_c_QIODevice_reset(QIODevice* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QIODevice_rollbackTransaction(QIODevice* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QIODevice_seek(QIODevice* this_ptr, qint64 pos);
QT_CORE_C_EXPORT void qt_core_c_QIODevice_setCurrentReadChannel(QIODevice* this_ptr, int channel);
QT_CORE_C_EXPORT void qt_core_c_QIODevice_setCurrentWriteChannel(QIODevice* this_ptr, int channel);
QT_CORE_C_EXPORT void qt_core_c_QIODevice_setTextModeEnabled(QIODevice* this_ptr, bool enabled);
QT_CORE_C_EXPORT qint64 qt_core_c_QIODevice_size(const QIODevice* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QIODevice_startTransaction(QIODevice* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QIODevice_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QIODevice_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QIODevice_ungetChar(QIODevice* this_ptr, char c);
QT_CORE_C_EXPORT bool qt_core_c_QIODevice_waitForBytesWritten(QIODevice* this_ptr, int msecs);
QT_CORE_C_EXPORT bool qt_core_c_QIODevice_waitForReadyRead(QIODevice* this_ptr, int msecs);
QT_CORE_C_EXPORT int qt_core_c_QIODevice_writeChannelCount(const QIODevice* this_ptr);
QT_CORE_C_EXPORT qint64 qt_core_c_QIODevice_write_QByteArray(QIODevice* this_ptr, const QByteArray* data);
QT_CORE_C_EXPORT qint64 qt_core_c_QIODevice_write_char(QIODevice* this_ptr, const char* data);
QT_CORE_C_EXPORT qint64 qt_core_c_QIODevice_write_char_qint64(QIODevice* this_ptr, const char* data, qint64 len);

} // extern "C"

#endif // QT_CORE_C_QIODEVICE_H
