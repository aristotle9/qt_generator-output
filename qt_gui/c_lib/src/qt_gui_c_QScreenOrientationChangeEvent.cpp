#include "qt_gui_c_QScreenOrientationChangeEvent.h"

QEvent* qt_gui_c_QScreenOrientationChangeEvent_G_static_cast_QEvent_ptr(QScreenOrientationChangeEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QScreenOrientationChangeEvent* qt_gui_c_QScreenOrientationChangeEvent_G_static_cast_QScreenOrientationChangeEvent_ptr(QEvent* ptr) {
  return static_cast<QScreenOrientationChangeEvent*>(ptr);
}

void qt_gui_c_QScreenOrientationChangeEvent_delete(QScreenOrientationChangeEvent* this_ptr) {
  delete this_ptr;
}

QScreenOrientationChangeEvent* qt_gui_c_QScreenOrientationChangeEvent_new(QScreen* screen, const Qt::ScreenOrientation* orientation) {
  return new QScreenOrientationChangeEvent(screen, *orientation);
}

QScreen* qt_gui_c_QScreenOrientationChangeEvent_screen(const QScreenOrientationChangeEvent* this_ptr) {
  return this_ptr->screen();
}

