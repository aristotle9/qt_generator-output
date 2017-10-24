#include "qt_widgets_c_QStyleOptionHeader.h"

QStyleOptionHeader* qt_widgets_c_QStyleOptionHeader_G_static_cast_QStyleOptionHeader_ptr(QStyleOption* ptr) {
  return static_cast<QStyleOptionHeader*>(ptr);
}

QStyleOption* qt_widgets_c_QStyleOptionHeader_G_static_cast_QStyleOption_ptr(QStyleOptionHeader* ptr) {
  return static_cast<QStyleOption*>(ptr);
}

void qt_widgets_c_QStyleOptionHeader_delete(QStyleOptionHeader* this_ptr) {
  delete this_ptr;
}

const QIcon* qt_widgets_c_QStyleOptionHeader_icon(const QStyleOptionHeader* this_ptr) {
  return &this_ptr->icon;
}

QIcon* qt_widgets_c_QStyleOptionHeader_icon_mut(QStyleOptionHeader* this_ptr) {
  return &this_ptr->icon;
}

QStyleOptionHeader* qt_widgets_c_QStyleOptionHeader_new_no_args() {
  return new QStyleOptionHeader();
}

QStyleOptionHeader* qt_widgets_c_QStyleOptionHeader_new_other(const QStyleOptionHeader* other) {
  return new QStyleOptionHeader(*other);
}

const Qt::Orientation* qt_widgets_c_QStyleOptionHeader_orientation(const QStyleOptionHeader* this_ptr) {
  return &this_ptr->orientation;
}

Qt::Orientation* qt_widgets_c_QStyleOptionHeader_orientation_mut(QStyleOptionHeader* this_ptr) {
  return &this_ptr->orientation;
}

QStyleOptionHeader::SectionPosition qt_widgets_c_QStyleOptionHeader_position(const QStyleOptionHeader* this_ptr) {
  return this_ptr->position;
}

int qt_widgets_c_QStyleOptionHeader_section(const QStyleOptionHeader* this_ptr) {
  return this_ptr->section;
}

QStyleOptionHeader::SelectedPosition qt_widgets_c_QStyleOptionHeader_selectedPosition(const QStyleOptionHeader* this_ptr) {
  return this_ptr->selectedPosition;
}

void qt_widgets_c_QStyleOptionHeader_set_icon(QStyleOptionHeader* this_ptr, const QIcon* value) {
  this_ptr->icon = *value;
}

void qt_widgets_c_QStyleOptionHeader_set_orientation(QStyleOptionHeader* this_ptr, const Qt::Orientation* value) {
  this_ptr->orientation = *value;
}

void qt_widgets_c_QStyleOptionHeader_set_position(QStyleOptionHeader* this_ptr, QStyleOptionHeader::SectionPosition value) {
  this_ptr->position = value;
}

void qt_widgets_c_QStyleOptionHeader_set_section(QStyleOptionHeader* this_ptr, int value) {
  this_ptr->section = value;
}

void qt_widgets_c_QStyleOptionHeader_set_selectedPosition(QStyleOptionHeader* this_ptr, QStyleOptionHeader::SelectedPosition value) {
  this_ptr->selectedPosition = value;
}

void qt_widgets_c_QStyleOptionHeader_set_sortIndicator(QStyleOptionHeader* this_ptr, QStyleOptionHeader::SortIndicator value) {
  this_ptr->sortIndicator = value;
}

void qt_widgets_c_QStyleOptionHeader_set_text(QStyleOptionHeader* this_ptr, const QString* value) {
  this_ptr->text = *value;
}

QStyleOptionHeader::SortIndicator qt_widgets_c_QStyleOptionHeader_sortIndicator(const QStyleOptionHeader* this_ptr) {
  return this_ptr->sortIndicator;
}

const QString* qt_widgets_c_QStyleOptionHeader_text(const QStyleOptionHeader* this_ptr) {
  return &this_ptr->text;
}

QString* qt_widgets_c_QStyleOptionHeader_text_mut(QStyleOptionHeader* this_ptr) {
  return &this_ptr->text;
}

