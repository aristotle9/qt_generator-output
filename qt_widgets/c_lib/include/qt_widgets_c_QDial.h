#ifndef QT_WIDGETS_C_QDIAL_H
#define QT_WIDGETS_C_QDIAL_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QDial* qt_widgets_c_QDial_G_dynamic_cast_QDial_ptr_QAbstractSlider(QAbstractSlider* ptr);
QT_WIDGETS_C_EXPORT QDial* qt_widgets_c_QDial_G_dynamic_cast_QDial_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractSlider* qt_widgets_c_QDial_G_static_cast_QAbstractSlider_ptr(QDial* ptr);
QT_WIDGETS_C_EXPORT QDial* qt_widgets_c_QDial_G_static_cast_QDial_ptr_QAbstractSlider(QAbstractSlider* ptr);
QT_WIDGETS_C_EXPORT QDial* qt_widgets_c_QDial_G_static_cast_QDial_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QDial* qt_widgets_c_QDial_G_static_cast_QDial_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QDial* qt_widgets_c_QDial_G_static_cast_QDial_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QDial_G_static_cast_QObject_ptr(QDial* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QDial_G_static_cast_QPaintDevice_ptr(QDial* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QDial_G_static_cast_QWidget_ptr(QDial* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDial_delete(QDial* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QDial_metaObject(const QDial* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDial_minimumSizeHint_to_output(const QDial* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QDial* qt_widgets_c_QDial_new_no_args();
QT_WIDGETS_C_EXPORT QDial* qt_widgets_c_QDial_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDial_notchSize(const QDial* this_ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QDial_notchTarget(const QDial* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QDial_notchesVisible(const QDial* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDial_qt_metacall(QDial* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QDial_qt_metacast(QDial* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDial_setNotchTarget(QDial* this_ptr, double target);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDial_setNotchesVisible(QDial* this_ptr, bool visible);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDial_setWrapping(QDial* this_ptr, bool on);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDial_sizeHint_to_output(const QDial* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDial_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDial_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QDial_wrapping(const QDial* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QDIAL_H
