#ifndef QT_GUI_C_QTEXTBLOCK_H
#define QT_GUI_C_QTEXTBLOCK_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QTextBlock_begin_to_output(const QTextBlock* this_ptr, QTextBlock::iterator* output);
QT_GUI_C_EXPORT int qt_gui_c_QTextBlock_blockFormatIndex(const QTextBlock* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlock_blockFormat_to_output(const QTextBlock* this_ptr, QTextBlockFormat* output);
QT_GUI_C_EXPORT int qt_gui_c_QTextBlock_blockNumber(const QTextBlock* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextBlock_charFormatIndex(const QTextBlock* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlock_charFormat_to_output(const QTextBlock* this_ptr, QTextCharFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlock_clearLayout(QTextBlock* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlock_constructor_no_args(QTextBlock* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlock_constructor_o(const QTextBlock* o, QTextBlock* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTextBlock_contains(const QTextBlock* this_ptr, int position);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlock_destructor(QTextBlock* this_ptr);
QT_GUI_C_EXPORT const QTextDocument* qt_gui_c_QTextBlock_document(const QTextBlock* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlock_end_to_output(const QTextBlock* this_ptr, QTextBlock::iterator* output);
QT_GUI_C_EXPORT int qt_gui_c_QTextBlock_firstLineNumber(const QTextBlock* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextBlock_fragmentIndex(const QTextBlock* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextBlock_isValid(const QTextBlock* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextBlock_isVisible(const QTextBlock* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextBlock_iterator_atEnd(const QTextBlock::iterator* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlock_iterator_constructor_no_args(QTextBlock::iterator* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlock_iterator_constructor_o(const QTextBlock::iterator* o, QTextBlock::iterator* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlock_iterator_destructor(QTextBlock::iterator* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlock_iterator_fragment_to_output(const QTextBlock::iterator* this_ptr, QTextFragment* output);
QT_GUI_C_EXPORT QTextBlock::iterator* qt_gui_c_QTextBlock_iterator_operator_dec(QTextBlock::iterator* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlock_iterator_operator_dec_postfix_to_output(QTextBlock::iterator* this_ptr, int arg1, QTextBlock::iterator* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTextBlock_iterator_operator_eq(const QTextBlock::iterator* this_ptr, const QTextBlock::iterator* o);
QT_GUI_C_EXPORT QTextBlock::iterator* qt_gui_c_QTextBlock_iterator_operator_inc(QTextBlock::iterator* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlock_iterator_operator_inc_postfix_to_output(QTextBlock::iterator* this_ptr, int arg1, QTextBlock::iterator* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTextBlock_iterator_operator_neq(const QTextBlock::iterator* this_ptr, const QTextBlock::iterator* o);
QT_GUI_C_EXPORT QTextLayout* qt_gui_c_QTextBlock_layout(const QTextBlock* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextBlock_length(const QTextBlock* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextBlock_lineCount(const QTextBlock* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlock_next_to_output(const QTextBlock* this_ptr, QTextBlock* output);
QT_GUI_C_EXPORT QTextBlock* qt_gui_c_QTextBlock_operator_assign(QTextBlock* this_ptr, const QTextBlock* o);
QT_GUI_C_EXPORT bool qt_gui_c_QTextBlock_operator_eq(const QTextBlock* this_ptr, const QTextBlock* o);
QT_GUI_C_EXPORT bool qt_gui_c_QTextBlock_operator_lt(const QTextBlock* this_ptr, const QTextBlock* o);
QT_GUI_C_EXPORT bool qt_gui_c_QTextBlock_operator_neq(const QTextBlock* this_ptr, const QTextBlock* o);
QT_GUI_C_EXPORT int qt_gui_c_QTextBlock_position(const QTextBlock* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlock_previous_to_output(const QTextBlock* this_ptr, QTextBlock* output);
QT_GUI_C_EXPORT int qt_gui_c_QTextBlock_revision(const QTextBlock* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlock_setLineCount(QTextBlock* this_ptr, int count);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlock_setRevision(QTextBlock* this_ptr, int rev);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlock_setUserData(QTextBlock* this_ptr, QTextBlockUserData* data);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlock_setUserState(QTextBlock* this_ptr, int state);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlock_setVisible(QTextBlock* this_ptr, bool visible);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlock_textFormats_to_output(const QTextBlock* this_ptr, QVector< QTextLayout::FormatRange >* output);
QT_GUI_C_EXPORT QTextList* qt_gui_c_QTextBlock_textList(const QTextBlock* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlock_text_to_output(const QTextBlock* this_ptr, QString* output);
QT_GUI_C_EXPORT QTextBlockUserData* qt_gui_c_QTextBlock_userData(const QTextBlock* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextBlock_userState(const QTextBlock* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QTEXTBLOCK_H
