#ifndef QT_GUI_C_QPICTUREFORMATPLUGIN_H
#define QT_GUI_C_QPICTUREFORMATPLUGIN_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QObject* qt_gui_c_QPictureFormatPlugin_G_static_cast_QObject_ptr(QPictureFormatPlugin* ptr);
QT_GUI_C_EXPORT QPictureFormatPlugin* qt_gui_c_QPictureFormatPlugin_G_static_cast_QPictureFormatPlugin_ptr(QObject* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPictureFormatPlugin_delete(QPictureFormatPlugin* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPictureFormatPlugin_installIOHandler(QPictureFormatPlugin* this_ptr, const QString* format);
QT_GUI_C_EXPORT bool qt_gui_c_QPictureFormatPlugin_loadPicture(QPictureFormatPlugin* this_ptr, const QString* format, const QString* filename, QPicture* pic);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QPictureFormatPlugin_metaObject(const QPictureFormatPlugin* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QPictureFormatPlugin_qt_metacall(QPictureFormatPlugin* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QPictureFormatPlugin_qt_metacast(QPictureFormatPlugin* this_ptr, const char* arg1);
QT_GUI_C_EXPORT bool qt_gui_c_QPictureFormatPlugin_savePicture(QPictureFormatPlugin* this_ptr, const QString* format, const QString* filename, const QPicture* pic);
QT_GUI_C_EXPORT void qt_gui_c_QPictureFormatPlugin_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QPictureFormatPlugin_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QPICTUREFORMATPLUGIN_H
