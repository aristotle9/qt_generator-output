#include "qt_widgets_c_QStyleOptionComboBox.h"

QStyleOptionComboBox* qt_widgets_c_QStyleOptionComboBox_G_static_cast_QStyleOptionComboBox_ptr_QStyleOption(QStyleOption* ptr) {
  return static_cast<QStyleOptionComboBox*>(ptr);
}

QStyleOptionComboBox* qt_widgets_c_QStyleOptionComboBox_G_static_cast_QStyleOptionComboBox_ptr_QStyleOptionComplex(QStyleOptionComplex* ptr) {
  return static_cast<QStyleOptionComboBox*>(ptr);
}

QStyleOptionComplex* qt_widgets_c_QStyleOptionComboBox_G_static_cast_QStyleOptionComplex_ptr(QStyleOptionComboBox* ptr) {
  return static_cast<QStyleOptionComplex*>(ptr);
}

QStyleOption* qt_widgets_c_QStyleOptionComboBox_G_static_cast_QStyleOption_ptr(QStyleOptionComboBox* ptr) {
  return static_cast<QStyleOption*>(ptr);
}

const QIcon* qt_widgets_c_QStyleOptionComboBox_currentIcon(const QStyleOptionComboBox* this_ptr) {
  return &this_ptr->currentIcon;
}

QIcon* qt_widgets_c_QStyleOptionComboBox_currentIcon_mut(QStyleOptionComboBox* this_ptr) {
  return &this_ptr->currentIcon;
}

const QString* qt_widgets_c_QStyleOptionComboBox_currentText(const QStyleOptionComboBox* this_ptr) {
  return &this_ptr->currentText;
}

QString* qt_widgets_c_QStyleOptionComboBox_currentText_mut(QStyleOptionComboBox* this_ptr) {
  return &this_ptr->currentText;
}

void qt_widgets_c_QStyleOptionComboBox_delete(QStyleOptionComboBox* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QStyleOptionComboBox_editable(const QStyleOptionComboBox* this_ptr) {
  return this_ptr->editable;
}

bool qt_widgets_c_QStyleOptionComboBox_frame(const QStyleOptionComboBox* this_ptr) {
  return this_ptr->frame;
}

const QSize* qt_widgets_c_QStyleOptionComboBox_iconSize(const QStyleOptionComboBox* this_ptr) {
  return &this_ptr->iconSize;
}

QSize* qt_widgets_c_QStyleOptionComboBox_iconSize_mut(QStyleOptionComboBox* this_ptr) {
  return &this_ptr->iconSize;
}

QStyleOptionComboBox* qt_widgets_c_QStyleOptionComboBox_new_no_args() {
  return new QStyleOptionComboBox();
}

QStyleOptionComboBox* qt_widgets_c_QStyleOptionComboBox_new_other(const QStyleOptionComboBox* other) {
  return new QStyleOptionComboBox(*other);
}

const QRect* qt_widgets_c_QStyleOptionComboBox_popupRect(const QStyleOptionComboBox* this_ptr) {
  return &this_ptr->popupRect;
}

QRect* qt_widgets_c_QStyleOptionComboBox_popupRect_mut(QStyleOptionComboBox* this_ptr) {
  return &this_ptr->popupRect;
}

void qt_widgets_c_QStyleOptionComboBox_set_currentIcon(QStyleOptionComboBox* this_ptr, const QIcon* value) {
  this_ptr->currentIcon = *value;
}

void qt_widgets_c_QStyleOptionComboBox_set_currentText(QStyleOptionComboBox* this_ptr, const QString* value) {
  this_ptr->currentText = *value;
}

void qt_widgets_c_QStyleOptionComboBox_set_editable(QStyleOptionComboBox* this_ptr, bool value) {
  this_ptr->editable = value;
}

void qt_widgets_c_QStyleOptionComboBox_set_frame(QStyleOptionComboBox* this_ptr, bool value) {
  this_ptr->frame = value;
}

void qt_widgets_c_QStyleOptionComboBox_set_iconSize(QStyleOptionComboBox* this_ptr, const QSize* value) {
  this_ptr->iconSize = *value;
}

void qt_widgets_c_QStyleOptionComboBox_set_popupRect(QStyleOptionComboBox* this_ptr, const QRect* value) {
  this_ptr->popupRect = *value;
}

