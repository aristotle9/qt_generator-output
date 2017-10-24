#ifndef QT_WIDGETS_C_QDESKTOPWIDGET_H
#define QT_WIDGETS_C_QDESKTOPWIDGET_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QDesktopWidget* qt_widgets_c_QDesktopWidget_G_dynamic_cast_QDesktopWidget_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QDesktopWidget* qt_widgets_c_QDesktopWidget_G_static_cast_QDesktopWidget_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QDesktopWidget* qt_widgets_c_QDesktopWidget_G_static_cast_QDesktopWidget_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QDesktopWidget* qt_widgets_c_QDesktopWidget_G_static_cast_QDesktopWidget_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QDesktopWidget_G_static_cast_QObject_ptr(QDesktopWidget* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QDesktopWidget_G_static_cast_QPaintDevice_ptr(QDesktopWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QDesktopWidget_G_static_cast_QWidget_ptr(QDesktopWidget* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDesktopWidget_availableGeometry_to_output_no_args(const QDesktopWidget* this_ptr, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDesktopWidget_availableGeometry_to_output_point(const QDesktopWidget* this_ptr, const QPoint* point, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDesktopWidget_availableGeometry_to_output_screen(const QDesktopWidget* this_ptr, int screen, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDesktopWidget_availableGeometry_to_output_widget(const QDesktopWidget* this_ptr, const QWidget* widget, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDesktopWidget_delete(QDesktopWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QDesktopWidget_isVirtualDesktop(const QDesktopWidget* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QDesktopWidget_metaObject(const QDesktopWidget* this_ptr);
QT_WIDGETS_C_EXPORT QDesktopWidget* qt_widgets_c_QDesktopWidget_new();
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDesktopWidget_numScreens(const QDesktopWidget* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDesktopWidget_primaryScreen(const QDesktopWidget* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDesktopWidget_qt_metacall(QDesktopWidget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QDesktopWidget_qt_metacast(QDesktopWidget* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDesktopWidget_screenCount(const QDesktopWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDesktopWidget_screenGeometry_to_output_no_args(const QDesktopWidget* this_ptr, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDesktopWidget_screenGeometry_to_output_point(const QDesktopWidget* this_ptr, const QPoint* point, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDesktopWidget_screenGeometry_to_output_screen(const QDesktopWidget* this_ptr, int screen, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDesktopWidget_screenGeometry_to_output_widget(const QDesktopWidget* this_ptr, const QWidget* widget, QRect* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDesktopWidget_screenNumber_arg1(const QDesktopWidget* this_ptr, const QPoint* arg1);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDesktopWidget_screenNumber_no_args(const QDesktopWidget* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDesktopWidget_screenNumber_widget(const QDesktopWidget* this_ptr, const QWidget* widget);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QDesktopWidget_screen_no_args(QDesktopWidget* this_ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QDesktopWidget_screen_screen(QDesktopWidget* this_ptr, int screen);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDesktopWidget_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDesktopWidget_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QDESKTOPWIDGET_H
