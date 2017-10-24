#include "qt_widgets_c_QAbstractGraphicsShapeItem.h"

QAbstractGraphicsShapeItem* qt_widgets_c_QAbstractGraphicsShapeItem_G_dynamic_cast_QAbstractGraphicsShapeItem_ptr(QGraphicsItem* ptr) {
  return dynamic_cast<QAbstractGraphicsShapeItem*>(ptr);
}

QAbstractGraphicsShapeItem* qt_widgets_c_QAbstractGraphicsShapeItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(QGraphicsItem* ptr) {
  return static_cast<QAbstractGraphicsShapeItem*>(ptr);
}

QGraphicsItem* qt_widgets_c_QAbstractGraphicsShapeItem_G_static_cast_QGraphicsItem_ptr(QAbstractGraphicsShapeItem* ptr) {
  return static_cast<QGraphicsItem*>(ptr);
}

void qt_widgets_c_QAbstractGraphicsShapeItem_brush_to_output(const QAbstractGraphicsShapeItem* this_ptr, QBrush* output) {
  new(output) QBrush(this_ptr->brush());
}

void qt_widgets_c_QAbstractGraphicsShapeItem_delete(QAbstractGraphicsShapeItem* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QAbstractGraphicsShapeItem_isObscuredBy(const QAbstractGraphicsShapeItem* this_ptr, const QGraphicsItem* item) {
  return this_ptr->isObscuredBy(item);
}

void qt_widgets_c_QAbstractGraphicsShapeItem_opaqueArea_to_output(const QAbstractGraphicsShapeItem* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->opaqueArea());
}

void qt_widgets_c_QAbstractGraphicsShapeItem_pen_to_output(const QAbstractGraphicsShapeItem* this_ptr, QPen* output) {
  new(output) QPen(this_ptr->pen());
}

void qt_widgets_c_QAbstractGraphicsShapeItem_setBrush(QAbstractGraphicsShapeItem* this_ptr, const QBrush* brush) {
  this_ptr->setBrush(*brush);
}

void qt_widgets_c_QAbstractGraphicsShapeItem_setPen(QAbstractGraphicsShapeItem* this_ptr, const QPen* pen) {
  this_ptr->setPen(*pen);
}

