#include "qt_widgets_c_QGraphicsPixmapItem.h"

QGraphicsPixmapItem* qt_widgets_c_QGraphicsPixmapItem_G_dynamic_cast_QGraphicsPixmapItem_ptr(QGraphicsItem* ptr) {
  return dynamic_cast<QGraphicsPixmapItem*>(ptr);
}

QGraphicsItem* qt_widgets_c_QGraphicsPixmapItem_G_static_cast_QGraphicsItem_ptr(QGraphicsPixmapItem* ptr) {
  return static_cast<QGraphicsItem*>(ptr);
}

QGraphicsPixmapItem* qt_widgets_c_QGraphicsPixmapItem_G_static_cast_QGraphicsPixmapItem_ptr(QGraphicsItem* ptr) {
  return static_cast<QGraphicsPixmapItem*>(ptr);
}

void qt_widgets_c_QGraphicsPixmapItem_boundingRect_to_output(const QGraphicsPixmapItem* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect());
}

bool qt_widgets_c_QGraphicsPixmapItem_contains(const QGraphicsPixmapItem* this_ptr, const QPointF* point) {
  return this_ptr->contains(*point);
}

void qt_widgets_c_QGraphicsPixmapItem_delete(QGraphicsPixmapItem* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QGraphicsPixmapItem_isObscuredBy(const QGraphicsPixmapItem* this_ptr, const QGraphicsItem* item) {
  return this_ptr->isObscuredBy(item);
}

QGraphicsPixmapItem* qt_widgets_c_QGraphicsPixmapItem_new_no_args() {
  return new QGraphicsPixmapItem();
}

QGraphicsPixmapItem* qt_widgets_c_QGraphicsPixmapItem_new_parent(QGraphicsItem* parent) {
  return new QGraphicsPixmapItem(parent);
}

QGraphicsPixmapItem* qt_widgets_c_QGraphicsPixmapItem_new_pixmap(const QPixmap* pixmap) {
  return new QGraphicsPixmapItem(*pixmap);
}

QGraphicsPixmapItem* qt_widgets_c_QGraphicsPixmapItem_new_pixmap_parent(const QPixmap* pixmap, QGraphicsItem* parent) {
  return new QGraphicsPixmapItem(*pixmap, parent);
}

void qt_widgets_c_QGraphicsPixmapItem_offset_to_output(const QGraphicsPixmapItem* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->offset());
}

void qt_widgets_c_QGraphicsPixmapItem_opaqueArea_to_output(const QGraphicsPixmapItem* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->opaqueArea());
}

void qt_widgets_c_QGraphicsPixmapItem_paint(QGraphicsPixmapItem* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget) {
  this_ptr->paint(painter, option, widget);
}

QPixmap* qt_widgets_c_QGraphicsPixmapItem_pixmap_as_ptr(const QGraphicsPixmapItem* this_ptr) {
  return new QPixmap(this_ptr->pixmap());
}

void qt_widgets_c_QGraphicsPixmapItem_setOffset_offset(QGraphicsPixmapItem* this_ptr, const QPointF* offset) {
  this_ptr->setOffset(*offset);
}

void qt_widgets_c_QGraphicsPixmapItem_setOffset_x_y(QGraphicsPixmapItem* this_ptr, double x, double y) {
  this_ptr->setOffset(x, y);
}

void qt_widgets_c_QGraphicsPixmapItem_setPixmap(QGraphicsPixmapItem* this_ptr, const QPixmap* pixmap) {
  this_ptr->setPixmap(*pixmap);
}

void qt_widgets_c_QGraphicsPixmapItem_setShapeMode(QGraphicsPixmapItem* this_ptr, QGraphicsPixmapItem::ShapeMode mode) {
  this_ptr->setShapeMode(mode);
}

void qt_widgets_c_QGraphicsPixmapItem_setTransformationMode(QGraphicsPixmapItem* this_ptr, const Qt::TransformationMode* mode) {
  this_ptr->setTransformationMode(*mode);
}

QGraphicsPixmapItem::ShapeMode qt_widgets_c_QGraphicsPixmapItem_shapeMode(const QGraphicsPixmapItem* this_ptr) {
  return this_ptr->shapeMode();
}

void qt_widgets_c_QGraphicsPixmapItem_shape_to_output(const QGraphicsPixmapItem* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->shape());
}

int qt_widgets_c_QGraphicsPixmapItem_type(const QGraphicsPixmapItem* this_ptr) {
  return this_ptr->type();
}

