#include "qt_widgets_c_QSlider.h"

QSlider* qt_widgets_c_QSlider_G_dynamic_cast_QSlider_ptr_QAbstractSlider(QAbstractSlider* ptr) {
  return dynamic_cast<QSlider*>(ptr);
}

QSlider* qt_widgets_c_QSlider_G_dynamic_cast_QSlider_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QSlider*>(ptr);
}

QAbstractSlider* qt_widgets_c_QSlider_G_static_cast_QAbstractSlider_ptr(QSlider* ptr) {
  return static_cast<QAbstractSlider*>(ptr);
}

QObject* qt_widgets_c_QSlider_G_static_cast_QObject_ptr(QSlider* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QSlider_G_static_cast_QPaintDevice_ptr(QSlider* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QSlider* qt_widgets_c_QSlider_G_static_cast_QSlider_ptr_QAbstractSlider(QAbstractSlider* ptr) {
  return static_cast<QSlider*>(ptr);
}

QSlider* qt_widgets_c_QSlider_G_static_cast_QSlider_ptr_QObject(QObject* ptr) {
  return static_cast<QSlider*>(ptr);
}

QSlider* qt_widgets_c_QSlider_G_static_cast_QSlider_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QSlider*>(ptr);
}

QSlider* qt_widgets_c_QSlider_G_static_cast_QSlider_ptr_QWidget(QWidget* ptr) {
  return static_cast<QSlider*>(ptr);
}

QWidget* qt_widgets_c_QSlider_G_static_cast_QWidget_ptr(QSlider* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QSlider_delete(QSlider* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QSlider_event(QSlider* this_ptr, QEvent* event) {
  return this_ptr->event(event);
}

const QMetaObject* qt_widgets_c_QSlider_metaObject(const QSlider* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QSlider_minimumSizeHint_to_output(const QSlider* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSizeHint());
}

QSlider* qt_widgets_c_QSlider_new_no_args() {
  return new QSlider();
}

QSlider* qt_widgets_c_QSlider_new_orientation(const Qt::Orientation* orientation) {
  return new QSlider(*orientation);
}

QSlider* qt_widgets_c_QSlider_new_orientation_parent(const Qt::Orientation* orientation, QWidget* parent) {
  return new QSlider(*orientation, parent);
}

QSlider* qt_widgets_c_QSlider_new_parent(QWidget* parent) {
  return new QSlider(parent);
}

int qt_widgets_c_QSlider_qt_metacall(QSlider* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QSlider_qt_metacast(QSlider* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QSlider_setTickInterval(QSlider* this_ptr, int ti) {
  this_ptr->setTickInterval(ti);
}

void qt_widgets_c_QSlider_setTickPosition(QSlider* this_ptr, QSlider::TickPosition position) {
  this_ptr->setTickPosition(position);
}

void qt_widgets_c_QSlider_sizeHint_to_output(const QSlider* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

int qt_widgets_c_QSlider_tickInterval(const QSlider* this_ptr) {
  return this_ptr->tickInterval();
}

QSlider::TickPosition qt_widgets_c_QSlider_tickPosition(const QSlider* this_ptr) {
  return this_ptr->tickPosition();
}

void qt_widgets_c_QSlider_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSlider::trUtf8(s, c, n));
}

void qt_widgets_c_QSlider_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSlider::tr(s, c, n));
}

