#include "qt_gui_c_QNativeGestureEvent.h"

QNativeGestureEvent* qt_gui_c_QNativeGestureEvent_G_dynamic_cast_QNativeGestureEvent_ptr(QInputEvent* ptr) {
  return dynamic_cast<QNativeGestureEvent*>(ptr);
}

QEvent* qt_gui_c_QNativeGestureEvent_G_static_cast_QEvent_ptr(QNativeGestureEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QInputEvent* qt_gui_c_QNativeGestureEvent_G_static_cast_QInputEvent_ptr(QNativeGestureEvent* ptr) {
  return static_cast<QInputEvent*>(ptr);
}

QNativeGestureEvent* qt_gui_c_QNativeGestureEvent_G_static_cast_QNativeGestureEvent_ptr_QEvent(QEvent* ptr) {
  return static_cast<QNativeGestureEvent*>(ptr);
}

QNativeGestureEvent* qt_gui_c_QNativeGestureEvent_G_static_cast_QNativeGestureEvent_ptr_QInputEvent(QInputEvent* ptr) {
  return static_cast<QNativeGestureEvent*>(ptr);
}

void qt_gui_c_QNativeGestureEvent_delete(QNativeGestureEvent* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QNativeGestureEvent_globalPos_to_output(const QNativeGestureEvent* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->globalPos());
}

const QPointF* qt_gui_c_QNativeGestureEvent_localPos(const QNativeGestureEvent* this_ptr) {
  return &this_ptr->localPos();
}

QNativeGestureEvent* qt_gui_c_QNativeGestureEvent_new(const Qt::NativeGestureType* type, const QPointF* localPos, const QPointF* windowPos, const QPointF* screenPos, double value, unsigned long sequenceId, quint64 intArgument) {
  return new QNativeGestureEvent(*type, *localPos, *windowPos, *screenPos, value, sequenceId, intArgument);
}

void qt_gui_c_QNativeGestureEvent_pos_to_output(const QNativeGestureEvent* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->pos());
}

const QPointF* qt_gui_c_QNativeGestureEvent_screenPos(const QNativeGestureEvent* this_ptr) {
  return &this_ptr->screenPos();
}

double qt_gui_c_QNativeGestureEvent_value(const QNativeGestureEvent* this_ptr) {
  return this_ptr->value();
}

const QPointF* qt_gui_c_QNativeGestureEvent_windowPos(const QNativeGestureEvent* this_ptr) {
  return &this_ptr->windowPos();
}

