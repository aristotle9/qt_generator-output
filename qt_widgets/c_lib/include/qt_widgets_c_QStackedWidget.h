#ifndef QT_WIDGETS_C_QSTACKEDWIDGET_H
#define QT_WIDGETS_C_QSTACKEDWIDGET_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QStackedWidget* qt_widgets_c_QStackedWidget_G_dynamic_cast_QStackedWidget_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QStackedWidget* qt_widgets_c_QStackedWidget_G_dynamic_cast_QStackedWidget_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QFrame* qt_widgets_c_QStackedWidget_G_static_cast_QFrame_ptr(QStackedWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QStackedWidget_G_static_cast_QObject_ptr(QStackedWidget* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QStackedWidget_G_static_cast_QPaintDevice_ptr(QStackedWidget* ptr);
QT_WIDGETS_C_EXPORT QStackedWidget* qt_widgets_c_QStackedWidget_G_static_cast_QStackedWidget_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QStackedWidget* qt_widgets_c_QStackedWidget_G_static_cast_QStackedWidget_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QStackedWidget* qt_widgets_c_QStackedWidget_G_static_cast_QStackedWidget_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QStackedWidget* qt_widgets_c_QStackedWidget_G_static_cast_QStackedWidget_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QStackedWidget_G_static_cast_QWidget_ptr(QStackedWidget* ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStackedWidget_addWidget(QStackedWidget* this_ptr, QWidget* w);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStackedWidget_count(const QStackedWidget* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStackedWidget_currentIndex(const QStackedWidget* this_ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QStackedWidget_currentWidget(const QStackedWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStackedWidget_delete(QStackedWidget* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStackedWidget_indexOf(const QStackedWidget* this_ptr, QWidget* arg1);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStackedWidget_insertWidget(QStackedWidget* this_ptr, int index, QWidget* w);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QStackedWidget_metaObject(const QStackedWidget* this_ptr);
QT_WIDGETS_C_EXPORT QStackedWidget* qt_widgets_c_QStackedWidget_new_no_args();
QT_WIDGETS_C_EXPORT QStackedWidget* qt_widgets_c_QStackedWidget_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStackedWidget_qt_metacall(QStackedWidget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QStackedWidget_qt_metacast(QStackedWidget* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStackedWidget_removeWidget(QStackedWidget* this_ptr, QWidget* w);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStackedWidget_setCurrentIndex(QStackedWidget* this_ptr, int index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStackedWidget_setCurrentWidget(QStackedWidget* this_ptr, QWidget* w);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStackedWidget_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStackedWidget_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QStackedWidget_widget(const QStackedWidget* this_ptr, int arg1);

} // extern "C"

#endif // QT_WIDGETS_C_QSTACKEDWIDGET_H
