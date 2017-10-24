#include "qt_gui_c_QEnterEvent.h"

QEnterEvent* qt_gui_c_QEnterEvent_G_static_cast_QEnterEvent_ptr(QEvent* ptr) {
  return static_cast<QEnterEvent*>(ptr);
}

QEvent* qt_gui_c_QEnterEvent_G_static_cast_QEvent_ptr(QEnterEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

void qt_gui_c_QEnterEvent_delete(QEnterEvent* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QEnterEvent_globalPos_to_output(const QEnterEvent* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->globalPos());
}

int qt_gui_c_QEnterEvent_globalX(const QEnterEvent* this_ptr) {
  return this_ptr->globalX();
}

int qt_gui_c_QEnterEvent_globalY(const QEnterEvent* this_ptr) {
  return this_ptr->globalY();
}

const QPointF* qt_gui_c_QEnterEvent_localPos(const QEnterEvent* this_ptr) {
  return &this_ptr->localPos();
}

QEnterEvent* qt_gui_c_QEnterEvent_new(const QPointF* localPos, const QPointF* windowPos, const QPointF* screenPos) {
  return new QEnterEvent(*localPos, *windowPos, *screenPos);
}

void qt_gui_c_QEnterEvent_pos_to_output(const QEnterEvent* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->pos());
}

const QPointF* qt_gui_c_QEnterEvent_screenPos(const QEnterEvent* this_ptr) {
  return &this_ptr->screenPos();
}

const QPointF* qt_gui_c_QEnterEvent_windowPos(const QEnterEvent* this_ptr) {
  return &this_ptr->windowPos();
}

int qt_gui_c_QEnterEvent_x(const QEnterEvent* this_ptr) {
  return this_ptr->x();
}

int qt_gui_c_QEnterEvent_y(const QEnterEvent* this_ptr) {
  return this_ptr->y();
}

