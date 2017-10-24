#include "qt_gui_c_QFontMetricsF.h"

double qt_gui_c_QFontMetricsF_ascent(const QFontMetricsF* this_ptr) {
  return this_ptr->ascent();
}

double qt_gui_c_QFontMetricsF_averageCharWidth(const QFontMetricsF* this_ptr) {
  return this_ptr->averageCharWidth();
}

void qt_gui_c_QFontMetricsF_boundingRect_to_output_arg1(const QFontMetricsF* this_ptr, const QChar* arg1, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect(*arg1));
}

void qt_gui_c_QFontMetricsF_boundingRect_to_output_r_flags_string(const QFontMetricsF* this_ptr, const QRectF* r, int flags, const QString* string, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect(*r, flags, *string));
}

void qt_gui_c_QFontMetricsF_boundingRect_to_output_r_flags_string_tabstops(const QFontMetricsF* this_ptr, const QRectF* r, int flags, const QString* string, int tabstops, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect(*r, flags, *string, tabstops));
}

void qt_gui_c_QFontMetricsF_boundingRect_to_output_r_flags_string_tabstops_tabarray(const QFontMetricsF* this_ptr, const QRectF* r, int flags, const QString* string, int tabstops, int* tabarray, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect(*r, flags, *string, tabstops, tabarray));
}

void qt_gui_c_QFontMetricsF_boundingRect_to_output_string(const QFontMetricsF* this_ptr, const QString* string, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect(*string));
}

double qt_gui_c_QFontMetricsF_capHeight(const QFontMetricsF* this_ptr) {
  return this_ptr->capHeight();
}

void qt_gui_c_QFontMetricsF_constructor_QFont(const QFont* arg1, QFontMetricsF* output) {
  new(output) QFontMetricsF(*arg1);
}

void qt_gui_c_QFontMetricsF_constructor_QFontMetrics(const QFontMetrics* arg1, QFontMetricsF* output) {
  new(output) QFontMetricsF(*arg1);
}

void qt_gui_c_QFontMetricsF_constructor_QFontMetricsF(const QFontMetricsF* arg1, QFontMetricsF* output) {
  new(output) QFontMetricsF(*arg1);
}

void qt_gui_c_QFontMetricsF_constructor_QFont_QPaintDevice(const QFont* arg1, QPaintDevice* pd, QFontMetricsF* output) {
  new(output) QFontMetricsF(*arg1, pd);
}

double qt_gui_c_QFontMetricsF_descent(const QFontMetricsF* this_ptr) {
  return this_ptr->descent();
}

