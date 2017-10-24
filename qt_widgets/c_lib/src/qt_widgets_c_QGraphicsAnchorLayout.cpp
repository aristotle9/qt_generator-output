#include "qt_widgets_c_QGraphicsAnchorLayout.h"

QGraphicsAnchorLayout* qt_widgets_c_QGraphicsAnchorLayout_G_dynamic_cast_QGraphicsAnchorLayout_ptr_QGraphicsLayout(QGraphicsLayout* ptr) {
  return dynamic_cast<QGraphicsAnchorLayout*>(ptr);
}

QGraphicsAnchorLayout* qt_widgets_c_QGraphicsAnchorLayout_G_dynamic_cast_QGraphicsAnchorLayout_ptr_QGraphicsLayoutItem(QGraphicsLayoutItem* ptr) {
  return dynamic_cast<QGraphicsAnchorLayout*>(ptr);
}

QGraphicsAnchorLayout* qt_widgets_c_QGraphicsAnchorLayout_G_static_cast_QGraphicsAnchorLayout_ptr_QGraphicsLayout(QGraphicsLayout* ptr) {
  return static_cast<QGraphicsAnchorLayout*>(ptr);
}

QGraphicsAnchorLayout* qt_widgets_c_QGraphicsAnchorLayout_G_static_cast_QGraphicsAnchorLayout_ptr_QGraphicsLayoutItem(QGraphicsLayoutItem* ptr) {
  return static_cast<QGraphicsAnchorLayout*>(ptr);
}

QGraphicsLayoutItem* qt_widgets_c_QGraphicsAnchorLayout_G_static_cast_QGraphicsLayoutItem_ptr(QGraphicsAnchorLayout* ptr) {
  return static_cast<QGraphicsLayoutItem*>(ptr);
}

QGraphicsLayout* qt_widgets_c_QGraphicsAnchorLayout_G_static_cast_QGraphicsLayout_ptr(QGraphicsAnchorLayout* ptr) {
  return static_cast<QGraphicsLayout*>(ptr);
}

QGraphicsAnchor* qt_widgets_c_QGraphicsAnchorLayout_addAnchor(QGraphicsAnchorLayout* this_ptr, QGraphicsLayoutItem* firstItem, const Qt::AnchorPoint* firstEdge, QGraphicsLayoutItem* secondItem, const Qt::AnchorPoint* secondEdge) {
  return this_ptr->addAnchor(firstItem, *firstEdge, secondItem, *secondEdge);
}

void qt_widgets_c_QGraphicsAnchorLayout_addCornerAnchors(QGraphicsAnchorLayout* this_ptr, QGraphicsLayoutItem* firstItem, const Qt::Corner* firstCorner, QGraphicsLayoutItem* secondItem, const Qt::Corner* secondCorner) {
  this_ptr->addCornerAnchors(firstItem, *firstCorner, secondItem, *secondCorner);
}

QGraphicsAnchor* qt_widgets_c_QGraphicsAnchorLayout_anchor(QGraphicsAnchorLayout* this_ptr, QGraphicsLayoutItem* firstItem, const Qt::AnchorPoint* firstEdge, QGraphicsLayoutItem* secondItem, const Qt::AnchorPoint* secondEdge) {
  return this_ptr->anchor(firstItem, *firstEdge, secondItem, *secondEdge);
}

int qt_widgets_c_QGraphicsAnchorLayout_count(const QGraphicsAnchorLayout* this_ptr) {
  return this_ptr->count();
}

void qt_widgets_c_QGraphicsAnchorLayout_delete(QGraphicsAnchorLayout* this_ptr) {
  delete this_ptr;
}

double qt_widgets_c_QGraphicsAnchorLayout_horizontalSpacing(const QGraphicsAnchorLayout* this_ptr) {
  return this_ptr->horizontalSpacing();
}

void qt_widgets_c_QGraphicsAnchorLayout_invalidate(QGraphicsAnchorLayout* this_ptr) {
  this_ptr->invalidate();
}

QGraphicsLayoutItem* qt_widgets_c_QGraphicsAnchorLayout_itemAt(const QGraphicsAnchorLayout* this_ptr, int index) {
  return this_ptr->itemAt(index);
}

QGraphicsAnchorLayout* qt_widgets_c_QGraphicsAnchorLayout_new_no_args() {
  return new QGraphicsAnchorLayout();
}

QGraphicsAnchorLayout* qt_widgets_c_QGraphicsAnchorLayout_new_parent(QGraphicsLayoutItem* parent) {
  return new QGraphicsAnchorLayout(parent);
}

void qt_widgets_c_QGraphicsAnchorLayout_removeAt(QGraphicsAnchorLayout* this_ptr, int index) {
  this_ptr->removeAt(index);
}

void qt_widgets_c_QGraphicsAnchorLayout_setGeometry(QGraphicsAnchorLayout* this_ptr, const QRectF* rect) {
  this_ptr->setGeometry(*rect);
}

void qt_widgets_c_QGraphicsAnchorLayout_setHorizontalSpacing(QGraphicsAnchorLayout* this_ptr, double spacing) {
  this_ptr->setHorizontalSpacing(spacing);
}

void qt_widgets_c_QGraphicsAnchorLayout_setSpacing(QGraphicsAnchorLayout* this_ptr, double spacing) {
  this_ptr->setSpacing(spacing);
}

void qt_widgets_c_QGraphicsAnchorLayout_setVerticalSpacing(QGraphicsAnchorLayout* this_ptr, double spacing) {
  this_ptr->setVerticalSpacing(spacing);
}

double qt_widgets_c_QGraphicsAnchorLayout_verticalSpacing(const QGraphicsAnchorLayout* this_ptr) {
  return this_ptr->verticalSpacing();
}

