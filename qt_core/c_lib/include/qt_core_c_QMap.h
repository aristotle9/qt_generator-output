#ifndef QT_CORE_C_QMAP_H
#define QT_CORE_C_QMAP_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QMap_QString_QVariant_clear(QMap< QString, QVariant >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMap_QString_QVariant_constructor_no_args(QMap< QString, QVariant >* output);
QT_CORE_C_EXPORT void qt_core_c_QMap_QString_QVariant_constructor_other(const QMap< QString, QVariant >* other, QMap< QString, QVariant >* output);
QT_CORE_C_EXPORT bool qt_core_c_QMap_QString_QVariant_contains(const QMap< QString, QVariant >* this_ptr, const QString* key);
QT_CORE_C_EXPORT int qt_core_c_QMap_QString_QVariant_count_key(const QMap< QString, QVariant >* this_ptr, const QString* key);
QT_CORE_C_EXPORT int qt_core_c_QMap_QString_QVariant_count_no_args(const QMap< QString, QVariant >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMap_QString_QVariant_destructor(QMap< QString, QVariant >* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMap_QString_QVariant_empty(const QMap< QString, QVariant >* this_ptr);
QT_CORE_C_EXPORT QVariant* qt_core_c_QMap_QString_QVariant_first(QMap< QString, QVariant >* this_ptr);
QT_CORE_C_EXPORT const QString* qt_core_c_QMap_QString_QVariant_firstKey(const QMap< QString, QVariant >* this_ptr);
QT_CORE_C_EXPORT const QVariant* qt_core_c_QMap_QString_QVariant_first_const(const QMap< QString, QVariant >* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMap_QString_QVariant_isEmpty(const QMap< QString, QVariant >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMap_QString_QVariant_key_to_output_value(const QMap< QString, QVariant >* this_ptr, const QVariant* value, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QMap_QString_QVariant_key_to_output_value_defaultKey(const QMap< QString, QVariant >* this_ptr, const QVariant* value, const QString* defaultKey, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QMap_QString_QVariant_keys_to_output_no_args(const QMap< QString, QVariant >* this_ptr, QList< QString >* output);
QT_CORE_C_EXPORT void qt_core_c_QMap_QString_QVariant_keys_to_output_value(const QMap< QString, QVariant >* this_ptr, const QVariant* value, QList< QString >* output);
QT_CORE_C_EXPORT QVariant* qt_core_c_QMap_QString_QVariant_last(QMap< QString, QVariant >* this_ptr);
QT_CORE_C_EXPORT const QString* qt_core_c_QMap_QString_QVariant_lastKey(const QMap< QString, QVariant >* this_ptr);
QT_CORE_C_EXPORT const QVariant* qt_core_c_QMap_QString_QVariant_last_const(const QMap< QString, QVariant >* this_ptr);
QT_CORE_C_EXPORT QMap< QString, QVariant >* qt_core_c_QMap_QString_QVariant_operator_assign(QMap< QString, QVariant >* this_ptr, const QMap< QString, QVariant >* other);
QT_CORE_C_EXPORT bool qt_core_c_QMap_QString_QVariant_operator_eq(const QMap< QString, QVariant >* this_ptr, const QMap< QString, QVariant >* other);
QT_CORE_C_EXPORT QVariant* qt_core_c_QMap_QString_QVariant_operator_index(QMap< QString, QVariant >* this_ptr, const QString* key);
QT_CORE_C_EXPORT void qt_core_c_QMap_QString_QVariant_operator_index_to_output(const QMap< QString, QVariant >* this_ptr, const QString* key, QVariant* output);
QT_CORE_C_EXPORT bool qt_core_c_QMap_QString_QVariant_operator_neq(const QMap< QString, QVariant >* this_ptr, const QMap< QString, QVariant >* other);
QT_CORE_C_EXPORT int qt_core_c_QMap_QString_QVariant_remove(QMap< QString, QVariant >* this_ptr, const QString* key);
QT_CORE_C_EXPORT int qt_core_c_QMap_QString_QVariant_size(const QMap< QString, QVariant >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMap_QString_QVariant_swap(QMap< QString, QVariant >* this_ptr, QMap< QString, QVariant >* other);
QT_CORE_C_EXPORT void qt_core_c_QMap_QString_QVariant_take_to_output(QMap< QString, QVariant >* this_ptr, const QString* key, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QMap_QString_QVariant_uniqueKeys_to_output(const QMap< QString, QVariant >* this_ptr, QList< QString >* output);
QT_CORE_C_EXPORT QMap< QString, QVariant >* qt_core_c_QMap_QString_QVariant_unite(QMap< QString, QVariant >* this_ptr, const QMap< QString, QVariant >* other);
QT_CORE_C_EXPORT void qt_core_c_QMap_QString_QVariant_value_to_output_key(const QMap< QString, QVariant >* this_ptr, const QString* key, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QMap_QString_QVariant_value_to_output_key_defaultValue(const QMap< QString, QVariant >* this_ptr, const QString* key, const QVariant* defaultValue, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QMap_QString_QVariant_values_to_output_key(const QMap< QString, QVariant >* this_ptr, const QString* key, QList< QVariant >* output);
QT_CORE_C_EXPORT void qt_core_c_QMap_QString_QVariant_values_to_output_no_args(const QMap< QString, QVariant >* this_ptr, QList< QVariant >* output);
QT_CORE_C_EXPORT void qt_core_c_QMap_int_QVariant_clear(QMap< int, QVariant >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMap_int_QVariant_constructor_no_args(QMap< int, QVariant >* output);
QT_CORE_C_EXPORT void qt_core_c_QMap_int_QVariant_constructor_other(const QMap< int, QVariant >* other, QMap< int, QVariant >* output);
QT_CORE_C_EXPORT bool qt_core_c_QMap_int_QVariant_contains(const QMap< int, QVariant >* this_ptr, const int* key);
QT_CORE_C_EXPORT int qt_core_c_QMap_int_QVariant_count_key(const QMap< int, QVariant >* this_ptr, const int* key);
QT_CORE_C_EXPORT int qt_core_c_QMap_int_QVariant_count_no_args(const QMap< int, QVariant >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMap_int_QVariant_destructor(QMap< int, QVariant >* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMap_int_QVariant_empty(const QMap< int, QVariant >* this_ptr);
QT_CORE_C_EXPORT QVariant* qt_core_c_QMap_int_QVariant_first(QMap< int, QVariant >* this_ptr);
QT_CORE_C_EXPORT const int* qt_core_c_QMap_int_QVariant_firstKey(const QMap< int, QVariant >* this_ptr);
QT_CORE_C_EXPORT const QVariant* qt_core_c_QMap_int_QVariant_first_const(const QMap< int, QVariant >* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMap_int_QVariant_isEmpty(const QMap< int, QVariant >* this_ptr);
QT_CORE_C_EXPORT const int qt_core_c_QMap_int_QVariant_key_value(const QMap< int, QVariant >* this_ptr, const QVariant* value);
QT_CORE_C_EXPORT const int qt_core_c_QMap_int_QVariant_key_value_defaultKey(const QMap< int, QVariant >* this_ptr, const QVariant* value, const int* defaultKey);
QT_CORE_C_EXPORT void qt_core_c_QMap_int_QVariant_keys_to_output_no_args(const QMap< int, QVariant >* this_ptr, QList< int >* output);
QT_CORE_C_EXPORT void qt_core_c_QMap_int_QVariant_keys_to_output_value(const QMap< int, QVariant >* this_ptr, const QVariant* value, QList< int >* output);
QT_CORE_C_EXPORT QVariant* qt_core_c_QMap_int_QVariant_last(QMap< int, QVariant >* this_ptr);
QT_CORE_C_EXPORT const int* qt_core_c_QMap_int_QVariant_lastKey(const QMap< int, QVariant >* this_ptr);
QT_CORE_C_EXPORT const QVariant* qt_core_c_QMap_int_QVariant_last_const(const QMap< int, QVariant >* this_ptr);
QT_CORE_C_EXPORT QMap< int, QVariant >* qt_core_c_QMap_int_QVariant_operator_assign(QMap< int, QVariant >* this_ptr, const QMap< int, QVariant >* other);
QT_CORE_C_EXPORT bool qt_core_c_QMap_int_QVariant_operator_eq(const QMap< int, QVariant >* this_ptr, const QMap< int, QVariant >* other);
QT_CORE_C_EXPORT QVariant* qt_core_c_QMap_int_QVariant_operator_index(QMap< int, QVariant >* this_ptr, const int* key);
QT_CORE_C_EXPORT void qt_core_c_QMap_int_QVariant_operator_index_to_output(const QMap< int, QVariant >* this_ptr, const int* key, QVariant* output);
QT_CORE_C_EXPORT bool qt_core_c_QMap_int_QVariant_operator_neq(const QMap< int, QVariant >* this_ptr, const QMap< int, QVariant >* other);
QT_CORE_C_EXPORT int qt_core_c_QMap_int_QVariant_remove(QMap< int, QVariant >* this_ptr, const int* key);
QT_CORE_C_EXPORT int qt_core_c_QMap_int_QVariant_size(const QMap< int, QVariant >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMap_int_QVariant_swap(QMap< int, QVariant >* this_ptr, QMap< int, QVariant >* other);
QT_CORE_C_EXPORT void qt_core_c_QMap_int_QVariant_take_to_output(QMap< int, QVariant >* this_ptr, const int* key, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QMap_int_QVariant_uniqueKeys_to_output(const QMap< int, QVariant >* this_ptr, QList< int >* output);
QT_CORE_C_EXPORT QMap< int, QVariant >* qt_core_c_QMap_int_QVariant_unite(QMap< int, QVariant >* this_ptr, const QMap< int, QVariant >* other);
QT_CORE_C_EXPORT void qt_core_c_QMap_int_QVariant_value_to_output_key(const QMap< int, QVariant >* this_ptr, const int* key, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QMap_int_QVariant_value_to_output_key_defaultValue(const QMap< int, QVariant >* this_ptr, const int* key, const QVariant* defaultValue, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QMap_int_QVariant_values_to_output_key(const QMap< int, QVariant >* this_ptr, const int* key, QList< QVariant >* output);
QT_CORE_C_EXPORT void qt_core_c_QMap_int_QVariant_values_to_output_no_args(const QMap< int, QVariant >* this_ptr, QList< QVariant >* output);

} // extern "C"

#endif // QT_CORE_C_QMAP_H
