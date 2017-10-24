#ifndef QT_WIDGETS_C_QDOCKWIDGET_H
#define QT_WIDGETS_C_QDOCKWIDGET_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QDockWidget* qt_widgets_c_QDockWidget_G_dynamic_cast_QDockWidget_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QDockWidget* qt_widgets_c_QDockWidget_G_static_cast_QDockWidget_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QDockWidget* qt_widgets_c_QDockWidget_G_static_cast_QDockWidget_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QDockWidget* qt_widgets_c_QDockWidget_G_static_cast_QDockWidget_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QDockWidget_G_static_cast_QObject_ptr(QDockWidget* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QDockWidget_G_static_cast_QPaintDevice_ptr(QDockWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QDockWidget_G_static_cast_QWidget_ptr(QDockWidget* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDockWidget_delete(QDockWidget* this_ptr);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QDockWidget_features(const QDockWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QDockWidget_isAreaAllowed(const QDockWidget* this_ptr, const Qt::DockWidgetArea* area);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QDockWidget_isFloating(const QDockWidget* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QDockWidget_metaObject(const QDockWidget* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDockWidget_qt_metacall(QDockWidget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QDockWidget_qt_metacast(QDockWidget* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDockWidget_setFeatures(QDockWidget* this_ptr, unsigned int features);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDockWidget_setFloating(QDockWidget* this_ptr, bool floating);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDockWidget_setTitleBarWidget(QDockWidget* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDockWidget_setWidget(QDockWidget* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QDockWidget_titleBarWidget(const QDockWidget* this_ptr);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QDockWidget_toggleViewAction(const QDockWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDockWidget_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDockWidget_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QDockWidget_widget(const QDockWidget* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QDOCKWIDGET_H
