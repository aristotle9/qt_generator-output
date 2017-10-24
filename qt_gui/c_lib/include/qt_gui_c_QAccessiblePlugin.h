#ifndef QT_GUI_C_QACCESSIBLEPLUGIN_H
#define QT_GUI_C_QACCESSIBLEPLUGIN_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QAccessiblePlugin* qt_gui_c_QAccessiblePlugin_G_static_cast_QAccessiblePlugin_ptr(QObject* ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QAccessiblePlugin_G_static_cast_QObject_ptr(QAccessiblePlugin* ptr);
QT_GUI_C_EXPORT QAccessibleInterface* qt_gui_c_QAccessiblePlugin_create(QAccessiblePlugin* this_ptr, const QString* key, QObject* object);
QT_GUI_C_EXPORT void qt_gui_c_QAccessiblePlugin_delete(QAccessiblePlugin* this_ptr);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QAccessiblePlugin_metaObject(const QAccessiblePlugin* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QAccessiblePlugin_qt_metacall(QAccessiblePlugin* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QAccessiblePlugin_qt_metacast(QAccessiblePlugin* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QAccessiblePlugin_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QAccessiblePlugin_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QACCESSIBLEPLUGIN_H
