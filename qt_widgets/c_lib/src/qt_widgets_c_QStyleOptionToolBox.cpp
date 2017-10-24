#include "qt_widgets_c_QStyleOptionToolBox.h"

QStyleOptionToolBox* qt_widgets_c_QStyleOptionToolBox_G_static_cast_QStyleOptionToolBox_ptr(QStyleOption* ptr) {
  return static_cast<QStyleOptionToolBox*>(ptr);
}

QStyleOption* qt_widgets_c_QStyleOptionToolBox_G_static_cast_QStyleOption_ptr(QStyleOptionToolBox* ptr) {
  return static_cast<QStyleOption*>(ptr);
}

void qt_widgets_c_QStyleOptionToolBox_constructor_no_args(QStyleOptionToolBox* output) {
  new(output) QStyleOptionToolBox();
}

void qt_widgets_c_QStyleOptionToolBox_constructor_other(const QStyleOptionToolBox* other, QStyleOptionToolBox* output) {
  new(output) QStyleOptionToolBox(*other);
}

void qt_widgets_c_QStyleOptionToolBox_destructor(QStyleOptionToolBox* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

const QIcon* qt_widgets_c_QStyleOptionToolBox_icon(const QStyleOptionToolBox* this_ptr) {
  return &this_ptr->icon;
}

QIcon* qt_widgets_c_QStyleOptionToolBox_icon_mut(QStyleOptionToolBox* this_ptr) {
  return &this_ptr->icon;
}

QStyleOptionToolBox::TabPosition qt_widgets_c_QStyleOptionToolBox_position(const QStyleOptionToolBox* this_ptr) {
  return this_ptr->position;
}

QStyleOptionToolBox::SelectedPosition qt_widgets_c_QStyleOptionToolBox_selectedPosition(const QStyleOptionToolBox* this_ptr) {
  return this_ptr->selectedPosition;
}

void qt_widgets_c_QStyleOptionToolBox_set_icon(QStyleOptionToolBox* this_ptr, const QIcon* value) {
  this_ptr->icon = *value;
}

void qt_widgets_c_QStyleOptionToolBox_set_position(QStyleOptionToolBox* this_ptr, QStyleOptionToolBox::TabPosition value) {
  this_ptr->position = value;
}

void qt_widgets_c_QStyleOptionToolBox_set_selectedPosition(QStyleOptionToolBox* this_ptr, QStyleOptionToolBox::SelectedPosition value) {
  this_ptr->selectedPosition = value;
}

void qt_widgets_c_QStyleOptionToolBox_set_text(QStyleOptionToolBox* this_ptr, const QString* value) {
  this_ptr->text = *value;
}

const QString* qt_widgets_c_QStyleOptionToolBox_text(const QStyleOptionToolBox* this_ptr) {
  return &this_ptr->text;
}

QString* qt_widgets_c_QStyleOptionToolBox_text_mut(QStyleOptionToolBox* this_ptr) {
  return &this_ptr->text;
}

