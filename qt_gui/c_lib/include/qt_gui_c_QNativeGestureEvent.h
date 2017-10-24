#ifndef QT_GUI_C_QNATIVEGESTUREEVENT_H
#define QT_GUI_C_QNATIVEGESTUREEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QNativeGestureEvent* qt_gui_c_QNativeGestureEvent_G_dynamic_cast_QNativeGestureEvent_ptr(QInputEvent* ptr);
QT_GUI_C_EXPORT QEvent* qt_gui_c_QNativeGestureEvent_G_static_cast_QEvent_ptr(QNativeGestureEvent* ptr);
QT_GUI_C_EXPORT QInputEvent* qt_gui_c_QNativeGestureEvent_G_static_cast_QInputEvent_ptr(QNativeGestureEvent* ptr);
QT_GUI_C_EXPORT QNativeGestureEvent* qt_gui_c_QNativeGestureEvent_G_static_cast_QNativeGestureEvent_ptr_QEvent(QEvent* ptr);
QT_GUI_C_EXPORT QNativeGestureEvent* qt_gui_c_QNativeGestureEvent_G_static_cast_QNativeGestureEvent_ptr_QInputEvent(QInputEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QNativeGestureEvent_delete(QNativeGestureEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QNativeGestureEvent_globalPos_to_output(const QNativeGestureEvent* this_ptr, QPoint* output);
QT_GUI_C_EXPORT const QPointF* qt_gui_c_QNativeGestureEvent_localPos(const QNativeGestureEvent* this_ptr);
QT_GUI_C_EXPORT QNativeGestureEvent* qt_gui_c_QNativeGestureEvent_new(const Qt::NativeGestureType* type, const QPointF* localPos, const QPointF* windowPos, const QPointF* screenPos, double value, unsigned long sequenceId, quint64 intArgument);
QT_GUI_C_EXPORT void qt_gui_c_QNativeGestureEvent_pos_to_output(const QNativeGestureEvent* this_ptr, QPoint* output);
QT_GUI_C_EXPORT const QPointF* qt_gui_c_QNativeGestureEvent_screenPos(const QNativeGestureEvent* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QNativeGestureEvent_value(const QNativeGestureEvent* this_ptr);
QT_GUI_C_EXPORT const QPointF* qt_gui_c_QNativeGestureEvent_windowPos(const QNativeGestureEvent* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QNATIVEGESTUREEVENT_H