void qt_gui_c_QFontMetricsF_destructor(QFontMetricsF* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QFontMetricsF_elidedText_to_output_text_mode_width(const QFontMetricsF* this_ptr, const QString* text, const Qt::TextElideMode* mode, double width, QString* output) {
  new(output) QString(this_ptr->elidedText(*text, *mode, width));
}

void qt_gui_c_QFontMetricsF_elidedText_to_output_text_mode_width_flags(const QFontMetricsF* this_ptr, const QString* text, const Qt::TextElideMode* mode, double width, int flags, QString* output) {
  new(output) QString(this_ptr->elidedText(*text, *mode, width, flags));
}

double qt_gui_c_QFontMetricsF_height(const QFontMetricsF* this_ptr) {
  return this_ptr->height();
}

bool qt_gui_c_QFontMetricsF_inFont(const QFontMetricsF* this_ptr, const QChar* arg1) {
  return this_ptr->inFont(*arg1);
}

bool qt_gui_c_QFontMetricsF_inFontUcs4(const QFontMetricsF* this_ptr, unsigned int ucs4) {
  return this_ptr->inFontUcs4(ucs4);
}

double qt_gui_c_QFontMetricsF_leading(const QFontMetricsF* this_ptr) {
  return this_ptr->leading();
}

double qt_gui_c_QFontMetricsF_leftBearing(const QFontMetricsF* this_ptr, const QChar* arg1) {
  return this_ptr->leftBearing(*arg1);
}

double qt_gui_c_QFontMetricsF_lineSpacing(const QFontMetricsF* this_ptr) {
  return this_ptr->lineSpacing();
}

double qt_gui_c_QFontMetricsF_lineWidth(const QFontMetricsF* this_ptr) {
  return this_ptr->lineWidth();
}

double qt_gui_c_QFontMetricsF_maxWidth(const QFontMetricsF* this_ptr) {
  return this_ptr->maxWidth();
}

double qt_gui_c_QFontMetricsF_minLeftBearing(const QFontMetricsF* this_ptr) {
  return this_ptr->minLeftBearing();
}

double qt_gui_c_QFontMetricsF_minRightBearing(const QFontMetricsF* this_ptr) {
  return this_ptr->minRightBearing();
}

QFontMetricsF* qt_gui_c_QFontMetricsF_operator_assign_QFontMetrics(QFontMetricsF* this_ptr, const QFontMetrics* arg1) {
  return &this_ptr->operator=(*arg1);
}

QFontMetricsF* qt_gui_c_QFontMetricsF_operator_assign_QFontMetricsF(QFontMetricsF* this_ptr, const QFontMetricsF* arg1) {
  return &this_ptr->operator=(*arg1);
}

bool qt_gui_c_QFontMetricsF_operator_eq(const QFontMetricsF* this_ptr, const QFontMetricsF* other) {
  return this_ptr->operator==(*other);
}

bool qt_gui_c_QFontMetricsF_operator_neq(const QFontMetricsF* this_ptr, const QFontMetricsF* other) {
  return this_ptr->operator!=(*other);
}

double qt_gui_c_QFontMetricsF_overlinePos(const QFontMetricsF* this_ptr) {
  return this_ptr->overlinePos();
}

double qt_gui_c_QFontMetricsF_rightBearing(const QFontMetricsF* this_ptr, const QChar* arg1) {
  return this_ptr->rightBearing(*arg1);
}

void qt_gui_c_QFontMetricsF_size_to_output_flags_str(const QFontMetricsF* this_ptr, int flags, const QString* str, QSizeF* output) {
  new(output) QSizeF(this_ptr->size(flags, *str));
}

void qt_gui_c_QFontMetricsF_size_to_output_flags_str_tabstops(const QFontMetricsF* this_ptr, int flags, const QString* str, int tabstops, QSizeF* output) {
  new(output) QSizeF(this_ptr->size(flags, *str, tabstops));
}

void qt_gui_c_QFontMetricsF_size_to_output_flags_str_tabstops_tabarray(const QFontMetricsF* this_ptr, int flags, const QString* str, int tabstops, int* tabarray, QSizeF* output) {
  new(output) QSizeF(this_ptr->size(flags, *str, tabstops, tabarray));
}

double qt_gui_c_QFontMetricsF_strikeOutPos(const QFontMetricsF* this_ptr) {
  return this_ptr->strikeOutPos();
}

void qt_gui_c_QFontMetricsF_swap(QFontMetricsF* this_ptr, QFontMetricsF* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QFontMetricsF_tightBoundingRect_to_output(const QFontMetricsF* this_ptr, const QString* text, QRectF* output) {
  new(output) QRectF(this_ptr->tightBoundingRect(*text));
}

double qt_gui_c_QFontMetricsF_underlinePos(const QFontMetricsF* this_ptr) {
  return this_ptr->underlinePos();
}

double qt_gui_c_QFontMetricsF_width_arg1(const QFontMetricsF* this_ptr, const QChar* arg1) {
  return this_ptr->width(*arg1);
}

double qt_gui_c_QFontMetricsF_width_string(const QFontMetricsF* this_ptr, const QString* string) {
  return this_ptr->width(*string);
}

double qt_gui_c_QFontMetricsF_xHeight(const QFontMetricsF* this_ptr) {
  return this_ptr->xHeight();
}

