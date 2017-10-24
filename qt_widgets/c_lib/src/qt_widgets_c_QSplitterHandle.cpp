#include "qt_widgets_c_QSplitterHandle.h"

QSplitterHandle* qt_widgets_c_QSplitterHandle_G_dynamic_cast_QSplitterHandle_ptr(QWidget* ptr) {
  return dynamic_cast<QSplitterHandle*>(ptr);
}

QObject* qt_widgets_c_QSplitterHandle_G_static_cast_QObject_ptr(QSplitterHandle* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QSplitterHandle_G_static_cast_QPaintDevice_ptr(QSplitterHandle* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QSplitterHandle* qt_widgets_c_QSplitterHandle_G_static_cast_QSplitterHandle_ptr_QObject(QObject* ptr) {
  return static_cast<QSplitterHandle*>(ptr);
}

QSplitterHandle* qt_widgets_c_QSplitterHandle_G_static_cast_QSplitterHandle_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QSplitterHandle*>(ptr);
}

QSplitterHandle* qt_widgets_c_QSplitterHandle_G_static_cast_QSplitterHandle_ptr_QWidget(QWidget* ptr) {
  return static_cast<QSplitterHandle*>(ptr);
}

QWidget* qt_widgets_c_QSplitterHandle_G_static_cast_QWidget_ptr(QSplitterHandle* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QSplitterHandle_delete(QSplitterHandle* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QSplitterHandle_metaObject(const QSplitterHandle* this_ptr) {
  return this_ptr->metaObject();
}

QSplitterHandle* qt_widgets_c_QSplitterHandle_new(const Qt::Orientation* o, QSplitter* parent) {
  return new QSplitterHandle(*o, parent);
}

bool qt_widgets_c_QSplitterHandle_opaqueResize(const QSplitterHandle* this_ptr) {
  return this_ptr->opaqueResize();
}

int qt_widgets_c_QSplitterHandle_qt_metacall(QSplitterHandle* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QSplitterHandle_qt_metacast(QSplitterHandle* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QSplitterHandle_setOrientation(QSplitterHandle* this_ptr, const Qt::Orientation* o) {
  this_ptr->setOrientation(*o);
}

void qt_widgets_c_QSplitterHandle_sizeHint_to_output(const QSplitterHandle* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

QSplitter* qt_widgets_c_QSplitterHandle_splitter(const QSplitterHandle* this_ptr) {
  return this_ptr->splitter();
}

void qt_widgets_c_QSplitterHandle_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSplitterHandle::trUtf8(s, c, n));
}

void qt_widgets_c_QSplitterHandle_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSplitterHandle::tr(s, c, n));
}

