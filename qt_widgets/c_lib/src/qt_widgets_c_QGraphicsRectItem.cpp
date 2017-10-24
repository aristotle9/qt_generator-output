#include "qt_widgets_c_QGraphicsRectItem.h"

QGraphicsRectItem* qt_widgets_c_QGraphicsRectItem_G_dynamic_cast_QGraphicsRectItem_ptr_QAbstractGraphicsShapeItem(QAbstractGraphicsShapeItem* ptr) {
  return dynamic_cast<QGraphicsRectItem*>(ptr);
}

QGraphicsRectItem* qt_widgets_c_QGraphicsRectItem_G_dynamic_cast_QGraphicsRectItem_ptr_QGraphicsItem(QGraphicsItem* ptr) {
  return dynamic_cast<QGraphicsRectItem*>(ptr);
}

QAbstractGraphicsShapeItem* qt_widgets_c_QGraphicsRectItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(QGraphicsRectItem* ptr) {
  return static_cast<QAbstractGraphicsShapeItem*>(ptr);
}

QGraphicsItem* qt_widgets_c_QGraphicsRectItem_G_static_cast_QGraphicsItem_ptr(QGraphicsRectItem* ptr) {
  return static_cast<QGraphicsItem*>(ptr);
}

QGraphicsRectItem* qt_widgets_c_QGraphicsRectItem_G_static_cast_QGraphicsRectItem_ptr_QAbstractGraphicsShapeItem(QAbstractGraphicsShapeItem* ptr) {
  return static_cast<QGraphicsRectItem*>(ptr);
}

QGraphicsRectItem* qt_widgets_c_QGraphicsRectItem_G_static_cast_QGraphicsRectItem_ptr_QGraphicsItem(QGraphicsItem* ptr) {
  return static_cast<QGraphicsRectItem*>(ptr);
}

void qt_widgets_c_QGraphicsRectItem_boundingRect_to_output(const QGraphicsRectItem* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect());
}

bool qt_widgets_c_QGraphicsRectItem_contains(const QGraphicsRectItem* this_ptr, const QPointF* point) {
  return this_ptr->contains(*point);
}

void qt_widgets_c_QGraphicsRectItem_delete(QGraphicsRectItem* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QGraphicsRectItem_isObscuredBy(const QGraphicsRectItem* this_ptr, const QGraphicsItem* item) {
  return this_ptr->isObscuredBy(item);
}

QGraphicsRectItem* qt_widgets_c_QGraphicsRectItem_new_no_args() {
  return new QGraphicsRectItem();
}

QGraphicsRectItem* qt_widgets_c_QGraphicsRectItem_new_parent(QGraphicsItem* parent) {
  return new QGraphicsRectItem(parent);
}

QGraphicsRectItem* qt_widgets_c_QGraphicsRectItem_new_rect(const QRectF* rect) {
  return new QGraphicsRectItem(*rect);
}

QGraphicsRectItem* qt_widgets_c_QGraphicsRectItem_new_rect_parent(const QRectF* rect, QGraphicsItem* parent) {
  return new QGraphicsRectItem(*rect, parent);
}

QGraphicsRectItem* qt_widgets_c_QGraphicsRectItem_new_x_y_w_h(double x, double y, double w, double h) {
  return new QGraphicsRectItem(x, y, w, h);
}

QGraphicsRectItem* qt_widgets_c_QGraphicsRectItem_new_x_y_w_h_parent(double x, double y, double w, double h, QGraphicsItem* parent) {
  return new QGraphicsRectItem(x, y, w, h, parent);
}

void qt_widgets_c_QGraphicsRectItem_opaqueArea_to_output(const QGraphicsRectItem* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->opaqueArea());
}

void qt_widgets_c_QGraphicsRectItem_paint_painter_option(QGraphicsRectItem* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option) {
  this_ptr->paint(painter, option);
}

void qt_widgets_c_QGraphicsRectItem_paint_painter_option_widget(QGraphicsRectItem* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget) {
  this_ptr->paint(painter, option, widget);
}

void qt_widgets_c_QGraphicsRectItem_rect_to_output(const QGraphicsRectItem* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->rect());
}

void qt_widgets_c_QGraphicsRectItem_setRect_rect(QGraphicsRectItem* this_ptr, const QRectF* rect) {
  this_ptr->setRect(*rect);
}

void qt_widgets_c_QGraphicsRectItem_setRect_x_y_w_h(QGraphicsRectItem* this_ptr, double x, double y, double w, double h) {
  this_ptr->setRect(x, y, w, h);
}

void qt_widgets_c_QGraphicsRectItem_shape_to_output(const QGraphicsRectItem* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->shape());
}

int qt_widgets_c_QGraphicsRectItem_type(const QGraphicsRectItem* this_ptr) {
  return this_ptr->type();
}

