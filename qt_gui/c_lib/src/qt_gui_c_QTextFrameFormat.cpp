#include "qt_gui_c_QTextFrameFormat.h"

QTextFormat* qt_gui_c_QTextFrameFormat_G_static_cast_QTextFormat_ptr(QTextFrameFormat* ptr) {
  return static_cast<QTextFormat*>(ptr);
}

QTextFrameFormat* qt_gui_c_QTextFrameFormat_G_static_cast_QTextFrameFormat_ptr(QTextFormat* ptr) {
  return static_cast<QTextFrameFormat*>(ptr);
}

double qt_gui_c_QTextFrameFormat_border(const QTextFrameFormat* this_ptr) {
  return this_ptr->border();
}

void qt_gui_c_QTextFrameFormat_borderBrush_to_output(const QTextFrameFormat* this_ptr, QBrush* output) {
  new(output) QBrush(this_ptr->borderBrush());
}

QTextFrameFormat::BorderStyle qt_gui_c_QTextFrameFormat_borderStyle(const QTextFrameFormat* this_ptr) {
  return this_ptr->borderStyle();
}

double qt_gui_c_QTextFrameFormat_bottomMargin(const QTextFrameFormat* this_ptr) {
  return this_ptr->bottomMargin();
}

void qt_gui_c_QTextFrameFormat_constructor(QTextFrameFormat* output) {
  new(output) QTextFrameFormat();
}

void qt_gui_c_QTextFrameFormat_destructor(QTextFrameFormat* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QTextFrameFormat_height_to_output(const QTextFrameFormat* this_ptr, QTextLength* output) {
  new(output) QTextLength(this_ptr->height());
}

bool qt_gui_c_QTextFrameFormat_isValid(const QTextFrameFormat* this_ptr) {
  return this_ptr->isValid();
}

double qt_gui_c_QTextFrameFormat_leftMargin(const QTextFrameFormat* this_ptr) {
  return this_ptr->leftMargin();
}

double qt_gui_c_QTextFrameFormat_margin(const QTextFrameFormat* this_ptr) {
  return this_ptr->margin();
}

double qt_gui_c_QTextFrameFormat_padding(const QTextFrameFormat* this_ptr) {
  return this_ptr->padding();
}

unsigned int qt_gui_c_QTextFrameFormat_pageBreakPolicy(const QTextFrameFormat* this_ptr) {
  return uint(this_ptr->pageBreakPolicy());
}

QTextFrameFormat::Position qt_gui_c_QTextFrameFormat_position(const QTextFrameFormat* this_ptr) {
  return this_ptr->position();
}

double qt_gui_c_QTextFrameFormat_rightMargin(const QTextFrameFormat* this_ptr) {
  return this_ptr->rightMargin();
}

void qt_gui_c_QTextFrameFormat_setBorder(QTextFrameFormat* this_ptr, double border) {
  this_ptr->setBorder(border);
}

void qt_gui_c_QTextFrameFormat_setBorderBrush(QTextFrameFormat* this_ptr, const QBrush* brush) {
  this_ptr->setBorderBrush(*brush);
}

void qt_gui_c_QTextFrameFormat_setBorderStyle(QTextFrameFormat* this_ptr, QTextFrameFormat::BorderStyle style) {
  this_ptr->setBorderStyle(style);
}

void qt_gui_c_QTextFrameFormat_setBottomMargin(QTextFrameFormat* this_ptr, double margin) {
  this_ptr->setBottomMargin(margin);
}

void qt_gui_c_QTextFrameFormat_setHeight_QTextLength(QTextFrameFormat* this_ptr, const QTextLength* height) {
  this_ptr->setHeight(*height);
}

void qt_gui_c_QTextFrameFormat_setHeight_double(QTextFrameFormat* this_ptr, double height) {
  this_ptr->setHeight(height);
}

void qt_gui_c_QTextFrameFormat_setLeftMargin(QTextFrameFormat* this_ptr, double margin) {
  this_ptr->setLeftMargin(margin);
}

void qt_gui_c_QTextFrameFormat_setMargin(QTextFrameFormat* this_ptr, double margin) {
  this_ptr->setMargin(margin);
}

void qt_gui_c_QTextFrameFormat_setPadding(QTextFrameFormat* this_ptr, double padding) {
  this_ptr->setPadding(padding);
}

void qt_gui_c_QTextFrameFormat_setPageBreakPolicy(QTextFrameFormat* this_ptr, unsigned int flags) {
  this_ptr->setPageBreakPolicy(QFlags< QTextFormat::PageBreakFlag >(flags));
}

void qt_gui_c_QTextFrameFormat_setPosition(QTextFrameFormat* this_ptr, QTextFrameFormat::Position f) {
  this_ptr->setPosition(f);
}

void qt_gui_c_QTextFrameFormat_setRightMargin(QTextFrameFormat* this_ptr, double margin) {
  this_ptr->setRightMargin(margin);
}

void qt_gui_c_QTextFrameFormat_setTopMargin(QTextFrameFormat* this_ptr, double margin) {
  this_ptr->setTopMargin(margin);
}

void qt_gui_c_QTextFrameFormat_setWidth_length(QTextFrameFormat* this_ptr, const QTextLength* length) {
  this_ptr->setWidth(*length);
}

void qt_gui_c_QTextFrameFormat_setWidth_width(QTextFrameFormat* this_ptr, double width) {
  this_ptr->setWidth(width);
}

double qt_gui_c_QTextFrameFormat_topMargin(const QTextFrameFormat* this_ptr) {
  return this_ptr->topMargin();
}

void qt_gui_c_QTextFrameFormat_width_to_output(const QTextFrameFormat* this_ptr, QTextLength* output) {
  new(output) QTextLength(this_ptr->width());
}

