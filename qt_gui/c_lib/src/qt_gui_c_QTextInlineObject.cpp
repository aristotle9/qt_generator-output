#include "qt_gui_c_QTextInlineObject.h"

double qt_gui_c_QTextInlineObject_ascent(const QTextInlineObject* this_ptr) {
  return this_ptr->ascent();
}

void qt_gui_c_QTextInlineObject_constructor(QTextInlineObject* output) {
  new(output) QTextInlineObject();
}

double qt_gui_c_QTextInlineObject_descent(const QTextInlineObject* this_ptr) {
  return this_ptr->descent();
}

void qt_gui_c_QTextInlineObject_destructor(QTextInlineObject* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

int qt_gui_c_QTextInlineObject_formatIndex(const QTextInlineObject* this_ptr) {
  return this_ptr->formatIndex();
}

void qt_gui_c_QTextInlineObject_format_to_output(const QTextInlineObject* this_ptr, QTextFormat* output) {
  new(output) QTextFormat(this_ptr->format());
}

double qt_gui_c_QTextInlineObject_height(const QTextInlineObject* this_ptr) {
  return this_ptr->height();
}

bool qt_gui_c_QTextInlineObject_isValid(const QTextInlineObject* this_ptr) {
  return this_ptr->isValid();
}

void qt_gui_c_QTextInlineObject_rect_to_output(const QTextInlineObject* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->rect());
}

void qt_gui_c_QTextInlineObject_setAscent(QTextInlineObject* this_ptr, double a) {
  this_ptr->setAscent(a);
}

void qt_gui_c_QTextInlineObject_setDescent(QTextInlineObject* this_ptr, double d) {
  this_ptr->setDescent(d);
}

void qt_gui_c_QTextInlineObject_setWidth(QTextInlineObject* this_ptr, double w) {
  this_ptr->setWidth(w);
}

int qt_gui_c_QTextInlineObject_textPosition(const QTextInlineObject* this_ptr) {
  return this_ptr->textPosition();
}

double qt_gui_c_QTextInlineObject_width(const QTextInlineObject* this_ptr) {
  return this_ptr->width();
}

