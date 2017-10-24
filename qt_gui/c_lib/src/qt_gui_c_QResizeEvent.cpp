#include "qt_gui_c_QResizeEvent.h"

QEvent* qt_gui_c_QResizeEvent_G_static_cast_QEvent_ptr(QResizeEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QResizeEvent* qt_gui_c_QResizeEvent_G_static_cast_QResizeEvent_ptr(QEvent* ptr) {
  return static_cast<QResizeEvent*>(ptr);
}

void qt_gui_c_QResizeEvent_delete(QResizeEvent* this_ptr) {
  delete this_ptr;
}

QResizeEvent* qt_gui_c_QResizeEvent_new(const QSize* size, const QSize* oldSize) {
  return new QResizeEvent(*size, *oldSize);
}

const QSize* qt_gui_c_QResizeEvent_oldSize(const QResizeEvent* this_ptr) {
  return &this_ptr->oldSize();
}

const QSize* qt_gui_c_QResizeEvent_size(const QResizeEvent* this_ptr) {
  return &this_ptr->size();
}

