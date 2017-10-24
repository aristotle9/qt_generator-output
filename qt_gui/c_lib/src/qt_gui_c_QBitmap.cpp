#include "qt_gui_c_QBitmap.h"

QBitmap* qt_gui_c_QBitmap_G_dynamic_cast_QBitmap_ptr_QPaintDevice(QPaintDevice* ptr) {
  return dynamic_cast<QBitmap*>(ptr);
}

QBitmap* qt_gui_c_QBitmap_G_dynamic_cast_QBitmap_ptr_QPixmap(QPixmap* ptr) {
  return dynamic_cast<QBitmap*>(ptr);
}

QBitmap* qt_gui_c_QBitmap_G_static_cast_QBitmap_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QBitmap*>(ptr);
}

QBitmap* qt_gui_c_QBitmap_G_static_cast_QBitmap_ptr_QPixmap(QPixmap* ptr) {
  return static_cast<QBitmap*>(ptr);
}

QPaintDevice* qt_gui_c_QBitmap_G_static_cast_QPaintDevice_ptr(QBitmap* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QPixmap* qt_gui_c_QBitmap_G_static_cast_QPixmap_ptr(QBitmap* ptr) {
  return static_cast<QPixmap*>(ptr);
}

void qt_gui_c_QBitmap_G_swap(QBitmap* value1, QBitmap* value2) {
  swap(*value1, *value2);
}

void qt_gui_c_QBitmap_clear(QBitmap* this_ptr) {
  this_ptr->clear();
}

void qt_gui_c_QBitmap_convert_to_QVariant_to_output(const QBitmap* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

void qt_gui_c_QBitmap_delete(QBitmap* this_ptr) {
  delete this_ptr;
}

QBitmap* qt_gui_c_QBitmap_fromData_as_ptr_size_bits(const QSize* size, const unsigned char* bits) {
  return new QBitmap(QBitmap::fromData(*size, bits));
}

QBitmap* qt_gui_c_QBitmap_fromData_as_ptr_size_bits_monoFormat(const QSize* size, const unsigned char* bits, const QImage::Format* monoFormat) {
  return new QBitmap(QBitmap::fromData(*size, bits, *monoFormat));
}

QBitmap* qt_gui_c_QBitmap_new_QBitmap(const QBitmap* other) {
  return new QBitmap(*other);
}

QBitmap* qt_gui_c_QBitmap_new_QPixmap(const QPixmap* arg1) {
  return new QBitmap(*arg1);
}

QBitmap* qt_gui_c_QBitmap_new_QSize(const QSize* arg1) {
  return new QBitmap(*arg1);
}

QBitmap* qt_gui_c_QBitmap_new_QString(const QString* fileName) {
  return new QBitmap(*fileName);
}

QBitmap* qt_gui_c_QBitmap_new_QString_char(const QString* fileName, const char* format) {
  return new QBitmap(*fileName, format);
}

QBitmap* qt_gui_c_QBitmap_new_int_int(int w, int h) {
  return new QBitmap(w, h);
}

QBitmap* qt_gui_c_QBitmap_new_no_args() {
  return new QBitmap();
}

QBitmap* qt_gui_c_QBitmap_operator_assign_arg1(QBitmap* this_ptr, const QPixmap* arg1) {
  return &this_ptr->operator=(*arg1);
}

QBitmap* qt_gui_c_QBitmap_operator_assign_other(QBitmap* this_ptr, const QBitmap* other) {
  return &this_ptr->operator=(*other);
}

void qt_gui_c_QBitmap_swap(QBitmap* this_ptr, QBitmap* other) {
  this_ptr->swap(*other);
}

QBitmap* qt_gui_c_QBitmap_transformed_as_ptr_arg1(const QBitmap* this_ptr, const QMatrix* arg1) {
  return new QBitmap(this_ptr->transformed(*arg1));
}

QBitmap* qt_gui_c_QBitmap_transformed_as_ptr_matrix(const QBitmap* this_ptr, const QTransform* matrix) {
  return new QBitmap(this_ptr->transformed(*matrix));
}

