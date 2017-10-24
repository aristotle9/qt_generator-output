#include "qt_gui_c_QTextCursor.h"

void qt_gui_c_QTextCursor_G_swap(QTextCursor* value1, QTextCursor* value2) {
  swap(*value1, *value2);
}

int qt_gui_c_QTextCursor_anchor(const QTextCursor* this_ptr) {
  return this_ptr->anchor();
}

bool qt_gui_c_QTextCursor_atBlockEnd(const QTextCursor* this_ptr) {
  return this_ptr->atBlockEnd();
}

bool qt_gui_c_QTextCursor_atBlockStart(const QTextCursor* this_ptr) {
  return this_ptr->atBlockStart();
}

bool qt_gui_c_QTextCursor_atEnd(const QTextCursor* this_ptr) {
  return this_ptr->atEnd();
}

bool qt_gui_c_QTextCursor_atStart(const QTextCursor* this_ptr) {
  return this_ptr->atStart();
}

void qt_gui_c_QTextCursor_beginEditBlock(QTextCursor* this_ptr) {
  this_ptr->beginEditBlock();
}

void qt_gui_c_QTextCursor_blockCharFormat_to_output(const QTextCursor* this_ptr, QTextCharFormat* output) {
  new(output) QTextCharFormat(this_ptr->blockCharFormat());
}

void qt_gui_c_QTextCursor_blockFormat_to_output(const QTextCursor* this_ptr, QTextBlockFormat* output) {
  new(output) QTextBlockFormat(this_ptr->blockFormat());
}

int qt_gui_c_QTextCursor_blockNumber(const QTextCursor* this_ptr) {
  return this_ptr->blockNumber();
}

void qt_gui_c_QTextCursor_block_to_output(const QTextCursor* this_ptr, QTextBlock* output) {
  new(output) QTextBlock(this_ptr->block());
}

void qt_gui_c_QTextCursor_charFormat_to_output(const QTextCursor* this_ptr, QTextCharFormat* output) {
  new(output) QTextCharFormat(this_ptr->charFormat());
}

void qt_gui_c_QTextCursor_clearSelection(QTextCursor* this_ptr) {
  this_ptr->clearSelection();
}

int qt_gui_c_QTextCursor_columnNumber(const QTextCursor* this_ptr) {
  return this_ptr->columnNumber();
}

QTextList* qt_gui_c_QTextCursor_createList_format(QTextCursor* this_ptr, const QTextListFormat* format) {
  return this_ptr->createList(*format);
}

QTextList* qt_gui_c_QTextCursor_createList_style(QTextCursor* this_ptr, const QTextListFormat::Style* style) {
  return this_ptr->createList(*style);
}

QTextFrame* qt_gui_c_QTextCursor_currentFrame(const QTextCursor* this_ptr) {
  return this_ptr->currentFrame();
}

QTextList* qt_gui_c_QTextCursor_currentList(const QTextCursor* this_ptr) {
  return this_ptr->currentList();
}

QTextTable* qt_gui_c_QTextCursor_currentTable(const QTextCursor* this_ptr) {
  return this_ptr->currentTable();
}

