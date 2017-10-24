#include "qt_gui_c_QHelpEvent.h"

QEvent* qt_gui_c_QHelpEvent_G_static_cast_QEvent_ptr(QHelpEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QHelpEvent* qt_gui_c_QHelpEvent_G_static_cast_QHelpEvent_ptr(QEvent* ptr) {
  return static_cast<QHelpEvent*>(ptr);
}

void qt_gui_c_QHelpEvent_delete(QHelpEvent* this_ptr) {
  delete this_ptr;
}

const QPoint* qt_gui_c_QHelpEvent_globalPos(const QHelpEvent* this_ptr) {
  return &this_ptr->globalPos();
}

int qt_gui_c_QHelpEvent_globalX(const QHelpEvent* this_ptr) {
  return this_ptr->globalX();
}

int qt_gui_c_QHelpEvent_globalY(const QHelpEvent* this_ptr) {
  return this_ptr->globalY();
}

QHelpEvent* qt_gui_c_QHelpEvent_new(QEvent::Type type, const QPoint* pos, const QPoint* globalPos) {
  return new QHelpEvent(type, *pos, *globalPos);
}

const QPoint* qt_gui_c_QHelpEvent_pos(const QHelpEvent* this_ptr) {
  return &this_ptr->pos();
}

int qt_gui_c_QHelpEvent_x(const QHelpEvent* this_ptr) {
  return this_ptr->x();
}

int qt_gui_c_QHelpEvent_y(const QHelpEvent* this_ptr) {
  return this_ptr->y();
}

