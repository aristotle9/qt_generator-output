#include "qt_widgets_c_QStyleOptionSlider.h"

QStyleOptionComplex* qt_widgets_c_QStyleOptionSlider_G_static_cast_QStyleOptionComplex_ptr(QStyleOptionSlider* ptr) {
  return static_cast<QStyleOptionComplex*>(ptr);
}

QStyleOptionSlider* qt_widgets_c_QStyleOptionSlider_G_static_cast_QStyleOptionSlider_ptr_QStyleOption(QStyleOption* ptr) {
  return static_cast<QStyleOptionSlider*>(ptr);
}

QStyleOptionSlider* qt_widgets_c_QStyleOptionSlider_G_static_cast_QStyleOptionSlider_ptr_QStyleOptionComplex(QStyleOptionComplex* ptr) {
  return static_cast<QStyleOptionSlider*>(ptr);
}

QStyleOption* qt_widgets_c_QStyleOptionSlider_G_static_cast_QStyleOption_ptr(QStyleOptionSlider* ptr) {
  return static_cast<QStyleOption*>(ptr);
}

void qt_widgets_c_QStyleOptionSlider_delete(QStyleOptionSlider* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QStyleOptionSlider_dialWrapping(const QStyleOptionSlider* this_ptr) {
  return this_ptr->dialWrapping;
}

int qt_widgets_c_QStyleOptionSlider_maximum(const QStyleOptionSlider* this_ptr) {
  return this_ptr->maximum;
}

int qt_widgets_c_QStyleOptionSlider_minimum(const QStyleOptionSlider* this_ptr) {
  return this_ptr->minimum;
}

QStyleOptionSlider* qt_widgets_c_QStyleOptionSlider_new_no_args() {
  return new QStyleOptionSlider();
}

QStyleOptionSlider* qt_widgets_c_QStyleOptionSlider_new_other(const QStyleOptionSlider* other) {
  return new QStyleOptionSlider(*other);
}

double qt_widgets_c_QStyleOptionSlider_notchTarget(const QStyleOptionSlider* this_ptr) {
  return this_ptr->notchTarget;
}

const Qt::Orientation* qt_widgets_c_QStyleOptionSlider_orientation(const QStyleOptionSlider* this_ptr) {
  return &this_ptr->orientation;
}

Qt::Orientation* qt_widgets_c_QStyleOptionSlider_orientation_mut(QStyleOptionSlider* this_ptr) {
  return &this_ptr->orientation;
}

int qt_widgets_c_QStyleOptionSlider_pageStep(const QStyleOptionSlider* this_ptr) {
  return this_ptr->pageStep;
}

void qt_widgets_c_QStyleOptionSlider_set_dialWrapping(QStyleOptionSlider* this_ptr, bool value) {
  this_ptr->dialWrapping = value;
}

void qt_widgets_c_QStyleOptionSlider_set_maximum(QStyleOptionSlider* this_ptr, int value) {
  this_ptr->maximum = value;
}

void qt_widgets_c_QStyleOptionSlider_set_minimum(QStyleOptionSlider* this_ptr, int value) {
  this_ptr->minimum = value;
}

void qt_widgets_c_QStyleOptionSlider_set_notchTarget(QStyleOptionSlider* this_ptr, double value) {
  this_ptr->notchTarget = value;
}

void qt_widgets_c_QStyleOptionSlider_set_orientation(QStyleOptionSlider* this_ptr, const Qt::Orientation* value) {
  this_ptr->orientation = *value;
}

void qt_widgets_c_QStyleOptionSlider_set_pageStep(QStyleOptionSlider* this_ptr, int value) {
  this_ptr->pageStep = value;
}

void qt_widgets_c_QStyleOptionSlider_set_singleStep(QStyleOptionSlider* this_ptr, int value) {
  this_ptr->singleStep = value;
}

void qt_widgets_c_QStyleOptionSlider_set_sliderPosition(QStyleOptionSlider* this_ptr, int value) {
  this_ptr->sliderPosition = value;
}

void qt_widgets_c_QStyleOptionSlider_set_sliderValue(QStyleOptionSlider* this_ptr, int value) {
  this_ptr->sliderValue = value;
}

void qt_widgets_c_QStyleOptionSlider_set_tickInterval(QStyleOptionSlider* this_ptr, int value) {
  this_ptr->tickInterval = value;
}

void qt_widgets_c_QStyleOptionSlider_set_tickPosition(QStyleOptionSlider* this_ptr, const QSlider::TickPosition* value) {
  this_ptr->tickPosition = *value;
}

void qt_widgets_c_QStyleOptionSlider_set_upsideDown(QStyleOptionSlider* this_ptr, bool value) {
  this_ptr->upsideDown = value;
}

int qt_widgets_c_QStyleOptionSlider_singleStep(const QStyleOptionSlider* this_ptr) {
  return this_ptr->singleStep;
}

int qt_widgets_c_QStyleOptionSlider_sliderPosition(const QStyleOptionSlider* this_ptr) {
  return this_ptr->sliderPosition;
}

int qt_widgets_c_QStyleOptionSlider_sliderValue(const QStyleOptionSlider* this_ptr) {
  return this_ptr->sliderValue;
}

int qt_widgets_c_QStyleOptionSlider_tickInterval(const QStyleOptionSlider* this_ptr) {
  return this_ptr->tickInterval;
}

const QSlider::TickPosition* qt_widgets_c_QStyleOptionSlider_tickPosition(const QStyleOptionSlider* this_ptr) {
  return &this_ptr->tickPosition;
}

QSlider::TickPosition* qt_widgets_c_QStyleOptionSlider_tickPosition_mut(QStyleOptionSlider* this_ptr) {
  return &this_ptr->tickPosition;
}

bool qt_widgets_c_QStyleOptionSlider_upsideDown(const QStyleOptionSlider* this_ptr) {
  return this_ptr->upsideDown;
}

