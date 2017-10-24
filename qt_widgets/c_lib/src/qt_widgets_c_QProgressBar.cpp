#include "qt_widgets_c_QProgressBar.h"

QProgressBar* qt_widgets_c_QProgressBar_G_dynamic_cast_QProgressBar_ptr(QWidget* ptr) {
  return dynamic_cast<QProgressBar*>(ptr);
}

QObject* qt_widgets_c_QProgressBar_G_static_cast_QObject_ptr(QProgressBar* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QProgressBar_G_static_cast_QPaintDevice_ptr(QProgressBar* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QProgressBar* qt_widgets_c_QProgressBar_G_static_cast_QProgressBar_ptr_QObject(QObject* ptr) {
  return static_cast<QProgressBar*>(ptr);
}

QProgressBar* qt_widgets_c_QProgressBar_G_static_cast_QProgressBar_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QProgressBar*>(ptr);
}

QProgressBar* qt_widgets_c_QProgressBar_G_static_cast_QProgressBar_ptr_QWidget(QWidget* ptr) {
  return static_cast<QProgressBar*>(ptr);
}

QWidget* qt_widgets_c_QProgressBar_G_static_cast_QWidget_ptr(QProgressBar* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QProgressBar_delete(QProgressBar* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QProgressBar_format_to_output(const QProgressBar* this_ptr, QString* output) {
  new(output) QString(this_ptr->format());
}

bool qt_widgets_c_QProgressBar_invertedAppearance(const QProgressBar* this_ptr) {
  return this_ptr->invertedAppearance();
}

bool qt_widgets_c_QProgressBar_isTextVisible(const QProgressBar* this_ptr) {
  return this_ptr->isTextVisible();
}

int qt_widgets_c_QProgressBar_maximum(const QProgressBar* this_ptr) {
  return this_ptr->maximum();
}

const QMetaObject* qt_widgets_c_QProgressBar_metaObject(const QProgressBar* this_ptr) {
  return this_ptr->metaObject();
}

int qt_widgets_c_QProgressBar_minimum(const QProgressBar* this_ptr) {
  return this_ptr->minimum();
}

void qt_widgets_c_QProgressBar_minimumSizeHint_to_output(const QProgressBar* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSizeHint());
}

QProgressBar* qt_widgets_c_QProgressBar_new_no_args() {
  return new QProgressBar();
}

QProgressBar* qt_widgets_c_QProgressBar_new_parent(QWidget* parent) {
  return new QProgressBar(parent);
}

int qt_widgets_c_QProgressBar_qt_metacall(QProgressBar* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QProgressBar_qt_metacast(QProgressBar* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QProgressBar_reset(QProgressBar* this_ptr) {
  this_ptr->reset();
}

void qt_widgets_c_QProgressBar_resetFormat(QProgressBar* this_ptr) {
  this_ptr->resetFormat();
}

void qt_widgets_c_QProgressBar_setFormat(QProgressBar* this_ptr, const QString* format) {
  this_ptr->setFormat(*format);
}

void qt_widgets_c_QProgressBar_setInvertedAppearance(QProgressBar* this_ptr, bool invert) {
  this_ptr->setInvertedAppearance(invert);
}

void qt_widgets_c_QProgressBar_setMaximum(QProgressBar* this_ptr, int maximum) {
  this_ptr->setMaximum(maximum);
}

void qt_widgets_c_QProgressBar_setMinimum(QProgressBar* this_ptr, int minimum) {
  this_ptr->setMinimum(minimum);
}

void qt_widgets_c_QProgressBar_setOrientation(QProgressBar* this_ptr, const Qt::Orientation* arg1) {
  this_ptr->setOrientation(*arg1);
}

void qt_widgets_c_QProgressBar_setRange(QProgressBar* this_ptr, int minimum, int maximum) {
  this_ptr->setRange(minimum, maximum);
}

void qt_widgets_c_QProgressBar_setTextDirection(QProgressBar* this_ptr, const QProgressBar::Direction* textDirection) {
  this_ptr->setTextDirection(*textDirection);
}

void qt_widgets_c_QProgressBar_setTextVisible(QProgressBar* this_ptr, bool visible) {
  this_ptr->setTextVisible(visible);
}

void qt_widgets_c_QProgressBar_setValue(QProgressBar* this_ptr, int value) {
  this_ptr->setValue(value);
}

void qt_widgets_c_QProgressBar_sizeHint_to_output(const QProgressBar* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QProgressBar_text_to_output(const QProgressBar* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

void qt_widgets_c_QProgressBar_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QProgressBar::trUtf8(s, c, n));
}

void qt_widgets_c_QProgressBar_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QProgressBar::tr(s, c, n));
}

int qt_widgets_c_QProgressBar_value(const QProgressBar* this_ptr) {
  return this_ptr->value();
}

