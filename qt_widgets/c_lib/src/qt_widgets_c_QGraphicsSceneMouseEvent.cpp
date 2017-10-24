#include "qt_widgets_c_QGraphicsSceneMouseEvent.h"

QGraphicsSceneMouseEvent* qt_widgets_c_QGraphicsSceneMouseEvent_G_dynamic_cast_QGraphicsSceneMouseEvent_ptr(QGraphicsSceneEvent* ptr) {
  return dynamic_cast<QGraphicsSceneMouseEvent*>(ptr);
}

QEvent* qt_widgets_c_QGraphicsSceneMouseEvent_G_static_cast_QEvent_ptr(QGraphicsSceneMouseEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QGraphicsSceneEvent* qt_widgets_c_QGraphicsSceneMouseEvent_G_static_cast_QGraphicsSceneEvent_ptr(QGraphicsSceneMouseEvent* ptr) {
  return static_cast<QGraphicsSceneEvent*>(ptr);
}

QGraphicsSceneMouseEvent* qt_widgets_c_QGraphicsSceneMouseEvent_G_static_cast_QGraphicsSceneMouseEvent_ptr_QEvent(QEvent* ptr) {
  return static_cast<QGraphicsSceneMouseEvent*>(ptr);
}

QGraphicsSceneMouseEvent* qt_widgets_c_QGraphicsSceneMouseEvent_G_static_cast_QGraphicsSceneMouseEvent_ptr_QGraphicsSceneEvent(QGraphicsSceneEvent* ptr) {
  return static_cast<QGraphicsSceneMouseEvent*>(ptr);
}

void qt_widgets_c_QGraphicsSceneMouseEvent_buttonDownPos_to_output(const QGraphicsSceneMouseEvent* this_ptr, const Qt::MouseButton* button, QPointF* output) {
  new(output) QPointF(this_ptr->buttonDownPos(*button));
}

void qt_widgets_c_QGraphicsSceneMouseEvent_buttonDownScenePos_to_output(const QGraphicsSceneMouseEvent* this_ptr, const Qt::MouseButton* button, QPointF* output) {
  new(output) QPointF(this_ptr->buttonDownScenePos(*button));
}

void qt_widgets_c_QGraphicsSceneMouseEvent_buttonDownScreenPos_to_output(const QGraphicsSceneMouseEvent* this_ptr, const Qt::MouseButton* button, QPoint* output) {
  new(output) QPoint(this_ptr->buttonDownScreenPos(*button));
}

void qt_widgets_c_QGraphicsSceneMouseEvent_delete(QGraphicsSceneMouseEvent* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QGraphicsSceneMouseEvent_lastPos_to_output(const QGraphicsSceneMouseEvent* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->lastPos());
}

void qt_widgets_c_QGraphicsSceneMouseEvent_lastScenePos_to_output(const QGraphicsSceneMouseEvent* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->lastScenePos());
}

void qt_widgets_c_QGraphicsSceneMouseEvent_lastScreenPos_to_output(const QGraphicsSceneMouseEvent* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->lastScreenPos());
}

QGraphicsSceneMouseEvent* qt_widgets_c_QGraphicsSceneMouseEvent_new_no_args() {
  return new QGraphicsSceneMouseEvent();
}

QGraphicsSceneMouseEvent* qt_widgets_c_QGraphicsSceneMouseEvent_new_type(QEvent::Type type) {
  return new QGraphicsSceneMouseEvent(type);
}

void qt_widgets_c_QGraphicsSceneMouseEvent_pos_to_output(const QGraphicsSceneMouseEvent* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->pos());
}

void qt_widgets_c_QGraphicsSceneMouseEvent_scenePos_to_output(const QGraphicsSceneMouseEvent* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->scenePos());
}

void qt_widgets_c_QGraphicsSceneMouseEvent_screenPos_to_output(const QGraphicsSceneMouseEvent* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->screenPos());
}

void qt_widgets_c_QGraphicsSceneMouseEvent_setButton(QGraphicsSceneMouseEvent* this_ptr, const Qt::MouseButton* button) {
  this_ptr->setButton(*button);
}

void qt_widgets_c_QGraphicsSceneMouseEvent_setButtonDownPos(QGraphicsSceneMouseEvent* this_ptr, const Qt::MouseButton* button, const QPointF* pos) {
  this_ptr->setButtonDownPos(*button, *pos);
}

void qt_widgets_c_QGraphicsSceneMouseEvent_setButtonDownScenePos(QGraphicsSceneMouseEvent* this_ptr, const Qt::MouseButton* button, const QPointF* pos) {
  this_ptr->setButtonDownScenePos(*button, *pos);
}

void qt_widgets_c_QGraphicsSceneMouseEvent_setButtonDownScreenPos(QGraphicsSceneMouseEvent* this_ptr, const Qt::MouseButton* button, const QPoint* pos) {
  this_ptr->setButtonDownScreenPos(*button, *pos);
}

void qt_widgets_c_QGraphicsSceneMouseEvent_setLastPos(QGraphicsSceneMouseEvent* this_ptr, const QPointF* pos) {
  this_ptr->setLastPos(*pos);
}

void qt_widgets_c_QGraphicsSceneMouseEvent_setLastScenePos(QGraphicsSceneMouseEvent* this_ptr, const QPointF* pos) {
  this_ptr->setLastScenePos(*pos);
}

void qt_widgets_c_QGraphicsSceneMouseEvent_setLastScreenPos(QGraphicsSceneMouseEvent* this_ptr, const QPoint* pos) {
  this_ptr->setLastScreenPos(*pos);
}

void qt_widgets_c_QGraphicsSceneMouseEvent_setPos(QGraphicsSceneMouseEvent* this_ptr, const QPointF* pos) {
  this_ptr->setPos(*pos);
}

void qt_widgets_c_QGraphicsSceneMouseEvent_setScenePos(QGraphicsSceneMouseEvent* this_ptr, const QPointF* pos) {
  this_ptr->setScenePos(*pos);
}

void qt_widgets_c_QGraphicsSceneMouseEvent_setScreenPos(QGraphicsSceneMouseEvent* this_ptr, const QPoint* pos) {
  this_ptr->setScreenPos(*pos);
}

void qt_widgets_c_QGraphicsSceneMouseEvent_setSource(QGraphicsSceneMouseEvent* this_ptr, const Qt::MouseEventSource* source) {
  this_ptr->setSource(*source);
}

