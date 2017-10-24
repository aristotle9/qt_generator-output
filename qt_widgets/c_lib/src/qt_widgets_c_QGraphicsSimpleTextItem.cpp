#include "qt_widgets_c_QGraphicsSimpleTextItem.h"

QGraphicsSimpleTextItem* qt_widgets_c_QGraphicsSimpleTextItem_G_dynamic_cast_QGraphicsSimpleTextItem_ptr_QAbstractGraphicsShapeItem(QAbstractGraphicsShapeItem* ptr) {
  return dynamic_cast<QGraphicsSimpleTextItem*>(ptr);
}

QGraphicsSimpleTextItem* qt_widgets_c_QGraphicsSimpleTextItem_G_dynamic_cast_QGraphicsSimpleTextItem_ptr_QGraphicsItem(QGraphicsItem* ptr) {
  return dynamic_cast<QGraphicsSimpleTextItem*>(ptr);
}

QAbstractGraphicsShapeItem* qt_widgets_c_QGraphicsSimpleTextItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(QGraphicsSimpleTextItem* ptr) {
  return static_cast<QAbstractGraphicsShapeItem*>(ptr);
}

QGraphicsItem* qt_widgets_c_QGraphicsSimpleTextItem_G_static_cast_QGraphicsItem_ptr(QGraphicsSimpleTextItem* ptr) {
  return static_cast<QGraphicsItem*>(ptr);
}

QGraphicsSimpleTextItem* qt_widgets_c_QGraphicsSimpleTextItem_G_static_cast_QGraphicsSimpleTextItem_ptr_QAbstractGraphicsShapeItem(QAbstractGraphicsShapeItem* ptr) {
  return static_cast<QGraphicsSimpleTextItem*>(ptr);
}

QGraphicsSimpleTextItem* qt_widgets_c_QGraphicsSimpleTextItem_G_static_cast_QGraphicsSimpleTextItem_ptr_QGraphicsItem(QGraphicsItem* ptr) {
  return static_cast<QGraphicsSimpleTextItem*>(ptr);
}

void qt_widgets_c_QGraphicsSimpleTextItem_boundingRect_to_output(const QGraphicsSimpleTextItem* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect());
}

bool qt_widgets_c_QGraphicsSimpleTextItem_contains(const QGraphicsSimpleTextItem* this_ptr, const QPointF* point) {
  return this_ptr->contains(*point);
}

void qt_widgets_c_QGraphicsSimpleTextItem_delete(QGraphicsSimpleTextItem* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QGraphicsSimpleTextItem_font_to_output(const QGraphicsSimpleTextItem* this_ptr, QFont* output) {
  new(output) QFont(this_ptr->font());
}

bool qt_widgets_c_QGraphicsSimpleTextItem_isObscuredBy(const QGraphicsSimpleTextItem* this_ptr, const QGraphicsItem* item) {
  return this_ptr->isObscuredBy(item);
}

QGraphicsSimpleTextItem* qt_widgets_c_QGraphicsSimpleTextItem_new_no_args() {
  return new QGraphicsSimpleTextItem();
}

QGraphicsSimpleTextItem* qt_widgets_c_QGraphicsSimpleTextItem_new_parent(QGraphicsItem* parent) {
  return new QGraphicsSimpleTextItem(parent);
}

QGraphicsSimpleTextItem* qt_widgets_c_QGraphicsSimpleTextItem_new_text(const QString* text) {
  return new QGraphicsSimpleTextItem(*text);
}

QGraphicsSimpleTextItem* qt_widgets_c_QGraphicsSimpleTextItem_new_text_parent(const QString* text, QGraphicsItem* parent) {
  return new QGraphicsSimpleTextItem(*text, parent);
}

void qt_widgets_c_QGraphicsSimpleTextItem_opaqueArea_to_output(const QGraphicsSimpleTextItem* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->opaqueArea());
}

void qt_widgets_c_QGraphicsSimpleTextItem_paint(QGraphicsSimpleTextItem* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget) {
  this_ptr->paint(painter, option, widget);
}

void qt_widgets_c_QGraphicsSimpleTextItem_setFont(QGraphicsSimpleTextItem* this_ptr, const QFont* font) {
  this_ptr->setFont(*font);
}

void qt_widgets_c_QGraphicsSimpleTextItem_setText(QGraphicsSimpleTextItem* this_ptr, const QString* text) {
  this_ptr->setText(*text);
}

void qt_widgets_c_QGraphicsSimpleTextItem_shape_to_output(const QGraphicsSimpleTextItem* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->shape());
}

void qt_widgets_c_QGraphicsSimpleTextItem_text_to_output(const QGraphicsSimpleTextItem* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

int qt_widgets_c_QGraphicsSimpleTextItem_type(const QGraphicsSimpleTextItem* this_ptr) {
  return this_ptr->type();
}

