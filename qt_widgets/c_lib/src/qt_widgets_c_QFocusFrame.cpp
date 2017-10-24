#include "qt_widgets_c_QFocusFrame.h"

QFocusFrame* qt_widgets_c_QFocusFrame_G_dynamic_cast_QFocusFrame_ptr(QWidget* ptr) {
  return dynamic_cast<QFocusFrame*>(ptr);
}

QFocusFrame* qt_widgets_c_QFocusFrame_G_static_cast_QFocusFrame_ptr_QObject(QObject* ptr) {
  return static_cast<QFocusFrame*>(ptr);
}

QFocusFrame* qt_widgets_c_QFocusFrame_G_static_cast_QFocusFrame_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QFocusFrame*>(ptr);
}

QFocusFrame* qt_widgets_c_QFocusFrame_G_static_cast_QFocusFrame_ptr_QWidget(QWidget* ptr) {
  return static_cast<QFocusFrame*>(ptr);
}

QObject* qt_widgets_c_QFocusFrame_G_static_cast_QObject_ptr(QFocusFrame* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QFocusFrame_G_static_cast_QPaintDevice_ptr(QFocusFrame* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QFocusFrame_G_static_cast_QWidget_ptr(QFocusFrame* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QFocusFrame_delete(QFocusFrame* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QFocusFrame_metaObject(const QFocusFrame* this_ptr) {
  return this_ptr->metaObject();
}

QFocusFrame* qt_widgets_c_QFocusFrame_new_no_args() {
  return new QFocusFrame();
}

QFocusFrame* qt_widgets_c_QFocusFrame_new_parent(QWidget* parent) {
  return new QFocusFrame(parent);
}

int qt_widgets_c_QFocusFrame_qt_metacall(QFocusFrame* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QFocusFrame_qt_metacast(QFocusFrame* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QFocusFrame_setWidget(QFocusFrame* this_ptr, QWidget* widget) {
  this_ptr->setWidget(widget);
}

void qt_widgets_c_QFocusFrame_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFocusFrame::trUtf8(s, c, n));
}

void qt_widgets_c_QFocusFrame_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFocusFrame::tr(s, c, n));
}

QWidget* qt_widgets_c_QFocusFrame_widget(const QFocusFrame* this_ptr) {
  return this_ptr->widget();
}

