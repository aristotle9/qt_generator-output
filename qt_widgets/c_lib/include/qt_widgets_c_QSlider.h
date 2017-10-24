#ifndef QT_WIDGETS_C_QSLIDER_H
#define QT_WIDGETS_C_QSLIDER_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QSlider* qt_widgets_c_QSlider_G_dynamic_cast_QSlider_ptr_QAbstractSlider(QAbstractSlider* ptr);
QT_WIDGETS_C_EXPORT QSlider* qt_widgets_c_QSlider_G_dynamic_cast_QSlider_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractSlider* qt_widgets_c_QSlider_G_static_cast_QAbstractSlider_ptr(QSlider* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QSlider_G_static_cast_QObject_ptr(QSlider* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QSlider_G_static_cast_QPaintDevice_ptr(QSlider* ptr);
QT_WIDGETS_C_EXPORT QSlider* qt_widgets_c_QSlider_G_static_cast_QSlider_ptr_QAbstractSlider(QAbstractSlider* ptr);
QT_WIDGETS_C_EXPORT QSlider* qt_widgets_c_QSlider_G_static_cast_QSlider_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QSlider* qt_widgets_c_QSlider_G_static_cast_QSlider_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QSlider* qt_widgets_c_QSlider_G_static_cast_QSlider_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QSlider_G_static_cast_QWidget_ptr(QSlider* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSlider_delete(QSlider* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QSlider_event(QSlider* this_ptr, QEvent* event);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QSlider_metaObject(const QSlider* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSlider_minimumSizeHint_to_output(const QSlider* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QSlider* qt_widgets_c_QSlider_new_no_args();
QT_WIDGETS_C_EXPORT QSlider* qt_widgets_c_QSlider_new_orientation(const Qt::Orientation* orientation);
QT_WIDGETS_C_EXPORT QSlider* qt_widgets_c_QSlider_new_orientation_parent(const Qt::Orientation* orientation, QWidget* parent);
QT_WIDGETS_C_EXPORT QSlider* qt_widgets_c_QSlider_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QSlider_qt_metacall(QSlider* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QSlider_qt_metacast(QSlider* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSlider_setTickInterval(QSlider* this_ptr, int ti);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSlider_setTickPosition(QSlider* this_ptr, QSlider::TickPosition position);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSlider_sizeHint_to_output(const QSlider* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QSlider_tickInterval(const QSlider* this_ptr);
QT_WIDGETS_C_EXPORT QSlider::TickPosition qt_widgets_c_QSlider_tickPosition(const QSlider* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSlider_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSlider_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QSLIDER_H
