#include "qt_widgets_c_QGraphicsSceneContextMenuEvent.h"

QGraphicsSceneContextMenuEvent* qt_widgets_c_QGraphicsSceneContextMenuEvent_G_dynamic_cast_QGraphicsSceneContextMenuEvent_ptr(QGraphicsSceneEvent* ptr) {
  return dynamic_cast<QGraphicsSceneContextMenuEvent*>(ptr);
}

QEvent* qt_widgets_c_QGraphicsSceneContextMenuEvent_G_static_cast_QEvent_ptr(QGraphicsSceneContextMenuEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QGraphicsSceneContextMenuEvent* qt_widgets_c_QGraphicsSceneContextMenuEvent_G_static_cast_QGraphicsSceneContextMenuEvent_ptr_QEvent(QEvent* ptr) {
  return static_cast<QGraphicsSceneContextMenuEvent*>(ptr);
}

QGraphicsSceneContextMenuEvent* qt_widgets_c_QGraphicsSceneContextMenuEvent_G_static_cast_QGraphicsSceneContextMenuEvent_ptr_QGraphicsSceneEvent(QGraphicsSceneEvent* ptr) {
  return static_cast<QGraphicsSceneContextMenuEvent*>(ptr);
}

QGraphicsSceneEvent* qt_widgets_c_QGraphicsSceneContextMenuEvent_G_static_cast_QGraphicsSceneEvent_ptr(QGraphicsSceneContextMenuEvent* ptr) {
  return static_cast<QGraphicsSceneEvent*>(ptr);
}

void qt_widgets_c_QGraphicsSceneContextMenuEvent_delete(QGraphicsSceneContextMenuEvent* this_ptr) {
  delete this_ptr;
}

QGraphicsSceneContextMenuEvent* qt_widgets_c_QGraphicsSceneContextMenuEvent_new_no_args() {
  return new QGraphicsSceneContextMenuEvent();
}

QGraphicsSceneContextMenuEvent* qt_widgets_c_QGraphicsSceneContextMenuEvent_new_type(QEvent::Type type) {
  return new QGraphicsSceneContextMenuEvent(type);
}

void qt_widgets_c_QGraphicsSceneContextMenuEvent_pos_to_output(const QGraphicsSceneContextMenuEvent* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->pos());
}

QGraphicsSceneContextMenuEvent::Reason qt_widgets_c_QGraphicsSceneContextMenuEvent_reason(const QGraphicsSceneContextMenuEvent* this_ptr) {
  return this_ptr->reason();
}

void qt_widgets_c_QGraphicsSceneContextMenuEvent_scenePos_to_output(const QGraphicsSceneContextMenuEvent* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->scenePos());
}

void qt_widgets_c_QGraphicsSceneContextMenuEvent_screenPos_to_output(const QGraphicsSceneContextMenuEvent* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->screenPos());
}

void qt_widgets_c_QGraphicsSceneContextMenuEvent_setPos(QGraphicsSceneContextMenuEvent* this_ptr, const QPointF* pos) {
  this_ptr->setPos(*pos);
}

void qt_widgets_c_QGraphicsSceneContextMenuEvent_setReason(QGraphicsSceneContextMenuEvent* this_ptr, QGraphicsSceneContextMenuEvent::Reason reason) {
  this_ptr->setReason(reason);
}

void qt_widgets_c_QGraphicsSceneContextMenuEvent_setScenePos(QGraphicsSceneContextMenuEvent* this_ptr, const QPointF* pos) {
  this_ptr->setScenePos(*pos);
}

void qt_widgets_c_QGraphicsSceneContextMenuEvent_setScreenPos(QGraphicsSceneContextMenuEvent* this_ptr, const QPoint* pos) {
  this_ptr->setScreenPos(*pos);
}

