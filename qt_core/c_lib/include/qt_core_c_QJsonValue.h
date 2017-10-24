#ifndef QT_CORE_C_QJSONVALUE_H
#define QT_CORE_C_QJSONVALUE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QJsonValue_constructor_QJsonArray(const QJsonArray* a, QJsonValue* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonValue_constructor_QJsonObject(const QJsonObject* o, QJsonValue* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonValue_constructor_QJsonValue(const QJsonValue* other, QJsonValue* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonValue_constructor_QJsonValue_Type(QJsonValue::Type arg1, QJsonValue* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonValue_constructor_QLatin1String(const QLatin1String* s, QJsonValue* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonValue_constructor_QString(const QString* s, QJsonValue* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonValue_constructor_bool(bool b, QJsonValue* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonValue_constructor_char(const char* s, QJsonValue* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonValue_constructor_double(double n, QJsonValue* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonValue_constructor_int(int n, QJsonValue* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonValue_constructor_no_args(QJsonValue* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonValue_constructor_qint64(qint64 n, QJsonValue* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonValue_destructor(QJsonValue* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonValue_fromVariant_to_output(const QVariant* variant, QJsonValue* output);
QT_CORE_C_EXPORT bool qt_core_c_QJsonValue_isArray(const QJsonValue* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QJsonValue_isBool(const QJsonValue* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QJsonValue_isDouble(const QJsonValue* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QJsonValue_isNull(const QJsonValue* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QJsonValue_isObject(const QJsonValue* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QJsonValue_isString(const QJsonValue* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QJsonValue_isUndefined(const QJsonValue* this_ptr);
QT_CORE_C_EXPORT QJsonValue* qt_core_c_QJsonValue_operator_assign(QJsonValue* this_ptr, const QJsonValue* other);
QT_CORE_C_EXPORT bool qt_core_c_QJsonValue_operator_eq(const QJsonValue* this_ptr, const QJsonValue* other);
QT_CORE_C_EXPORT bool qt_core_c_QJsonValue_operator_neq(const QJsonValue* this_ptr, const QJsonValue* other);
QT_CORE_C_EXPORT void qt_core_c_QJsonValue_toArray_to_output_defaultValue(const QJsonValue* this_ptr, const QJsonArray* defaultValue, QJsonArray* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonValue_toArray_to_output_no_args(const QJsonValue* this_ptr, QJsonArray* output);
QT_CORE_C_EXPORT bool qt_core_c_QJsonValue_toBool_defaultValue(const QJsonValue* this_ptr, bool defaultValue);
QT_CORE_C_EXPORT bool qt_core_c_QJsonValue_toBool_no_args(const QJsonValue* this_ptr);
QT_CORE_C_EXPORT double qt_core_c_QJsonValue_toDouble_defaultValue(const QJsonValue* this_ptr, double defaultValue);
QT_CORE_C_EXPORT double qt_core_c_QJsonValue_toDouble_no_args(const QJsonValue* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QJsonValue_toInt_defaultValue(const QJsonValue* this_ptr, int defaultValue);
QT_CORE_C_EXPORT int qt_core_c_QJsonValue_toInt_no_args(const QJsonValue* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonValue_toObject_to_output_defaultValue(const QJsonValue* this_ptr, const QJsonObject* defaultValue, QJsonObject* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonValue_toObject_to_output_no_args(const QJsonValue* this_ptr, QJsonObject* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonValue_toString_to_output_defaultValue(const QJsonValue* this_ptr, const QString* defaultValue, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonValue_toString_to_output_no_args(const QJsonValue* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonValue_toVariant_to_output(const QJsonValue* this_ptr, QVariant* output);
QT_CORE_C_EXPORT QJsonValue::Type qt_core_c_QJsonValue_type(const QJsonValue* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QJSONVALUE_H
