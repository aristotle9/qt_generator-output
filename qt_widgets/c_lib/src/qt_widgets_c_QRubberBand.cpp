#include "qt_widgets_c_QRubberBand.h"

QRubberBand* qt_widgets_c_QRubberBand_G_dynamic_cast_QRubberBand_ptr(QWidget* ptr) {
  return dynamic_cast<QRubberBand*>(ptr);
}

QObject* qt_widgets_c_QRubberBand_G_static_cast_QObject_ptr(QRubberBand* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QRubberBand_G_static_cast_QPaintDevice_ptr(QRubberBand* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QRubberBand* qt_widgets_c_QRubberBand_G_static_cast_QRubberBand_ptr_QObject(QObject* ptr) {
  return static_cast<QRubberBand*>(ptr);
}

QRubberBand* qt_widgets_c_QRubberBand_G_static_cast_QRubberBand_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QRubberBand*>(ptr);
}

QRubberBand* qt_widgets_c_QRubberBand_G_static_cast_QRubberBand_ptr_QWidget(QWidget* ptr) {
  return static_cast<QRubberBand*>(ptr);
}

QWidget* qt_widgets_c_QRubberBand_G_static_cast_QWidget_ptr(QRubberBand* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QRubberBand_delete(QRubberBand* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QRubberBand_metaObject(const QRubberBand* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QRubberBand_move_p(QRubberBand* this_ptr, const QPoint* p) {
  this_ptr->move(*p);
}

void qt_widgets_c_QRubberBand_move_x_y(QRubberBand* this_ptr, int x, int y) {
  this_ptr->move(x, y);
}

QRubberBand* qt_widgets_c_QRubberBand_new_arg1(QRubberBand::Shape arg1) {
  return new QRubberBand(arg1);
}

QRubberBand* qt_widgets_c_QRubberBand_new_arg1_arg2(QRubberBand::Shape arg1, QWidget* arg2) {
  return new QRubberBand(arg1, arg2);
}

int qt_widgets_c_QRubberBand_qt_metacall(QRubberBand* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QRubberBand_qt_metacast(QRubberBand* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QRubberBand_resize_s(QRubberBand* this_ptr, const QSize* s) {
  this_ptr->resize(*s);
}

void qt_widgets_c_QRubberBand_resize_w_h(QRubberBand* this_ptr, int w, int h) {
  this_ptr->resize(w, h);
}

void qt_widgets_c_QRubberBand_setGeometry_r(QRubberBand* this_ptr, const QRect* r) {
  this_ptr->setGeometry(*r);
}

void qt_widgets_c_QRubberBand_setGeometry_x_y_w_h(QRubberBand* this_ptr, int x, int y, int w, int h) {
  this_ptr->setGeometry(x, y, w, h);
}

QRubberBand::Shape qt_widgets_c_QRubberBand_shape(const QRubberBand* this_ptr) {
  return this_ptr->shape();
}

void qt_widgets_c_QRubberBand_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QRubberBand::trUtf8(s, c, n));
}

void qt_widgets_c_QRubberBand_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QRubberBand::tr(s, c, n));
}

