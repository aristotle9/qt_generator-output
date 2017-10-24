#ifndef QT_CORE_C_QJSONOBJECT_H
#define QT_CORE_C_QJSONOBJECT_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QJsonObject_begin_to_output(QJsonObject* this_ptr, QJsonObject::iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_begin_to_output_const(const QJsonObject* this_ptr, QJsonObject::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_constBegin_to_output(const QJsonObject* this_ptr, QJsonObject::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_constEnd_to_output(const QJsonObject* this_ptr, QJsonObject::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_constFind_to_output_QLatin1String(const QJsonObject* this_ptr, const QLatin1String* key, QJsonObject::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_constFind_to_output_QString(const QJsonObject* this_ptr, const QString* key, QJsonObject::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_const_iterator_constructor_no_args(QJsonObject::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_const_iterator_constructor_obj_index(const QJsonObject* obj, int index, QJsonObject::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_const_iterator_constructor_other(const QJsonObject::iterator* other, QJsonObject::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_const_iterator_destructor(QJsonObject::const_iterator* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_const_iterator_key_to_output(const QJsonObject::const_iterator* this_ptr, QString* output);
QT_CORE_C_EXPORT QJsonObject::const_iterator* qt_core_c_QJsonObject_const_iterator_operator_add_assign(QJsonObject::const_iterator* this_ptr, int j);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_const_iterator_operator_add_to_output(const QJsonObject::const_iterator* this_ptr, int j, QJsonObject::const_iterator* output);
QT_CORE_C_EXPORT QJsonObject::const_iterator* qt_core_c_QJsonObject_const_iterator_operator_dec(QJsonObject::const_iterator* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_const_iterator_operator_dec_postfix_to_output(QJsonObject::const_iterator* this_ptr, int arg1, QJsonObject::const_iterator* output);
QT_CORE_C_EXPORT bool qt_core_c_QJsonObject_const_iterator_operator_eq_QJsonObject_const_iterator(const QJsonObject::const_iterator* this_ptr, const QJsonObject::const_iterator* other);
QT_CORE_C_EXPORT bool qt_core_c_QJsonObject_const_iterator_operator_eq_QJsonObject_iterator(const QJsonObject::const_iterator* this_ptr, const QJsonObject::iterator* other);
QT_CORE_C_EXPORT QJsonObject::const_iterator* qt_core_c_QJsonObject_const_iterator_operator_inc(QJsonObject::const_iterator* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_const_iterator_operator_inc_postfix_to_output(QJsonObject::const_iterator* this_ptr, int arg1, QJsonObject::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_const_iterator_operator_indirection_to_output(const QJsonObject::const_iterator* this_ptr, QJsonValue* output);
QT_CORE_C_EXPORT bool qt_core_c_QJsonObject_const_iterator_operator_neq_QJsonObject_const_iterator(const QJsonObject::const_iterator* this_ptr, const QJsonObject::const_iterator* other);
QT_CORE_C_EXPORT bool qt_core_c_QJsonObject_const_iterator_operator_neq_QJsonObject_iterator(const QJsonObject::const_iterator* this_ptr, const QJsonObject::iterator* other);
QT_CORE_C_EXPORT QJsonObject::const_iterator* qt_core_c_QJsonObject_const_iterator_operator_sub_assign(QJsonObject::const_iterator* this_ptr, int j);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_const_iterator_operator_sub_to_output(const QJsonObject::const_iterator* this_ptr, int j, QJsonObject::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_const_iterator_value_to_output(const QJsonObject::const_iterator* this_ptr, QJsonValue* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_constructor_no_args(QJsonObject* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_constructor_other(const QJsonObject* other, QJsonObject* output);
QT_CORE_C_EXPORT bool qt_core_c_QJsonObject_contains_QLatin1String(const QJsonObject* this_ptr, const QLatin1String* key);
QT_CORE_C_EXPORT bool qt_core_c_QJsonObject_contains_QString(const QJsonObject* this_ptr, const QString* key);
QT_CORE_C_EXPORT int qt_core_c_QJsonObject_count(const QJsonObject* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_destructor(QJsonObject* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QJsonObject_empty(const QJsonObject* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_end_to_output(QJsonObject* this_ptr, QJsonObject::iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_end_to_output_const(const QJsonObject* this_ptr, QJsonObject::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_erase_to_output(QJsonObject* this_ptr, const QJsonObject::iterator* it, QJsonObject::iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_find_to_output_QLatin1String(QJsonObject* this_ptr, const QLatin1String* key, QJsonObject::iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_find_to_output_QString(QJsonObject* this_ptr, const QString* key, QJsonObject::iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_find_to_output_const_QLatin1String(const QJsonObject* this_ptr, const QLatin1String* key, QJsonObject::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_find_to_output_const_QString(const QJsonObject* this_ptr, const QString* key, QJsonObject::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_fromVariantHash_to_output(const QHash< QString, QVariant >* map, QJsonObject* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_fromVariantMap_to_output(const QMap< QString, QVariant >* map, QJsonObject* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_insert_to_output(QJsonObject* this_ptr, const QString* key, const QJsonValue* value, QJsonObject::iterator* output);
QT_CORE_C_EXPORT bool qt_core_c_QJsonObject_isEmpty(const QJsonObject* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_iterator_constructor_no_args(QJsonObject::iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_iterator_constructor_obj_index(QJsonObject* obj, int index, QJsonObject::iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_iterator_destructor(QJsonObject::iterator* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_iterator_key_to_output(const QJsonObject::iterator* this_ptr, QString* output);
QT_CORE_C_EXPORT QJsonObject::iterator* qt_core_c_QJsonObject_iterator_operator_add_assign(QJsonObject::iterator* this_ptr, int j);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_iterator_operator_add_to_output(const QJsonObject::iterator* this_ptr, int j, QJsonObject::iterator* output);
QT_CORE_C_EXPORT QJsonObject::iterator* qt_core_c_QJsonObject_iterator_operator_dec(QJsonObject::iterator* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_iterator_operator_dec_postfix_to_output(QJsonObject::iterator* this_ptr, int arg1, QJsonObject::iterator* output);
QT_CORE_C_EXPORT bool qt_core_c_QJsonObject_iterator_operator_eq_QJsonObject_const_iterator(const QJsonObject::iterator* this_ptr, const QJsonObject::const_iterator* other);
QT_CORE_C_EXPORT bool qt_core_c_QJsonObject_iterator_operator_eq_QJsonObject_iterator(const QJsonObject::iterator* this_ptr, const QJsonObject::iterator* other);
QT_CORE_C_EXPORT QJsonObject::iterator* qt_core_c_QJsonObject_iterator_operator_inc(QJsonObject::iterator* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_iterator_operator_inc_postfix_to_output(QJsonObject::iterator* this_ptr, int arg1, QJsonObject::iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_iterator_operator_indirection_to_output(const QJsonObject::iterator* this_ptr, QJsonValueRef* output);
QT_CORE_C_EXPORT bool qt_core_c_QJsonObject_iterator_operator_neq_QJsonObject_const_iterator(const QJsonObject::iterator* this_ptr, const QJsonObject::const_iterator* other);
QT_CORE_C_EXPORT bool qt_core_c_QJsonObject_iterator_operator_neq_QJsonObject_iterator(const QJsonObject::iterator* this_ptr, const QJsonObject::iterator* other);
QT_CORE_C_EXPORT QJsonObject::iterator* qt_core_c_QJsonObject_iterator_operator_sub_assign(QJsonObject::iterator* this_ptr, int j);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_iterator_operator_sub_to_output(const QJsonObject::iterator* this_ptr, int j, QJsonObject::iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_iterator_value_to_output(const QJsonObject::iterator* this_ptr, QJsonValueRef* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_keys_to_output(const QJsonObject* this_ptr, QStringList* output);
QT_CORE_C_EXPORT int qt_core_c_QJsonObject_length(const QJsonObject* this_ptr);
QT_CORE_C_EXPORT QJsonObject* qt_core_c_QJsonObject_operator_assign(QJsonObject* this_ptr, const QJsonObject* other);
QT_CORE_C_EXPORT bool qt_core_c_QJsonObject_operator_eq(const QJsonObject* this_ptr, const QJsonObject* other);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_operator_index_to_output_QLatin1String(QJsonObject* this_ptr, const QLatin1String* key, QJsonValueRef* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_operator_index_to_output_QString(QJsonObject* this_ptr, const QString* key, QJsonValueRef* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_operator_index_to_output_const_QLatin1String(const QJsonObject* this_ptr, const QLatin1String* key, QJsonValue* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_operator_index_to_output_const_QString(const QJsonObject* this_ptr, const QString* key, QJsonValue* output);
QT_CORE_C_EXPORT bool qt_core_c_QJsonObject_operator_neq(const QJsonObject* this_ptr, const QJsonObject* other);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_remove(QJsonObject* this_ptr, const QString* key);
QT_CORE_C_EXPORT int qt_core_c_QJsonObject_size(const QJsonObject* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_take_to_output(QJsonObject* this_ptr, const QString* key, QJsonValue* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_toVariantHash_to_output(const QJsonObject* this_ptr, QHash< QString, QVariant >* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_toVariantMap_to_output(const QJsonObject* this_ptr, QMap< QString, QVariant >* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_value_to_output_QLatin1String(const QJsonObject* this_ptr, const QLatin1String* key, QJsonValue* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonObject_value_to_output_QString(const QJsonObject* this_ptr, const QString* key, QJsonValue* output);

} // extern "C"

#endif // QT_CORE_C_QJSONOBJECT_H
