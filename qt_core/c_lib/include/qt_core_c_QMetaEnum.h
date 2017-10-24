#ifndef QT_CORE_C_QMETAENUM_H
#define QT_CORE_C_QMETAENUM_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QMetaEnum_constructor(QMetaEnum* output);
QT_CORE_C_EXPORT void qt_core_c_QMetaEnum_destructor(QMetaEnum* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QMetaEnum_enclosingMetaObject(const QMetaEnum* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMetaEnum_isFlag(const QMetaEnum* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMetaEnum_isScoped(const QMetaEnum* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMetaEnum_isValid(const QMetaEnum* this_ptr);
QT_CORE_C_EXPORT const char* qt_core_c_QMetaEnum_key(const QMetaEnum* this_ptr, int index);
QT_CORE_C_EXPORT int qt_core_c_QMetaEnum_keyCount(const QMetaEnum* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QMetaEnum_keyToValue_key(const QMetaEnum* this_ptr, const char* key);
QT_CORE_C_EXPORT int qt_core_c_QMetaEnum_keyToValue_key_ok(const QMetaEnum* this_ptr, const char* key, bool* ok);
QT_CORE_C_EXPORT int qt_core_c_QMetaEnum_keysToValue_keys(const QMetaEnum* this_ptr, const char* keys);
QT_CORE_C_EXPORT int qt_core_c_QMetaEnum_keysToValue_keys_ok(const QMetaEnum* this_ptr, const char* keys, bool* ok);
QT_CORE_C_EXPORT const char* qt_core_c_QMetaEnum_name(const QMetaEnum* this_ptr);
QT_CORE_C_EXPORT const char* qt_core_c_QMetaEnum_scope(const QMetaEnum* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QMetaEnum_value(const QMetaEnum* this_ptr, int index);
QT_CORE_C_EXPORT const char* qt_core_c_QMetaEnum_valueToKey(const QMetaEnum* this_ptr, int value);
QT_CORE_C_EXPORT void qt_core_c_QMetaEnum_valueToKeys_to_output(const QMetaEnum* this_ptr, int value, QByteArray* output);

} // extern "C"

#endif // QT_CORE_C_QMETAENUM_H
