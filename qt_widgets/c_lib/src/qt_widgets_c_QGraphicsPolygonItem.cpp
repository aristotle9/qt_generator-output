#include "qt_widgets_c_QGraphicsPolygonItem.h"

QGraphicsPolygonItem* qt_widgets_c_QGraphicsPolygonItem_G_dynamic_cast_QGraphicsPolygonItem_ptr_QAbstractGraphicsShapeItem(QAbstractGraphicsShapeItem* ptr) {
  return dynamic_cast<QGraphicsPolygonItem*>(ptr);
}

QGraphicsPolygonItem* qt_widgets_c_QGraphicsPolygonItem_G_dynamic_cast_QGraphicsPolygonItem_ptr_QGraphicsItem(QGraphicsItem* ptr) {
  return dynamic_cast<QGraphicsPolygonItem*>(ptr);
}

QAbstractGraphicsShapeItem* qt_widgets_c_QGraphicsPolygonItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(QGraphicsPolygonItem* ptr) {
  return static_cast<QAbstractGraphicsShapeItem*>(ptr);
}

QGraphicsItem* qt_widgets_c_QGraphicsPolygonItem_G_static_cast_QGraphicsItem_ptr(QGraphicsPolygonItem* ptr) {
  return static_cast<QGraphicsItem*>(ptr);
}

QGraphicsPolygonItem* qt_widgets_c_QGraphicsPolygonItem_G_static_cast_QGraphicsPolygonItem_ptr_QAbstractGraphicsShapeItem(QAbstractGraphicsShapeItem* ptr) {
  return static_cast<QGraphicsPolygonItem*>(ptr);
}

QGraphicsPolygonItem* qt_widgets_c_QGraphicsPolygonItem_G_static_cast_QGraphicsPolygonItem_ptr_QGraphicsItem(QGraphicsItem* ptr) {
  return static_cast<QGraphicsPolygonItem*>(ptr);
}

void qt_widgets_c_QGraphicsPolygonItem_boundingRect_to_output(const QGraphicsPolygonItem* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect());
}

bool qt_widgets_c_QGraphicsPolygonItem_contains(const QGraphicsPolygonItem* this_ptr, const QPointF* point) {
  return this_ptr->contains(*point);
}

void qt_widgets_c_QGraphicsPolygonItem_delete(QGraphicsPolygonItem* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QGraphicsPolygonItem_isObscuredBy(const QGraphicsPolygonItem* this_ptr, const QGraphicsItem* item) {
  return this_ptr->isObscuredBy(item);
}

QGraphicsPolygonItem* qt_widgets_c_QGraphicsPolygonItem_new_no_args() {
  return new QGraphicsPolygonItem();
}

QGraphicsPolygonItem* qt_widgets_c_QGraphicsPolygonItem_new_parent(QGraphicsItem* parent) {
  return new QGraphicsPolygonItem(parent);
}

QGraphicsPolygonItem* qt_widgets_c_QGraphicsPolygonItem_new_polygon(const QPolygonF* polygon) {
  return new QGraphicsPolygonItem(*polygon);
}

QGraphicsPolygonItem* qt_widgets_c_QGraphicsPolygonItem_new_polygon_parent(const QPolygonF* polygon, QGraphicsItem* parent) {
  return new QGraphicsPolygonItem(*polygon, parent);
}

void qt_widgets_c_QGraphicsPolygonItem_opaqueArea_to_output(const QGraphicsPolygonItem* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->opaqueArea());
}

void qt_widgets_c_QGraphicsPolygonItem_paint_painter_option(QGraphicsPolygonItem* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option) {
  this_ptr->paint(painter, option);
}

void qt_widgets_c_QGraphicsPolygonItem_paint_painter_option_widget(QGraphicsPolygonItem* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget) {
  this_ptr->paint(painter, option, widget);
}

void qt_widgets_c_QGraphicsPolygonItem_polygon_to_output(const QGraphicsPolygonItem* this_ptr, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->polygon());
}

void qt_widgets_c_QGraphicsPolygonItem_setFillRule(QGraphicsPolygonItem* this_ptr, const Qt::FillRule* rule) {
  this_ptr->setFillRule(*rule);
}

void qt_widgets_c_QGraphicsPolygonItem_setPolygon(QGraphicsPolygonItem* this_ptr, const QPolygonF* polygon) {
  this_ptr->setPolygon(*polygon);
}

void qt_widgets_c_QGraphicsPolygonItem_shape_to_output(const QGraphicsPolygonItem* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->shape());
}

int qt_widgets_c_QGraphicsPolygonItem_type(const QGraphicsPolygonItem* this_ptr) {
  return this_ptr->type();
}

