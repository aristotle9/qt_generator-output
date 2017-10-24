#include "qt_widgets_c_QGraphicsSceneHoverEvent.h"

QGraphicsSceneHoverEvent* qt_widgets_c_QGraphicsSceneHoverEvent_G_dynamic_cast_QGraphicsSceneHoverEvent_ptr(QGraphicsSceneEvent* ptr) {
  return dynamic_cast<QGraphicsSceneHoverEvent*>(ptr);
}

QEvent* qt_widgets_c_QGraphicsSceneHoverEvent_G_static_cast_QEvent_ptr(QGraphicsSceneHoverEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QGraphicsSceneEvent* qt_widgets_c_QGraphicsSceneHoverEvent_G_static_cast_QGraphicsSceneEvent_ptr(QGraphicsSceneHoverEvent* ptr) {
  return static_cast<QGraphicsSceneEvent*>(ptr);
}

QGraphicsSceneHoverEvent* qt_widgets_c_QGraphicsSceneHoverEvent_G_static_cast_QGraphicsSceneHoverEvent_ptr_QEvent(QEvent* ptr) {
  return static_cast<QGraphicsSceneHoverEvent*>(ptr);
}

QGraphicsSceneHoverEvent* qt_widgets_c_QGraphicsSceneHoverEvent_G_static_cast_QGraphicsSceneHoverEvent_ptr_QGraphicsSceneEvent(QGraphicsSceneEvent* ptr) {
  return static_cast<QGraphicsSceneHoverEvent*>(ptr);
}

void qt_widgets_c_QGraphicsSceneHoverEvent_delete(QGraphicsSceneHoverEvent* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QGraphicsSceneHoverEvent_lastPos_to_output(const QGraphicsSceneHoverEvent* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->lastPos());
}

void qt_widgets_c_QGraphicsSceneHoverEvent_lastScenePos_to_output(const QGraphicsSceneHoverEvent* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->lastScenePos());
}

void qt_widgets_c_QGraphicsSceneHoverEvent_lastScreenPos_to_output(const QGraphicsSceneHoverEvent* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->lastScreenPos());
}

QGraphicsSceneHoverEvent* qt_widgets_c_QGraphicsSceneHoverEvent_new_no_args() {
  return new QGraphicsSceneHoverEvent();
}

QGraphicsSceneHoverEvent* qt_widgets_c_QGraphicsSceneHoverEvent_new_type(QEvent::Type type) {
  return new QGraphicsSceneHoverEvent(type);
}

void qt_widgets_c_QGraphicsSceneHoverEvent_pos_to_output(const QGraphicsSceneHoverEvent* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->pos());
}

void qt_widgets_c_QGraphicsSceneHoverEvent_scenePos_to_output(const QGraphicsSceneHoverEvent* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->scenePos());
}

void qt_widgets_c_QGraphicsSceneHoverEvent_screenPos_to_output(const QGraphicsSceneHoverEvent* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->screenPos());
}

void qt_widgets_c_QGraphicsSceneHoverEvent_setLastPos(QGraphicsSceneHoverEvent* this_ptr, const QPointF* pos) {
  this_ptr->setLastPos(*pos);
}

void qt_widgets_c_QGraphicsSceneHoverEvent_setLastScenePos(QGraphicsSceneHoverEvent* this_ptr, const QPointF* pos) {
  this_ptr->setLastScenePos(*pos);
}

void qt_widgets_c_QGraphicsSceneHoverEvent_setLastScreenPos(QGraphicsSceneHoverEvent* this_ptr, const QPoint* pos) {
  this_ptr->setLastScreenPos(*pos);
}

void qt_widgets_c_QGraphicsSceneHoverEvent_setPos(QGraphicsSceneHoverEvent* this_ptr, const QPointF* pos) {
  this_ptr->setPos(*pos);
}

void qt_widgets_c_QGraphicsSceneHoverEvent_setScenePos(QGraphicsSceneHoverEvent* this_ptr, const QPointF* pos) {
  this_ptr->setScenePos(*pos);
}

void qt_widgets_c_QGraphicsSceneHoverEvent_setScreenPos(QGraphicsSceneHoverEvent* this_ptr, const QPoint* pos) {
  this_ptr->setScreenPos(*pos);
}

