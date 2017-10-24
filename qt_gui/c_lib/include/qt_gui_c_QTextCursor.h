#ifndef QT_GUI_C_QTEXTCURSOR_H
#define QT_GUI_C_QTEXTCURSOR_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_G_swap(QTextCursor* value1, QTextCursor* value2);
QT_GUI_C_EXPORT int qt_gui_c_QTextCursor_anchor(const QTextCursor* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCursor_atBlockEnd(const QTextCursor* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCursor_atBlockStart(const QTextCursor* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCursor_atEnd(const QTextCursor* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCursor_atStart(const QTextCursor* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_beginEditBlock(QTextCursor* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_blockCharFormat_to_output(const QTextCursor* this_ptr, QTextCharFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_blockFormat_to_output(const QTextCursor* this_ptr, QTextBlockFormat* output);
QT_GUI_C_EXPORT int qt_gui_c_QTextCursor_blockNumber(const QTextCursor* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_block_to_output(const QTextCursor* this_ptr, QTextBlock* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_charFormat_to_output(const QTextCursor* this_ptr, QTextCharFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_clearSelection(QTextCursor* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextCursor_columnNumber(const QTextCursor* this_ptr);
QT_GUI_C_EXPORT QTextList* qt_gui_c_QTextCursor_createList_format(QTextCursor* this_ptr, const QTextListFormat* format);
QT_GUI_C_EXPORT QTextList* qt_gui_c_QTextCursor_createList_style(QTextCursor* this_ptr, const QTextListFormat::Style* style);
QT_GUI_C_EXPORT QTextFrame* qt_gui_c_QTextCursor_currentFrame(const QTextCursor* this_ptr);
QT_GUI_C_EXPORT QTextList* qt_gui_c_QTextCursor_currentList(const QTextCursor* this_ptr);
QT_GUI_C_EXPORT QTextTable* qt_gui_c_QTextCursor_currentTable(const QTextCursor* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_delete(QTextCursor* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_deleteChar(QTextCursor* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_deletePreviousChar(QTextCursor* this_ptr);
QT_GUI_C_EXPORT QTextDocument* qt_gui_c_QTextCursor_document(const QTextCursor* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_endEditBlock(QTextCursor* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCursor_hasComplexSelection(const QTextCursor* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCursor_hasSelection(const QTextCursor* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_insertBlock_format(QTextCursor* this_ptr, const QTextBlockFormat* format);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_insertBlock_format_charFormat(QTextCursor* this_ptr, const QTextBlockFormat* format, const QTextCharFormat* charFormat);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_insertBlock_no_args(QTextCursor* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_insertFragment(QTextCursor* this_ptr, const QTextDocumentFragment* fragment);
QT_GUI_C_EXPORT QTextFrame* qt_gui_c_QTextCursor_insertFrame(QTextCursor* this_ptr, const QTextFrameFormat* format);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_insertHtml(QTextCursor* this_ptr, const QString* html);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_insertImage_format(QTextCursor* this_ptr, const QTextImageFormat* format);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_insertImage_format_alignment(QTextCursor* this_ptr, const QTextImageFormat* format, const QTextFrameFormat::Position* alignment);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_insertImage_image(QTextCursor* this_ptr, const QImage* image);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_insertImage_image_name(QTextCursor* this_ptr, const QImage* image, const QString* name);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_insertImage_name(QTextCursor* this_ptr, const QString* name);
QT_GUI_C_EXPORT QTextList* qt_gui_c_QTextCursor_insertList_format(QTextCursor* this_ptr, const QTextListFormat* format);
QT_GUI_C_EXPORT QTextList* qt_gui_c_QTextCursor_insertList_style(QTextCursor* this_ptr, const QTextListFormat::Style* style);
QT_GUI_C_EXPORT QTextTable* qt_gui_c_QTextCursor_insertTable_rows_cols(QTextCursor* this_ptr, int rows, int cols);
QT_GUI_C_EXPORT QTextTable* qt_gui_c_QTextCursor_insertTable_rows_cols_format(QTextCursor* this_ptr, int rows, int cols, const QTextTableFormat* format);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_insertText_text(QTextCursor* this_ptr, const QString* text);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_insertText_text_format(QTextCursor* this_ptr, const QString* text, const QTextCharFormat* format);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCursor_isCopyOf(const QTextCursor* this_ptr, const QTextCursor* other);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCursor_isNull(const QTextCursor* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_joinPreviousEditBlock(QTextCursor* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCursor_keepPositionOnInsert(const QTextCursor* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_mergeBlockCharFormat(QTextCursor* this_ptr, const QTextCharFormat* modifier);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_mergeBlockFormat(QTextCursor* this_ptr, const QTextBlockFormat* modifier);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_mergeCharFormat(QTextCursor* this_ptr, const QTextCharFormat* modifier);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCursor_movePosition_op(QTextCursor* this_ptr, QTextCursor::MoveOperation op);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCursor_movePosition_op_arg2(QTextCursor* this_ptr, QTextCursor::MoveOperation op, QTextCursor::MoveMode arg2);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCursor_movePosition_op_arg2_n(QTextCursor* this_ptr, QTextCursor::MoveOperation op, QTextCursor::MoveMode arg2, int n);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextCursor_new_block(const QTextBlock* block);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextCursor_new_cursor(const QTextCursor* cursor);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextCursor_new_document(QTextDocument* document);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextCursor_new_frame(QTextFrame* frame);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextCursor_new_no_args();
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextCursor_operator_assign(QTextCursor* this_ptr, const QTextCursor* other);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCursor_operator_eq(const QTextCursor* this_ptr, const QTextCursor* rhs);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCursor_operator_ge(const QTextCursor* this_ptr, const QTextCursor* rhs);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCursor_operator_gt(const QTextCursor* this_ptr, const QTextCursor* rhs);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCursor_operator_le(const QTextCursor* this_ptr, const QTextCursor* rhs);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCursor_operator_lt(const QTextCursor* this_ptr, const QTextCursor* rhs);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCursor_operator_neq(const QTextCursor* this_ptr, const QTextCursor* rhs);
QT_GUI_C_EXPORT int qt_gui_c_QTextCursor_position(const QTextCursor* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextCursor_positionInBlock(const QTextCursor* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_removeSelectedText(QTextCursor* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_select(QTextCursor* this_ptr, QTextCursor::SelectionType selection);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_selectedTableCells(const QTextCursor* this_ptr, int* firstRow, int* numRows, int* firstColumn, int* numColumns);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_selectedText_to_output(const QTextCursor* this_ptr, QString* output);
QT_GUI_C_EXPORT int qt_gui_c_QTextCursor_selectionEnd(const QTextCursor* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextCursor_selectionStart(const QTextCursor* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_selection_to_output(const QTextCursor* this_ptr, QTextDocumentFragment* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_setBlockCharFormat(QTextCursor* this_ptr, const QTextCharFormat* format);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_setBlockFormat(QTextCursor* this_ptr, const QTextBlockFormat* format);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_setCharFormat(QTextCursor* this_ptr, const QTextCharFormat* format);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_setKeepPositionOnInsert(QTextCursor* this_ptr, bool b);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_setPosition_pos(QTextCursor* this_ptr, int pos);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_setPosition_pos_mode(QTextCursor* this_ptr, int pos, QTextCursor::MoveMode mode);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_setVerticalMovementX(QTextCursor* this_ptr, int x);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_setVisualNavigation(QTextCursor* this_ptr, bool b);
QT_GUI_C_EXPORT void qt_gui_c_QTextCursor_swap(QTextCursor* this_ptr, QTextCursor* other);
QT_GUI_C_EXPORT int qt_gui_c_QTextCursor_verticalMovementX(const QTextCursor* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCursor_visualNavigation(const QTextCursor* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QTEXTCURSOR_H
