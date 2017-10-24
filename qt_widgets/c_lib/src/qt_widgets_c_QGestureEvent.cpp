#include "qt_widgets_c_QGestureEvent.h"

QEvent* qt_widgets_c_QGestureEvent_G_static_cast_QEvent_ptr(QGestureEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QGestureEvent* qt_widgets_c_QGestureEvent_G_static_cast_QGestureEvent_ptr(QEvent* ptr) {
  return static_cast<QGestureEvent*>(ptr);
}

void qt_widgets_c_QGestureEvent_accept_QGesture(QGestureEvent* this_ptr, QGesture* arg1) {
  this_ptr->accept(arg1);
}

void qt_widgets_c_QGestureEvent_accept_Qt_GestureType(QGestureEvent* this_ptr, const Qt::GestureType* arg1) {
  this_ptr->accept(*arg1);
}

void qt_widgets_c_QGestureEvent_activeGestures_to_output(const QGestureEvent* this_ptr, QList< QGesture* >* output) {
  new(output) QList< QGesture* >(this_ptr->activeGestures());
}

void qt_widgets_c_QGestureEvent_canceledGestures_to_output(const QGestureEvent* this_ptr, QList< QGesture* >* output) {
  new(output) QList< QGesture* >(this_ptr->canceledGestures());
}

void qt_widgets_c_QGestureEvent_delete(QGestureEvent* this_ptr) {
  delete this_ptr;
}

QGesture* qt_widgets_c_QGestureEvent_gesture(const QGestureEvent* this_ptr, const Qt::GestureType* type) {
  return this_ptr->gesture(*type);
}

void qt_widgets_c_QGestureEvent_gestures_to_output(const QGestureEvent* this_ptr, QList< QGesture* >* output) {
  new(output) QList< QGesture* >(this_ptr->gestures());
}

void qt_widgets_c_QGestureEvent_ignore_QGesture(QGestureEvent* this_ptr, QGesture* arg1) {
  this_ptr->ignore(arg1);
}

void qt_widgets_c_QGestureEvent_ignore_Qt_GestureType(QGestureEvent* this_ptr, const Qt::GestureType* arg1) {
  this_ptr->ignore(*arg1);
}

bool qt_widgets_c_QGestureEvent_isAccepted_QGesture(const QGestureEvent* this_ptr, QGesture* arg1) {
  return this_ptr->isAccepted(arg1);
}

bool qt_widgets_c_QGestureEvent_isAccepted_Qt_GestureType(const QGestureEvent* this_ptr, const Qt::GestureType* arg1) {
  return this_ptr->isAccepted(*arg1);
}

void qt_widgets_c_QGestureEvent_mapToGraphicsScene_to_output(const QGestureEvent* this_ptr, const QPointF* gesturePoint, QPointF* output) {
  new(output) QPointF(this_ptr->mapToGraphicsScene(*gesturePoint));
}

QGestureEvent* qt_widgets_c_QGestureEvent_new(const QList< QGesture* >* gestures) {
  return new QGestureEvent(*gestures);
}

void qt_widgets_c_QGestureEvent_setAccepted_QGesture_bool(QGestureEvent* this_ptr, QGesture* arg1, bool arg2) {
  this_ptr->setAccepted(arg1, arg2);
}

void qt_widgets_c_QGestureEvent_setAccepted_Qt_GestureType_bool(QGestureEvent* this_ptr, const Qt::GestureType* arg1, bool arg2) {
  this_ptr->setAccepted(*arg1, arg2);
}

void qt_widgets_c_QGestureEvent_setWidget(QGestureEvent* this_ptr, QWidget* widget) {
  this_ptr->setWidget(widget);
}

QWidget* qt_widgets_c_QGestureEvent_widget(const QGestureEvent* this_ptr) {
  return this_ptr->widget();
}

