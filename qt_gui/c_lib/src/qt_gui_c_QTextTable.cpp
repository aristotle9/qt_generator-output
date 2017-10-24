#include "qt_gui_c_QTextTable.h"

QTextTable* qt_gui_c_QTextTable_G_dynamic_cast_QTextTable_ptr_QTextFrame(QTextFrame* ptr) {
  return dynamic_cast<QTextTable*>(ptr);
}

QTextTable* qt_gui_c_QTextTable_G_dynamic_cast_QTextTable_ptr_QTextObject(QTextObject* ptr) {
  return dynamic_cast<QTextTable*>(ptr);
}

QObject* qt_gui_c_QTextTable_G_static_cast_QObject_ptr(QTextTable* ptr) {
  return static_cast<QObject*>(ptr);
}

QTextFrame* qt_gui_c_QTextTable_G_static_cast_QTextFrame_ptr(QTextTable* ptr) {
  return static_cast<QTextFrame*>(ptr);
}

QTextObject* qt_gui_c_QTextTable_G_static_cast_QTextObject_ptr(QTextTable* ptr) {
  return static_cast<QTextObject*>(ptr);
}

QTextTable* qt_gui_c_QTextTable_G_static_cast_QTextTable_ptr_QObject(QObject* ptr) {
  return static_cast<QTextTable*>(ptr);
}

QTextTable* qt_gui_c_QTextTable_G_static_cast_QTextTable_ptr_QTextFrame(QTextFrame* ptr) {
  return static_cast<QTextTable*>(ptr);
}

QTextTable* qt_gui_c_QTextTable_G_static_cast_QTextTable_ptr_QTextObject(QTextObject* ptr) {
  return static_cast<QTextTable*>(ptr);
}

void qt_gui_c_QTextTable_appendColumns(QTextTable* this_ptr, int count) {
  this_ptr->appendColumns(count);
}

void qt_gui_c_QTextTable_appendRows(QTextTable* this_ptr, int count) {
  this_ptr->appendRows(count);
}

void qt_gui_c_QTextTable_cellAt_to_output_c(const QTextTable* this_ptr, const QTextCursor* c, QTextTableCell* output) {
  new(output) QTextTableCell(this_ptr->cellAt(*c));
}

void qt_gui_c_QTextTable_cellAt_to_output_position(const QTextTable* this_ptr, int position, QTextTableCell* output) {
  new(output) QTextTableCell(this_ptr->cellAt(position));
}

void qt_gui_c_QTextTable_cellAt_to_output_row_col(const QTextTable* this_ptr, int row, int col, QTextTableCell* output) {
  new(output) QTextTableCell(this_ptr->cellAt(row, col));
}

int qt_gui_c_QTextTable_columns(const QTextTable* this_ptr) {
  return this_ptr->columns();
}

void qt_gui_c_QTextTable_delete(QTextTable* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QTextTable_format_to_output(const QTextTable* this_ptr, QTextTableFormat* output) {
  new(output) QTextTableFormat(this_ptr->format());
}

void qt_gui_c_QTextTable_insertColumns(QTextTable* this_ptr, int pos, int num) {
  this_ptr->insertColumns(pos, num);
}

void qt_gui_c_QTextTable_insertRows(QTextTable* this_ptr, int pos, int num) {
  this_ptr->insertRows(pos, num);
}

void qt_gui_c_QTextTable_mergeCells_cursor(QTextTable* this_ptr, const QTextCursor* cursor) {
  this_ptr->mergeCells(*cursor);
}

void qt_gui_c_QTextTable_mergeCells_row_col_numRows_numCols(QTextTable* this_ptr, int row, int col, int numRows, int numCols) {
  this_ptr->mergeCells(row, col, numRows, numCols);
}

const QMetaObject* qt_gui_c_QTextTable_metaObject(const QTextTable* this_ptr) {
  return this_ptr->metaObject();
}

QTextTable* qt_gui_c_QTextTable_new(QTextDocument* doc) {
  return new QTextTable(doc);
}

int qt_gui_c_QTextTable_qt_metacall(QTextTable* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QTextTable_qt_metacast(QTextTable* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QTextTable_removeColumns(QTextTable* this_ptr, int pos, int num) {
  this_ptr->removeColumns(pos, num);
}

void qt_gui_c_QTextTable_removeRows(QTextTable* this_ptr, int pos, int num) {
  this_ptr->removeRows(pos, num);
}

void qt_gui_c_QTextTable_resize(QTextTable* this_ptr, int rows, int cols) {
  this_ptr->resize(rows, cols);
}

QTextCursor* qt_gui_c_QTextTable_rowEnd_as_ptr(const QTextTable* this_ptr, const QTextCursor* c) {
  return new QTextCursor(this_ptr->rowEnd(*c));
}

QTextCursor* qt_gui_c_QTextTable_rowStart_as_ptr(const QTextTable* this_ptr, const QTextCursor* c) {
  return new QTextCursor(this_ptr->rowStart(*c));
}

int qt_gui_c_QTextTable_rows(const QTextTable* this_ptr) {
  return this_ptr->rows();
}

void qt_gui_c_QTextTable_setFormat(QTextTable* this_ptr, const QTextTableFormat* format) {
  this_ptr->setFormat(*format);
}

void qt_gui_c_QTextTable_splitCell(QTextTable* this_ptr, int row, int col, int numRows, int numCols) {
  this_ptr->splitCell(row, col, numRows, numCols);
}

void qt_gui_c_QTextTable_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTextTable::trUtf8(s, c, n));
}

void qt_gui_c_QTextTable_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTextTable::tr(s, c, n));
}

