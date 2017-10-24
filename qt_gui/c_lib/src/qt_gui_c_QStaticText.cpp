#include "qt_gui_c_QStaticText.h"

void qt_gui_c_QStaticText_G_swap(QStaticText* value1, QStaticText* value2) {
  swap(*value1, *value2);
}

void qt_gui_c_QStaticText_constructor_no_args(QStaticText* output) {
  new(output) QStaticText();
}

void qt_gui_c_QStaticText_constructor_other(const QStaticText* other, QStaticText* output) {
  new(output) QStaticText(*other);
}

void qt_gui_c_QStaticText_constructor_text(const QString* text, QStaticText* output) {
  new(output) QStaticText(*text);
}

void qt_gui_c_QStaticText_destructor(QStaticText* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

QStaticText* qt_gui_c_QStaticText_operator_assign(QStaticText* this_ptr, const QStaticText* arg1) {
  return &this_ptr->operator=(*arg1);
}

bool qt_gui_c_QStaticText_operator_eq(const QStaticText* this_ptr, const QStaticText* arg1) {
  return this_ptr->operator==(*arg1);
}

bool qt_gui_c_QStaticText_operator_neq(const QStaticText* this_ptr, const QStaticText* arg1) {
  return this_ptr->operator!=(*arg1);
}

QStaticText::PerformanceHint qt_gui_c_QStaticText_performanceHint(const QStaticText* this_ptr) {
  return this_ptr->performanceHint();
}

void qt_gui_c_QStaticText_prepare_matrix(QStaticText* this_ptr, const QTransform* matrix) {
  this_ptr->prepare(*matrix);
}

void qt_gui_c_QStaticText_prepare_matrix_font(QStaticText* this_ptr, const QTransform* matrix, const QFont* font) {
  this_ptr->prepare(*matrix, *font);
}

void qt_gui_c_QStaticText_prepare_no_args(QStaticText* this_ptr) {
  this_ptr->prepare();
}

void qt_gui_c_QStaticText_setPerformanceHint(QStaticText* this_ptr, QStaticText::PerformanceHint performanceHint) {
  this_ptr->setPerformanceHint(performanceHint);
}

void qt_gui_c_QStaticText_setText(QStaticText* this_ptr, const QString* text) {
  this_ptr->setText(*text);
}

void qt_gui_c_QStaticText_setTextFormat(QStaticText* this_ptr, const Qt::TextFormat* textFormat) {
  this_ptr->setTextFormat(*textFormat);
}

void qt_gui_c_QStaticText_setTextOption(QStaticText* this_ptr, const QTextOption* textOption) {
  this_ptr->setTextOption(*textOption);
}

void qt_gui_c_QStaticText_setTextWidth(QStaticText* this_ptr, double textWidth) {
  this_ptr->setTextWidth(textWidth);
}

void qt_gui_c_QStaticText_size_to_output(const QStaticText* this_ptr, QSizeF* output) {
  new(output) QSizeF(this_ptr->size());
}

void qt_gui_c_QStaticText_swap(QStaticText* this_ptr, QStaticText* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QStaticText_textOption_to_output(const QStaticText* this_ptr, QTextOption* output) {
  new(output) QTextOption(this_ptr->textOption());
}

double qt_gui_c_QStaticText_textWidth(const QStaticText* this_ptr) {
  return this_ptr->textWidth();
}

void qt_gui_c_QStaticText_text_to_output(const QStaticText* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

