#include "qt_widgets_c_QGraphicsItemGroup.h"

QGraphicsItemGroup* qt_widgets_c_QGraphicsItemGroup_G_dynamic_cast_QGraphicsItemGroup_ptr(QGraphicsItem* ptr) {
  return dynamic_cast<QGraphicsItemGroup*>(ptr);
}

QGraphicsItemGroup* qt_widgets_c_QGraphicsItemGroup_G_static_cast_QGraphicsItemGroup_ptr(QGraphicsItem* ptr) {
  return static_cast<QGraphicsItemGroup*>(ptr);
}

QGraphicsItem* qt_widgets_c_QGraphicsItemGroup_G_static_cast_QGraphicsItem_ptr(QGraphicsItemGroup* ptr) {
  return static_cast<QGraphicsItem*>(ptr);
}

void qt_widgets_c_QGraphicsItemGroup_addToGroup(QGraphicsItemGroup* this_ptr, QGraphicsItem* item) {
  this_ptr->addToGroup(item);
}

void qt_widgets_c_QGraphicsItemGroup_boundingRect_to_output(const QGraphicsItemGroup* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect());
}

void qt_widgets_c_QGraphicsItemGroup_delete(QGraphicsItemGroup* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QGraphicsItemGroup_isObscuredBy(const QGraphicsItemGroup* this_ptr, const QGraphicsItem* item) {
  return this_ptr->isObscuredBy(item);
}

QGraphicsItemGroup* qt_widgets_c_QGraphicsItemGroup_new_no_args() {
  return new QGraphicsItemGroup();
}

QGraphicsItemGroup* qt_widgets_c_QGraphicsItemGroup_new_parent(QGraphicsItem* parent) {
  return new QGraphicsItemGroup(parent);
}

void qt_widgets_c_QGraphicsItemGroup_opaqueArea_to_output(const QGraphicsItemGroup* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->opaqueArea());
}

void qt_widgets_c_QGraphicsItemGroup_paint_painter_option(QGraphicsItemGroup* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option) {
  this_ptr->paint(painter, option);
}

void qt_widgets_c_QGraphicsItemGroup_paint_painter_option_widget(QGraphicsItemGroup* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget) {
  this_ptr->paint(painter, option, widget);
}

void qt_widgets_c_QGraphicsItemGroup_removeFromGroup(QGraphicsItemGroup* this_ptr, QGraphicsItem* item) {
  this_ptr->removeFromGroup(item);
}

int qt_widgets_c_QGraphicsItemGroup_type(const QGraphicsItemGroup* this_ptr) {
  return this_ptr->type();
}

