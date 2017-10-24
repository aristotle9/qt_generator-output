#include "qt_widgets_c_QStyleOptionGraphicsItem.h"

QStyleOptionGraphicsItem* qt_widgets_c_QStyleOptionGraphicsItem_G_static_cast_QStyleOptionGraphicsItem_ptr(QStyleOption* ptr) {
  return static_cast<QStyleOptionGraphicsItem*>(ptr);
}

QStyleOption* qt_widgets_c_QStyleOptionGraphicsItem_G_static_cast_QStyleOption_ptr(QStyleOptionGraphicsItem* ptr) {
  return static_cast<QStyleOption*>(ptr);
}

void qt_widgets_c_QStyleOptionGraphicsItem_delete(QStyleOptionGraphicsItem* this_ptr) {
  delete this_ptr;
}

const QRectF* qt_widgets_c_QStyleOptionGraphicsItem_exposedRect(const QStyleOptionGraphicsItem* this_ptr) {
  return &this_ptr->exposedRect;
}

QRectF* qt_widgets_c_QStyleOptionGraphicsItem_exposedRect_mut(QStyleOptionGraphicsItem* this_ptr) {
  return &this_ptr->exposedRect;
}

double qt_widgets_c_QStyleOptionGraphicsItem_levelOfDetail(const QStyleOptionGraphicsItem* this_ptr) {
  return this_ptr->levelOfDetail;
}

double qt_widgets_c_QStyleOptionGraphicsItem_levelOfDetailFromTransform(const QTransform* worldTransform) {
  return QStyleOptionGraphicsItem::levelOfDetailFromTransform(*worldTransform);
}

const QMatrix* qt_widgets_c_QStyleOptionGraphicsItem_matrix(const QStyleOptionGraphicsItem* this_ptr) {
  return &this_ptr->matrix;
}

QMatrix* qt_widgets_c_QStyleOptionGraphicsItem_matrix_mut(QStyleOptionGraphicsItem* this_ptr) {
  return &this_ptr->matrix;
}

QStyleOptionGraphicsItem* qt_widgets_c_QStyleOptionGraphicsItem_new_no_args() {
  return new QStyleOptionGraphicsItem();
}

QStyleOptionGraphicsItem* qt_widgets_c_QStyleOptionGraphicsItem_new_other(const QStyleOptionGraphicsItem* other) {
  return new QStyleOptionGraphicsItem(*other);
}

void qt_widgets_c_QStyleOptionGraphicsItem_set_exposedRect(QStyleOptionGraphicsItem* this_ptr, const QRectF* value) {
  this_ptr->exposedRect = *value;
}

void qt_widgets_c_QStyleOptionGraphicsItem_set_levelOfDetail(QStyleOptionGraphicsItem* this_ptr, double value) {
  this_ptr->levelOfDetail = value;
}

void qt_widgets_c_QStyleOptionGraphicsItem_set_matrix(QStyleOptionGraphicsItem* this_ptr, const QMatrix* value) {
  this_ptr->matrix = *value;
}

