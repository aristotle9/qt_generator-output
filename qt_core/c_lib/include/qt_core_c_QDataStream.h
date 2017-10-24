#ifndef QT_CORE_C_QDATASTREAM_H
#define QT_CORE_C_QDATASTREAM_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QDataStream_abortTransaction(QDataStream* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QDataStream_atEnd(const QDataStream* this_ptr);
QT_CORE_C_EXPORT QDataStream::ByteOrder qt_core_c_QDataStream_byteOrder(const QDataStream* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QDataStream_commitTransaction(QDataStream* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDataStream_constructor_QByteArray(const QByteArray* arg1, QDataStream* output);
QT_CORE_C_EXPORT void qt_core_c_QDataStream_constructor_QIODevice(QIODevice* arg1, QDataStream* output);
QT_CORE_C_EXPORT void qt_core_c_QDataStream_constructor_no_args(QDataStream* output);
QT_CORE_C_EXPORT void qt_core_c_QDataStream_destructor(QDataStream* this_ptr);
QT_CORE_C_EXPORT QIODevice* qt_core_c_QDataStream_device(const QDataStream* this_ptr);
QT_CORE_C_EXPORT QDataStream::FloatingPointPrecision qt_core_c_QDataStream_floatingPointPrecision(const QDataStream* this_ptr);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shl_bool(QDataStream* this_ptr, bool i);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shl_char(QDataStream* this_ptr, const char* str);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shl_double(QDataStream* this_ptr, double f);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shl_float(QDataStream* this_ptr, float f);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shl_qint16(QDataStream* this_ptr, qint16 i);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shl_qint32(QDataStream* this_ptr, qint32 i);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shl_qint64(QDataStream* this_ptr, qint64 i);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shl_qint8(QDataStream* this_ptr, qint8 i);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shl_quint16(QDataStream* this_ptr, quint16 i);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shl_quint32(QDataStream* this_ptr, quint32 i);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shl_quint64(QDataStream* this_ptr, quint64 i);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shl_quint8(QDataStream* this_ptr, quint8 i);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shr_bool(QDataStream* this_ptr, bool* i);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shr_char(QDataStream* this_ptr, char** str);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shr_double(QDataStream* this_ptr, double* f);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shr_float(QDataStream* this_ptr, float* f);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shr_qint16(QDataStream* this_ptr, qint16* i);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shr_qint32(QDataStream* this_ptr, qint32* i);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shr_qint64(QDataStream* this_ptr, qint64* i);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shr_qint8(QDataStream* this_ptr, qint8* i);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shr_quint16(QDataStream* this_ptr, quint16* i);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shr_quint32(QDataStream* this_ptr, quint32* i);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shr_quint64(QDataStream* this_ptr, quint64* i);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_operator_shr_quint8(QDataStream* this_ptr, quint8* i);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_readBytes(QDataStream* this_ptr, char** arg1, unsigned int* len);
QT_CORE_C_EXPORT int qt_core_c_QDataStream_readRawData(QDataStream* this_ptr, char* arg1, int len);
QT_CORE_C_EXPORT void qt_core_c_QDataStream_resetStatus(QDataStream* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDataStream_rollbackTransaction(QDataStream* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDataStream_setByteOrder(QDataStream* this_ptr, QDataStream::ByteOrder arg1);
QT_CORE_C_EXPORT void qt_core_c_QDataStream_setDevice(QDataStream* this_ptr, QIODevice* arg1);
QT_CORE_C_EXPORT void qt_core_c_QDataStream_setFloatingPointPrecision(QDataStream* this_ptr, QDataStream::FloatingPointPrecision precision);
QT_CORE_C_EXPORT void qt_core_c_QDataStream_setStatus(QDataStream* this_ptr, QDataStream::Status status);
QT_CORE_C_EXPORT void qt_core_c_QDataStream_setVersion(QDataStream* this_ptr, int arg1);
QT_CORE_C_EXPORT int qt_core_c_QDataStream_skipRawData(QDataStream* this_ptr, int len);
QT_CORE_C_EXPORT void qt_core_c_QDataStream_startTransaction(QDataStream* this_ptr);
QT_CORE_C_EXPORT QDataStream::Status qt_core_c_QDataStream_status(const QDataStream* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDataStream_unsetDevice(QDataStream* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QDataStream_version(const QDataStream* this_ptr);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QDataStream_writeBytes(QDataStream* this_ptr, const char* arg1, unsigned int len);
QT_CORE_C_EXPORT int qt_core_c_QDataStream_writeRawData(QDataStream* this_ptr, const char* arg1, int len);

} // extern "C"

#endif // QT_CORE_C_QDATASTREAM_H
