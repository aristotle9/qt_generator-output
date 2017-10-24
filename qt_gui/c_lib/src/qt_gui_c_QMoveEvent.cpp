#include "qt_gui_c_QMoveEvent.h"

QEvent* qt_gui_c_QMoveEvent_G_static_cast_QEvent_ptr(QMoveEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QMoveEvent* qt_gui_c_QMoveEvent_G_static_cast_QMoveEvent_ptr(QEvent* ptr) {
  return static_cast<QMoveEvent*>(ptr);
}

void qt_gui_c_QMoveEvent_delete(QMoveEvent* this_ptr) {
  delete this_ptr;
}

QMoveEvent* qt_gui_c_QMoveEvent_new(const QPoint* pos, const QPoint* oldPos) {
  return new QMoveEvent(*pos, *oldPos);
}

const QPoint* qt_gui_c_QMoveEvent_oldPos(const QMoveEvent* this_ptr) {
  return &this_ptr->oldPos();
}

const QPoint* qt_gui_c_QMoveEvent_pos(const QMoveEvent* this_ptr) {
  return &this_ptr->pos();
}

