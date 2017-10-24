#include "qt_gui_c_QFontMetrics.h"

void qt_gui_c_QFontMetrics_G_swap_QFontMetricsF_QFontMetricsF(QFontMetricsF* value1, QFontMetricsF* value2) {
  swap(*value1, *value2);
}

void qt_gui_c_QFontMetrics_G_swap_QFontMetrics_QFontMetrics(QFontMetrics* value1, QFontMetrics* value2) {
  swap(*value1, *value2);
}

int qt_gui_c_QFontMetrics_ascent(const QFontMetrics* this_ptr) {
  return this_ptr->ascent();
}

int qt_gui_c_QFontMetrics_averageCharWidth(const QFontMetrics* this_ptr) {
  return this_ptr->averageCharWidth();
}

void qt_gui_c_QFontMetrics_boundingRect_to_output_arg1(const QFontMetrics* this_ptr, const QChar* arg1, QRect* output) {
  new(output) QRect(this_ptr->boundingRect(*arg1));
}

void qt_gui_c_QFontMetrics_boundingRect_to_output_r_flags_text(const QFontMetrics* this_ptr, const QRect* r, int flags, const QString* text, QRect* output) {
  new(output) QRect(this_ptr->boundingRect(*r, flags, *text));
}

void qt_gui_c_QFontMetrics_boundingRect_to_output_r_flags_text_tabstops(const QFontMetrics* this_ptr, const QRect* r, int flags, const QString* text, int tabstops, QRect* output) {
  new(output) QRect(this_ptr->boundingRect(*r, flags, *text, tabstops));
}

void qt_gui_c_QFontMetrics_boundingRect_to_output_r_flags_text_tabstops_tabarray(const QFontMetrics* this_ptr, const QRect* r, int flags, const QString* text, int tabstops, int* tabarray, QRect* output) {
  new(output) QRect(this_ptr->boundingRect(*r, flags, *text, tabstops, tabarray));
}

void qt_gui_c_QFontMetrics_boundingRect_to_output_text(const QFontMetrics* this_ptr, const QString* text, QRect* output) {
  new(output) QRect(this_ptr->boundingRect(*text));
}

void qt_gui_c_QFontMetrics_boundingRect_to_output_x_y_w_h_flags_text(const QFontMetrics* this_ptr, int x, int y, int w, int h, int flags, const QString* text, QRect* output) {
  new(output) QRect(this_ptr->boundingRect(x, y, w, h, flags, *text));
}

void qt_gui_c_QFontMetrics_boundingRect_to_output_x_y_w_h_flags_text_tabstops(const QFontMetrics* this_ptr, int x, int y, int w, int h, int flags, const QString* text, int tabstops, QRect* output) {
  new(output) QRect(this_ptr->boundingRect(x, y, w, h, flags, *text, tabstops));
}

void qt_gui_c_QFontMetrics_boundingRect_to_output_x_y_w_h_flags_text_tabstops_tabarray(const QFontMetrics* this_ptr, int x, int y, int w, int h, int flags, const QString* text, int tabstops, int* tabarray, QRect* output) {
  new(output) QRect(this_ptr->boundingRect(x, y, w, h, flags, *text, tabstops, tabarray));
}

int qt_gui_c_QFontMetrics_capHeight(const QFontMetrics* this_ptr) {
  return this_ptr->capHeight();
}

int qt_gui_c_QFontMetrics_charWidth(const QFontMetrics* this_ptr, const QString* str, int pos) {
  return this_ptr->charWidth(*str, pos);
}

void qt_gui_c_QFontMetrics_constructor_QFont(const QFont* arg1, QFontMetrics* output) {
  new(output) QFontMetrics(*arg1);
}

void qt_gui_c_QFontMetrics_constructor_QFontMetrics(const QFontMetrics* arg1, QFontMetrics* output) {
  new(output) QFontMetrics(*arg1);
}

void qt_gui_c_QFontMetrics_constructor_QFont_QPaintDevice(const QFont* arg1, QPaintDevice* pd, QFontMetrics* output) {
  new(output) QFontMetrics(*arg1, pd);
}

int qt_gui_c_QFontMetrics_descent(const QFontMetrics* this_ptr) {
  return this_ptr->descent();
}

