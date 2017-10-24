#include "qt_widgets_c_QGraphicsSceneWheelEvent.h"

QGraphicsSceneWheelEvent* qt_widgets_c_QGraphicsSceneWheelEvent_G_dynamic_cast_QGraphicsSceneWheelEvent_ptr(QGraphicsSceneEvent* ptr) {
  return dynamic_cast<QGraphicsSceneWheelEvent*>(ptr);
}

QEvent* qt_widgets_c_QGraphicsSceneWheelEvent_G_static_cast_QEvent_ptr(QGraphicsSceneWheelEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QGraphicsSceneEvent* qt_widgets_c_QGraphicsSceneWheelEvent_G_static_cast_QGraphicsSceneEvent_ptr(QGraphicsSceneWheelEvent* ptr) {
  return static_cast<QGraphicsSceneEvent*>(ptr);
}

QGraphicsSceneWheelEvent* qt_widgets_c_QGraphicsSceneWheelEvent_G_static_cast_QGraphicsSceneWheelEvent_ptr_QEvent(QEvent* ptr) {
  return static_cast<QGraphicsSceneWheelEvent*>(ptr);
}

QGraphicsSceneWheelEvent* qt_widgets_c_QGraphicsSceneWheelEvent_G_static_cast_QGraphicsSceneWheelEvent_ptr_QGraphicsSceneEvent(QGraphicsSceneEvent* ptr) {
  return static_cast<QGraphicsSceneWheelEvent*>(ptr);
}

void qt_widgets_c_QGraphicsSceneWheelEvent_delete(QGraphicsSceneWheelEvent* this_ptr) {
  delete this_ptr;
}

int qt_widgets_c_QGraphicsSceneWheelEvent_delta(const QGraphicsSceneWheelEvent* this_ptr) {
  return this_ptr->delta();
}

QGraphicsSceneWheelEvent* qt_widgets_c_QGraphicsSceneWheelEvent_new_no_args() {
  return new QGraphicsSceneWheelEvent();
}

QGraphicsSceneWheelEvent* qt_widgets_c_QGraphicsSceneWheelEvent_new_type(QEvent::Type type) {
  return new QGraphicsSceneWheelEvent(type);
}

void qt_widgets_c_QGraphicsSceneWheelEvent_pos_to_output(const QGraphicsSceneWheelEvent* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->pos());
}

void qt_widgets_c_QGraphicsSceneWheelEvent_scenePos_to_output(const QGraphicsSceneWheelEvent* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->scenePos());
}

void qt_widgets_c_QGraphicsSceneWheelEvent_screenPos_to_output(const QGraphicsSceneWheelEvent* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->screenPos());
}

void qt_widgets_c_QGraphicsSceneWheelEvent_setDelta(QGraphicsSceneWheelEvent* this_ptr, int delta) {
  this_ptr->setDelta(delta);
}

void qt_widgets_c_QGraphicsSceneWheelEvent_setOrientation(QGraphicsSceneWheelEvent* this_ptr, const Qt::Orientation* orientation) {
  this_ptr->setOrientation(*orientation);
}

void qt_widgets_c_QGraphicsSceneWheelEvent_setPos(QGraphicsSceneWheelEvent* this_ptr, const QPointF* pos) {
  this_ptr->setPos(*pos);
}

void qt_widgets_c_QGraphicsSceneWheelEvent_setScenePos(QGraphicsSceneWheelEvent* this_ptr, const QPointF* pos) {
  this_ptr->setScenePos(*pos);
}

void qt_widgets_c_QGraphicsSceneWheelEvent_setScreenPos(QGraphicsSceneWheelEvent* this_ptr, const QPoint* pos) {
  this_ptr->setScreenPos(*pos);
}

