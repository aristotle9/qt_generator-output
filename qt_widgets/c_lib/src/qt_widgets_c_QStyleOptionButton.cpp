#include "qt_widgets_c_QStyleOptionButton.h"

QStyleOptionButton* qt_widgets_c_QStyleOptionButton_G_static_cast_QStyleOptionButton_ptr(QStyleOption* ptr) {
  return static_cast<QStyleOptionButton*>(ptr);
}

QStyleOption* qt_widgets_c_QStyleOptionButton_G_static_cast_QStyleOption_ptr(QStyleOptionButton* ptr) {
  return static_cast<QStyleOption*>(ptr);
}

void qt_widgets_c_QStyleOptionButton_delete(QStyleOptionButton* this_ptr) {
  delete this_ptr;
}

unsigned int qt_widgets_c_QStyleOptionButton_features(const QStyleOptionButton* this_ptr) {
  return uint(this_ptr->features);
}

const QIcon* qt_widgets_c_QStyleOptionButton_icon(const QStyleOptionButton* this_ptr) {
  return &this_ptr->icon;
}

const QSize* qt_widgets_c_QStyleOptionButton_iconSize(const QStyleOptionButton* this_ptr) {
  return &this_ptr->iconSize;
}

QSize* qt_widgets_c_QStyleOptionButton_iconSize_mut(QStyleOptionButton* this_ptr) {
  return &this_ptr->iconSize;
}

QIcon* qt_widgets_c_QStyleOptionButton_icon_mut(QStyleOptionButton* this_ptr) {
  return &this_ptr->icon;
}

QStyleOptionButton* qt_widgets_c_QStyleOptionButton_new_no_args() {
  return new QStyleOptionButton();
}

QStyleOptionButton* qt_widgets_c_QStyleOptionButton_new_other(const QStyleOptionButton* other) {
  return new QStyleOptionButton(*other);
}

void qt_widgets_c_QStyleOptionButton_set_features(QStyleOptionButton* this_ptr, unsigned int value) {
  this_ptr->features = QFlags< QStyleOptionButton::ButtonFeature >(value);
}

void qt_widgets_c_QStyleOptionButton_set_icon(QStyleOptionButton* this_ptr, const QIcon* value) {
  this_ptr->icon = *value;
}

void qt_widgets_c_QStyleOptionButton_set_iconSize(QStyleOptionButton* this_ptr, const QSize* value) {
  this_ptr->iconSize = *value;
}

void qt_widgets_c_QStyleOptionButton_set_text(QStyleOptionButton* this_ptr, const QString* value) {
  this_ptr->text = *value;
}

const QString* qt_widgets_c_QStyleOptionButton_text(const QStyleOptionButton* this_ptr) {
  return &this_ptr->text;
}

QString* qt_widgets_c_QStyleOptionButton_text_mut(QStyleOptionButton* this_ptr) {
  return &this_ptr->text;
}

