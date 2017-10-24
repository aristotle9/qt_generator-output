#include "qt_widgets_c_QStyleOptionToolBar.h"

QStyleOptionToolBar* qt_widgets_c_QStyleOptionToolBar_G_static_cast_QStyleOptionToolBar_ptr(QStyleOption* ptr) {
  return static_cast<QStyleOptionToolBar*>(ptr);
}

QStyleOption* qt_widgets_c_QStyleOptionToolBar_G_static_cast_QStyleOption_ptr(QStyleOptionToolBar* ptr) {
  return static_cast<QStyleOption*>(ptr);
}

void qt_widgets_c_QStyleOptionToolBar_delete(QStyleOptionToolBar* this_ptr) {
  delete this_ptr;
}

unsigned int qt_widgets_c_QStyleOptionToolBar_features(const QStyleOptionToolBar* this_ptr) {
  return uint(this_ptr->features);
}

int qt_widgets_c_QStyleOptionToolBar_lineWidth(const QStyleOptionToolBar* this_ptr) {
  return this_ptr->lineWidth;
}

int qt_widgets_c_QStyleOptionToolBar_midLineWidth(const QStyleOptionToolBar* this_ptr) {
  return this_ptr->midLineWidth;
}

QStyleOptionToolBar* qt_widgets_c_QStyleOptionToolBar_new_no_args() {
  return new QStyleOptionToolBar();
}

QStyleOptionToolBar* qt_widgets_c_QStyleOptionToolBar_new_other(const QStyleOptionToolBar* other) {
  return new QStyleOptionToolBar(*other);
}

QStyleOptionToolBar::ToolBarPosition qt_widgets_c_QStyleOptionToolBar_positionOfLine(const QStyleOptionToolBar* this_ptr) {
  return this_ptr->positionOfLine;
}

QStyleOptionToolBar::ToolBarPosition qt_widgets_c_QStyleOptionToolBar_positionWithinLine(const QStyleOptionToolBar* this_ptr) {
  return this_ptr->positionWithinLine;
}

void qt_widgets_c_QStyleOptionToolBar_set_features(QStyleOptionToolBar* this_ptr, unsigned int value) {
  this_ptr->features = QFlags< QStyleOptionToolBar::ToolBarFeature >(value);
}

void qt_widgets_c_QStyleOptionToolBar_set_lineWidth(QStyleOptionToolBar* this_ptr, int value) {
  this_ptr->lineWidth = value;
}

void qt_widgets_c_QStyleOptionToolBar_set_midLineWidth(QStyleOptionToolBar* this_ptr, int value) {
  this_ptr->midLineWidth = value;
}

void qt_widgets_c_QStyleOptionToolBar_set_positionOfLine(QStyleOptionToolBar* this_ptr, QStyleOptionToolBar::ToolBarPosition value) {
  this_ptr->positionOfLine = value;
}

void qt_widgets_c_QStyleOptionToolBar_set_positionWithinLine(QStyleOptionToolBar* this_ptr, QStyleOptionToolBar::ToolBarPosition value) {
  this_ptr->positionWithinLine = value;
}

void qt_widgets_c_QStyleOptionToolBar_set_toolBarArea(QStyleOptionToolBar* this_ptr, const Qt::ToolBarArea* value) {
  this_ptr->toolBarArea = *value;
}

const Qt::ToolBarArea* qt_widgets_c_QStyleOptionToolBar_toolBarArea(const QStyleOptionToolBar* this_ptr) {
  return &this_ptr->toolBarArea;
}

Qt::ToolBarArea* qt_widgets_c_QStyleOptionToolBar_toolBarArea_mut(QStyleOptionToolBar* this_ptr) {
  return &this_ptr->toolBarArea;
}

