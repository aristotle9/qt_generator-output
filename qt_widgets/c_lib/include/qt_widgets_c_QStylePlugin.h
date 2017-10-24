#ifndef QT_WIDGETS_C_QSTYLEPLUGIN_H
#define QT_WIDGETS_C_QSTYLEPLUGIN_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QStylePlugin_G_static_cast_QObject_ptr(QStylePlugin* ptr);
QT_WIDGETS_C_EXPORT QStylePlugin* qt_widgets_c_QStylePlugin_G_static_cast_QStylePlugin_ptr(QObject* ptr);
QT_WIDGETS_C_EXPORT QStyle* qt_widgets_c_QStylePlugin_create(QStylePlugin* this_ptr, const QString* key);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStylePlugin_delete(QStylePlugin* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QStylePlugin_metaObject(const QStylePlugin* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStylePlugin_qt_metacall(QStylePlugin* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QStylePlugin_qt_metacast(QStylePlugin* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStylePlugin_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStylePlugin_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QSTYLEPLUGIN_H
