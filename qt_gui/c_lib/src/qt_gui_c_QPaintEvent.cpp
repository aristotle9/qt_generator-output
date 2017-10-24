#include "qt_gui_c_QPaintEvent.h"

QEvent* qt_gui_c_QPaintEvent_G_static_cast_QEvent_ptr(QPaintEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QPaintEvent* qt_gui_c_QPaintEvent_G_static_cast_QPaintEvent_ptr(QEvent* ptr) {
  return static_cast<QPaintEvent*>(ptr);
}

void qt_gui_c_QPaintEvent_delete(QPaintEvent* this_ptr) {
  delete this_ptr;
}

QPaintEvent* qt_gui_c_QPaintEvent_new_paintRect(const QRect* paintRect) {
  return new QPaintEvent(*paintRect);
}

QPaintEvent* qt_gui_c_QPaintEvent_new_paintRegion(const QRegion* paintRegion) {
  return new QPaintEvent(*paintRegion);
}

const QRect* qt_gui_c_QPaintEvent_rect(const QPaintEvent* this_ptr) {
  return &this_ptr->rect();
}

const QRegion* qt_gui_c_QPaintEvent_region(const QPaintEvent* this_ptr) {
  return &this_ptr->region();
}

