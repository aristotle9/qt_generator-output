#include "qt_gui_c_QHoverEvent.h"

QHoverEvent* qt_gui_c_QHoverEvent_G_dynamic_cast_QHoverEvent_ptr(QInputEvent* ptr) {
  return dynamic_cast<QHoverEvent*>(ptr);
}

QEvent* qt_gui_c_QHoverEvent_G_static_cast_QEvent_ptr(QHoverEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QHoverEvent* qt_gui_c_QHoverEvent_G_static_cast_QHoverEvent_ptr_QEvent(QEvent* ptr) {
  return static_cast<QHoverEvent*>(ptr);
}

QHoverEvent* qt_gui_c_QHoverEvent_G_static_cast_QHoverEvent_ptr_QInputEvent(QInputEvent* ptr) {
  return static_cast<QHoverEvent*>(ptr);
}

QInputEvent* qt_gui_c_QHoverEvent_G_static_cast_QInputEvent_ptr(QHoverEvent* ptr) {
  return static_cast<QInputEvent*>(ptr);
}

void qt_gui_c_QHoverEvent_delete(QHoverEvent* this_ptr) {
  delete this_ptr;
}

const QPointF* qt_gui_c_QHoverEvent_oldPosF(const QHoverEvent* this_ptr) {
  return &this_ptr->oldPosF();
}

void qt_gui_c_QHoverEvent_oldPos_to_output(const QHoverEvent* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->oldPos());
}

const QPointF* qt_gui_c_QHoverEvent_posF(const QHoverEvent* this_ptr) {
  return &this_ptr->posF();
}

void qt_gui_c_QHoverEvent_pos_to_output(const QHoverEvent* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->pos());
}

