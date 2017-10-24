#include "qt_widgets_c_QGraphicsSceneDragDropEvent.h"

QGraphicsSceneDragDropEvent* qt_widgets_c_QGraphicsSceneDragDropEvent_G_dynamic_cast_QGraphicsSceneDragDropEvent_ptr(QGraphicsSceneEvent* ptr) {
  return dynamic_cast<QGraphicsSceneDragDropEvent*>(ptr);
}

QEvent* qt_widgets_c_QGraphicsSceneDragDropEvent_G_static_cast_QEvent_ptr(QGraphicsSceneDragDropEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QGraphicsSceneDragDropEvent* qt_widgets_c_QGraphicsSceneDragDropEvent_G_static_cast_QGraphicsSceneDragDropEvent_ptr_QEvent(QEvent* ptr) {
  return static_cast<QGraphicsSceneDragDropEvent*>(ptr);
}

QGraphicsSceneDragDropEvent* qt_widgets_c_QGraphicsSceneDragDropEvent_G_static_cast_QGraphicsSceneDragDropEvent_ptr_QGraphicsSceneEvent(QGraphicsSceneEvent* ptr) {
  return static_cast<QGraphicsSceneDragDropEvent*>(ptr);
}

QGraphicsSceneEvent* qt_widgets_c_QGraphicsSceneDragDropEvent_G_static_cast_QGraphicsSceneEvent_ptr(QGraphicsSceneDragDropEvent* ptr) {
  return static_cast<QGraphicsSceneEvent*>(ptr);
}

void qt_widgets_c_QGraphicsSceneDragDropEvent_acceptProposedAction(QGraphicsSceneDragDropEvent* this_ptr) {
  this_ptr->acceptProposedAction();
}

void qt_widgets_c_QGraphicsSceneDragDropEvent_delete(QGraphicsSceneDragDropEvent* this_ptr) {
  delete this_ptr;
}

const QMimeData* qt_widgets_c_QGraphicsSceneDragDropEvent_mimeData(const QGraphicsSceneDragDropEvent* this_ptr) {
  return this_ptr->mimeData();
}

QGraphicsSceneDragDropEvent* qt_widgets_c_QGraphicsSceneDragDropEvent_new_no_args() {
  return new QGraphicsSceneDragDropEvent();
}

QGraphicsSceneDragDropEvent* qt_widgets_c_QGraphicsSceneDragDropEvent_new_type(QEvent::Type type) {
  return new QGraphicsSceneDragDropEvent(type);
}

void qt_widgets_c_QGraphicsSceneDragDropEvent_pos_to_output(const QGraphicsSceneDragDropEvent* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->pos());
}

void qt_widgets_c_QGraphicsSceneDragDropEvent_scenePos_to_output(const QGraphicsSceneDragDropEvent* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->scenePos());
}

void qt_widgets_c_QGraphicsSceneDragDropEvent_screenPos_to_output(const QGraphicsSceneDragDropEvent* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->screenPos());
}

void qt_widgets_c_QGraphicsSceneDragDropEvent_setDropAction(QGraphicsSceneDragDropEvent* this_ptr, const Qt::DropAction* action) {
  this_ptr->setDropAction(*action);
}

void qt_widgets_c_QGraphicsSceneDragDropEvent_setMimeData(QGraphicsSceneDragDropEvent* this_ptr, const QMimeData* data) {
  this_ptr->setMimeData(data);
}

void qt_widgets_c_QGraphicsSceneDragDropEvent_setPos(QGraphicsSceneDragDropEvent* this_ptr, const QPointF* pos) {
  this_ptr->setPos(*pos);
}

void qt_widgets_c_QGraphicsSceneDragDropEvent_setProposedAction(QGraphicsSceneDragDropEvent* this_ptr, const Qt::DropAction* action) {
  this_ptr->setProposedAction(*action);
}

void qt_widgets_c_QGraphicsSceneDragDropEvent_setScenePos(QGraphicsSceneDragDropEvent* this_ptr, const QPointF* pos) {
  this_ptr->setScenePos(*pos);
}

void qt_widgets_c_QGraphicsSceneDragDropEvent_setScreenPos(QGraphicsSceneDragDropEvent* this_ptr, const QPoint* pos) {
  this_ptr->setScreenPos(*pos);
}

void qt_widgets_c_QGraphicsSceneDragDropEvent_setSource(QGraphicsSceneDragDropEvent* this_ptr, QWidget* source) {
  this_ptr->setSource(source);
}

QWidget* qt_widgets_c_QGraphicsSceneDragDropEvent_source(const QGraphicsSceneDragDropEvent* this_ptr) {
  return this_ptr->source();
}

