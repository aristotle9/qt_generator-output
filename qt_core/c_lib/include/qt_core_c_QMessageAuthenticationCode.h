#ifndef QT_CORE_C_QMESSAGEAUTHENTICATIONCODE_H
#define QT_CORE_C_QMESSAGEAUTHENTICATIONCODE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QMessageAuthenticationCode_addData_data(QMessageAuthenticationCode* this_ptr, const QByteArray* data);
QT_CORE_C_EXPORT void qt_core_c_QMessageAuthenticationCode_addData_data_length(QMessageAuthenticationCode* this_ptr, const char* data, int length);
QT_CORE_C_EXPORT bool qt_core_c_QMessageAuthenticationCode_addData_device(QMessageAuthenticationCode* this_ptr, QIODevice* device);
QT_CORE_C_EXPORT void qt_core_c_QMessageAuthenticationCode_constructor_method(const QCryptographicHash::Algorithm* method, QMessageAuthenticationCode* output);
QT_CORE_C_EXPORT void qt_core_c_QMessageAuthenticationCode_constructor_method_key(const QCryptographicHash::Algorithm* method, const QByteArray* key, QMessageAuthenticationCode* output);
QT_CORE_C_EXPORT void qt_core_c_QMessageAuthenticationCode_destructor(QMessageAuthenticationCode* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMessageAuthenticationCode_hash_to_output(const QByteArray* message, const QByteArray* key, const QCryptographicHash::Algorithm* method, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QMessageAuthenticationCode_reset(QMessageAuthenticationCode* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMessageAuthenticationCode_result_to_output(const QMessageAuthenticationCode* this_ptr, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QMessageAuthenticationCode_setKey(QMessageAuthenticationCode* this_ptr, const QByteArray* key);

} // extern "C"

#endif // QT_CORE_C_QMESSAGEAUTHENTICATIONCODE_H
