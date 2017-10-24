#ifndef QT_CORE_C_QHASH_H
#define QT_CORE_C_QHASH_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT int qt_core_c_QHash_QString_QVariant_capacity(const QHash< QString, QVariant >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QHash_QString_QVariant_clear(QHash< QString, QVariant >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QHash_QString_QVariant_constructor(QHash< QString, QVariant >* output);
QT_CORE_C_EXPORT bool qt_core_c_QHash_QString_QVariant_contains(const QHash< QString, QVariant >* this_ptr, const QString* key);
QT_CORE_C_EXPORT int qt_core_c_QHash_QString_QVariant_count_key(const QHash< QString, QVariant >* this_ptr, const QString* key);
QT_CORE_C_EXPORT int qt_core_c_QHash_QString_QVariant_count_no_args(const QHash< QString, QVariant >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QHash_QString_QVariant_destructor(QHash< QString, QVariant >* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QHash_QString_QVariant_empty(const QHash< QString, QVariant >* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QHash_QString_QVariant_isEmpty(const QHash< QString, QVariant >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QHash_QString_QVariant_key_to_output_value(const QHash< QString, QVariant >* this_ptr, const QVariant* value, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QHash_QString_QVariant_key_to_output_value_defaultKey(const QHash< QString, QVariant >* this_ptr, const QVariant* value, const QString* defaultKey, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QHash_QString_QVariant_keys_to_output_no_args(const QHash< QString, QVariant >* this_ptr, QList< QString >* output);
QT_CORE_C_EXPORT void qt_core_c_QHash_QString_QVariant_keys_to_output_value(const QHash< QString, QVariant >* this_ptr, const QVariant* value, QList< QString >* output);
QT_CORE_C_EXPORT QVariant* qt_core_c_QHash_QString_QVariant_operator_index(QHash< QString, QVariant >* this_ptr, const QString* key);
QT_CORE_C_EXPORT void qt_core_c_QHash_QString_QVariant_operator_index_to_output(const QHash< QString, QVariant >* this_ptr, const QString* key, QVariant* output);
QT_CORE_C_EXPORT int qt_core_c_QHash_QString_QVariant_remove(QHash< QString, QVariant >* this_ptr, const QString* key);
QT_CORE_C_EXPORT void qt_core_c_QHash_QString_QVariant_reserve(QHash< QString, QVariant >* this_ptr, int size);
QT_CORE_C_EXPORT int qt_core_c_QHash_QString_QVariant_size(const QHash< QString, QVariant >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QHash_QString_QVariant_squeeze(QHash< QString, QVariant >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QHash_QString_QVariant_take_to_output(QHash< QString, QVariant >* this_ptr, const QString* key, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QHash_QString_QVariant_uniqueKeys_to_output(const QHash< QString, QVariant >* this_ptr, QList< QString >* output);
QT_CORE_C_EXPORT void qt_core_c_QHash_QString_QVariant_value_to_output_key(const QHash< QString, QVariant >* this_ptr, const QString* key, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QHash_QString_QVariant_value_to_output_key_defaultValue(const QHash< QString, QVariant >* this_ptr, const QString* key, const QVariant* defaultValue, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QHash_QString_QVariant_values_to_output_key(const QHash< QString, QVariant >* this_ptr, const QString* key, QList< QVariant >* output);
QT_CORE_C_EXPORT void qt_core_c_QHash_QString_QVariant_values_to_output_no_args(const QHash< QString, QVariant >* this_ptr, QList< QVariant >* output);
QT_CORE_C_EXPORT int qt_core_c_QHash_int_QByteArray_capacity(const QHash< int, QByteArray >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QHash_int_QByteArray_clear(QHash< int, QByteArray >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QHash_int_QByteArray_constructor(QHash< int, QByteArray >* output);
QT_CORE_C_EXPORT bool qt_core_c_QHash_int_QByteArray_contains(const QHash< int, QByteArray >* this_ptr, const int* key);
QT_CORE_C_EXPORT int qt_core_c_QHash_int_QByteArray_count_key(const QHash< int, QByteArray >* this_ptr, const int* key);
QT_CORE_C_EXPORT int qt_core_c_QHash_int_QByteArray_count_no_args(const QHash< int, QByteArray >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QHash_int_QByteArray_destructor(QHash< int, QByteArray >* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QHash_int_QByteArray_empty(const QHash< int, QByteArray >* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QHash_int_QByteArray_isEmpty(const QHash< int, QByteArray >* this_ptr);
QT_CORE_C_EXPORT const int qt_core_c_QHash_int_QByteArray_key_value(const QHash< int, QByteArray >* this_ptr, const QByteArray* value);
QT_CORE_C_EXPORT const int qt_core_c_QHash_int_QByteArray_key_value_defaultKey(const QHash< int, QByteArray >* this_ptr, const QByteArray* value, const int* defaultKey);
QT_CORE_C_EXPORT void qt_core_c_QHash_int_QByteArray_keys_to_output_no_args(const QHash< int, QByteArray >* this_ptr, QList< int >* output);
QT_CORE_C_EXPORT void qt_core_c_QHash_int_QByteArray_keys_to_output_value(const QHash< int, QByteArray >* this_ptr, const QByteArray* value, QList< int >* output);
QT_CORE_C_EXPORT QByteArray* qt_core_c_QHash_int_QByteArray_operator_index(QHash< int, QByteArray >* this_ptr, const int* key);
QT_CORE_C_EXPORT void qt_core_c_QHash_int_QByteArray_operator_index_to_output(const QHash< int, QByteArray >* this_ptr, const int* key, QByteArray* output);
QT_CORE_C_EXPORT int qt_core_c_QHash_int_QByteArray_remove(QHash< int, QByteArray >* this_ptr, const int* key);
QT_CORE_C_EXPORT void qt_core_c_QHash_int_QByteArray_reserve(QHash< int, QByteArray >* this_ptr, int size);
QT_CORE_C_EXPORT int qt_core_c_QHash_int_QByteArray_size(const QHash< int, QByteArray >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QHash_int_QByteArray_squeeze(QHash< int, QByteArray >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QHash_int_QByteArray_take_to_output(QHash< int, QByteArray >* this_ptr, const int* key, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QHash_int_QByteArray_uniqueKeys_to_output(const QHash< int, QByteArray >* this_ptr, QList< int >* output);
QT_CORE_C_EXPORT void qt_core_c_QHash_int_QByteArray_value_to_output_key(const QHash< int, QByteArray >* this_ptr, const int* key, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QHash_int_QByteArray_value_to_output_key_defaultValue(const QHash< int, QByteArray >* this_ptr, const int* key, const QByteArray* defaultValue, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QHash_int_QByteArray_values_to_output_key(const QHash< int, QByteArray >* this_ptr, const int* key, QList< QByteArray >* output);
QT_CORE_C_EXPORT void qt_core_c_QHash_int_QByteArray_values_to_output_no_args(const QHash< int, QByteArray >* this_ptr, QList< QByteArray >* output);

} // extern "C"

#endif // QT_CORE_C_QHASH_H
