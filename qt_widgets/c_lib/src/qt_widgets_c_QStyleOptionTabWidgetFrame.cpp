#include "qt_widgets_c_QStyleOptionTabWidgetFrame.h"

QStyleOptionTabWidgetFrame* qt_widgets_c_QStyleOptionTabWidgetFrame_G_static_cast_QStyleOptionTabWidgetFrame_ptr(QStyleOption* ptr) {
  return static_cast<QStyleOptionTabWidgetFrame*>(ptr);
}

QStyleOption* qt_widgets_c_QStyleOptionTabWidgetFrame_G_static_cast_QStyleOption_ptr(QStyleOptionTabWidgetFrame* ptr) {
  return static_cast<QStyleOption*>(ptr);
}

void qt_widgets_c_QStyleOptionTabWidgetFrame_delete(QStyleOptionTabWidgetFrame* this_ptr) {
  delete this_ptr;
}

const QSize* qt_widgets_c_QStyleOptionTabWidgetFrame_leftCornerWidgetSize(const QStyleOptionTabWidgetFrame* this_ptr) {
  return &this_ptr->leftCornerWidgetSize;
}

QSize* qt_widgets_c_QStyleOptionTabWidgetFrame_leftCornerWidgetSize_mut(QStyleOptionTabWidgetFrame* this_ptr) {
  return &this_ptr->leftCornerWidgetSize;
}

int qt_widgets_c_QStyleOptionTabWidgetFrame_lineWidth(const QStyleOptionTabWidgetFrame* this_ptr) {
  return this_ptr->lineWidth;
}

int qt_widgets_c_QStyleOptionTabWidgetFrame_midLineWidth(const QStyleOptionTabWidgetFrame* this_ptr) {
  return this_ptr->midLineWidth;
}

QStyleOptionTabWidgetFrame* qt_widgets_c_QStyleOptionTabWidgetFrame_new_no_args() {
  return new QStyleOptionTabWidgetFrame();
}

QStyleOptionTabWidgetFrame* qt_widgets_c_QStyleOptionTabWidgetFrame_new_other(const QStyleOptionTabWidgetFrame* other) {
  return new QStyleOptionTabWidgetFrame(*other);
}

const QSize* qt_widgets_c_QStyleOptionTabWidgetFrame_rightCornerWidgetSize(const QStyleOptionTabWidgetFrame* this_ptr) {
  return &this_ptr->rightCornerWidgetSize;
}

QSize* qt_widgets_c_QStyleOptionTabWidgetFrame_rightCornerWidgetSize_mut(QStyleOptionTabWidgetFrame* this_ptr) {
  return &this_ptr->rightCornerWidgetSize;
}

const QRect* qt_widgets_c_QStyleOptionTabWidgetFrame_selectedTabRect(const QStyleOptionTabWidgetFrame* this_ptr) {
  return &this_ptr->selectedTabRect;
}

QRect* qt_widgets_c_QStyleOptionTabWidgetFrame_selectedTabRect_mut(QStyleOptionTabWidgetFrame* this_ptr) {
  return &this_ptr->selectedTabRect;
}

void qt_widgets_c_QStyleOptionTabWidgetFrame_set_leftCornerWidgetSize(QStyleOptionTabWidgetFrame* this_ptr, const QSize* value) {
  this_ptr->leftCornerWidgetSize = *value;
}

void qt_widgets_c_QStyleOptionTabWidgetFrame_set_lineWidth(QStyleOptionTabWidgetFrame* this_ptr, int value) {
  this_ptr->lineWidth = value;
}

void qt_widgets_c_QStyleOptionTabWidgetFrame_set_midLineWidth(QStyleOptionTabWidgetFrame* this_ptr, int value) {
  this_ptr->midLineWidth = value;
}

void qt_widgets_c_QStyleOptionTabWidgetFrame_set_rightCornerWidgetSize(QStyleOptionTabWidgetFrame* this_ptr, const QSize* value) {
  this_ptr->rightCornerWidgetSize = *value;
}

void qt_widgets_c_QStyleOptionTabWidgetFrame_set_selectedTabRect(QStyleOptionTabWidgetFrame* this_ptr, const QRect* value) {
  this_ptr->selectedTabRect = *value;
}

void qt_widgets_c_QStyleOptionTabWidgetFrame_set_shape(QStyleOptionTabWidgetFrame* this_ptr, const QTabBar::Shape* value) {
  this_ptr->shape = *value;
}

void qt_widgets_c_QStyleOptionTabWidgetFrame_set_tabBarRect(QStyleOptionTabWidgetFrame* this_ptr, const QRect* value) {
  this_ptr->tabBarRect = *value;
}

void qt_widgets_c_QStyleOptionTabWidgetFrame_set_tabBarSize(QStyleOptionTabWidgetFrame* this_ptr, const QSize* value) {
  this_ptr->tabBarSize = *value;
}

const QTabBar::Shape* qt_widgets_c_QStyleOptionTabWidgetFrame_shape(const QStyleOptionTabWidgetFrame* this_ptr) {
  return &this_ptr->shape;
}

QTabBar::Shape* qt_widgets_c_QStyleOptionTabWidgetFrame_shape_mut(QStyleOptionTabWidgetFrame* this_ptr) {
  return &this_ptr->shape;
}

const QRect* qt_widgets_c_QStyleOptionTabWidgetFrame_tabBarRect(const QStyleOptionTabWidgetFrame* this_ptr) {
  return &this_ptr->tabBarRect;
}

QRect* qt_widgets_c_QStyleOptionTabWidgetFrame_tabBarRect_mut(QStyleOptionTabWidgetFrame* this_ptr) {
  return &this_ptr->tabBarRect;
}

const QSize* qt_widgets_c_QStyleOptionTabWidgetFrame_tabBarSize(const QStyleOptionTabWidgetFrame* this_ptr) {
  return &this_ptr->tabBarSize;
}

QSize* qt_widgets_c_QStyleOptionTabWidgetFrame_tabBarSize_mut(QStyleOptionTabWidgetFrame* this_ptr) {
  return &this_ptr->tabBarSize;
}

