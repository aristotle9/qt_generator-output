#include "qt_gui_c_QExposeEvent.h"

QEvent* qt_gui_c_QExposeEvent_G_static_cast_QEvent_ptr(QExposeEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QExposeEvent* qt_gui_c_QExposeEvent_G_static_cast_QExposeEvent_ptr(QEvent* ptr) {
  return static_cast<QExposeEvent*>(ptr);
}

void qt_gui_c_QExposeEvent_delete(QExposeEvent* this_ptr) {
  delete this_ptr;
}

QExposeEvent* qt_gui_c_QExposeEvent_new(const QRegion* rgn) {
  return new QExposeEvent(*rgn);
}

const QRegion* qt_gui_c_QExposeEvent_region(const QExposeEvent* this_ptr) {
  return &this_ptr->region();
}

