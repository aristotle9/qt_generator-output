#ifndef QT_CORE_C_QJSONDOCUMENT_H
#define QT_CORE_C_QJSONDOCUMENT_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QJsonDocument_array_to_output(const QJsonDocument* this_ptr, QJsonArray* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonDocument_constructor_array(const QJsonArray* array, QJsonDocument* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonDocument_constructor_no_args(QJsonDocument* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonDocument_constructor_object(const QJsonObject* object, QJsonDocument* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonDocument_constructor_other(const QJsonDocument* other, QJsonDocument* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonDocument_destructor(QJsonDocument* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonDocument_fromBinaryData_to_output_data(const QByteArray* data, QJsonDocument* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonDocument_fromBinaryData_to_output_data_validation(const QByteArray* data, QJsonDocument::DataValidation validation, QJsonDocument* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonDocument_fromJson_to_output_json(const QByteArray* json, QJsonDocument* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonDocument_fromJson_to_output_json_error(const QByteArray* json, QJsonParseError* error, QJsonDocument* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonDocument_fromRawData_to_output_data_size(const char* data, int size, QJsonDocument* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonDocument_fromRawData_to_output_data_size_validation(const char* data, int size, QJsonDocument::DataValidation validation, QJsonDocument* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonDocument_fromVariant_to_output(const QVariant* variant, QJsonDocument* output);
QT_CORE_C_EXPORT bool qt_core_c_QJsonDocument_isArray(const QJsonDocument* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QJsonDocument_isEmpty(const QJsonDocument* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QJsonDocument_isNull(const QJsonDocument* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QJsonDocument_isObject(const QJsonDocument* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonDocument_object_to_output(const QJsonDocument* this_ptr, QJsonObject* output);
QT_CORE_C_EXPORT QJsonDocument* qt_core_c_QJsonDocument_operator_assign(QJsonDocument* this_ptr, const QJsonDocument* other);
QT_CORE_C_EXPORT bool qt_core_c_QJsonDocument_operator_eq(const QJsonDocument* this_ptr, const QJsonDocument* other);
QT_CORE_C_EXPORT bool qt_core_c_QJsonDocument_operator_neq(const QJsonDocument* this_ptr, const QJsonDocument* other);
QT_CORE_C_EXPORT const char* qt_core_c_QJsonDocument_rawData(const QJsonDocument* this_ptr, int* size);
QT_CORE_C_EXPORT void qt_core_c_QJsonDocument_setArray(QJsonDocument* this_ptr, const QJsonArray* array);
QT_CORE_C_EXPORT void qt_core_c_QJsonDocument_setObject(QJsonDocument* this_ptr, const QJsonObject* object);
QT_CORE_C_EXPORT void qt_core_c_QJsonDocument_toBinaryData_to_output(const QJsonDocument* this_ptr, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonDocument_toJson_to_output_format(const QJsonDocument* this_ptr, QJsonDocument::JsonFormat format, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonDocument_toJson_to_output_no_args(const QJsonDocument* this_ptr, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonDocument_toVariant_to_output(const QJsonDocument* this_ptr, QVariant* output);

} // extern "C"

#endif // QT_CORE_C_QJSONDOCUMENT_H
