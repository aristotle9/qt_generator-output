#include "qt_widgets_c_QStyleOptionProgressBar.h"

QStyleOptionProgressBar* qt_widgets_c_QStyleOptionProgressBar_G_static_cast_QStyleOptionProgressBar_ptr(QStyleOption* ptr) {
  return static_cast<QStyleOptionProgressBar*>(ptr);
}

QStyleOption* qt_widgets_c_QStyleOptionProgressBar_G_static_cast_QStyleOption_ptr(QStyleOptionProgressBar* ptr) {
  return static_cast<QStyleOption*>(ptr);
}

bool qt_widgets_c_QStyleOptionProgressBar_bottomToTop(const QStyleOptionProgressBar* this_ptr) {
  return this_ptr->bottomToTop;
}

void qt_widgets_c_QStyleOptionProgressBar_delete(QStyleOptionProgressBar* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QStyleOptionProgressBar_invertedAppearance(const QStyleOptionProgressBar* this_ptr) {
  return this_ptr->invertedAppearance;
}

int qt_widgets_c_QStyleOptionProgressBar_maximum(const QStyleOptionProgressBar* this_ptr) {
  return this_ptr->maximum;
}

int qt_widgets_c_QStyleOptionProgressBar_minimum(const QStyleOptionProgressBar* this_ptr) {
  return this_ptr->minimum;
}

QStyleOptionProgressBar* qt_widgets_c_QStyleOptionProgressBar_new_no_args() {
  return new QStyleOptionProgressBar();
}

QStyleOptionProgressBar* qt_widgets_c_QStyleOptionProgressBar_new_other(const QStyleOptionProgressBar* other) {
  return new QStyleOptionProgressBar(*other);
}

const Qt::Orientation* qt_widgets_c_QStyleOptionProgressBar_orientation(const QStyleOptionProgressBar* this_ptr) {
  return &this_ptr->orientation;
}

Qt::Orientation* qt_widgets_c_QStyleOptionProgressBar_orientation_mut(QStyleOptionProgressBar* this_ptr) {
  return &this_ptr->orientation;
}

int qt_widgets_c_QStyleOptionProgressBar_progress(const QStyleOptionProgressBar* this_ptr) {
  return this_ptr->progress;
}

void qt_widgets_c_QStyleOptionProgressBar_set_bottomToTop(QStyleOptionProgressBar* this_ptr, bool value) {
  this_ptr->bottomToTop = value;
}

void qt_widgets_c_QStyleOptionProgressBar_set_invertedAppearance(QStyleOptionProgressBar* this_ptr, bool value) {
  this_ptr->invertedAppearance = value;
}

void qt_widgets_c_QStyleOptionProgressBar_set_maximum(QStyleOptionProgressBar* this_ptr, int value) {
  this_ptr->maximum = value;
}

void qt_widgets_c_QStyleOptionProgressBar_set_minimum(QStyleOptionProgressBar* this_ptr, int value) {
  this_ptr->minimum = value;
}

void qt_widgets_c_QStyleOptionProgressBar_set_orientation(QStyleOptionProgressBar* this_ptr, const Qt::Orientation* value) {
  this_ptr->orientation = *value;
}

void qt_widgets_c_QStyleOptionProgressBar_set_progress(QStyleOptionProgressBar* this_ptr, int value) {
  this_ptr->progress = value;
}

void qt_widgets_c_QStyleOptionProgressBar_set_text(QStyleOptionProgressBar* this_ptr, const QString* value) {
  this_ptr->text = *value;
}

void qt_widgets_c_QStyleOptionProgressBar_set_textVisible(QStyleOptionProgressBar* this_ptr, bool value) {
  this_ptr->textVisible = value;
}

const QString* qt_widgets_c_QStyleOptionProgressBar_text(const QStyleOptionProgressBar* this_ptr) {
  return &this_ptr->text;
}

bool qt_widgets_c_QStyleOptionProgressBar_textVisible(const QStyleOptionProgressBar* this_ptr) {
  return this_ptr->textVisible;
}

QString* qt_widgets_c_QStyleOptionProgressBar_text_mut(QStyleOptionProgressBar* this_ptr) {
  return &this_ptr->text;
}

