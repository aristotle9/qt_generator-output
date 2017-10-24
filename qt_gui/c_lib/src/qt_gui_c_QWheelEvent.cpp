#include "qt_gui_c_QWheelEvent.h"

QWheelEvent* qt_gui_c_QWheelEvent_G_dynamic_cast_QWheelEvent_ptr(QInputEvent* ptr) {
  return dynamic_cast<QWheelEvent*>(ptr);
}

QEvent* qt_gui_c_QWheelEvent_G_static_cast_QEvent_ptr(QWheelEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QInputEvent* qt_gui_c_QWheelEvent_G_static_cast_QInputEvent_ptr(QWheelEvent* ptr) {
  return static_cast<QInputEvent*>(ptr);
}

QWheelEvent* qt_gui_c_QWheelEvent_G_static_cast_QWheelEvent_ptr_QEvent(QEvent* ptr) {
  return static_cast<QWheelEvent*>(ptr);
}

QWheelEvent* qt_gui_c_QWheelEvent_G_static_cast_QWheelEvent_ptr_QInputEvent(QInputEvent* ptr) {
  return static_cast<QWheelEvent*>(ptr);
}

void qt_gui_c_QWheelEvent_angleDelta_to_output(const QWheelEvent* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->angleDelta());
}

void qt_gui_c_QWheelEvent_delete(QWheelEvent* this_ptr) {
  delete this_ptr;
}

int qt_gui_c_QWheelEvent_delta(const QWheelEvent* this_ptr) {
  return this_ptr->delta();
}

const QPointF* qt_gui_c_QWheelEvent_globalPosF(const QWheelEvent* this_ptr) {
  return &this_ptr->globalPosF();
}

void qt_gui_c_QWheelEvent_globalPos_to_output(const QWheelEvent* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->globalPos());
}

int qt_gui_c_QWheelEvent_globalX(const QWheelEvent* this_ptr) {
  return this_ptr->globalX();
}

int qt_gui_c_QWheelEvent_globalY(const QWheelEvent* this_ptr) {
  return this_ptr->globalY();
}

bool qt_gui_c_QWheelEvent_inverted(const QWheelEvent* this_ptr) {
  return this_ptr->inverted();
}

void qt_gui_c_QWheelEvent_pixelDelta_to_output(const QWheelEvent* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->pixelDelta());
}

const QPointF* qt_gui_c_QWheelEvent_posF(const QWheelEvent* this_ptr) {
  return &this_ptr->posF();
}

void qt_gui_c_QWheelEvent_pos_to_output(const QWheelEvent* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->pos());
}

int qt_gui_c_QWheelEvent_x(const QWheelEvent* this_ptr) {
  return this_ptr->x();
}

int qt_gui_c_QWheelEvent_y(const QWheelEvent* this_ptr) {
  return this_ptr->y();
}

