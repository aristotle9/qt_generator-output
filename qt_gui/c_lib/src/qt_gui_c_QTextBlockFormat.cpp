#include "qt_gui_c_QTextBlockFormat.h"

QTextBlockFormat* qt_gui_c_QTextBlockFormat_G_static_cast_QTextBlockFormat_ptr(QTextFormat* ptr) {
  return static_cast<QTextBlockFormat*>(ptr);
}

QTextFormat* qt_gui_c_QTextBlockFormat_G_static_cast_QTextFormat_ptr(QTextBlockFormat* ptr) {
  return static_cast<QTextFormat*>(ptr);
}

double qt_gui_c_QTextBlockFormat_bottomMargin(const QTextBlockFormat* this_ptr) {
  return this_ptr->bottomMargin();
}

void qt_gui_c_QTextBlockFormat_constructor(QTextBlockFormat* output) {
  new(output) QTextBlockFormat();
}

void qt_gui_c_QTextBlockFormat_destructor(QTextBlockFormat* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

int qt_gui_c_QTextBlockFormat_indent(const QTextBlockFormat* this_ptr) {
  return this_ptr->indent();
}

bool qt_gui_c_QTextBlockFormat_isValid(const QTextBlockFormat* this_ptr) {
  return this_ptr->isValid();
}

double qt_gui_c_QTextBlockFormat_leftMargin(const QTextBlockFormat* this_ptr) {
  return this_ptr->leftMargin();
}

int qt_gui_c_QTextBlockFormat_lineHeightType(const QTextBlockFormat* this_ptr) {
  return this_ptr->lineHeightType();
}

double qt_gui_c_QTextBlockFormat_lineHeight_no_args(const QTextBlockFormat* this_ptr) {
  return this_ptr->lineHeight();
}

double qt_gui_c_QTextBlockFormat_lineHeight_scriptLineHeight_scaling(const QTextBlockFormat* this_ptr, double scriptLineHeight, double scaling) {
  return this_ptr->lineHeight(scriptLineHeight, scaling);
}

bool qt_gui_c_QTextBlockFormat_nonBreakableLines(const QTextBlockFormat* this_ptr) {
  return this_ptr->nonBreakableLines();
}

unsigned int qt_gui_c_QTextBlockFormat_pageBreakPolicy(const QTextBlockFormat* this_ptr) {
  return uint(this_ptr->pageBreakPolicy());
}

double qt_gui_c_QTextBlockFormat_rightMargin(const QTextBlockFormat* this_ptr) {
  return this_ptr->rightMargin();
}

void qt_gui_c_QTextBlockFormat_setBottomMargin(QTextBlockFormat* this_ptr, double margin) {
  this_ptr->setBottomMargin(margin);
}

void qt_gui_c_QTextBlockFormat_setIndent(QTextBlockFormat* this_ptr, int indent) {
  this_ptr->setIndent(indent);
}

void qt_gui_c_QTextBlockFormat_setLeftMargin(QTextBlockFormat* this_ptr, double margin) {
  this_ptr->setLeftMargin(margin);
}

void qt_gui_c_QTextBlockFormat_setLineHeight(QTextBlockFormat* this_ptr, double height, int heightType) {
  this_ptr->setLineHeight(height, heightType);
}

void qt_gui_c_QTextBlockFormat_setNonBreakableLines(QTextBlockFormat* this_ptr, bool b) {
  this_ptr->setNonBreakableLines(b);
}

void qt_gui_c_QTextBlockFormat_setPageBreakPolicy(QTextBlockFormat* this_ptr, unsigned int flags) {
  this_ptr->setPageBreakPolicy(QFlags< QTextFormat::PageBreakFlag >(flags));
}

void qt_gui_c_QTextBlockFormat_setRightMargin(QTextBlockFormat* this_ptr, double margin) {
  this_ptr->setRightMargin(margin);
}

void qt_gui_c_QTextBlockFormat_setTabPositions(QTextBlockFormat* this_ptr, const QList< QTextOption::Tab >* tabs) {
  this_ptr->setTabPositions(*tabs);
}

void qt_gui_c_QTextBlockFormat_setTextIndent(QTextBlockFormat* this_ptr, double aindent) {
  this_ptr->setTextIndent(aindent);
}

void qt_gui_c_QTextBlockFormat_setTopMargin(QTextBlockFormat* this_ptr, double margin) {
  this_ptr->setTopMargin(margin);
}

void qt_gui_c_QTextBlockFormat_tabPositions_to_output(const QTextBlockFormat* this_ptr, QList< QTextOption::Tab >* output) {
  new(output) QList< QTextOption::Tab >(this_ptr->tabPositions());
}

double qt_gui_c_QTextBlockFormat_textIndent(const QTextBlockFormat* this_ptr) {
  return this_ptr->textIndent();
}

double qt_gui_c_QTextBlockFormat_topMargin(const QTextBlockFormat* this_ptr) {
  return this_ptr->topMargin();
}

