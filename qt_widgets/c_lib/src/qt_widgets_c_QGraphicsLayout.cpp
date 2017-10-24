#include "qt_widgets_c_QGraphicsLayout.h"

QGraphicsLayout* qt_widgets_c_QGraphicsLayout_G_dynamic_cast_QGraphicsLayout_ptr(QGraphicsLayoutItem* ptr) {
  return dynamic_cast<QGraphicsLayout*>(ptr);
}

QGraphicsLayoutItem* qt_widgets_c_QGraphicsLayout_G_static_cast_QGraphicsLayoutItem_ptr(QGraphicsLayout* ptr) {
  return static_cast<QGraphicsLayoutItem*>(ptr);
}

QGraphicsLayout* qt_widgets_c_QGraphicsLayout_G_static_cast_QGraphicsLayout_ptr(QGraphicsLayoutItem* ptr) {
  return static_cast<QGraphicsLayout*>(ptr);
}

void qt_widgets_c_QGraphicsLayout_activate(QGraphicsLayout* this_ptr) {
  this_ptr->activate();
}

int qt_widgets_c_QGraphicsLayout_count(const QGraphicsLayout* this_ptr) {
  return this_ptr->count();
}

void qt_widgets_c_QGraphicsLayout_delete(QGraphicsLayout* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QGraphicsLayout_getContentsMargins(const QGraphicsLayout* this_ptr, double* left, double* top, double* right, double* bottom) {
  this_ptr->getContentsMargins(left, top, right, bottom);
}

bool qt_widgets_c_QGraphicsLayout_instantInvalidatePropagation() {
  return QGraphicsLayout::instantInvalidatePropagation();
}

void qt_widgets_c_QGraphicsLayout_invalidate(QGraphicsLayout* this_ptr) {
  this_ptr->invalidate();
}

bool qt_widgets_c_QGraphicsLayout_isActivated(const QGraphicsLayout* this_ptr) {
  return this_ptr->isActivated();
}

QGraphicsLayoutItem* qt_widgets_c_QGraphicsLayout_itemAt(const QGraphicsLayout* this_ptr, int i) {
  return this_ptr->itemAt(i);
}

void qt_widgets_c_QGraphicsLayout_removeAt(QGraphicsLayout* this_ptr, int index) {
  this_ptr->removeAt(index);
}

void qt_widgets_c_QGraphicsLayout_setContentsMargins(QGraphicsLayout* this_ptr, double left, double top, double right, double bottom) {
  this_ptr->setContentsMargins(left, top, right, bottom);
}

void qt_widgets_c_QGraphicsLayout_setInstantInvalidatePropagation(bool enable) {
  QGraphicsLayout::setInstantInvalidatePropagation(enable);
}

void qt_widgets_c_QGraphicsLayout_updateGeometry(QGraphicsLayout* this_ptr) {
  this_ptr->updateGeometry();
}

void qt_widgets_c_QGraphicsLayout_widgetEvent(QGraphicsLayout* this_ptr, QEvent* e) {
  this_ptr->widgetEvent(e);
}

