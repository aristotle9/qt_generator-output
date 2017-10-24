#include "qt_gui_c_QMouseEvent.h"

QMouseEvent* qt_gui_c_QMouseEvent_G_dynamic_cast_QMouseEvent_ptr(QInputEvent* ptr) {
  return dynamic_cast<QMouseEvent*>(ptr);
}

QEvent* qt_gui_c_QMouseEvent_G_static_cast_QEvent_ptr(QMouseEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QInputEvent* qt_gui_c_QMouseEvent_G_static_cast_QInputEvent_ptr(QMouseEvent* ptr) {
  return static_cast<QInputEvent*>(ptr);
}

QMouseEvent* qt_gui_c_QMouseEvent_G_static_cast_QMouseEvent_ptr_QEvent(QEvent* ptr) {
  return static_cast<QMouseEvent*>(ptr);
}

QMouseEvent* qt_gui_c_QMouseEvent_G_static_cast_QMouseEvent_ptr_QInputEvent(QInputEvent* ptr) {
  return static_cast<QMouseEvent*>(ptr);
}

void qt_gui_c_QMouseEvent_delete(QMouseEvent* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QMouseEvent_globalPos_to_output(const QMouseEvent* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->globalPos());
}

int qt_gui_c_QMouseEvent_globalX(const QMouseEvent* this_ptr) {
  return this_ptr->globalX();
}

int qt_gui_c_QMouseEvent_globalY(const QMouseEvent* this_ptr) {
  return this_ptr->globalY();
}

const QPointF* qt_gui_c_QMouseEvent_localPos(const QMouseEvent* this_ptr) {
  return &this_ptr->localPos();
}

void qt_gui_c_QMouseEvent_pos_to_output(const QMouseEvent* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->pos());
}

const QPointF* qt_gui_c_QMouseEvent_screenPos(const QMouseEvent* this_ptr) {
  return &this_ptr->screenPos();
}

void qt_gui_c_QMouseEvent_setLocalPos(QMouseEvent* this_ptr, const QPointF* localPosition) {
  this_ptr->setLocalPos(*localPosition);
}

const QPointF* qt_gui_c_QMouseEvent_windowPos(const QMouseEvent* this_ptr) {
  return &this_ptr->windowPos();
}

int qt_gui_c_QMouseEvent_x(const QMouseEvent* this_ptr) {
  return this_ptr->x();
}

int qt_gui_c_QMouseEvent_y(const QMouseEvent* this_ptr) {
  return this_ptr->y();
}

