#include "qt_widgets_c_QGestureRecognizer.h"

QGesture* qt_widgets_c_QGestureRecognizer_create(QGestureRecognizer* this_ptr, QObject* target) {
  return this_ptr->create(target);
}

void qt_widgets_c_QGestureRecognizer_delete(QGestureRecognizer* this_ptr) {
  delete this_ptr;
}

unsigned int qt_widgets_c_QGestureRecognizer_recognize(QGestureRecognizer* this_ptr, QGesture* state, QObject* watched, QEvent* event) {
  return uint(this_ptr->recognize(state, watched, event));
}

void qt_widgets_c_QGestureRecognizer_reset(QGestureRecognizer* this_ptr, QGesture* state) {
  this_ptr->reset(state);
}

void qt_widgets_c_QGestureRecognizer_unregisterRecognizer(const Qt::GestureType* type) {
  QGestureRecognizer::unregisterRecognizer(*type);
}

