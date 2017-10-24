#include "qt_widgets_c_QScrollBar.h"

QScrollBar* qt_widgets_c_QScrollBar_G_dynamic_cast_QScrollBar_ptr_QAbstractSlider(QAbstractSlider* ptr) {
  return dynamic_cast<QScrollBar*>(ptr);
}

QScrollBar* qt_widgets_c_QScrollBar_G_dynamic_cast_QScrollBar_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QScrollBar*>(ptr);
}

QAbstractSlider* qt_widgets_c_QScrollBar_G_static_cast_QAbstractSlider_ptr(QScrollBar* ptr) {
  return static_cast<QAbstractSlider*>(ptr);
}

QObject* qt_widgets_c_QScrollBar_G_static_cast_QObject_ptr(QScrollBar* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QScrollBar_G_static_cast_QPaintDevice_ptr(QScrollBar* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QScrollBar* qt_widgets_c_QScrollBar_G_static_cast_QScrollBar_ptr_QAbstractSlider(QAbstractSlider* ptr) {
  return static_cast<QScrollBar*>(ptr);
}

QScrollBar* qt_widgets_c_QScrollBar_G_static_cast_QScrollBar_ptr_QObject(QObject* ptr) {
  return static_cast<QScrollBar*>(ptr);
}

QScrollBar* qt_widgets_c_QScrollBar_G_static_cast_QScrollBar_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QScrollBar*>(ptr);
}

QScrollBar* qt_widgets_c_QScrollBar_G_static_cast_QScrollBar_ptr_QWidget(QWidget* ptr) {
  return static_cast<QScrollBar*>(ptr);
}

QWidget* qt_widgets_c_QScrollBar_G_static_cast_QWidget_ptr(QScrollBar* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QScrollBar_delete(QScrollBar* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QScrollBar_event(QScrollBar* this_ptr, QEvent* event) {
  return this_ptr->event(event);
}

const QMetaObject* qt_widgets_c_QScrollBar_metaObject(const QScrollBar* this_ptr) {
  return this_ptr->metaObject();
}

QScrollBar* qt_widgets_c_QScrollBar_new_arg1(const Qt::Orientation* arg1) {
  return new QScrollBar(*arg1);
}

QScrollBar* qt_widgets_c_QScrollBar_new_arg1_parent(const Qt::Orientation* arg1, QWidget* parent) {
  return new QScrollBar(*arg1, parent);
}

QScrollBar* qt_widgets_c_QScrollBar_new_no_args() {
  return new QScrollBar();
}

QScrollBar* qt_widgets_c_QScrollBar_new_parent(QWidget* parent) {
  return new QScrollBar(parent);
}

int qt_widgets_c_QScrollBar_qt_metacall(QScrollBar* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QScrollBar_qt_metacast(QScrollBar* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QScrollBar_sizeHint_to_output(const QScrollBar* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QScrollBar_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QScrollBar::trUtf8(s, c, n));
}

void qt_widgets_c_QScrollBar_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QScrollBar::tr(s, c, n));
}

