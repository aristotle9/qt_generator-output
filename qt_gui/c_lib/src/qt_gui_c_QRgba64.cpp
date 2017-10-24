#include "qt_gui_c_QRgba64.h"

unsigned int qt_gui_c_QRgba64_G_qAlpha(const QRgba64* rgb) {
  return qAlpha(*rgb);
}

unsigned int qt_gui_c_QRgba64_G_qBlue(const QRgba64* rgb) {
  return qBlue(*rgb);
}

unsigned int qt_gui_c_QRgba64_G_qGreen(const QRgba64* rgb) {
  return qGreen(*rgb);
}

void qt_gui_c_QRgba64_G_qPremultiply_to_output(const QRgba64* c, QRgba64* output) {
  new(output) QRgba64(qPremultiply(*c));
}

unsigned int qt_gui_c_QRgba64_G_qRed(const QRgba64* rgb) {
  return qRed(*rgb);
}

void qt_gui_c_QRgba64_G_qRgba64_to_output_c(quint64 c, QRgba64* output) {
  new(output) QRgba64(qRgba64(c));
}

void qt_gui_c_QRgba64_G_qRgba64_to_output_r_g_b_a(quint16 r, quint16 g, quint16 b, quint16 a, QRgba64* output) {
  new(output) QRgba64(qRgba64(r, g, b, a));
}

void qt_gui_c_QRgba64_G_qUnpremultiply_to_output(const QRgba64* c, QRgba64* output) {
  new(output) QRgba64(qUnpremultiply(*c));
}

quint16 qt_gui_c_QRgba64_alpha(const QRgba64* this_ptr) {
  return this_ptr->alpha();
}

quint8 qt_gui_c_QRgba64_alpha8(const QRgba64* this_ptr) {
  return this_ptr->alpha8();
}

quint16 qt_gui_c_QRgba64_blue(const QRgba64* this_ptr) {
  return this_ptr->blue();
}

quint8 qt_gui_c_QRgba64_blue8(const QRgba64* this_ptr) {
  return this_ptr->blue8();
}

void qt_gui_c_QRgba64_constructor(QRgba64* output) {
  new(output) QRgba64();
}

quint64 qt_gui_c_QRgba64_convert_to_unsigned_long_long(const QRgba64* this_ptr) {
  return this_ptr->operator unsigned long long();
}

void qt_gui_c_QRgba64_destructor(QRgba64* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QRgba64_fromArgb32_to_output(unsigned int rgb, QRgba64* output) {
  new(output) QRgba64(QRgba64::fromArgb32(rgb));
}

void qt_gui_c_QRgba64_fromRgba64_to_output_c(quint64 c, QRgba64* output) {
  new(output) QRgba64(QRgba64::fromRgba64(c));
}

void qt_gui_c_QRgba64_fromRgba64_to_output_red_green_blue_alpha(quint16 red, quint16 green, quint16 blue, quint16 alpha, QRgba64* output) {
  new(output) QRgba64(QRgba64::fromRgba64(red, green, blue, alpha));
}

void qt_gui_c_QRgba64_fromRgba_to_output(quint8 red, quint8 green, quint8 blue, quint8 alpha, QRgba64* output) {
  new(output) QRgba64(QRgba64::fromRgba(red, green, blue, alpha));
}

quint16 qt_gui_c_QRgba64_green(const QRgba64* this_ptr) {
  return this_ptr->green();
}

quint8 qt_gui_c_QRgba64_green8(const QRgba64* this_ptr) {
  return this_ptr->green8();
}

bool qt_gui_c_QRgba64_isOpaque(const QRgba64* this_ptr) {
  return this_ptr->isOpaque();
}

bool qt_gui_c_QRgba64_isTransparent(const QRgba64* this_ptr) {
  return this_ptr->isTransparent();
}

void qt_gui_c_QRgba64_operator_assign_to_output(QRgba64* this_ptr, quint64 _rgba, QRgba64* output) {
  new(output) QRgba64(this_ptr->operator=(_rgba));
}

void qt_gui_c_QRgba64_premultiplied_to_output(const QRgba64* this_ptr, QRgba64* output) {
  new(output) QRgba64(this_ptr->premultiplied());
}

quint16 qt_gui_c_QRgba64_red(const QRgba64* this_ptr) {
  return this_ptr->red();
}

quint8 qt_gui_c_QRgba64_red8(const QRgba64* this_ptr) {
  return this_ptr->red8();
}

void qt_gui_c_QRgba64_setAlpha(QRgba64* this_ptr, quint16 _alpha) {
  this_ptr->setAlpha(_alpha);
}

void qt_gui_c_QRgba64_setBlue(QRgba64* this_ptr, quint16 _blue) {
  this_ptr->setBlue(_blue);
}

void qt_gui_c_QRgba64_setGreen(QRgba64* this_ptr, quint16 _green) {
  this_ptr->setGreen(_green);
}

void qt_gui_c_QRgba64_setRed(QRgba64* this_ptr, quint16 _red) {
  this_ptr->setRed(_red);
}

unsigned int qt_gui_c_QRgba64_toArgb32(const QRgba64* this_ptr) {
  return this_ptr->toArgb32();
}

unsigned short qt_gui_c_QRgba64_toRgb16(const QRgba64* this_ptr) {
  return this_ptr->toRgb16();
}

void qt_gui_c_QRgba64_unpremultiplied_to_output(const QRgba64* this_ptr, QRgba64* output) {
  new(output) QRgba64(this_ptr->unpremultiplied());
}

