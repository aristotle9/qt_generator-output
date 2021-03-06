#ifndef QT_WIDGETS_C_QGESTUREEVENT_H
#define QT_WIDGETS_C_QGESTUREEVENT_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QEvent* qt_widgets_c_QGestureEvent_G_static_cast_QEvent_ptr(QGestureEvent* ptr);
QT_WIDGETS_C_EXPORT QGestureEvent* qt_widgets_c_QGestureEvent_G_static_cast_QGestureEvent_ptr(QEvent* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGestureEvent_accept_QGesture(QGestureEvent* this_ptr, QGesture* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGestureEvent_accept_Qt_GestureType(QGestureEvent* this_ptr, const Qt::GestureType* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGestureEvent_activeGestures_to_output(const QGestureEvent* this_ptr, QList< QGesture* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGestureEvent_canceledGestures_to_output(const QGestureEvent* this_ptr, QList< QGesture* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGestureEvent_delete(QGestureEvent* this_ptr);
QT_WIDGETS_C_EXPORT QGesture* qt_widgets_c_QGestureEvent_gesture(const QGestureEvent* this_ptr, const Qt::GestureType* type);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGestureEvent_gestures_to_output(const QGestureEvent* this_ptr, QList< QGesture* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGestureEvent_ignore_QGesture(QGestureEvent* this_ptr, QGesture* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGestureEvent_ignore_Qt_GestureType(QGestureEvent* this_ptr, const Qt::GestureType* arg1);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGestureEvent_isAccepted_QGesture(const QGestureEvent* this_ptr, QGesture* arg1);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGestureEvent_isAccepted_Qt_GestureType(const QGestureEvent* this_ptr, const Qt::GestureType* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGestureEvent_mapToGraphicsScene_to_output(const QGestureEvent* this_ptr, const QPointF* gesturePoint, QPointF* output);
QT_WIDGETS_C_EXPORT QGestureEvent* qt_widgets_c_QGestureEvent_new(const QList< QGesture* >* gestures);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGestureEvent_setAccepted_QGesture_bool(QGestureEvent* this_ptr, QGesture* arg1, bool arg2);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGestureEvent_setAccepted_Qt_GestureType_bool(QGestureEvent* this_ptr, const Qt::GestureType* arg1, bool arg2);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGestureEvent_setWidget(QGestureEvent* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QGestureEvent_widget(const QGestureEvent* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QGESTUREEVENT_H
