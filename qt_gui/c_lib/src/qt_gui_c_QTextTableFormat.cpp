#include "qt_gui_c_QTextTableFormat.h"

QTextFormat* qt_gui_c_QTextTableFormat_G_static_cast_QTextFormat_ptr(QTextTableFormat* ptr) {
  return static_cast<QTextFormat*>(ptr);
}

QTextFrameFormat* qt_gui_c_QTextTableFormat_G_static_cast_QTextFrameFormat_ptr(QTextTableFormat* ptr) {
  return static_cast<QTextFrameFormat*>(ptr);
}

QTextTableFormat* qt_gui_c_QTextTableFormat_G_static_cast_QTextTableFormat_ptr_QTextFormat(QTextFormat* ptr) {
  return static_cast<QTextTableFormat*>(ptr);
}

QTextTableFormat* qt_gui_c_QTextTableFormat_G_static_cast_QTextTableFormat_ptr_QTextFrameFormat(QTextFrameFormat* ptr) {
  return static_cast<QTextTableFormat*>(ptr);
}

double qt_gui_c_QTextTableFormat_cellPadding(const QTextTableFormat* this_ptr) {
  return this_ptr->cellPadding();
}

double qt_gui_c_QTextTableFormat_cellSpacing(const QTextTableFormat* this_ptr) {
  return this_ptr->cellSpacing();
}

void qt_gui_c_QTextTableFormat_clearColumnWidthConstraints(QTextTableFormat* this_ptr) {
  this_ptr->clearColumnWidthConstraints();
}

void qt_gui_c_QTextTableFormat_columnWidthConstraints_to_output(const QTextTableFormat* this_ptr, QVector< QTextLength >* output) {
  new(output) QVector< QTextLength >(this_ptr->columnWidthConstraints());
}

int qt_gui_c_QTextTableFormat_columns(const QTextTableFormat* this_ptr) {
  return this_ptr->columns();
}

void qt_gui_c_QTextTableFormat_constructor(QTextTableFormat* output) {
  new(output) QTextTableFormat();
}

void qt_gui_c_QTextTableFormat_destructor(QTextTableFormat* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

int qt_gui_c_QTextTableFormat_headerRowCount(const QTextTableFormat* this_ptr) {
  return this_ptr->headerRowCount();
}

bool qt_gui_c_QTextTableFormat_isValid(const QTextTableFormat* this_ptr) {
  return this_ptr->isValid();
}

void qt_gui_c_QTextTableFormat_setCellPadding(QTextTableFormat* this_ptr, double padding) {
  this_ptr->setCellPadding(padding);
}

void qt_gui_c_QTextTableFormat_setCellSpacing(QTextTableFormat* this_ptr, double spacing) {
  this_ptr->setCellSpacing(spacing);
}

void qt_gui_c_QTextTableFormat_setColumnWidthConstraints(QTextTableFormat* this_ptr, const QVector< QTextLength >* constraints) {
  this_ptr->setColumnWidthConstraints(*constraints);
}

void qt_gui_c_QTextTableFormat_setColumns(QTextTableFormat* this_ptr, int columns) {
  this_ptr->setColumns(columns);
}

void qt_gui_c_QTextTableFormat_setHeaderRowCount(QTextTableFormat* this_ptr, int count) {
  this_ptr->setHeaderRowCount(count);
}

