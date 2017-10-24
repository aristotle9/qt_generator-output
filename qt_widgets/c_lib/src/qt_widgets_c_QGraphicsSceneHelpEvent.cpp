#include "qt_widgets_c_QGraphicsSceneHelpEvent.h"

QGraphicsSceneHelpEvent* qt_widgets_c_QGraphicsSceneHelpEvent_G_dynamic_cast_QGraphicsSceneHelpEvent_ptr(QGraphicsSceneEvent* ptr) {
  return dynamic_cast<QGraphicsSceneHelpEvent*>(ptr);
}

QEvent* qt_widgets_c_QGraphicsSceneHelpEvent_G_static_cast_QEvent_ptr(QGraphicsSceneHelpEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QGraphicsSceneEvent* qt_widgets_c_QGraphicsSceneHelpEvent_G_static_cast_QGraphicsSceneEvent_ptr(QGraphicsSceneHelpEvent* ptr) {
  return static_cast<QGraphicsSceneEvent*>(ptr);
}

QGraphicsSceneHelpEvent* qt_widgets_c_QGraphicsSceneHelpEvent_G_static_cast_QGraphicsSceneHelpEvent_ptr_QEvent(QEvent* ptr) {
  return static_cast<QGraphicsSceneHelpEvent*>(ptr);
}

QGraphicsSceneHelpEvent* qt_widgets_c_QGraphicsSceneHelpEvent_G_static_cast_QGraphicsSceneHelpEvent_ptr_QGraphicsSceneEvent(QGraphicsSceneEvent* ptr) {
  return static_cast<QGraphicsSceneHelpEvent*>(ptr);
}

void qt_widgets_c_QGraphicsSceneHelpEvent_delete(QGraphicsSceneHelpEvent* this_ptr) {
  delete this_ptr;
}

QGraphicsSceneHelpEvent* qt_widgets_c_QGraphicsSceneHelpEvent_new_no_args() {
  return new QGraphicsSceneHelpEvent();
}

QGraphicsSceneHelpEvent* qt_widgets_c_QGraphicsSceneHelpEvent_new_type(QEvent::Type type) {
  return new QGraphicsSceneHelpEvent(type);
}

void qt_widgets_c_QGraphicsSceneHelpEvent_scenePos_to_output(const QGraphicsSceneHelpEvent* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->scenePos());
}

void qt_widgets_c_QGraphicsSceneHelpEvent_screenPos_to_output(const QGraphicsSceneHelpEvent* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->screenPos());
}

void qt_widgets_c_QGraphicsSceneHelpEvent_setScenePos(QGraphicsSceneHelpEvent* this_ptr, const QPointF* pos) {
  this_ptr->setScenePos(*pos);
}

void qt_widgets_c_QGraphicsSceneHelpEvent_setScreenPos(QGraphicsSceneHelpEvent* this_ptr, const QPoint* pos) {
  this_ptr->setScreenPos(*pos);
}

