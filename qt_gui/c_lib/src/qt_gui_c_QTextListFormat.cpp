#include "qt_gui_c_QTextListFormat.h"

QTextFormat* qt_gui_c_QTextListFormat_G_static_cast_QTextFormat_ptr(QTextListFormat* ptr) {
  return static_cast<QTextFormat*>(ptr);
}

QTextListFormat* qt_gui_c_QTextListFormat_G_static_cast_QTextListFormat_ptr(QTextFormat* ptr) {
  return static_cast<QTextListFormat*>(ptr);
}

void qt_gui_c_QTextListFormat_constructor(QTextListFormat* output) {
  new(output) QTextListFormat();
}

void qt_gui_c_QTextListFormat_destructor(QTextListFormat* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

int qt_gui_c_QTextListFormat_indent(const QTextListFormat* this_ptr) {
  return this_ptr->indent();
}

bool qt_gui_c_QTextListFormat_isValid(const QTextListFormat* this_ptr) {
  return this_ptr->isValid();
}

void qt_gui_c_QTextListFormat_numberPrefix_to_output(const QTextListFormat* this_ptr, QString* output) {
  new(output) QString(this_ptr->numberPrefix());
}

void qt_gui_c_QTextListFormat_numberSuffix_to_output(const QTextListFormat* this_ptr, QString* output) {
  new(output) QString(this_ptr->numberSuffix());
}

void qt_gui_c_QTextListFormat_setIndent(QTextListFormat* this_ptr, int indent) {
  this_ptr->setIndent(indent);
}

void qt_gui_c_QTextListFormat_setNumberPrefix(QTextListFormat* this_ptr, const QString* numberPrefix) {
  this_ptr->setNumberPrefix(*numberPrefix);
}

void qt_gui_c_QTextListFormat_setNumberSuffix(QTextListFormat* this_ptr, const QString* numberSuffix) {
  this_ptr->setNumberSuffix(*numberSuffix);
}

void qt_gui_c_QTextListFormat_setStyle(QTextListFormat* this_ptr, QTextListFormat::Style style) {
  this_ptr->setStyle(style);
}

QTextListFormat::Style qt_gui_c_QTextListFormat_style(const QTextListFormat* this_ptr) {
  return this_ptr->style();
}

