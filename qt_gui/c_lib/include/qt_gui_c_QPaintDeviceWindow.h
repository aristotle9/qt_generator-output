#ifndef QT_GUI_C_QPAINTDEVICEWINDOW_H
#define QT_GUI_C_QPAINTDEVICEWINDOW_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QPaintDeviceWindow* qt_gui_c_QPaintDeviceWindow_G_dynamic_cast_QPaintDeviceWindow_ptr_QPaintDevice(QPaintDevice* ptr);
QT_GUI_C_EXPORT QPaintDeviceWindow* qt_gui_c_QPaintDeviceWindow_G_dynamic_cast_QPaintDeviceWindow_ptr_QSurface(QSurface* ptr);
QT_GUI_C_EXPORT QPaintDeviceWindow* qt_gui_c_QPaintDeviceWindow_G_dynamic_cast_QPaintDeviceWindow_ptr_QWindow(QWindow* ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QPaintDeviceWindow_G_static_cast_QObject_ptr(QPaintDeviceWindow* ptr);
QT_GUI_C_EXPORT QPaintDeviceWindow* qt_gui_c_QPaintDeviceWindow_G_static_cast_QPaintDeviceWindow_ptr_QObject(QObject* ptr);
QT_GUI_C_EXPORT QPaintDeviceWindow* qt_gui_c_QPaintDeviceWindow_G_static_cast_QPaintDeviceWindow_ptr_QPaintDevice(QPaintDevice* ptr);
QT_GUI_C_EXPORT QPaintDeviceWindow* qt_gui_c_QPaintDeviceWindow_G_static_cast_QPaintDeviceWindow_ptr_QSurface(QSurface* ptr);
QT_GUI_C_EXPORT QPaintDeviceWindow* qt_gui_c_QPaintDeviceWindow_G_static_cast_QPaintDeviceWindow_ptr_QWindow(QWindow* ptr);
QT_GUI_C_EXPORT QPaintDevice* qt_gui_c_QPaintDeviceWindow_G_static_cast_QPaintDevice_ptr(QPaintDeviceWindow* ptr);
QT_GUI_C_EXPORT QSurface* qt_gui_c_QPaintDeviceWindow_G_static_cast_QSurface_ptr(QPaintDeviceWindow* ptr);
QT_GUI_C_EXPORT QWindow* qt_gui_c_QPaintDeviceWindow_G_static_cast_QWindow_ptr(QPaintDeviceWindow* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPaintDeviceWindow_delete(QPaintDeviceWindow* this_ptr);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QPaintDeviceWindow_metaObject(const QPaintDeviceWindow* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QPaintDeviceWindow_qt_metacall(QPaintDeviceWindow* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QPaintDeviceWindow_qt_metacast(QPaintDeviceWindow* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QPaintDeviceWindow_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QPaintDeviceWindow_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QPaintDeviceWindow_update_no_args(QPaintDeviceWindow* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPaintDeviceWindow_update_rect(QPaintDeviceWindow* this_ptr, const QRect* rect);
QT_GUI_C_EXPORT void qt_gui_c_QPaintDeviceWindow_update_region(QPaintDeviceWindow* this_ptr, const QRegion* region);

} // extern "C"

#endif // QT_GUI_C_QPAINTDEVICEWINDOW_H
