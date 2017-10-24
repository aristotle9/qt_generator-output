#ifndef QT_WIDGETS_C_QSCROLLBAR_H
#define QT_WIDGETS_C_QSCROLLBAR_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QScrollBar* qt_widgets_c_QScrollBar_G_dynamic_cast_QScrollBar_ptr_QAbstractSlider(QAbstractSlider* ptr);
QT_WIDGETS_C_EXPORT QScrollBar* qt_widgets_c_QScrollBar_G_dynamic_cast_QScrollBar_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractSlider* qt_widgets_c_QScrollBar_G_static_cast_QAbstractSlider_ptr(QScrollBar* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QScrollBar_G_static_cast_QObject_ptr(QScrollBar* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QScrollBar_G_static_cast_QPaintDevice_ptr(QScrollBar* ptr);
QT_WIDGETS_C_EXPORT QScrollBar* qt_widgets_c_QScrollBar_G_static_cast_QScrollBar_ptr_QAbstractSlider(QAbstractSlider* ptr);
QT_WIDGETS_C_EXPORT QScrollBar* qt_widgets_c_QScrollBar_G_static_cast_QScrollBar_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QScrollBar* qt_widgets_c_QScrollBar_G_static_cast_QScrollBar_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QScrollBar* qt_widgets_c_QScrollBar_G_static_cast_QScrollBar_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QScrollBar_G_static_cast_QWidget_ptr(QScrollBar* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScrollBar_delete(QScrollBar* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QScrollBar_event(QScrollBar* this_ptr, QEvent* event);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QScrollBar_metaObject(const QScrollBar* this_ptr);
QT_WIDGETS_C_EXPORT QScrollBar* qt_widgets_c_QScrollBar_new_arg1(const Qt::Orientation* arg1);
QT_WIDGETS_C_EXPORT QScrollBar* qt_widgets_c_QScrollBar_new_arg1_parent(const Qt::Orientation* arg1, QWidget* parent);
QT_WIDGETS_C_EXPORT QScrollBar* qt_widgets_c_QScrollBar_new_no_args();
QT_WIDGETS_C_EXPORT QScrollBar* qt_widgets_c_QScrollBar_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QScrollBar_qt_metacall(QScrollBar* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QScrollBar_qt_metacast(QScrollBar* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScrollBar_sizeHint_to_output(const QScrollBar* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScrollBar_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScrollBar_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QSCROLLBAR_H
