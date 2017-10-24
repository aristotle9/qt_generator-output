#include "qt_gui_c_QDragMoveEvent.h"

QDragMoveEvent* qt_gui_c_QDragMoveEvent_G_dynamic_cast_QDragMoveEvent_ptr(QDropEvent* ptr) {
  return dynamic_cast<QDragMoveEvent*>(ptr);
}

QDragMoveEvent* qt_gui_c_QDragMoveEvent_G_static_cast_QDragMoveEvent_ptr_QDropEvent(QDropEvent* ptr) {
  return static_cast<QDragMoveEvent*>(ptr);
}

QDragMoveEvent* qt_gui_c_QDragMoveEvent_G_static_cast_QDragMoveEvent_ptr_QEvent(QEvent* ptr) {
  return static_cast<QDragMoveEvent*>(ptr);
}

QDropEvent* qt_gui_c_QDragMoveEvent_G_static_cast_QDropEvent_ptr(QDragMoveEvent* ptr) {
  return static_cast<QDropEvent*>(ptr);
}

QEvent* qt_gui_c_QDragMoveEvent_G_static_cast_QEvent_ptr(QDragMoveEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

void qt_gui_c_QDragMoveEvent_accept_no_args(QDragMoveEvent* this_ptr) {
  this_ptr->accept();
}

void qt_gui_c_QDragMoveEvent_accept_r(QDragMoveEvent* this_ptr, const QRect* r) {
  this_ptr->accept(*r);
}

void qt_gui_c_QDragMoveEvent_answerRect_to_output(const QDragMoveEvent* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->answerRect());
}

void qt_gui_c_QDragMoveEvent_delete(QDragMoveEvent* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QDragMoveEvent_ignore_no_args(QDragMoveEvent* this_ptr) {
  this_ptr->ignore();
}

void qt_gui_c_QDragMoveEvent_ignore_r(QDragMoveEvent* this_ptr, const QRect* r) {
  this_ptr->ignore(*r);
}

