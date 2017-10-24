#include "qt_widgets_c_QGraphicsLineItem.h"

QGraphicsLineItem* qt_widgets_c_QGraphicsLineItem_G_dynamic_cast_QGraphicsLineItem_ptr(QGraphicsItem* ptr) {
  return dynamic_cast<QGraphicsLineItem*>(ptr);
}

QGraphicsItem* qt_widgets_c_QGraphicsLineItem_G_static_cast_QGraphicsItem_ptr(QGraphicsLineItem* ptr) {
  return static_cast<QGraphicsItem*>(ptr);
}

QGraphicsLineItem* qt_widgets_c_QGraphicsLineItem_G_static_cast_QGraphicsLineItem_ptr(QGraphicsItem* ptr) {
  return static_cast<QGraphicsLineItem*>(ptr);
}

void qt_widgets_c_QGraphicsLineItem_boundingRect_to_output(const QGraphicsLineItem* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect());
}

bool qt_widgets_c_QGraphicsLineItem_contains(const QGraphicsLineItem* this_ptr, const QPointF* point) {
  return this_ptr->contains(*point);
}

void qt_widgets_c_QGraphicsLineItem_delete(QGraphicsLineItem* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QGraphicsLineItem_isObscuredBy(const QGraphicsLineItem* this_ptr, const QGraphicsItem* item) {
  return this_ptr->isObscuredBy(item);
}

void qt_widgets_c_QGraphicsLineItem_line_to_output(const QGraphicsLineItem* this_ptr, QLineF* output) {
  new(output) QLineF(this_ptr->line());
}

QGraphicsLineItem* qt_widgets_c_QGraphicsLineItem_new_line(const QLineF* line) {
  return new QGraphicsLineItem(*line);
}

QGraphicsLineItem* qt_widgets_c_QGraphicsLineItem_new_line_parent(const QLineF* line, QGraphicsItem* parent) {
  return new QGraphicsLineItem(*line, parent);
}

QGraphicsLineItem* qt_widgets_c_QGraphicsLineItem_new_no_args() {
  return new QGraphicsLineItem();
}

QGraphicsLineItem* qt_widgets_c_QGraphicsLineItem_new_parent(QGraphicsItem* parent) {
  return new QGraphicsLineItem(parent);
}

QGraphicsLineItem* qt_widgets_c_QGraphicsLineItem_new_x1_y1_x2_y2(double x1, double y1, double x2, double y2) {
  return new QGraphicsLineItem(x1, y1, x2, y2);
}

QGraphicsLineItem* qt_widgets_c_QGraphicsLineItem_new_x1_y1_x2_y2_parent(double x1, double y1, double x2, double y2, QGraphicsItem* parent) {
  return new QGraphicsLineItem(x1, y1, x2, y2, parent);
}

void qt_widgets_c_QGraphicsLineItem_opaqueArea_to_output(const QGraphicsLineItem* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->opaqueArea());
}

void qt_widgets_c_QGraphicsLineItem_paint_painter_option(QGraphicsLineItem* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option) {
  this_ptr->paint(painter, option);
}

void qt_widgets_c_QGraphicsLineItem_paint_painter_option_widget(QGraphicsLineItem* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget) {
  this_ptr->paint(painter, option, widget);
}

void qt_widgets_c_QGraphicsLineItem_pen_to_output(const QGraphicsLineItem* this_ptr, QPen* output) {
  new(output) QPen(this_ptr->pen());
}

void qt_widgets_c_QGraphicsLineItem_setLine_line(QGraphicsLineItem* this_ptr, const QLineF* line) {
  this_ptr->setLine(*line);
}

void qt_widgets_c_QGraphicsLineItem_setLine_x1_y1_x2_y2(QGraphicsLineItem* this_ptr, double x1, double y1, double x2, double y2) {
  this_ptr->setLine(x1, y1, x2, y2);
}

void qt_widgets_c_QGraphicsLineItem_setPen(QGraphicsLineItem* this_ptr, const QPen* pen) {
  this_ptr->setPen(*pen);
}

void qt_widgets_c_QGraphicsLineItem_shape_to_output(const QGraphicsLineItem* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->shape());
}

int qt_widgets_c_QGraphicsLineItem_type(const QGraphicsLineItem* this_ptr) {
  return this_ptr->type();
}

