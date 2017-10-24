#ifndef QT_CORE_C_QJSONVALUEREF_H
#define QT_CORE_C_QJSONVALUEREF_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QJsonValueRef_constructor_array_idx(QJsonArray* array, int idx, QJsonValueRef* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonValueRef_constructor_object_idx(QJsonObject* object, int idx, QJsonValueRef* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonValueRef_convert_to_QJsonValue_to_output(const QJsonValueRef* this_ptr, QJsonValue* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonValueRef_destructor(QJsonValueRef* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QJsonValueRef_isArray(const QJsonValueRef* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QJsonValueRef_isBool(const QJsonValueRef* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QJsonValueRef_isDouble(const QJsonValueRef* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QJsonValueRef_isNull(const QJsonValueRef* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QJsonValueRef_isObject(const QJsonValueRef* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QJsonValueRef_isString(const QJsonValueRef* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QJsonValueRef_isUndefined(const QJsonValueRef* this_ptr);
QT_CORE_C_EXPORT QJsonValueRef* qt_core_c_QJsonValueRef_operator_assign_QJsonValue(QJsonValueRef* this_ptr, const QJsonValue* val);
QT_CORE_C_EXPORT QJsonValueRef* qt_core_c_QJsonValueRef_operator_assign_QJsonValueRef(QJsonValueRef* this_ptr, const QJsonValueRef* val);
QT_CORE_C_EXPORT bool qt_core_c_QJsonValueRef_operator_eq(const QJsonValueRef* this_ptr, const QJsonValue* other);
QT_CORE_C_EXPORT bool qt_core_c_QJsonValueRef_operator_neq(const QJsonValueRef* this_ptr, const QJsonValue* other);
QT_CORE_C_EXPORT void qt_core_c_QJsonValueRef_toArray_to_output(const QJsonValueRef* this_ptr, QJsonArray* output);
QT_CORE_C_EXPORT bool qt_core_c_QJsonValueRef_toBool_defaultValue(const QJsonValueRef* this_ptr, bool defaultValue);
QT_CORE_C_EXPORT bool qt_core_c_QJsonValueRef_toBool_no_args(const QJsonValueRef* this_ptr);
QT_CORE_C_EXPORT double qt_core_c_QJsonValueRef_toDouble_defaultValue(const QJsonValueRef* this_ptr, double defaultValue);
QT_CORE_C_EXPORT double qt_core_c_QJsonValueRef_toDouble_no_args(const QJsonValueRef* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QJsonValueRef_toInt_defaultValue(const QJsonValueRef* this_ptr, int defaultValue);
QT_CORE_C_EXPORT int qt_core_c_QJsonValueRef_toInt_no_args(const QJsonValueRef* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonValueRef_toObject_to_output(const QJsonValueRef* this_ptr, QJsonObject* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonValueRef_toString_to_output_defaultValue(const QJsonValueRef* this_ptr, const QString* defaultValue, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonValueRef_toString_to_output_no_args(const QJsonValueRef* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonValueRef_toVariant_to_output(const QJsonValueRef* this_ptr, QVariant* output);

} // extern "C"

#endif // QT_CORE_C_QJSONVALUEREF_H
