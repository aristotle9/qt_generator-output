#include "qt_gui_c_QContextMenuEvent.h"

QContextMenuEvent* qt_gui_c_QContextMenuEvent_G_dynamic_cast_QContextMenuEvent_ptr(QInputEvent* ptr) {
  return dynamic_cast<QContextMenuEvent*>(ptr);
}

QContextMenuEvent* qt_gui_c_QContextMenuEvent_G_static_cast_QContextMenuEvent_ptr_QEvent(QEvent* ptr) {
  return static_cast<QContextMenuEvent*>(ptr);
}

QContextMenuEvent* qt_gui_c_QContextMenuEvent_G_static_cast_QContextMenuEvent_ptr_QInputEvent(QInputEvent* ptr) {
  return static_cast<QContextMenuEvent*>(ptr);
}

QEvent* qt_gui_c_QContextMenuEvent_G_static_cast_QEvent_ptr(QContextMenuEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QInputEvent* qt_gui_c_QContextMenuEvent_G_static_cast_QInputEvent_ptr(QContextMenuEvent* ptr) {
  return static_cast<QInputEvent*>(ptr);
}

void qt_gui_c_QContextMenuEvent_delete(QContextMenuEvent* this_ptr) {
  delete this_ptr;
}

const QPoint* qt_gui_c_QContextMenuEvent_globalPos(const QContextMenuEvent* this_ptr) {
  return &this_ptr->globalPos();
}

int qt_gui_c_QContextMenuEvent_globalX(const QContextMenuEvent* this_ptr) {
  return this_ptr->globalX();
}

int qt_gui_c_QContextMenuEvent_globalY(const QContextMenuEvent* this_ptr) {
  return this_ptr->globalY();
}

QContextMenuEvent* qt_gui_c_QContextMenuEvent_new_reason_pos(QContextMenuEvent::Reason reason, const QPoint* pos) {
  return new QContextMenuEvent(reason, *pos);
}

QContextMenuEvent* qt_gui_c_QContextMenuEvent_new_reason_pos_globalPos(QContextMenuEvent::Reason reason, const QPoint* pos, const QPoint* globalPos) {
  return new QContextMenuEvent(reason, *pos, *globalPos);
}

const QPoint* qt_gui_c_QContextMenuEvent_pos(const QContextMenuEvent* this_ptr) {
  return &this_ptr->pos();
}

QContextMenuEvent::Reason qt_gui_c_QContextMenuEvent_reason(const QContextMenuEvent* this_ptr) {
  return this_ptr->reason();
}

int qt_gui_c_QContextMenuEvent_x(const QContextMenuEvent* this_ptr) {
  return this_ptr->x();
}

int qt_gui_c_QContextMenuEvent_y(const QContextMenuEvent* this_ptr) {
  return this_ptr->y();
}

