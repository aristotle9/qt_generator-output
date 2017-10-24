#ifndef QT_GUI_C_QGENERICPLUGINFACTORY_H
#define QT_GUI_C_QGENERICPLUGINFACTORY_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QObject* qt_gui_c_QGenericPluginFactory_create(const QString* arg1, const QString* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QGenericPluginFactory_delete(QGenericPluginFactory* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QGenericPluginFactory_keys_to_output(QStringList* output);

} // extern "C"

#endif // QT_GUI_C_QGENERICPLUGINFACTORY_H
