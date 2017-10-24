#include "qt_widgets_c_QProgressDialog.h"

QProgressDialog* qt_widgets_c_QProgressDialog_G_dynamic_cast_QProgressDialog_ptr_QDialog(QDialog* ptr) {
  return dynamic_cast<QProgressDialog*>(ptr);
}

QProgressDialog* qt_widgets_c_QProgressDialog_G_dynamic_cast_QProgressDialog_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QProgressDialog*>(ptr);
}

QDialog* qt_widgets_c_QProgressDialog_G_static_cast_QDialog_ptr(QProgressDialog* ptr) {
  return static_cast<QDialog*>(ptr);
}

QObject* qt_widgets_c_QProgressDialog_G_static_cast_QObject_ptr(QProgressDialog* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QProgressDialog_G_static_cast_QPaintDevice_ptr(QProgressDialog* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QProgressDialog* qt_widgets_c_QProgressDialog_G_static_cast_QProgressDialog_ptr_QDialog(QDialog* ptr) {
  return static_cast<QProgressDialog*>(ptr);
}

QProgressDialog* qt_widgets_c_QProgressDialog_G_static_cast_QProgressDialog_ptr_QObject(QObject* ptr) {
  return static_cast<QProgressDialog*>(ptr);
}

QProgressDialog* qt_widgets_c_QProgressDialog_G_static_cast_QProgressDialog_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QProgressDialog*>(ptr);
}

QProgressDialog* qt_widgets_c_QProgressDialog_G_static_cast_QProgressDialog_ptr_QWidget(QWidget* ptr) {
  return static_cast<QProgressDialog*>(ptr);
}

QWidget* qt_widgets_c_QProgressDialog_G_static_cast_QWidget_ptr(QProgressDialog* ptr) {
  return static_cast<QWidget*>(ptr);
}

bool qt_widgets_c_QProgressDialog_autoClose(const QProgressDialog* this_ptr) {
  return this_ptr->autoClose();
}

bool qt_widgets_c_QProgressDialog_autoReset(const QProgressDialog* this_ptr) {
  return this_ptr->autoReset();
}

void qt_widgets_c_QProgressDialog_cancel(QProgressDialog* this_ptr) {
  this_ptr->cancel();
}

void qt_widgets_c_QProgressDialog_delete(QProgressDialog* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QProgressDialog_labelText_to_output(const QProgressDialog* this_ptr, QString* output) {
  new(output) QString(this_ptr->labelText());
}

int qt_widgets_c_QProgressDialog_maximum(const QProgressDialog* this_ptr) {
  return this_ptr->maximum();
}

const QMetaObject* qt_widgets_c_QProgressDialog_metaObject(const QProgressDialog* this_ptr) {
  return this_ptr->metaObject();
}

int qt_widgets_c_QProgressDialog_minimum(const QProgressDialog* this_ptr) {
  return this_ptr->minimum();
}

int qt_widgets_c_QProgressDialog_minimumDuration(const QProgressDialog* this_ptr) {
  return this_ptr->minimumDuration();
}

void qt_widgets_c_QProgressDialog_open(QProgressDialog* this_ptr, QObject* receiver, const char* member) {
  this_ptr->open(receiver, member);
}

int qt_widgets_c_QProgressDialog_qt_metacall(QProgressDialog* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QProgressDialog_qt_metacast(QProgressDialog* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QProgressDialog_reset(QProgressDialog* this_ptr) {
  this_ptr->reset();
}

void qt_widgets_c_QProgressDialog_setAutoClose(QProgressDialog* this_ptr, bool close) {
  this_ptr->setAutoClose(close);
}

void qt_widgets_c_QProgressDialog_setAutoReset(QProgressDialog* this_ptr, bool reset) {
  this_ptr->setAutoReset(reset);
}

void qt_widgets_c_QProgressDialog_setBar(QProgressDialog* this_ptr, QProgressBar* bar) {
  this_ptr->setBar(bar);
}

void qt_widgets_c_QProgressDialog_setCancelButton(QProgressDialog* this_ptr, QPushButton* button) {
  this_ptr->setCancelButton(button);
}

void qt_widgets_c_QProgressDialog_setCancelButtonText(QProgressDialog* this_ptr, const QString* text) {
  this_ptr->setCancelButtonText(*text);
}

void qt_widgets_c_QProgressDialog_setLabel(QProgressDialog* this_ptr, QLabel* label) {
  this_ptr->setLabel(label);
}

void qt_widgets_c_QProgressDialog_setLabelText(QProgressDialog* this_ptr, const QString* text) {
  this_ptr->setLabelText(*text);
}

void qt_widgets_c_QProgressDialog_setMaximum(QProgressDialog* this_ptr, int maximum) {
  this_ptr->setMaximum(maximum);
}

void qt_widgets_c_QProgressDialog_setMinimum(QProgressDialog* this_ptr, int minimum) {
  this_ptr->setMinimum(minimum);
}

void qt_widgets_c_QProgressDialog_setMinimumDuration(QProgressDialog* this_ptr, int ms) {
  this_ptr->setMinimumDuration(ms);
}

void qt_widgets_c_QProgressDialog_setRange(QProgressDialog* this_ptr, int minimum, int maximum) {
  this_ptr->setRange(minimum, maximum);
}

void qt_widgets_c_QProgressDialog_setValue(QProgressDialog* this_ptr, int progress) {
  this_ptr->setValue(progress);
}

void qt_widgets_c_QProgressDialog_sizeHint_to_output(const QProgressDialog* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QProgressDialog_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QProgressDialog::trUtf8(s, c, n));
}

void qt_widgets_c_QProgressDialog_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QProgressDialog::tr(s, c, n));
}

int qt_widgets_c_QProgressDialog_value(const QProgressDialog* this_ptr) {
  return this_ptr->value();
}

bool qt_widgets_c_QProgressDialog_wasCanceled(const QProgressDialog* this_ptr) {
  return this_ptr->wasCanceled();
}

