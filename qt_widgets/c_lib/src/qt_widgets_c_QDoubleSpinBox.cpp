#include "qt_widgets_c_QDoubleSpinBox.h"

QDoubleSpinBox* qt_widgets_c_QDoubleSpinBox_G_dynamic_cast_QDoubleSpinBox_ptr_QAbstractSpinBox(QAbstractSpinBox* ptr) {
  return dynamic_cast<QDoubleSpinBox*>(ptr);
}

QDoubleSpinBox* qt_widgets_c_QDoubleSpinBox_G_dynamic_cast_QDoubleSpinBox_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QDoubleSpinBox*>(ptr);
}

QAbstractSpinBox* qt_widgets_c_QDoubleSpinBox_G_static_cast_QAbstractSpinBox_ptr(QDoubleSpinBox* ptr) {
  return static_cast<QAbstractSpinBox*>(ptr);
}

QDoubleSpinBox* qt_widgets_c_QDoubleSpinBox_G_static_cast_QDoubleSpinBox_ptr_QAbstractSpinBox(QAbstractSpinBox* ptr) {
  return static_cast<QDoubleSpinBox*>(ptr);
}

QDoubleSpinBox* qt_widgets_c_QDoubleSpinBox_G_static_cast_QDoubleSpinBox_ptr_QObject(QObject* ptr) {
  return static_cast<QDoubleSpinBox*>(ptr);
}

QDoubleSpinBox* qt_widgets_c_QDoubleSpinBox_G_static_cast_QDoubleSpinBox_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QDoubleSpinBox*>(ptr);
}

QDoubleSpinBox* qt_widgets_c_QDoubleSpinBox_G_static_cast_QDoubleSpinBox_ptr_QWidget(QWidget* ptr) {
  return static_cast<QDoubleSpinBox*>(ptr);
}

QObject* qt_widgets_c_QDoubleSpinBox_G_static_cast_QObject_ptr(QDoubleSpinBox* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QDoubleSpinBox_G_static_cast_QPaintDevice_ptr(QDoubleSpinBox* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QDoubleSpinBox_G_static_cast_QWidget_ptr(QDoubleSpinBox* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QDoubleSpinBox_cleanText_to_output(const QDoubleSpinBox* this_ptr, QString* output) {
  new(output) QString(this_ptr->cleanText());
}

int qt_widgets_c_QDoubleSpinBox_decimals(const QDoubleSpinBox* this_ptr) {
  return this_ptr->decimals();
}

void qt_widgets_c_QDoubleSpinBox_delete(QDoubleSpinBox* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QDoubleSpinBox_fixup(const QDoubleSpinBox* this_ptr, QString* str) {
  this_ptr->fixup(*str);
}

double qt_widgets_c_QDoubleSpinBox_maximum(const QDoubleSpinBox* this_ptr) {
  return this_ptr->maximum();
}

const QMetaObject* qt_widgets_c_QDoubleSpinBox_metaObject(const QDoubleSpinBox* this_ptr) {
  return this_ptr->metaObject();
}

double qt_widgets_c_QDoubleSpinBox_minimum(const QDoubleSpinBox* this_ptr) {
  return this_ptr->minimum();
}

QDoubleSpinBox* qt_widgets_c_QDoubleSpinBox_new_no_args() {
  return new QDoubleSpinBox();
}

QDoubleSpinBox* qt_widgets_c_QDoubleSpinBox_new_parent(QWidget* parent) {
  return new QDoubleSpinBox(parent);
}

void qt_widgets_c_QDoubleSpinBox_prefix_to_output(const QDoubleSpinBox* this_ptr, QString* output) {
  new(output) QString(this_ptr->prefix());
}

int qt_widgets_c_QDoubleSpinBox_qt_metacall(QDoubleSpinBox* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QDoubleSpinBox_qt_metacast(QDoubleSpinBox* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QDoubleSpinBox_setDecimals(QDoubleSpinBox* this_ptr, int prec) {
  this_ptr->setDecimals(prec);
}

void qt_widgets_c_QDoubleSpinBox_setMaximum(QDoubleSpinBox* this_ptr, double max) {
  this_ptr->setMaximum(max);
}

void qt_widgets_c_QDoubleSpinBox_setMinimum(QDoubleSpinBox* this_ptr, double min) {
  this_ptr->setMinimum(min);
}

void qt_widgets_c_QDoubleSpinBox_setPrefix(QDoubleSpinBox* this_ptr, const QString* prefix) {
  this_ptr->setPrefix(*prefix);
}

void qt_widgets_c_QDoubleSpinBox_setRange(QDoubleSpinBox* this_ptr, double min, double max) {
  this_ptr->setRange(min, max);
}

void qt_widgets_c_QDoubleSpinBox_setSingleStep(QDoubleSpinBox* this_ptr, double val) {
  this_ptr->setSingleStep(val);
}

void qt_widgets_c_QDoubleSpinBox_setSuffix(QDoubleSpinBox* this_ptr, const QString* suffix) {
  this_ptr->setSuffix(*suffix);
}

void qt_widgets_c_QDoubleSpinBox_setValue(QDoubleSpinBox* this_ptr, double val) {
  this_ptr->setValue(val);
}

double qt_widgets_c_QDoubleSpinBox_singleStep(const QDoubleSpinBox* this_ptr) {
  return this_ptr->singleStep();
}

void qt_widgets_c_QDoubleSpinBox_suffix_to_output(const QDoubleSpinBox* this_ptr, QString* output) {
  new(output) QString(this_ptr->suffix());
}

void qt_widgets_c_QDoubleSpinBox_textFromValue_to_output(const QDoubleSpinBox* this_ptr, double val, QString* output) {
  new(output) QString(this_ptr->textFromValue(val));
}

void qt_widgets_c_QDoubleSpinBox_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDoubleSpinBox::trUtf8(s, c, n));
}

void qt_widgets_c_QDoubleSpinBox_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDoubleSpinBox::tr(s, c, n));
}

double qt_widgets_c_QDoubleSpinBox_value(const QDoubleSpinBox* this_ptr) {
  return this_ptr->value();
}

double qt_widgets_c_QDoubleSpinBox_valueFromText(const QDoubleSpinBox* this_ptr, const QString* text) {
  return this_ptr->valueFromText(*text);
}

