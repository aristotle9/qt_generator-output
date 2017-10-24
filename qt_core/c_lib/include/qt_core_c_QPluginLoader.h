#ifndef QT_CORE_C_QPLUGINLOADER_H
#define QT_CORE_C_QPLUGINLOADER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QPluginLoader* qt_core_c_QPluginLoader_G_dynamic_cast_QPluginLoader_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QPluginLoader_G_static_cast_QObject_ptr(QPluginLoader* ptr);
QT_CORE_C_EXPORT QPluginLoader* qt_core_c_QPluginLoader_G_static_cast_QPluginLoader_ptr(QObject* ptr);
QT_CORE_C_EXPORT void qt_core_c_QPluginLoader_delete(QPluginLoader* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QPluginLoader_errorString_to_output(const QPluginLoader* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QPluginLoader_fileName_to_output(const QPluginLoader* this_ptr, QString* output);
QT_CORE_C_EXPORT QObject* qt_core_c_QPluginLoader_instance(QPluginLoader* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QPluginLoader_isLoaded(const QPluginLoader* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QPluginLoader_load(QPluginLoader* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QPluginLoader_metaData_to_output(const QPluginLoader* this_ptr, QJsonObject* output);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QPluginLoader_metaObject(const QPluginLoader* this_ptr);
QT_CORE_C_EXPORT QPluginLoader* qt_core_c_QPluginLoader_new_fileName(const QString* fileName);
QT_CORE_C_EXPORT QPluginLoader* qt_core_c_QPluginLoader_new_fileName_parent(const QString* fileName, QObject* parent);
QT_CORE_C_EXPORT QPluginLoader* qt_core_c_QPluginLoader_new_no_args();
QT_CORE_C_EXPORT QPluginLoader* qt_core_c_QPluginLoader_new_parent(QObject* parent);
QT_CORE_C_EXPORT void qt_core_c_QPluginLoader_setFileName(QPluginLoader* this_ptr, const QString* fileName);
QT_CORE_C_EXPORT void qt_core_c_QPluginLoader_staticInstances_to_output(QList< QObject* >* output);
QT_CORE_C_EXPORT void qt_core_c_QPluginLoader_staticPlugins_to_output(QVector< QStaticPlugin >* output);
QT_CORE_C_EXPORT void qt_core_c_QPluginLoader_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QPluginLoader_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QPluginLoader_unload(QPluginLoader* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QPLUGINLOADER_H
