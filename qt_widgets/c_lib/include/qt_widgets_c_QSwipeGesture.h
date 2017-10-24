#ifndef QT_WIDGETS_C_QSWIPEGESTURE_H
#define QT_WIDGETS_C_QSWIPEGESTURE_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QSwipeGesture* qt_widgets_c_QSwipeGesture_G_dynamic_cast_QSwipeGesture_ptr(QGesture* ptr);
QT_WIDGETS_C_EXPORT QGesture* qt_widgets_c_QSwipeGesture_G_static_cast_QGesture_ptr(QSwipeGesture* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QSwipeGesture_G_static_cast_QObject_ptr(QSwipeGesture* ptr);
QT_WIDGETS_C_EXPORT QSwipeGesture* qt_widgets_c_QSwipeGesture_G_static_cast_QSwipeGesture_ptr_QGesture(QGesture* ptr);
QT_WIDGETS_C_EXPORT QSwipeGesture* qt_widgets_c_QSwipeGesture_G_static_cast_QSwipeGesture_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSwipeGesture_delete(QSwipeGesture* this_ptr);
QT_WIDGETS_C_EXPORT QSwipeGesture::SwipeDirection qt_widgets_c_QSwipeGesture_horizontalDirection(const QSwipeGesture* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QSwipeGesture_metaObject(const QSwipeGesture* this_ptr);
QT_WIDGETS_C_EXPORT QSwipeGesture* qt_widgets_c_QSwipeGesture_new_no_args();
QT_WIDGETS_C_EXPORT QSwipeGesture* qt_widgets_c_QSwipeGesture_new_parent(QObject* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QSwipeGesture_qt_metacall(QSwipeGesture* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QSwipeGesture_qt_metacast(QSwipeGesture* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSwipeGesture_setSwipeAngle(QSwipeGesture* this_ptr, double value);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QSwipeGesture_swipeAngle(const QSwipeGesture* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSwipeGesture_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSwipeGesture_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT QSwipeGesture::SwipeDirection qt_widgets_c_QSwipeGesture_verticalDirection(const QSwipeGesture* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QSWIPEGESTURE_H
