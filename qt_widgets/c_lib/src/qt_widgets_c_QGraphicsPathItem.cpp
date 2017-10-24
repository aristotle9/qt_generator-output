#include "qt_widgets_c_QGraphicsPathItem.h"

QGraphicsPathItem* qt_widgets_c_QGraphicsPathItem_G_dynamic_cast_QGraphicsPathItem_ptr_QAbstractGraphicsShapeItem(QAbstractGraphicsShapeItem* ptr) {
  return dynamic_cast<QGraphicsPathItem*>(ptr);
}

QGraphicsPathItem* qt_widgets_c_QGraphicsPathItem_G_dynamic_cast_QGraphicsPathItem_ptr_QGraphicsItem(QGraphicsItem* ptr) {
  return dynamic_cast<QGraphicsPathItem*>(ptr);
}

QAbstractGraphicsShapeItem* qt_widgets_c_QGraphicsPathItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(QGraphicsPathItem* ptr) {
  return static_cast<QAbstractGraphicsShapeItem*>(ptr);
}

QGraphicsItem* qt_widgets_c_QGraphicsPathItem_G_static_cast_QGraphicsItem_ptr(QGraphicsPathItem* ptr) {
  return static_cast<QGraphicsItem*>(ptr);
}

QGraphicsPathItem* qt_widgets_c_QGraphicsPathItem_G_static_cast_QGraphicsPathItem_ptr_QAbstractGraphicsShapeItem(QAbstractGraphicsShapeItem* ptr) {
  return static_cast<QGraphicsPathItem*>(ptr);
}

QGraphicsPathItem* qt_widgets_c_QGraphicsPathItem_G_static_cast_QGraphicsPathItem_ptr_QGraphicsItem(QGraphicsItem* ptr) {
  return static_cast<QGraphicsPathItem*>(ptr);
}

void qt_widgets_c_QGraphicsPathItem_boundingRect_to_output(const QGraphicsPathItem* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect());
}

bool qt_widgets_c_QGraphicsPathItem_contains(const QGraphicsPathItem* this_ptr, const QPointF* point) {
  return this_ptr->contains(*point);
}

void qt_widgets_c_QGraphicsPathItem_delete(QGraphicsPathItem* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QGraphicsPathItem_isObscuredBy(const QGraphicsPathItem* this_ptr, const QGraphicsItem* item) {
  return this_ptr->isObscuredBy(item);
}

QGraphicsPathItem* qt_widgets_c_QGraphicsPathItem_new_no_args() {
  return new QGraphicsPathItem();
}

QGraphicsPathItem* qt_widgets_c_QGraphicsPathItem_new_parent(QGraphicsItem* parent) {
  return new QGraphicsPathItem(parent);
}

QGraphicsPathItem* qt_widgets_c_QGraphicsPathItem_new_path(const QPainterPath* path) {
  return new QGraphicsPathItem(*path);
}

QGraphicsPathItem* qt_widgets_c_QGraphicsPathItem_new_path_parent(const QPainterPath* path, QGraphicsItem* parent) {
  return new QGraphicsPathItem(*path, parent);
}

void qt_widgets_c_QGraphicsPathItem_opaqueArea_to_output(const QGraphicsPathItem* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->opaqueArea());
}

void qt_widgets_c_QGraphicsPathItem_paint_painter_option(QGraphicsPathItem* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option) {
  this_ptr->paint(painter, option);
}

void qt_widgets_c_QGraphicsPathItem_paint_painter_option_widget(QGraphicsPathItem* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget) {
  this_ptr->paint(painter, option, widget);
}

void qt_widgets_c_QGraphicsPathItem_path_to_output(const QGraphicsPathItem* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->path());
}

void qt_widgets_c_QGraphicsPathItem_setPath(QGraphicsPathItem* this_ptr, const QPainterPath* path) {
  this_ptr->setPath(*path);
}

void qt_widgets_c_QGraphicsPathItem_shape_to_output(const QGraphicsPathItem* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->shape());
}

int qt_widgets_c_QGraphicsPathItem_type(const QGraphicsPathItem* this_ptr) {
  return this_ptr->type();
}

