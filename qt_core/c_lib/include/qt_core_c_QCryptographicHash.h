#ifndef QT_CORE_C_QCRYPTOGRAPHICHASH_H
#define QT_CORE_C_QCRYPTOGRAPHICHASH_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QCryptographicHash_addData_data(QCryptographicHash* this_ptr, const QByteArray* data);
QT_CORE_C_EXPORT void qt_core_c_QCryptographicHash_addData_data_length(QCryptographicHash* this_ptr, const char* data, int length);
QT_CORE_C_EXPORT bool qt_core_c_QCryptographicHash_addData_device(QCryptographicHash* this_ptr, QIODevice* device);
QT_CORE_C_EXPORT void qt_core_c_QCryptographicHash_constructor(QCryptographicHash::Algorithm method, QCryptographicHash* output);
QT_CORE_C_EXPORT void qt_core_c_QCryptographicHash_destructor(QCryptographicHash* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QCryptographicHash_hash_to_output(const QByteArray* data, QCryptographicHash::Algorithm method, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QCryptographicHash_reset(QCryptographicHash* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QCryptographicHash_result_to_output(const QCryptographicHash* this_ptr, QByteArray* output);

} // extern "C"

#endif // QT_CORE_C_QCRYPTOGRAPHICHASH_H