void qt_gui_c_QFontMetrics_destructor(QFontMetrics* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QFontMetrics_elidedText_to_output_text_mode_width(const QFontMetrics* this_ptr, const QString* text, const Qt::TextElideMode* mode, int width, QString* output) {
  new(output) QString(this_ptr->elidedText(*text, *mode, width));
}

void qt_gui_c_QFontMetrics_elidedText_to_output_text_mode_width_flags(const QFontMetrics* this_ptr, const QString* text, const Qt::TextElideMode* mode, int width, int flags, QString* output) {
  new(output) QString(this_ptr->elidedText(*text, *mode, width, flags));
}

int qt_gui_c_QFontMetrics_height(const QFontMetrics* this_ptr) {
  return this_ptr->height();
}

bool qt_gui_c_QFontMetrics_inFont(const QFontMetrics* this_ptr, const QChar* arg1) {
  return this_ptr->inFont(*arg1);
}

bool qt_gui_c_QFontMetrics_inFontUcs4(const QFontMetrics* this_ptr, unsigned int ucs4) {
  return this_ptr->inFontUcs4(ucs4);
}

int qt_gui_c_QFontMetrics_leading(const QFontMetrics* this_ptr) {
  return this_ptr->leading();
}

int qt_gui_c_QFontMetrics_leftBearing(const QFontMetrics* this_ptr, const QChar* arg1) {
  return this_ptr->leftBearing(*arg1);
}

int qt_gui_c_QFontMetrics_lineSpacing(const QFontMetrics* this_ptr) {
  return this_ptr->lineSpacing();
}

int qt_gui_c_QFontMetrics_lineWidth(const QFontMetrics* this_ptr) {
  return this_ptr->lineWidth();
}

int qt_gui_c_QFontMetrics_maxWidth(const QFontMetrics* this_ptr) {
  return this_ptr->maxWidth();
}

int qt_gui_c_QFontMetrics_minLeftBearing(const QFontMetrics* this_ptr) {
  return this_ptr->minLeftBearing();
}

int qt_gui_c_QFontMetrics_minRightBearing(const QFontMetrics* this_ptr) {
  return this_ptr->minRightBearing();
}

QFontMetrics* qt_gui_c_QFontMetrics_operator_assign(QFontMetrics* this_ptr, const QFontMetrics* arg1) {
  return &this_ptr->operator=(*arg1);
}

bool qt_gui_c_QFontMetrics_operator_eq(const QFontMetrics* this_ptr, const QFontMetrics* other) {
  return this_ptr->operator==(*other);
}

bool qt_gui_c_QFontMetrics_operator_neq(const QFontMetrics* this_ptr, const QFontMetrics* other) {
  return this_ptr->operator!=(*other);
}

int qt_gui_c_QFontMetrics_overlinePos(const QFontMetrics* this_ptr) {
  return this_ptr->overlinePos();
}

int qt_gui_c_QFontMetrics_rightBearing(const QFontMetrics* this_ptr, const QChar* arg1) {
  return this_ptr->rightBearing(*arg1);
}

void qt_gui_c_QFontMetrics_size_to_output_flags_str(const QFontMetrics* this_ptr, int flags, const QString* str, QSize* output) {
  new(output) QSize(this_ptr->size(flags, *str));
}

void qt_gui_c_QFontMetrics_size_to_output_flags_str_tabstops(const QFontMetrics* this_ptr, int flags, const QString* str, int tabstops, QSize* output) {
  new(output) QSize(this_ptr->size(flags, *str, tabstops));
}

void qt_gui_c_QFontMetrics_size_to_output_flags_str_tabstops_tabarray(const QFontMetrics* this_ptr, int flags, const QString* str, int tabstops, int* tabarray, QSize* output) {
  new(output) QSize(this_ptr->size(flags, *str, tabstops, tabarray));
}

int qt_gui_c_QFontMetrics_strikeOutPos(const QFontMetrics* this_ptr) {
  return this_ptr->strikeOutPos();
}

void qt_gui_c_QFontMetrics_swap(QFontMetrics* this_ptr, QFontMetrics* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QFontMetrics_tightBoundingRect_to_output(const QFontMetrics* this_ptr, const QString* text, QRect* output) {
  new(output) QRect(this_ptr->tightBoundingRect(*text));
}

int qt_gui_c_QFontMetrics_underlinePos(const QFontMetrics* this_ptr) {
  return this_ptr->underlinePos();
}

int qt_gui_c_QFontMetrics_width_QChar(const QFontMetrics* this_ptr, const QChar* arg1) {
  return this_ptr->width(*arg1);
}

int qt_gui_c_QFontMetrics_width_QString(const QFontMetrics* this_ptr, const QString* arg1) {
  return this_ptr->width(*arg1);
}

int qt_gui_c_QFontMetrics_width_QString_int(const QFontMetrics* this_ptr, const QString* arg1, int len) {
  return this_ptr->width(*arg1, len);
}

int qt_gui_c_QFontMetrics_width_QString_int_int(const QFontMetrics* this_ptr, const QString* arg1, int len, int flags) {
  return this_ptr->width(*arg1, len, flags);
}

int qt_gui_c_QFontMetrics_xHeight(const QFontMetrics* this_ptr) {
  return this_ptr->xHeight();
}

