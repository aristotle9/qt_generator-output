#include "qt_widgets_c_QGraphicsSceneMoveEvent.h"

QGraphicsSceneMoveEvent* qt_widgets_c_QGraphicsSceneMoveEvent_G_dynamic_cast_QGraphicsSceneMoveEvent_ptr(QGraphicsSceneEvent* ptr) {
  return dynamic_cast<QGraphicsSceneMoveEvent*>(ptr);
}

QEvent* qt_widgets_c_QGraphicsSceneMoveEvent_G_static_cast_QEvent_ptr(QGraphicsSceneMoveEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QGraphicsSceneEvent* qt_widgets_c_QGraphicsSceneMoveEvent_G_static_cast_QGraphicsSceneEvent_ptr(QGraphicsSceneMoveEvent* ptr) {
  return static_cast<QGraphicsSceneEvent*>(ptr);
}

QGraphicsSceneMoveEvent* qt_widgets_c_QGraphicsSceneMoveEvent_G_static_cast_QGraphicsSceneMoveEvent_ptr_QEvent(QEvent* ptr) {
  return static_cast<QGraphicsSceneMoveEvent*>(ptr);
}

QGraphicsSceneMoveEvent* qt_widgets_c_QGraphicsSceneMoveEvent_G_static_cast_QGraphicsSceneMoveEvent_ptr_QGraphicsSceneEvent(QGraphicsSceneEvent* ptr) {
  return static_cast<QGraphicsSceneMoveEvent*>(ptr);
}

void qt_widgets_c_QGraphicsSceneMoveEvent_delete(QGraphicsSceneMoveEvent* this_ptr) {
  delete this_ptr;
}

QGraphicsSceneMoveEvent* qt_widgets_c_QGraphicsSceneMoveEvent_new() {
  return new QGraphicsSceneMoveEvent();
}

void qt_widgets_c_QGraphicsSceneMoveEvent_newPos_to_output(const QGraphicsSceneMoveEvent* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->newPos());
}

void qt_widgets_c_QGraphicsSceneMoveEvent_oldPos_to_output(const QGraphicsSceneMoveEvent* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->oldPos());
}

void qt_widgets_c_QGraphicsSceneMoveEvent_setNewPos(QGraphicsSceneMoveEvent* this_ptr, const QPointF* pos) {
  this_ptr->setNewPos(*pos);
}

void qt_widgets_c_QGraphicsSceneMoveEvent_setOldPos(QGraphicsSceneMoveEvent* this_ptr, const QPointF* pos) {
  this_ptr->setOldPos(*pos);
}

