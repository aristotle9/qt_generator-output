#include "qt_widgets_c_QGraphicsSceneResizeEvent.h"

QGraphicsSceneResizeEvent* qt_widgets_c_QGraphicsSceneResizeEvent_G_dynamic_cast_QGraphicsSceneResizeEvent_ptr(QGraphicsSceneEvent* ptr) {
  return dynamic_cast<QGraphicsSceneResizeEvent*>(ptr);
}

QEvent* qt_widgets_c_QGraphicsSceneResizeEvent_G_static_cast_QEvent_ptr(QGraphicsSceneResizeEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QGraphicsSceneEvent* qt_widgets_c_QGraphicsSceneResizeEvent_G_static_cast_QGraphicsSceneEvent_ptr(QGraphicsSceneResizeEvent* ptr) {
  return static_cast<QGraphicsSceneEvent*>(ptr);
}

QGraphicsSceneResizeEvent* qt_widgets_c_QGraphicsSceneResizeEvent_G_static_cast_QGraphicsSceneResizeEvent_ptr_QEvent(QEvent* ptr) {
  return static_cast<QGraphicsSceneResizeEvent*>(ptr);
}

QGraphicsSceneResizeEvent* qt_widgets_c_QGraphicsSceneResizeEvent_G_static_cast_QGraphicsSceneResizeEvent_ptr_QGraphicsSceneEvent(QGraphicsSceneEvent* ptr) {
  return static_cast<QGraphicsSceneResizeEvent*>(ptr);
}

void qt_widgets_c_QGraphicsSceneResizeEvent_delete(QGraphicsSceneResizeEvent* this_ptr) {
  delete this_ptr;
}

QGraphicsSceneResizeEvent* qt_widgets_c_QGraphicsSceneResizeEvent_new() {
  return new QGraphicsSceneResizeEvent();
}

void qt_widgets_c_QGraphicsSceneResizeEvent_newSize_to_output(const QGraphicsSceneResizeEvent* this_ptr, QSizeF* output) {
  new(output) QSizeF(this_ptr->newSize());
}

void qt_widgets_c_QGraphicsSceneResizeEvent_oldSize_to_output(const QGraphicsSceneResizeEvent* this_ptr, QSizeF* output) {
  new(output) QSizeF(this_ptr->oldSize());
}

void qt_widgets_c_QGraphicsSceneResizeEvent_setNewSize(QGraphicsSceneResizeEvent* this_ptr, const QSizeF* size) {
  this_ptr->setNewSize(*size);
}

void qt_widgets_c_QGraphicsSceneResizeEvent_setOldSize(QGraphicsSceneResizeEvent* this_ptr, const QSizeF* size) {
  this_ptr->setOldSize(*size);
}

