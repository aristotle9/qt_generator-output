#ifndef QT_GUI_C_QTEXTTABLECELL_H
#define QT_GUI_C_QTEXTTABLECELL_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QTextTableCell_begin_to_output(const QTextTableCell* this_ptr, QTextFrame::iterator* output);
QT_GUI_C_EXPORT int qt_gui_c_QTextTableCell_column(const QTextTableCell* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextTableCell_columnSpan(const QTextTableCell* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextTableCell_constructor_no_args(QTextTableCell* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextTableCell_constructor_o(const QTextTableCell* o, QTextTableCell* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextTableCell_destructor(QTextTableCell* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextTableCell_end_to_output(const QTextTableCell* this_ptr, QTextFrame::iterator* output);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextTableCell_firstCursorPosition_as_ptr(const QTextTableCell* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextTableCell_firstPosition(const QTextTableCell* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextTableCell_format_to_output(const QTextTableCell* this_ptr, QTextCharFormat* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTextTableCell_isValid(const QTextTableCell* this_ptr);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextTableCell_lastCursorPosition_as_ptr(const QTextTableCell* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextTableCell_lastPosition(const QTextTableCell* this_ptr);
QT_GUI_C_EXPORT QTextTableCell* qt_gui_c_QTextTableCell_operator_assign(QTextTableCell* this_ptr, const QTextTableCell* o);
QT_GUI_C_EXPORT bool qt_gui_c_QTextTableCell_operator_eq(const QTextTableCell* this_ptr, const QTextTableCell* other);
QT_GUI_C_EXPORT bool qt_gui_c_QTextTableCell_operator_neq(const QTextTableCell* this_ptr, const QTextTableCell* other);
QT_GUI_C_EXPORT int qt_gui_c_QTextTableCell_row(const QTextTableCell* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextTableCell_rowSpan(const QTextTableCell* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextTableCell_setFormat(QTextTableCell* this_ptr, const QTextCharFormat* format);
QT_GUI_C_EXPORT int qt_gui_c_QTextTableCell_tableCellFormatIndex(const QTextTableCell* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QTEXTTABLECELL_H
