#ifndef QT_GUI_C_QGENERICPLUGIN_H
#define QT_GUI_C_QGENERICPLUGIN_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QGenericPlugin* qt_gui_c_QGenericPlugin_G_static_cast_QGenericPlugin_ptr(QObject* ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QGenericPlugin_G_static_cast_QObject_ptr(QGenericPlugin* ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QGenericPlugin_create(QGenericPlugin* this_ptr, const QString* name, const QString* spec);
QT_GUI_C_EXPORT void qt_gui_c_QGenericPlugin_delete(QGenericPlugin* this_ptr);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QGenericPlugin_metaObject(const QGenericPlugin* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QGenericPlugin_qt_metacall(QGenericPlugin* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QGenericPlugin_qt_metacast(QGenericPlugin* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QGenericPlugin_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QGenericPlugin_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QGENERICPLUGIN_H
