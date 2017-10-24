#include "qt_widgets_c_QAbstractSlider.h"

QAbstractSlider* qt_widgets_c_QAbstractSlider_G_dynamic_cast_QAbstractSlider_ptr(QWidget* ptr) {
  return dynamic_cast<QAbstractSlider*>(ptr);
}

QAbstractSlider* qt_widgets_c_QAbstractSlider_G_static_cast_QAbstractSlider_ptr_QObject(QObject* ptr) {
  return static_cast<QAbstractSlider*>(ptr);
}

QAbstractSlider* qt_widgets_c_QAbstractSlider_G_static_cast_QAbstractSlider_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QAbstractSlider*>(ptr);
}

QAbstractSlider* qt_widgets_c_QAbstractSlider_G_static_cast_QAbstractSlider_ptr_QWidget(QWidget* ptr) {
  return static_cast<QAbstractSlider*>(ptr);
}

QObject* qt_widgets_c_QAbstractSlider_G_static_cast_QObject_ptr(QAbstractSlider* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QAbstractSlider_G_static_cast_QPaintDevice_ptr(QAbstractSlider* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QAbstractSlider_G_static_cast_QWidget_ptr(QAbstractSlider* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QAbstractSlider_delete(QAbstractSlider* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QAbstractSlider_hasTracking(const QAbstractSlider* this_ptr) {
  return this_ptr->hasTracking();
}

bool qt_widgets_c_QAbstractSlider_invertedAppearance(const QAbstractSlider* this_ptr) {
  return this_ptr->invertedAppearance();
}

bool qt_widgets_c_QAbstractSlider_invertedControls(const QAbstractSlider* this_ptr) {
  return this_ptr->invertedControls();
}

bool qt_widgets_c_QAbstractSlider_isSliderDown(const QAbstractSlider* this_ptr) {
  return this_ptr->isSliderDown();
}

int qt_widgets_c_QAbstractSlider_maximum(const QAbstractSlider* this_ptr) {
  return this_ptr->maximum();
}

const QMetaObject* qt_widgets_c_QAbstractSlider_metaObject(const QAbstractSlider* this_ptr) {
  return this_ptr->metaObject();
}

int qt_widgets_c_QAbstractSlider_minimum(const QAbstractSlider* this_ptr) {
  return this_ptr->minimum();
}

QAbstractSlider* qt_widgets_c_QAbstractSlider_new_no_args() {
  return new QAbstractSlider();
}

QAbstractSlider* qt_widgets_c_QAbstractSlider_new_parent(QWidget* parent) {
  return new QAbstractSlider(parent);
}

int qt_widgets_c_QAbstractSlider_pageStep(const QAbstractSlider* this_ptr) {
  return this_ptr->pageStep();
}

int qt_widgets_c_QAbstractSlider_qt_metacall(QAbstractSlider* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QAbstractSlider_qt_metacast(QAbstractSlider* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QAbstractSlider_setInvertedAppearance(QAbstractSlider* this_ptr, bool arg1) {
  this_ptr->setInvertedAppearance(arg1);
}

void qt_widgets_c_QAbstractSlider_setInvertedControls(QAbstractSlider* this_ptr, bool arg1) {
  this_ptr->setInvertedControls(arg1);
}

void qt_widgets_c_QAbstractSlider_setMaximum(QAbstractSlider* this_ptr, int arg1) {
  this_ptr->setMaximum(arg1);
}

void qt_widgets_c_QAbstractSlider_setMinimum(QAbstractSlider* this_ptr, int arg1) {
  this_ptr->setMinimum(arg1);
}

void qt_widgets_c_QAbstractSlider_setOrientation(QAbstractSlider* this_ptr, const Qt::Orientation* arg1) {
  this_ptr->setOrientation(*arg1);
}

void qt_widgets_c_QAbstractSlider_setPageStep(QAbstractSlider* this_ptr, int arg1) {
  this_ptr->setPageStep(arg1);
}

void qt_widgets_c_QAbstractSlider_setRange(QAbstractSlider* this_ptr, int min, int max) {
  this_ptr->setRange(min, max);
}

void qt_widgets_c_QAbstractSlider_setSingleStep(QAbstractSlider* this_ptr, int arg1) {
  this_ptr->setSingleStep(arg1);
}

void qt_widgets_c_QAbstractSlider_setSliderDown(QAbstractSlider* this_ptr, bool arg1) {
  this_ptr->setSliderDown(arg1);
}

void qt_widgets_c_QAbstractSlider_setSliderPosition(QAbstractSlider* this_ptr, int arg1) {
  this_ptr->setSliderPosition(arg1);
}

void qt_widgets_c_QAbstractSlider_setTracking(QAbstractSlider* this_ptr, bool enable) {
  this_ptr->setTracking(enable);
}

void qt_widgets_c_QAbstractSlider_setValue(QAbstractSlider* this_ptr, int arg1) {
  this_ptr->setValue(arg1);
}

int qt_widgets_c_QAbstractSlider_singleStep(const QAbstractSlider* this_ptr) {
  return this_ptr->singleStep();
}

int qt_widgets_c_QAbstractSlider_sliderPosition(const QAbstractSlider* this_ptr) {
  return this_ptr->sliderPosition();
}

void qt_widgets_c_QAbstractSlider_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractSlider::trUtf8(s, c, n));
}

void qt_widgets_c_QAbstractSlider_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractSlider::tr(s, c, n));
}

void qt_widgets_c_QAbstractSlider_triggerAction(QAbstractSlider* this_ptr, QAbstractSlider::SliderAction action) {
  this_ptr->triggerAction(action);
}

int qt_widgets_c_QAbstractSlider_value(const QAbstractSlider* this_ptr) {
  return this_ptr->value();
}

