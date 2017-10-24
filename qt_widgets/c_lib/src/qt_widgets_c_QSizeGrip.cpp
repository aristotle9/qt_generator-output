#include "qt_widgets_c_QSizeGrip.h"

QSizeGrip* qt_widgets_c_QSizeGrip_G_dynamic_cast_QSizeGrip_ptr(QWidget* ptr) {
  return dynamic_cast<QSizeGrip*>(ptr);
}

QObject* qt_widgets_c_QSizeGrip_G_static_cast_QObject_ptr(QSizeGrip* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QSizeGrip_G_static_cast_QPaintDevice_ptr(QSizeGrip* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QSizeGrip* qt_widgets_c_QSizeGrip_G_static_cast_QSizeGrip_ptr_QObject(QObject* ptr) {
  return static_cast<QSizeGrip*>(ptr);
}

QSizeGrip* qt_widgets_c_QSizeGrip_G_static_cast_QSizeGrip_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QSizeGrip*>(ptr);
}

QSizeGrip* qt_widgets_c_QSizeGrip_G_static_cast_QSizeGrip_ptr_QWidget(QWidget* ptr) {
  return static_cast<QSizeGrip*>(ptr);
}

QWidget* qt_widgets_c_QSizeGrip_G_static_cast_QWidget_ptr(QSizeGrip* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QSizeGrip_delete(QSizeGrip* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QSizeGrip_metaObject(const QSizeGrip* this_ptr) {
  return this_ptr->metaObject();
}

QSizeGrip* qt_widgets_c_QSizeGrip_new(QWidget* parent) {
  return new QSizeGrip(parent);
}

int qt_widgets_c_QSizeGrip_qt_metacall(QSizeGrip* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QSizeGrip_qt_metacast(QSizeGrip* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QSizeGrip_setVisible(QSizeGrip* this_ptr, bool arg1) {
  this_ptr->setVisible(arg1);
}

void qt_widgets_c_QSizeGrip_sizeHint_to_output(const QSizeGrip* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QSizeGrip_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSizeGrip::trUtf8(s, c, n));
}

void qt_widgets_c_QSizeGrip_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSizeGrip::tr(s, c, n));
}

