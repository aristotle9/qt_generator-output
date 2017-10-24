#ifndef QT_GUI_C_QICONENGINEPLUGIN_H
#define QT_GUI_C_QICONENGINEPLUGIN_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QIconEnginePlugin* qt_gui_c_QIconEnginePlugin_G_static_cast_QIconEnginePlugin_ptr(QObject* ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QIconEnginePlugin_G_static_cast_QObject_ptr(QIconEnginePlugin* ptr);
QT_GUI_C_EXPORT QIconEngine* qt_gui_c_QIconEnginePlugin_create_filename(QIconEnginePlugin* this_ptr, const QString* filename);
QT_GUI_C_EXPORT QIconEngine* qt_gui_c_QIconEnginePlugin_create_no_args(QIconEnginePlugin* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QIconEnginePlugin_delete(QIconEnginePlugin* this_ptr);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QIconEnginePlugin_metaObject(const QIconEnginePlugin* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QIconEnginePlugin_qt_metacall(QIconEnginePlugin* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QIconEnginePlugin_qt_metacast(QIconEnginePlugin* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QIconEnginePlugin_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QIconEnginePlugin_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QICONENGINEPLUGIN_H
