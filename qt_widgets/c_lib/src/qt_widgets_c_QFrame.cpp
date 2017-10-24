#include "qt_widgets_c_QFrame.h"

QFrame* qt_widgets_c_QFrame_G_dynamic_cast_QFrame_ptr(QWidget* ptr) {
  return dynamic_cast<QFrame*>(ptr);
}

QFrame* qt_widgets_c_QFrame_G_static_cast_QFrame_ptr_QObject(QObject* ptr) {
  return static_cast<QFrame*>(ptr);
}

QFrame* qt_widgets_c_QFrame_G_static_cast_QFrame_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QFrame*>(ptr);
}

QFrame* qt_widgets_c_QFrame_G_static_cast_QFrame_ptr_QWidget(QWidget* ptr) {
  return static_cast<QFrame*>(ptr);
}

QObject* qt_widgets_c_QFrame_G_static_cast_QObject_ptr(QFrame* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QFrame_G_static_cast_QPaintDevice_ptr(QFrame* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QFrame_G_static_cast_QWidget_ptr(QFrame* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QFrame_delete(QFrame* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QFrame_frameRect_to_output(const QFrame* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->frameRect());
}

QFrame::Shadow qt_widgets_c_QFrame_frameShadow(const QFrame* this_ptr) {
  return this_ptr->frameShadow();
}

QFrame::Shape qt_widgets_c_QFrame_frameShape(const QFrame* this_ptr) {
  return this_ptr->frameShape();
}

int qt_widgets_c_QFrame_frameStyle(const QFrame* this_ptr) {
  return this_ptr->frameStyle();
}

int qt_widgets_c_QFrame_frameWidth(const QFrame* this_ptr) {
  return this_ptr->frameWidth();
}

int qt_widgets_c_QFrame_lineWidth(const QFrame* this_ptr) {
  return this_ptr->lineWidth();
}

const QMetaObject* qt_widgets_c_QFrame_metaObject(const QFrame* this_ptr) {
  return this_ptr->metaObject();
}

int qt_widgets_c_QFrame_midLineWidth(const QFrame* this_ptr) {
  return this_ptr->midLineWidth();
}

int qt_widgets_c_QFrame_qt_metacall(QFrame* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QFrame_qt_metacast(QFrame* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QFrame_setFrameRect(QFrame* this_ptr, const QRect* arg1) {
  this_ptr->setFrameRect(*arg1);
}

void qt_widgets_c_QFrame_setFrameShadow(QFrame* this_ptr, QFrame::Shadow arg1) {
  this_ptr->setFrameShadow(arg1);
}

void qt_widgets_c_QFrame_setFrameShape(QFrame* this_ptr, QFrame::Shape arg1) {
  this_ptr->setFrameShape(arg1);
}

void qt_widgets_c_QFrame_setFrameStyle(QFrame* this_ptr, int arg1) {
  this_ptr->setFrameStyle(arg1);
}

void qt_widgets_c_QFrame_setLineWidth(QFrame* this_ptr, int arg1) {
  this_ptr->setLineWidth(arg1);
}

void qt_widgets_c_QFrame_setMidLineWidth(QFrame* this_ptr, int arg1) {
  this_ptr->setMidLineWidth(arg1);
}

void qt_widgets_c_QFrame_sizeHint_to_output(const QFrame* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QFrame_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFrame::trUtf8(s, c, n));
}

void qt_widgets_c_QFrame_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFrame::tr(s, c, n));
}