void qt_gui_c_QTextCursor_delete(QTextCursor* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QTextCursor_deleteChar(QTextCursor* this_ptr) {
  this_ptr->deleteChar();
}

void qt_gui_c_QTextCursor_deletePreviousChar(QTextCursor* this_ptr) {
  this_ptr->deletePreviousChar();
}

QTextDocument* qt_gui_c_QTextCursor_document(const QTextCursor* this_ptr) {
  return this_ptr->document();
}

void qt_gui_c_QTextCursor_endEditBlock(QTextCursor* this_ptr) {
  this_ptr->endEditBlock();
}

bool qt_gui_c_QTextCursor_hasComplexSelection(const QTextCursor* this_ptr) {
  return this_ptr->hasComplexSelection();
}

bool qt_gui_c_QTextCursor_hasSelection(const QTextCursor* this_ptr) {
  return this_ptr->hasSelection();
}

void qt_gui_c_QTextCursor_insertBlock_format(QTextCursor* this_ptr, const QTextBlockFormat* format) {
  this_ptr->insertBlock(*format);
}

void qt_gui_c_QTextCursor_insertBlock_format_charFormat(QTextCursor* this_ptr, const QTextBlockFormat* format, const QTextCharFormat* charFormat) {
  this_ptr->insertBlock(*format, *charFormat);
}

void qt_gui_c_QTextCursor_insertBlock_no_args(QTextCursor* this_ptr) {
  this_ptr->insertBlock();
}

void qt_gui_c_QTextCursor_insertFragment(QTextCursor* this_ptr, const QTextDocumentFragment* fragment) {
  this_ptr->insertFragment(*fragment);
}

QTextFrame* qt_gui_c_QTextCursor_insertFrame(QTextCursor* this_ptr, const QTextFrameFormat* format) {
  return this_ptr->insertFrame(*format);
}

void qt_gui_c_QTextCursor_insertHtml(QTextCursor* this_ptr, const QString* html) {
  this_ptr->insertHtml(*html);
}

void qt_gui_c_QTextCursor_insertImage_format(QTextCursor* this_ptr, const QTextImageFormat* format) {
  this_ptr->insertImage(*format);
}

void qt_gui_c_QTextCursor_insertImage_format_alignment(QTextCursor* this_ptr, const QTextImageFormat* format, const QTextFrameFormat::Position* alignment) {
  this_ptr->insertImage(*format, *alignment);
}

void qt_gui_c_QTextCursor_insertImage_image(QTextCursor* this_ptr, const QImage* image) {
  this_ptr->insertImage(*image);
}

void qt_gui_c_QTextCursor_insertImage_image_name(QTextCursor* this_ptr, const QImage* image, const QString* name) {
  this_ptr->insertImage(*image, *name);
}

void qt_gui_c_QTextCursor_insertImage_name(QTextCursor* this_ptr, const QString* name) {
  this_ptr->insertImage(*name);
}

QTextList* qt_gui_c_QTextCursor_insertList_format(QTextCursor* this_ptr, const QTextListFormat* format) {
  return this_ptr->insertList(*format);
}

QTextList* qt_gui_c_QTextCursor_insertList_style(QTextCursor* this_ptr, const QTextListFormat::Style* style) {
  return this_ptr->insertList(*style);
}

QTextTable* qt_gui_c_QTextCursor_insertTable_rows_cols(QTextCursor* this_ptr, int rows, int cols) {
  return this_ptr->insertTable(rows, cols);
}

QTextTable* qt_gui_c_QTextCursor_insertTable_rows_cols_format(QTextCursor* this_ptr, int rows, int cols, const QTextTableFormat* format) {
  return this_ptr->insertTable(rows, cols, *format);
}

void qt_gui_c_QTextCursor_insertText_text(QTextCursor* this_ptr, const QString* text) {
  this_ptr->insertText(*text);
}

void qt_gui_c_QTextCursor_insertText_text_format(QTextCursor* this_ptr, const QString* text, const QTextCharFormat* format) {
  this_ptr->insertText(*text, *format);
}

bool qt_gui_c_QTextCursor_isCopyOf(const QTextCursor* this_ptr, const QTextCursor* other) {
  return this_ptr->isCopyOf(*other);
}

bool qt_gui_c_QTextCursor_isNull(const QTextCursor* this_ptr) {
  return this_ptr->isNull();
}

void qt_gui_c_QTextCursor_joinPreviousEditBlock(QTextCursor* this_ptr) {
  this_ptr->joinPreviousEditBlock();
}

bool qt_gui_c_QTextCursor_keepPositionOnInsert(const QTextCursor* this_ptr) {
  return this_ptr->keepPositionOnInsert();
}

void qt_gui_c_QTextCursor_mergeBlockCharFormat(QTextCursor* this_ptr, const QTextCharFormat* modifier) {
  this_ptr->mergeBlockCharFormat(*modifier);
}

void qt_gui_c_QTextCursor_mergeBlockFormat(QTextCursor* this_ptr, const QTextBlockFormat* modifier) {
  this_ptr->mergeBlockFormat(*modifier);
}

void qt_gui_c_QTextCursor_mergeCharFormat(QTextCursor* this_ptr, const QTextCharFormat* modifier) {
  this_ptr->mergeCharFormat(*modifier);
}

bool qt_gui_c_QTextCursor_movePosition_op(QTextCursor* this_ptr, QTextCursor::MoveOperation op) {
  return this_ptr->movePosition(op);
}

bool qt_gui_c_QTextCursor_movePosition_op_arg2(QTextCursor* this_ptr, QTextCursor::MoveOperation op, QTextCursor::MoveMode arg2) {
  return this_ptr->movePosition(op, arg2);
}

bool qt_gui_c_QTextCursor_movePosition_op_arg2_n(QTextCursor* this_ptr, QTextCursor::MoveOperation op, QTextCursor::MoveMode arg2, int n) {
  return this_ptr->movePosition(op, arg2, n);
}

QTextCursor* qt_gui_c_QTextCursor_new_block(const QTextBlock* block) {
  return new QTextCursor(*block);
}

QTextCursor* qt_gui_c_QTextCursor_new_cursor(const QTextCursor* cursor) {
  return new QTextCursor(*cursor);
}

QTextCursor* qt_gui_c_QTextCursor_new_document(QTextDocument* document) {
  return new QTextCursor(document);
}

QTextCursor* qt_gui_c_QTextCursor_new_frame(QTextFrame* frame) {
  return new QTextCursor(frame);
}

QTextCursor* qt_gui_c_QTextCursor_new_no_args() {
  return new QTextCursor();
}

QTextCursor* qt_gui_c_QTextCursor_operator_assign(QTextCursor* this_ptr, const QTextCursor* other) {
  return &this_ptr->operator=(*other);
}

bool qt_gui_c_QTextCursor_operator_eq(const QTextCursor* this_ptr, const QTextCursor* rhs) {
  return this_ptr->operator==(*rhs);
}

bool qt_gui_c_QTextCursor_operator_ge(const QTextCursor* this_ptr, const QTextCursor* rhs) {
  return this_ptr->operator>=(*rhs);
}

bool qt_gui_c_QTextCursor_operator_gt(const QTextCursor* this_ptr, const QTextCursor* rhs) {
  return this_ptr->operator>(*rhs);
}

bool qt_gui_c_QTextCursor_operator_le(const QTextCursor* this_ptr, const QTextCursor* rhs) {
  return this_ptr->operator<=(*rhs);
}

bool qt_gui_c_QTextCursor_operator_lt(const QTextCursor* this_ptr, const QTextCursor* rhs) {
  return this_ptr->operator<(*rhs);
}

bool qt_gui_c_QTextCursor_operator_neq(const QTextCursor* this_ptr, const QTextCursor* rhs) {
  return this_ptr->operator!=(*rhs);
}

int qt_gui_c_QTextCursor_position(const QTextCursor* this_ptr) {
  return this_ptr->position();
}

int qt_gui_c_QTextCursor_positionInBlock(const QTextCursor* this_ptr) {
  return this_ptr->positionInBlock();
}

void qt_gui_c_QTextCursor_removeSelectedText(QTextCursor* this_ptr) {
  this_ptr->removeSelectedText();
}

void qt_gui_c_QTextCursor_select(QTextCursor* this_ptr, QTextCursor::SelectionType selection) {
  this_ptr->select(selection);
}

void qt_gui_c_QTextCursor_selectedTableCells(const QTextCursor* this_ptr, int* firstRow, int* numRows, int* firstColumn, int* numColumns) {
  this_ptr->selectedTableCells(firstRow, numRows, firstColumn, numColumns);
}

void qt_gui_c_QTextCursor_selectedText_to_output(const QTextCursor* this_ptr, QString* output) {
  new(output) QString(this_ptr->selectedText());
}

int qt_gui_c_QTextCursor_selectionEnd(const QTextCursor* this_ptr) {
  return this_ptr->selectionEnd();
}

int qt_gui_c_QTextCursor_selectionStart(const QTextCursor* this_ptr) {
  return this_ptr->selectionStart();
}

void qt_gui_c_QTextCursor_selection_to_output(const QTextCursor* this_ptr, QTextDocumentFragment* output) {
  new(output) QTextDocumentFragment(this_ptr->selection());
}

void qt_gui_c_QTextCursor_setBlockCharFormat(QTextCursor* this_ptr, const QTextCharFormat* format) {
  this_ptr->setBlockCharFormat(*format);
}

void qt_gui_c_QTextCursor_setBlockFormat(QTextCursor* this_ptr, const QTextBlockFormat* format) {
  this_ptr->setBlockFormat(*format);
}

void qt_gui_c_QTextCursor_setCharFormat(QTextCursor* this_ptr, const QTextCharFormat* format) {
  this_ptr->setCharFormat(*format);
}

void qt_gui_c_QTextCursor_setKeepPositionOnInsert(QTextCursor* this_ptr, bool b) {
  this_ptr->setKeepPositionOnInsert(b);
}

void qt_gui_c_QTextCursor_setPosition_pos(QTextCursor* this_ptr, int pos) {
  this_ptr->setPosition(pos);
}

void qt_gui_c_QTextCursor_setPosition_pos_mode(QTextCursor* this_ptr, int pos, QTextCursor::MoveMode mode) {
  this_ptr->setPosition(pos, mode);
}

void qt_gui_c_QTextCursor_setVerticalMovementX(QTextCursor* this_ptr, int x) {
  this_ptr->setVerticalMovementX(x);
}

void qt_gui_c_QTextCursor_setVisualNavigation(QTextCursor* this_ptr, bool b) {
  this_ptr->setVisualNavigation(b);
}

void qt_gui_c_QTextCursor_swap(QTextCursor* this_ptr, QTextCursor* other) {
  this_ptr->swap(*other);
}

int qt_gui_c_QTextCursor_verticalMovementX(const QTextCursor* this_ptr) {
  return this_ptr->verticalMovementX();
}

bool qt_gui_c_QTextCursor_visualNavigation(const QTextCursor* this_ptr) {
  return this_ptr->visualNavigation();
}

