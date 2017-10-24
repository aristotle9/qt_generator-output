#ifndef QT_CORE_C_QCOMMANDLINEOPTION_H
#define QT_CORE_C_QCOMMANDLINEOPTION_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QCommandLineOption_G_swap(QCommandLineOption* value1, QCommandLineOption* value2);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineOption_constructor_name(const QString* name, QCommandLineOption* output);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineOption_constructor_name_description(const QString* name, const QString* description, QCommandLineOption* output);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineOption_constructor_name_description_valueName(const QString* name, const QString* description, const QString* valueName, QCommandLineOption* output);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineOption_constructor_name_description_valueName_defaultValue(const QString* name, const QString* description, const QString* valueName, const QString* defaultValue, QCommandLineOption* output);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineOption_constructor_names(const QStringList* names, QCommandLineOption* output);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineOption_constructor_names_description(const QStringList* names, const QString* description, QCommandLineOption* output);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineOption_constructor_names_description_valueName(const QStringList* names, const QString* description, const QString* valueName, QCommandLineOption* output);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineOption_constructor_names_description_valueName_defaultValue(const QStringList* names, const QString* description, const QString* valueName, const QString* defaultValue, QCommandLineOption* output);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineOption_constructor_other(const QCommandLineOption* other, QCommandLineOption* output);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineOption_defaultValues_to_output(const QCommandLineOption* this_ptr, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineOption_description_to_output(const QCommandLineOption* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineOption_destructor(QCommandLineOption* this_ptr);
QT_CORE_C_EXPORT unsigned int qt_core_c_QCommandLineOption_flags(const QCommandLineOption* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QCommandLineOption_isHidden(const QCommandLineOption* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineOption_names_to_output(const QCommandLineOption* this_ptr, QStringList* output);
QT_CORE_C_EXPORT QCommandLineOption* qt_core_c_QCommandLineOption_operator_assign(QCommandLineOption* this_ptr, const QCommandLineOption* other);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineOption_setDefaultValue(QCommandLineOption* this_ptr, const QString* defaultValue);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineOption_setDefaultValues(QCommandLineOption* this_ptr, const QStringList* defaultValues);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineOption_setDescription(QCommandLineOption* this_ptr, const QString* description);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineOption_setFlags(QCommandLineOption* this_ptr, unsigned int aflags);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineOption_setHidden(QCommandLineOption* this_ptr, bool hidden);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineOption_setValueName(QCommandLineOption* this_ptr, const QString* name);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineOption_swap(QCommandLineOption* this_ptr, QCommandLineOption* other);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineOption_valueName_to_output(const QCommandLineOption* this_ptr, QString* output);

} // extern "C"

#endif // QT_CORE_C_QCOMMANDLINEOPTION_H
