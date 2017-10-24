#ifndef QT_CORE_C_QSTATICPLUGIN_H
#define QT_CORE_C_QSTATICPLUGIN_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QStaticPlugin_G_qRegisterStaticPluginFunction(const QStaticPlugin* staticPlugin);
QT_CORE_C_EXPORT void qt_core_c_QStaticPlugin_destructor(QStaticPlugin* this_ptr);
QT_CORE_C_EXPORT QObject* (*qt_core_c_QStaticPlugin_instance(const QStaticPlugin* this_ptr))();
QT_CORE_C_EXPORT void qt_core_c_QStaticPlugin_metaData_to_output(const QStaticPlugin* this_ptr, QJsonObject* output);
QT_CORE_C_EXPORT const char* (*qt_core_c_QStaticPlugin_rawMetaData(const QStaticPlugin* this_ptr))();
QT_CORE_C_EXPORT void qt_core_c_QStaticPlugin_set_instance(QStaticPlugin* this_ptr, QObject* (*value)());
QT_CORE_C_EXPORT void qt_core_c_QStaticPlugin_set_rawMetaData(QStaticPlugin* this_ptr, const char* (*value)());

} // extern "C"

#endif // QT_CORE_C_QSTATICPLUGIN_H
