#include "qt_gui_c_QTextTableCell.h"

void qt_gui_c_QTextTableCell_begin_to_output(const QTextTableCell* this_ptr, QTextFrame::iterator* output) {
  new(output) QTextFrame::iterator(this_ptr->begin());
}

int qt_gui_c_QTextTableCell_column(const QTextTableCell* this_ptr) {
  return this_ptr->column();
}

int qt_gui_c_QTextTableCell_columnSpan(const QTextTableCell* this_ptr) {
  return this_ptr->columnSpan();
}

void qt_gui_c_QTextTableCell_constructor_no_args(QTextTableCell* output) {
  new(output) QTextTableCell();
}

void qt_gui_c_QTextTableCell_constructor_o(const QTextTableCell* o, QTextTableCell* output) {
  new(output) QTextTableCell(*o);
}

void qt_gui_c_QTextTableCell_destructor(QTextTableCell* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QTextTableCell_end_to_output(const QTextTableCell* this_ptr, QTextFrame::iterator* output) {
  new(output) QTextFrame::iterator(this_ptr->end());
}

QTextCursor* qt_gui_c_QTextTableCell_firstCursorPosition_as_ptr(const QTextTableCell* this_ptr) {
  return new QTextCursor(this_ptr->firstCursorPosition());
}

int qt_gui_c_QTextTableCell_firstPosition(const QTextTableCell* this_ptr) {
  return this_ptr->firstPosition();
}

void qt_gui_c_QTextTableCell_format_to_output(const QTextTableCell* this_ptr, QTextCharFormat* output) {
  new(output) QTextCharFormat(this_ptr->format());
}

bool qt_gui_c_QTextTableCell_isValid(const QTextTableCell* this_ptr) {
  return this_ptr->isValid();
}

QTextCursor* qt_gui_c_QTextTableCell_lastCursorPosition_as_ptr(const QTextTableCell* this_ptr) {
  return new QTextCursor(this_ptr->lastCursorPosition());
}

int qt_gui_c_QTextTableCell_lastPosition(const QTextTableCell* this_ptr) {
  return this_ptr->lastPosition();
}

QTextTableCell* qt_gui_c_QTextTableCell_operator_assign(QTextTableCell* this_ptr, const QTextTableCell* o) {
  return &this_ptr->operator=(*o);
}

bool qt_gui_c_QTextTableCell_operator_eq(const QTextTableCell* this_ptr, const QTextTableCell* other) {
  return this_ptr->operator==(*other);
}

bool qt_gui_c_QTextTableCell_operator_neq(const QTextTableCell* this_ptr, const QTextTableCell* other) {
  return this_ptr->operator!=(*other);
}

int qt_gui_c_QTextTableCell_row(const QTextTableCell* this_ptr) {
  return this_ptr->row();
}

int qt_gui_c_QTextTableCell_rowSpan(const QTextTableCell* this_ptr) {
  return this_ptr->rowSpan();
}

void qt_gui_c_QTextTableCell_setFormat(QTextTableCell* this_ptr, const QTextCharFormat* format) {
  this_ptr->setFormat(*format);
}

int qt_gui_c_QTextTableCell_tableCellFormatIndex(const QTextTableCell* this_ptr) {
  return this_ptr->tableCellFormatIndex();
}

