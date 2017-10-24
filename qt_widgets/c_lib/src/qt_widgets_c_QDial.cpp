#include "qt_widgets_c_QDial.h"

QDial* qt_widgets_c_QDial_G_dynamic_cast_QDial_ptr_QAbstractSlider(QAbstractSlider* ptr) {
  return dynamic_cast<QDial*>(ptr);
}

QDial* qt_widgets_c_QDial_G_dynamic_cast_QDial_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QDial*>(ptr);
}

QAbstractSlider* qt_widgets_c_QDial_G_static_cast_QAbstractSlider_ptr(QDial* ptr) {
  return static_cast<QAbstractSlider*>(ptr);
}

QDial* qt_widgets_c_QDial_G_static_cast_QDial_ptr_QAbstractSlider(QAbstractSlider* ptr) {
  return static_cast<QDial*>(ptr);
}

QDial* qt_widgets_c_QDial_G_static_cast_QDial_ptr_QObject(QObject* ptr) {
  return static_cast<QDial*>(ptr);
}

QDial* qt_widgets_c_QDial_G_static_cast_QDial_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QDial*>(ptr);
}

QDial* qt_widgets_c_QDial_G_static_cast_QDial_ptr_QWidget(QWidget* ptr) {
  return static_cast<QDial*>(ptr);
}

QObject* qt_widgets_c_QDial_G_static_cast_QObject_ptr(QDial* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QDial_G_static_cast_QPaintDevice_ptr(QDial* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QDial_G_static_cast_QWidget_ptr(QDial* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QDial_delete(QDial* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QDial_metaObject(const QDial* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QDial_minimumSizeHint_to_output(const QDial* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSizeHint());
}

QDial* qt_widgets_c_QDial_new_no_args() {
  return new QDial();
}

QDial* qt_widgets_c_QDial_new_parent(QWidget* parent) {
  return new QDial(parent);
}

int qt_widgets_c_QDial_notchSize(const QDial* this_ptr) {
  return this_ptr->notchSize();
}

double qt_widgets_c_QDial_notchTarget(const QDial* this_ptr) {
  return this_ptr->notchTarget();
}

bool qt_widgets_c_QDial_notchesVisible(const QDial* this_ptr) {
  return this_ptr->notchesVisible();
}

int qt_widgets_c_QDial_qt_metacall(QDial* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QDial_qt_metacast(QDial* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QDial_setNotchTarget(QDial* this_ptr, double target) {
  this_ptr->setNotchTarget(target);
}

void qt_widgets_c_QDial_setNotchesVisible(QDial* this_ptr, bool visible) {
  this_ptr->setNotchesVisible(visible);
}

void qt_widgets_c_QDial_setWrapping(QDial* this_ptr, bool on) {
  this_ptr->setWrapping(on);
}

void qt_widgets_c_QDial_sizeHint_to_output(const QDial* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QDial_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDial::trUtf8(s, c, n));
}

void qt_widgets_c_QDial_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDial::tr(s, c, n));
}

bool qt_widgets_c_QDial_wrapping(const QDial* this_ptr) {
  return this_ptr->wrapping();
}

