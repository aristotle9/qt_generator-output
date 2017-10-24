#include "qt_widgets_c_QSpinBox.h"

QSpinBox* qt_widgets_c_QSpinBox_G_dynamic_cast_QSpinBox_ptr_QAbstractSpinBox(QAbstractSpinBox* ptr) {
  return dynamic_cast<QSpinBox*>(ptr);
}

QSpinBox* qt_widgets_c_QSpinBox_G_dynamic_cast_QSpinBox_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QSpinBox*>(ptr);
}

QAbstractSpinBox* qt_widgets_c_QSpinBox_G_static_cast_QAbstractSpinBox_ptr(QSpinBox* ptr) {
  return static_cast<QAbstractSpinBox*>(ptr);
}

QObject* qt_widgets_c_QSpinBox_G_static_cast_QObject_ptr(QSpinBox* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QSpinBox_G_static_cast_QPaintDevice_ptr(QSpinBox* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QSpinBox* qt_widgets_c_QSpinBox_G_static_cast_QSpinBox_ptr_QAbstractSpinBox(QAbstractSpinBox* ptr) {
  return static_cast<QSpinBox*>(ptr);
}

QSpinBox* qt_widgets_c_QSpinBox_G_static_cast_QSpinBox_ptr_QObject(QObject* ptr) {
  return static_cast<QSpinBox*>(ptr);
}

QSpinBox* qt_widgets_c_QSpinBox_G_static_cast_QSpinBox_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QSpinBox*>(ptr);
}

QSpinBox* qt_widgets_c_QSpinBox_G_static_cast_QSpinBox_ptr_QWidget(QWidget* ptr) {
  return static_cast<QSpinBox*>(ptr);
}

QWidget* qt_widgets_c_QSpinBox_G_static_cast_QWidget_ptr(QSpinBox* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QSpinBox_cleanText_to_output(const QSpinBox* this_ptr, QString* output) {
  new(output) QString(this_ptr->cleanText());
}

void qt_widgets_c_QSpinBox_delete(QSpinBox* this_ptr) {
  delete this_ptr;
}

int qt_widgets_c_QSpinBox_displayIntegerBase(const QSpinBox* this_ptr) {
  return this_ptr->displayIntegerBase();
}

int qt_widgets_c_QSpinBox_maximum(const QSpinBox* this_ptr) {
  return this_ptr->maximum();
}

const QMetaObject* qt_widgets_c_QSpinBox_metaObject(const QSpinBox* this_ptr) {
  return this_ptr->metaObject();
}

int qt_widgets_c_QSpinBox_minimum(const QSpinBox* this_ptr) {
  return this_ptr->minimum();
}

QSpinBox* qt_widgets_c_QSpinBox_new_no_args() {
  return new QSpinBox();
}

QSpinBox* qt_widgets_c_QSpinBox_new_parent(QWidget* parent) {
  return new QSpinBox(parent);
}

void qt_widgets_c_QSpinBox_prefix_to_output(const QSpinBox* this_ptr, QString* output) {
  new(output) QString(this_ptr->prefix());
}

int qt_widgets_c_QSpinBox_qt_metacall(QSpinBox* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QSpinBox_qt_metacast(QSpinBox* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QSpinBox_setDisplayIntegerBase(QSpinBox* this_ptr, int base) {
  this_ptr->setDisplayIntegerBase(base);
}

void qt_widgets_c_QSpinBox_setMaximum(QSpinBox* this_ptr, int max) {
  this_ptr->setMaximum(max);
}

void qt_widgets_c_QSpinBox_setMinimum(QSpinBox* this_ptr, int min) {
  this_ptr->setMinimum(min);
}

void qt_widgets_c_QSpinBox_setPrefix(QSpinBox* this_ptr, const QString* prefix) {
  this_ptr->setPrefix(*prefix);
}

void qt_widgets_c_QSpinBox_setRange(QSpinBox* this_ptr, int min, int max) {
  this_ptr->setRange(min, max);
}

void qt_widgets_c_QSpinBox_setSingleStep(QSpinBox* this_ptr, int val) {
  this_ptr->setSingleStep(val);
}

void qt_widgets_c_QSpinBox_setSuffix(QSpinBox* this_ptr, const QString* suffix) {
  this_ptr->setSuffix(*suffix);
}

void qt_widgets_c_QSpinBox_setValue(QSpinBox* this_ptr, int val) {
  this_ptr->setValue(val);
}

int qt_widgets_c_QSpinBox_singleStep(const QSpinBox* this_ptr) {
  return this_ptr->singleStep();
}

void qt_widgets_c_QSpinBox_suffix_to_output(const QSpinBox* this_ptr, QString* output) {
  new(output) QString(this_ptr->suffix());
}

void qt_widgets_c_QSpinBox_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSpinBox::trUtf8(s, c, n));
}

void qt_widgets_c_QSpinBox_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSpinBox::tr(s, c, n));
}

int qt_widgets_c_QSpinBox_value(const QSpinBox* this_ptr) {
  return this_ptr->value();
}

