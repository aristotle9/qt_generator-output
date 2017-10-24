#ifndef QT_CORE_C_QPROCESSENVIRONMENT_H
#define QT_CORE_C_QPROCESSENVIRONMENT_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QProcessEnvironment_clear(QProcessEnvironment* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QProcessEnvironment_constructor_no_args(QProcessEnvironment* output);
QT_CORE_C_EXPORT void qt_core_c_QProcessEnvironment_constructor_other(const QProcessEnvironment* other, QProcessEnvironment* output);
QT_CORE_C_EXPORT bool qt_core_c_QProcessEnvironment_contains(const QProcessEnvironment* this_ptr, const QString* name);
QT_CORE_C_EXPORT void qt_core_c_QProcessEnvironment_destructor(QProcessEnvironment* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QProcessEnvironment_insert_e(QProcessEnvironment* this_ptr, const QProcessEnvironment* e);
QT_CORE_C_EXPORT void qt_core_c_QProcessEnvironment_insert_name_value(QProcessEnvironment* this_ptr, const QString* name, const QString* value);
QT_CORE_C_EXPORT bool qt_core_c_QProcessEnvironment_isEmpty(const QProcessEnvironment* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QProcessEnvironment_keys_to_output(const QProcessEnvironment* this_ptr, QStringList* output);
QT_CORE_C_EXPORT QProcessEnvironment* qt_core_c_QProcessEnvironment_operator_assign(QProcessEnvironment* this_ptr, const QProcessEnvironment* other);
QT_CORE_C_EXPORT bool qt_core_c_QProcessEnvironment_operator_eq(const QProcessEnvironment* this_ptr, const QProcessEnvironment* other);
QT_CORE_C_EXPORT bool qt_core_c_QProcessEnvironment_operator_neq(const QProcessEnvironment* this_ptr, const QProcessEnvironment* other);
QT_CORE_C_EXPORT void qt_core_c_QProcessEnvironment_remove(QProcessEnvironment* this_ptr, const QString* name);
QT_CORE_C_EXPORT void qt_core_c_QProcessEnvironment_swap(QProcessEnvironment* this_ptr, QProcessEnvironment* other);
QT_CORE_C_EXPORT void qt_core_c_QProcessEnvironment_systemEnvironment_to_output(QProcessEnvironment* output);
QT_CORE_C_EXPORT void qt_core_c_QProcessEnvironment_toStringList_to_output(const QProcessEnvironment* this_ptr, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QProcessEnvironment_value_to_output_name(const QProcessEnvironment* this_ptr, const QString* name, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QProcessEnvironment_value_to_output_name_defaultValue(const QProcessEnvironment* this_ptr, const QString* name, const QString* defaultValue, QString* output);

} // extern "C"

#endif // QT_CORE_C_QPROCESSENVIRONMENT_H
