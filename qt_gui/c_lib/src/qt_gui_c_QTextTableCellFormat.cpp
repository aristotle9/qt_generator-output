#include "qt_gui_c_QTextTableCellFormat.h"

QTextCharFormat* qt_gui_c_QTextTableCellFormat_G_static_cast_QTextCharFormat_ptr(QTextTableCellFormat* ptr) {
  return static_cast<QTextCharFormat*>(ptr);
}

QTextFormat* qt_gui_c_QTextTableCellFormat_G_static_cast_QTextFormat_ptr(QTextTableCellFormat* ptr) {
  return static_cast<QTextFormat*>(ptr);
}

QTextTableCellFormat* qt_gui_c_QTextTableCellFormat_G_static_cast_QTextTableCellFormat_ptr_QTextCharFormat(QTextCharFormat* ptr) {
  return static_cast<QTextTableCellFormat*>(ptr);
}

QTextTableCellFormat* qt_gui_c_QTextTableCellFormat_G_static_cast_QTextTableCellFormat_ptr_QTextFormat(QTextFormat* ptr) {
  return static_cast<QTextTableCellFormat*>(ptr);
}

double qt_gui_c_QTextTableCellFormat_bottomPadding(const QTextTableCellFormat* this_ptr) {
  return this_ptr->bottomPadding();
}

void qt_gui_c_QTextTableCellFormat_constructor(QTextTableCellFormat* output) {
  new(output) QTextTableCellFormat();
}

void qt_gui_c_QTextTableCellFormat_destructor(QTextTableCellFormat* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QTextTableCellFormat_isValid(const QTextTableCellFormat* this_ptr) {
  return this_ptr->isValid();
}

double qt_gui_c_QTextTableCellFormat_leftPadding(const QTextTableCellFormat* this_ptr) {
  return this_ptr->leftPadding();
}

double qt_gui_c_QTextTableCellFormat_rightPadding(const QTextTableCellFormat* this_ptr) {
  return this_ptr->rightPadding();
}

void qt_gui_c_QTextTableCellFormat_setBottomPadding(QTextTableCellFormat* this_ptr, double padding) {
  this_ptr->setBottomPadding(padding);
}

void qt_gui_c_QTextTableCellFormat_setLeftPadding(QTextTableCellFormat* this_ptr, double padding) {
  this_ptr->setLeftPadding(padding);
}

void qt_gui_c_QTextTableCellFormat_setPadding(QTextTableCellFormat* this_ptr, double padding) {
  this_ptr->setPadding(padding);
}

void qt_gui_c_QTextTableCellFormat_setRightPadding(QTextTableCellFormat* this_ptr, double padding) {
  this_ptr->setRightPadding(padding);
}

void qt_gui_c_QTextTableCellFormat_setTopPadding(QTextTableCellFormat* this_ptr, double padding) {
  this_ptr->setTopPadding(padding);
}

double qt_gui_c_QTextTableCellFormat_topPadding(const QTextTableCellFormat* this_ptr) {
  return this_ptr->topPadding();
}

