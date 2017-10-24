#include "qt_gui_c_QTextImageFormat.h"

QTextCharFormat* qt_gui_c_QTextImageFormat_G_static_cast_QTextCharFormat_ptr(QTextImageFormat* ptr) {
  return static_cast<QTextCharFormat*>(ptr);
}

QTextFormat* qt_gui_c_QTextImageFormat_G_static_cast_QTextFormat_ptr(QTextImageFormat* ptr) {
  return static_cast<QTextFormat*>(ptr);
}

QTextImageFormat* qt_gui_c_QTextImageFormat_G_static_cast_QTextImageFormat_ptr_QTextCharFormat(QTextCharFormat* ptr) {
  return static_cast<QTextImageFormat*>(ptr);
}

QTextImageFormat* qt_gui_c_QTextImageFormat_G_static_cast_QTextImageFormat_ptr_QTextFormat(QTextFormat* ptr) {
  return static_cast<QTextImageFormat*>(ptr);
}

void qt_gui_c_QTextImageFormat_constructor(QTextImageFormat* output) {
  new(output) QTextImageFormat();
}

void qt_gui_c_QTextImageFormat_destructor(QTextImageFormat* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

double qt_gui_c_QTextImageFormat_height(const QTextImageFormat* this_ptr) {
  return this_ptr->height();
}

bool qt_gui_c_QTextImageFormat_isValid(const QTextImageFormat* this_ptr) {
  return this_ptr->isValid();
}

void qt_gui_c_QTextImageFormat_name_to_output(const QTextImageFormat* this_ptr, QString* output) {
  new(output) QString(this_ptr->name());
}

void qt_gui_c_QTextImageFormat_setHeight(QTextImageFormat* this_ptr, double height) {
  this_ptr->setHeight(height);
}

void qt_gui_c_QTextImageFormat_setName(QTextImageFormat* this_ptr, const QString* name) {
  this_ptr->setName(*name);
}

void qt_gui_c_QTextImageFormat_setWidth(QTextImageFormat* this_ptr, double width) {
  this_ptr->setWidth(width);
}

double qt_gui_c_QTextImageFormat_width(const QTextImageFormat* this_ptr) {
  return this_ptr->width();
}

