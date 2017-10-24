#include "qt_widgets_c_QStyleOptionDockWidget.h"

QStyleOptionDockWidget* qt_widgets_c_QStyleOptionDockWidget_G_static_cast_QStyleOptionDockWidget_ptr(QStyleOption* ptr) {
  return static_cast<QStyleOptionDockWidget*>(ptr);
}

QStyleOption* qt_widgets_c_QStyleOptionDockWidget_G_static_cast_QStyleOption_ptr(QStyleOptionDockWidget* ptr) {
  return static_cast<QStyleOption*>(ptr);
}

bool qt_widgets_c_QStyleOptionDockWidget_closable(const QStyleOptionDockWidget* this_ptr) {
  return this_ptr->closable;
}

void qt_widgets_c_QStyleOptionDockWidget_delete(QStyleOptionDockWidget* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QStyleOptionDockWidget_floatable(const QStyleOptionDockWidget* this_ptr) {
  return this_ptr->floatable;
}

bool qt_widgets_c_QStyleOptionDockWidget_movable(const QStyleOptionDockWidget* this_ptr) {
  return this_ptr->movable;
}

QStyleOptionDockWidget* qt_widgets_c_QStyleOptionDockWidget_new_no_args() {
  return new QStyleOptionDockWidget();
}

QStyleOptionDockWidget* qt_widgets_c_QStyleOptionDockWidget_new_other(const QStyleOptionDockWidget* other) {
  return new QStyleOptionDockWidget(*other);
}

void qt_widgets_c_QStyleOptionDockWidget_set_closable(QStyleOptionDockWidget* this_ptr, bool value) {
  this_ptr->closable = value;
}

void qt_widgets_c_QStyleOptionDockWidget_set_floatable(QStyleOptionDockWidget* this_ptr, bool value) {
  this_ptr->floatable = value;
}

void qt_widgets_c_QStyleOptionDockWidget_set_movable(QStyleOptionDockWidget* this_ptr, bool value) {
  this_ptr->movable = value;
}

void qt_widgets_c_QStyleOptionDockWidget_set_title(QStyleOptionDockWidget* this_ptr, const QString* value) {
  this_ptr->title = *value;
}

void qt_widgets_c_QStyleOptionDockWidget_set_verticalTitleBar(QStyleOptionDockWidget* this_ptr, bool value) {
  this_ptr->verticalTitleBar = value;
}

const QString* qt_widgets_c_QStyleOptionDockWidget_title(const QStyleOptionDockWidget* this_ptr) {
  return &this_ptr->title;
}

QString* qt_widgets_c_QStyleOptionDockWidget_title_mut(QStyleOptionDockWidget* this_ptr) {
  return &this_ptr->title;
}

bool qt_widgets_c_QStyleOptionDockWidget_verticalTitleBar(const QStyleOptionDockWidget* this_ptr) {
  return this_ptr->verticalTitleBar;
}

