#include "qt_gui_c_QScrollEvent.h"

QEvent* qt_gui_c_QScrollEvent_G_static_cast_QEvent_ptr(QScrollEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QScrollEvent* qt_gui_c_QScrollEvent_G_static_cast_QScrollEvent_ptr(QEvent* ptr) {
  return static_cast<QScrollEvent*>(ptr);
}

void qt_gui_c_QScrollEvent_contentPos_to_output(const QScrollEvent* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->contentPos());
}

void qt_gui_c_QScrollEvent_delete(QScrollEvent* this_ptr) {
  delete this_ptr;
}

QScrollEvent* qt_gui_c_QScrollEvent_new(const QPointF* contentPos, const QPointF* overshoot, QScrollEvent::ScrollState scrollState) {
  return new QScrollEvent(*contentPos, *overshoot, scrollState);
}

void qt_gui_c_QScrollEvent_overshootDistance_to_output(const QScrollEvent* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->overshootDistance());
}

QScrollEvent::ScrollState qt_gui_c_QScrollEvent_scrollState(const QScrollEvent* this_ptr) {
  return this_ptr->scrollState();
}

