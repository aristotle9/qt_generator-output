#ifndef QT_GUI_C_QIMAGEIOPLUGIN_H
#define QT_GUI_C_QIMAGEIOPLUGIN_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QImageIOPlugin* qt_gui_c_QImageIOPlugin_G_static_cast_QImageIOPlugin_ptr(QObject* ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QImageIOPlugin_G_static_cast_QObject_ptr(QImageIOPlugin* ptr);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QImageIOPlugin_capabilities(const QImageIOPlugin* this_ptr, QIODevice* device, const QByteArray* format);
QT_GUI_C_EXPORT QImageIOHandler* qt_gui_c_QImageIOPlugin_create_device(const QImageIOPlugin* this_ptr, QIODevice* device);
QT_GUI_C_EXPORT QImageIOHandler* qt_gui_c_QImageIOPlugin_create_device_format(const QImageIOPlugin* this_ptr, QIODevice* device, const QByteArray* format);
QT_GUI_C_EXPORT void qt_gui_c_QImageIOPlugin_delete(QImageIOPlugin* this_ptr);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QImageIOPlugin_metaObject(const QImageIOPlugin* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QImageIOPlugin_qt_metacall(QImageIOPlugin* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QImageIOPlugin_qt_metacast(QImageIOPlugin* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QImageIOPlugin_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QImageIOPlugin_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QIMAGEIOPLUGIN_H
