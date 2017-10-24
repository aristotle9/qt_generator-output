#ifndef QT_GUI_C_QOFFSCREENSURFACE_H
#define QT_GUI_C_QOFFSCREENSURFACE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QOffscreenSurface* qt_gui_c_QOffscreenSurface_G_dynamic_cast_QOffscreenSurface_ptr(QSurface* ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QOffscreenSurface_G_static_cast_QObject_ptr(QOffscreenSurface* ptr);
QT_GUI_C_EXPORT QOffscreenSurface* qt_gui_c_QOffscreenSurface_G_static_cast_QOffscreenSurface_ptr_QObject(QObject* ptr);
QT_GUI_C_EXPORT QOffscreenSurface* qt_gui_c_QOffscreenSurface_G_static_cast_QOffscreenSurface_ptr_QSurface(QSurface* ptr);
QT_GUI_C_EXPORT QSurface* qt_gui_c_QOffscreenSurface_G_static_cast_QSurface_ptr(QOffscreenSurface* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOffscreenSurface_create(QOffscreenSurface* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOffscreenSurface_delete(QOffscreenSurface* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOffscreenSurface_destroy(QOffscreenSurface* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOffscreenSurface_format_to_output(const QOffscreenSurface* this_ptr, QSurfaceFormat* output);
QT_GUI_C_EXPORT bool qt_gui_c_QOffscreenSurface_isValid(const QOffscreenSurface* this_ptr);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QOffscreenSurface_metaObject(const QOffscreenSurface* this_ptr);
QT_GUI_C_EXPORT void* qt_gui_c_QOffscreenSurface_nativeHandle(const QOffscreenSurface* this_ptr);
QT_GUI_C_EXPORT QOffscreenSurface* qt_gui_c_QOffscreenSurface_new_no_args();
QT_GUI_C_EXPORT QOffscreenSurface* qt_gui_c_QOffscreenSurface_new_screen(QScreen* screen);
QT_GUI_C_EXPORT int qt_gui_c_QOffscreenSurface_qt_metacall(QOffscreenSurface* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QOffscreenSurface_qt_metacast(QOffscreenSurface* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QOffscreenSurface_requestedFormat_to_output(const QOffscreenSurface* this_ptr, QSurfaceFormat* output);
QT_GUI_C_EXPORT QScreen* qt_gui_c_QOffscreenSurface_screen(const QOffscreenSurface* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOffscreenSurface_setFormat(QOffscreenSurface* this_ptr, const QSurfaceFormat* format);
QT_GUI_C_EXPORT void qt_gui_c_QOffscreenSurface_setNativeHandle(QOffscreenSurface* this_ptr, void* handle);
QT_GUI_C_EXPORT void qt_gui_c_QOffscreenSurface_setScreen(QOffscreenSurface* this_ptr, QScreen* screen);
QT_GUI_C_EXPORT void qt_gui_c_QOffscreenSurface_size_to_output(const QOffscreenSurface* this_ptr, QSize* output);
QT_GUI_C_EXPORT QSurface::SurfaceType qt_gui_c_QOffscreenSurface_surfaceType(const QOffscreenSurface* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOffscreenSurface_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QOffscreenSurface_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QOFFSCREENSURFACE_H
