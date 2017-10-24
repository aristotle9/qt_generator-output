#ifndef QT_CORE_C_QSETTINGS_H
#define QT_CORE_C_QSETTINGS_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QSettings* qt_core_c_QSettings_G_dynamic_cast_QSettings_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QSettings_G_static_cast_QObject_ptr(QSettings* ptr);
QT_CORE_C_EXPORT QSettings* qt_core_c_QSettings_G_static_cast_QSettings_ptr(QObject* ptr);
QT_CORE_C_EXPORT void qt_core_c_QSettings_allKeys_to_output(const QSettings* this_ptr, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QSettings_applicationName_to_output(const QSettings* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QSettings_beginGroup(QSettings* this_ptr, const QString* prefix);
QT_CORE_C_EXPORT int qt_core_c_QSettings_beginReadArray(QSettings* this_ptr, const QString* prefix);
QT_CORE_C_EXPORT void qt_core_c_QSettings_beginWriteArray_prefix(QSettings* this_ptr, const QString* prefix);
QT_CORE_C_EXPORT void qt_core_c_QSettings_beginWriteArray_prefix_size(QSettings* this_ptr, const QString* prefix, int size);
QT_CORE_C_EXPORT void qt_core_c_QSettings_childGroups_to_output(const QSettings* this_ptr, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QSettings_childKeys_to_output(const QSettings* this_ptr, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QSettings_clear(QSettings* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QSettings_contains(const QSettings* this_ptr, const QString* key);
QT_CORE_C_EXPORT QSettings::Format qt_core_c_QSettings_defaultFormat();
QT_CORE_C_EXPORT void qt_core_c_QSettings_delete(QSettings* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSettings_endArray(QSettings* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSettings_endGroup(QSettings* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QSettings_fallbacksEnabled(const QSettings* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSettings_fileName_to_output(const QSettings* this_ptr, QString* output);
QT_CORE_C_EXPORT QSettings::Format qt_core_c_QSettings_format(const QSettings* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSettings_group_to_output(const QSettings* this_ptr, QString* output);
QT_CORE_C_EXPORT QTextCodec* qt_core_c_QSettings_iniCodec(const QSettings* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QSettings_isWritable(const QSettings* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QSettings_metaObject(const QSettings* this_ptr);
QT_CORE_C_EXPORT QSettings* qt_core_c_QSettings_new_fileName_format(const QString* fileName, QSettings::Format format);
QT_CORE_C_EXPORT QSettings* qt_core_c_QSettings_new_fileName_format_parent(const QString* fileName, QSettings::Format format, QObject* parent);
QT_CORE_C_EXPORT QSettings* qt_core_c_QSettings_new_format_scope_organization(QSettings::Format format, QSettings::Scope scope, const QString* organization);
QT_CORE_C_EXPORT QSettings* qt_core_c_QSettings_new_format_scope_organization_application(QSettings::Format format, QSettings::Scope scope, const QString* organization, const QString* application);
QT_CORE_C_EXPORT QSettings* qt_core_c_QSettings_new_format_scope_organization_application_parent(QSettings::Format format, QSettings::Scope scope, const QString* organization, const QString* application, QObject* parent);
QT_CORE_C_EXPORT QSettings* qt_core_c_QSettings_new_no_args();
QT_CORE_C_EXPORT QSettings* qt_core_c_QSettings_new_organization(const QString* organization);
QT_CORE_C_EXPORT QSettings* qt_core_c_QSettings_new_organization_application(const QString* organization, const QString* application);
QT_CORE_C_EXPORT QSettings* qt_core_c_QSettings_new_organization_application_parent(const QString* organization, const QString* application, QObject* parent);
QT_CORE_C_EXPORT QSettings* qt_core_c_QSettings_new_parent(QObject* parent);
QT_CORE_C_EXPORT QSettings* qt_core_c_QSettings_new_scope_organization(QSettings::Scope scope, const QString* organization);
QT_CORE_C_EXPORT QSettings* qt_core_c_QSettings_new_scope_organization_application(QSettings::Scope scope, const QString* organization, const QString* application);
QT_CORE_C_EXPORT QSettings* qt_core_c_QSettings_new_scope_organization_application_parent(QSettings::Scope scope, const QString* organization, const QString* application, QObject* parent);
QT_CORE_C_EXPORT void qt_core_c_QSettings_organizationName_to_output(const QSettings* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QSettings_remove(QSettings* this_ptr, const QString* key);
QT_CORE_C_EXPORT QSettings::Scope qt_core_c_QSettings_scope(const QSettings* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSettings_setArrayIndex(QSettings* this_ptr, int i);
QT_CORE_C_EXPORT void qt_core_c_QSettings_setDefaultFormat(QSettings::Format format);
QT_CORE_C_EXPORT void qt_core_c_QSettings_setFallbacksEnabled(QSettings* this_ptr, bool b);
QT_CORE_C_EXPORT void qt_core_c_QSettings_setIniCodec_codec(QSettings* this_ptr, QTextCodec* codec);
QT_CORE_C_EXPORT void qt_core_c_QSettings_setIniCodec_codecName(QSettings* this_ptr, const char* codecName);
QT_CORE_C_EXPORT void qt_core_c_QSettings_setPath(QSettings::Format format, QSettings::Scope scope, const QString* path);
QT_CORE_C_EXPORT void qt_core_c_QSettings_setSystemIniPath(const QString* dir);
QT_CORE_C_EXPORT void qt_core_c_QSettings_setUserIniPath(const QString* dir);
QT_CORE_C_EXPORT void qt_core_c_QSettings_setValue(QSettings* this_ptr, const QString* key, const QVariant* value);
QT_CORE_C_EXPORT QSettings::Status qt_core_c_QSettings_status(const QSettings* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSettings_sync(QSettings* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSettings_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QSettings_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QSettings_value_to_output_key(const QSettings* this_ptr, const QString* key, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QSettings_value_to_output_key_defaultValue(const QSettings* this_ptr, const QString* key, const QVariant* defaultValue, QVariant* output);

} // extern "C"

#endif // QT_CORE_C_QSETTINGS_H
