#include "qt_widgets_c_QGraphicsSceneEvent.h"

QEvent* qt_widgets_c_QGraphicsSceneEvent_G_static_cast_QEvent_ptr(QGraphicsSceneEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QGraphicsSceneEvent* qt_widgets_c_QGraphicsSceneEvent_G_static_cast_QGraphicsSceneEvent_ptr(QEvent* ptr) {
  return static_cast<QGraphicsSceneEvent*>(ptr);
}

void qt_widgets_c_QGraphicsSceneEvent_delete(QGraphicsSceneEvent* this_ptr) {
  delete this_ptr;
}

QGraphicsSceneEvent* qt_widgets_c_QGraphicsSceneEvent_new(QEvent::Type type) {
  return new QGraphicsSceneEvent(type);
}

void qt_widgets_c_QGraphicsSceneEvent_setWidget(QGraphicsSceneEvent* this_ptr, QWidget* widget) {
  this_ptr->setWidget(widget);
}

QWidget* qt_widgets_c_QGraphicsSceneEvent_widget(const QGraphicsSceneEvent* this_ptr) {
  return this_ptr->widget();
}

