#include "qt_widgets_c_QGraphicsEllipseItem.h"

QGraphicsEllipseItem* qt_widgets_c_QGraphicsEllipseItem_G_dynamic_cast_QGraphicsEllipseItem_ptr_QAbstractGraphicsShapeItem(QAbstractGraphicsShapeItem* ptr) {
  return dynamic_cast<QGraphicsEllipseItem*>(ptr);
}

QGraphicsEllipseItem* qt_widgets_c_QGraphicsEllipseItem_G_dynamic_cast_QGraphicsEllipseItem_ptr_QGraphicsItem(QGraphicsItem* ptr) {
  return dynamic_cast<QGraphicsEllipseItem*>(ptr);
}

QAbstractGraphicsShapeItem* qt_widgets_c_QGraphicsEllipseItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(QGraphicsEllipseItem* ptr) {
  return static_cast<QAbstractGraphicsShapeItem*>(ptr);
}

QGraphicsEllipseItem* qt_widgets_c_QGraphicsEllipseItem_G_static_cast_QGraphicsEllipseItem_ptr_QAbstractGraphicsShapeItem(QAbstractGraphicsShapeItem* ptr) {
  return static_cast<QGraphicsEllipseItem*>(ptr);
}

QGraphicsEllipseItem* qt_widgets_c_QGraphicsEllipseItem_G_static_cast_QGraphicsEllipseItem_ptr_QGraphicsItem(QGraphicsItem* ptr) {
  return static_cast<QGraphicsEllipseItem*>(ptr);
}

QGraphicsItem* qt_widgets_c_QGraphicsEllipseItem_G_static_cast_QGraphicsItem_ptr(QGraphicsEllipseItem* ptr) {
  return static_cast<QGraphicsItem*>(ptr);
}

void qt_widgets_c_QGraphicsEllipseItem_boundingRect_to_output(const QGraphicsEllipseItem* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect());
}

bool qt_widgets_c_QGraphicsEllipseItem_contains(const QGraphicsEllipseItem* this_ptr, const QPointF* point) {
  return this_ptr->contains(*point);
}

void qt_widgets_c_QGraphicsEllipseItem_delete(QGraphicsEllipseItem* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QGraphicsEllipseItem_isObscuredBy(const QGraphicsEllipseItem* this_ptr, const QGraphicsItem* item) {
  return this_ptr->isObscuredBy(item);
}

QGraphicsEllipseItem* qt_widgets_c_QGraphicsEllipseItem_new_no_args() {
  return new QGraphicsEllipseItem();
}

QGraphicsEllipseItem* qt_widgets_c_QGraphicsEllipseItem_new_parent(QGraphicsItem* parent) {
  return new QGraphicsEllipseItem(parent);
}

QGraphicsEllipseItem* qt_widgets_c_QGraphicsEllipseItem_new_rect(const QRectF* rect) {
  return new QGraphicsEllipseItem(*rect);
}

QGraphicsEllipseItem* qt_widgets_c_QGraphicsEllipseItem_new_rect_parent(const QRectF* rect, QGraphicsItem* parent) {
  return new QGraphicsEllipseItem(*rect, parent);
}

QGraphicsEllipseItem* qt_widgets_c_QGraphicsEllipseItem_new_x_y_w_h(double x, double y, double w, double h) {
  return new QGraphicsEllipseItem(x, y, w, h);
}

QGraphicsEllipseItem* qt_widgets_c_QGraphicsEllipseItem_new_x_y_w_h_parent(double x, double y, double w, double h, QGraphicsItem* parent) {
  return new QGraphicsEllipseItem(x, y, w, h, parent);
}

void qt_widgets_c_QGraphicsEllipseItem_opaqueArea_to_output(const QGraphicsEllipseItem* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->opaqueArea());
}

void qt_widgets_c_QGraphicsEllipseItem_paint_painter_option(QGraphicsEllipseItem* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option) {
  this_ptr->paint(painter, option);
}

void qt_widgets_c_QGraphicsEllipseItem_paint_painter_option_widget(QGraphicsEllipseItem* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget) {
  this_ptr->paint(painter, option, widget);
}

void qt_widgets_c_QGraphicsEllipseItem_rect_to_output(const QGraphicsEllipseItem* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->rect());
}

void qt_widgets_c_QGraphicsEllipseItem_setRect_rect(QGraphicsEllipseItem* this_ptr, const QRectF* rect) {
  this_ptr->setRect(*rect);
}

void qt_widgets_c_QGraphicsEllipseItem_setRect_x_y_w_h(QGraphicsEllipseItem* this_ptr, double x, double y, double w, double h) {
  this_ptr->setRect(x, y, w, h);
}

void qt_widgets_c_QGraphicsEllipseItem_setSpanAngle(QGraphicsEllipseItem* this_ptr, int angle) {
  this_ptr->setSpanAngle(angle);
}

void qt_widgets_c_QGraphicsEllipseItem_setStartAngle(QGraphicsEllipseItem* this_ptr, int angle) {
  this_ptr->setStartAngle(angle);
}

void qt_widgets_c_QGraphicsEllipseItem_shape_to_output(const QGraphicsEllipseItem* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->shape());
}

int qt_widgets_c_QGraphicsEllipseItem_spanAngle(const QGraphicsEllipseItem* this_ptr) {
  return this_ptr->spanAngle();
}

int qt_widgets_c_QGraphicsEllipseItem_startAngle(const QGraphicsEllipseItem* this_ptr) {
  return this_ptr->startAngle();
}

int qt_widgets_c_QGraphicsEllipseItem_type(const QGraphicsEllipseItem* this_ptr) {
  return this_ptr->type();
}

