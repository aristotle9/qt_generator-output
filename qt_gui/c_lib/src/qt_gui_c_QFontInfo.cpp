#include "qt_gui_c_QFontInfo.h"

void qt_gui_c_QFontInfo_G_swap(QFontInfo* value1, QFontInfo* value2) {
  swap(*value1, *value2);
}

bool qt_gui_c_QFontInfo_bold(const QFontInfo* this_ptr) {
  return this_ptr->bold();
}

void qt_gui_c_QFontInfo_constructor_QFont(const QFont* arg1, QFontInfo* output) {
  new(output) QFontInfo(*arg1);
}

void qt_gui_c_QFontInfo_constructor_QFontInfo(const QFontInfo* arg1, QFontInfo* output) {
  new(output) QFontInfo(*arg1);
}

void qt_gui_c_QFontInfo_destructor(QFontInfo* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QFontInfo_exactMatch(const QFontInfo* this_ptr) {
  return this_ptr->exactMatch();
}

void qt_gui_c_QFontInfo_family_to_output(const QFontInfo* this_ptr, QString* output) {
  new(output) QString(this_ptr->family());
}

bool qt_gui_c_QFontInfo_fixedPitch(const QFontInfo* this_ptr) {
  return this_ptr->fixedPitch();
}

bool qt_gui_c_QFontInfo_italic(const QFontInfo* this_ptr) {
  return this_ptr->italic();
}

QFontInfo* qt_gui_c_QFontInfo_operator_assign(QFontInfo* this_ptr, const QFontInfo* arg1) {
  return &this_ptr->operator=(*arg1);
}

bool qt_gui_c_QFontInfo_overline(const QFontInfo* this_ptr) {
  return this_ptr->overline();
}

int qt_gui_c_QFontInfo_pixelSize(const QFontInfo* this_ptr) {
  return this_ptr->pixelSize();
}

int qt_gui_c_QFontInfo_pointSize(const QFontInfo* this_ptr) {
  return this_ptr->pointSize();
}

double qt_gui_c_QFontInfo_pointSizeF(const QFontInfo* this_ptr) {
  return this_ptr->pointSizeF();
}

bool qt_gui_c_QFontInfo_rawMode(const QFontInfo* this_ptr) {
  return this_ptr->rawMode();
}

bool qt_gui_c_QFontInfo_strikeOut(const QFontInfo* this_ptr) {
  return this_ptr->strikeOut();
}

void qt_gui_c_QFontInfo_styleName_to_output(const QFontInfo* this_ptr, QString* output) {
  new(output) QString(this_ptr->styleName());
}

void qt_gui_c_QFontInfo_swap(QFontInfo* this_ptr, QFontInfo* other) {
  this_ptr->swap(*other);
}

bool qt_gui_c_QFontInfo_underline(const QFontInfo* this_ptr) {
  return this_ptr->underline();
}

int qt_gui_c_QFontInfo_weight(const QFontInfo* this_ptr) {
  return this_ptr->weight();
}

