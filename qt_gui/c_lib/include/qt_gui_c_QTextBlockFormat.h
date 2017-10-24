#ifndef QT_GUI_C_QTEXTBLOCKFORMAT_H
#define QT_GUI_C_QTEXTBLOCKFORMAT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QTextBlockFormat* qt_gui_c_QTextBlockFormat_G_static_cast_QTextBlockFormat_ptr(QTextFormat* ptr);
QT_GUI_C_EXPORT QTextFormat* qt_gui_c_QTextBlockFormat_G_static_cast_QTextFormat_ptr(QTextBlockFormat* ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextBlockFormat_bottomMargin(const QTextBlockFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlockFormat_constructor(QTextBlockFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlockFormat_destructor(QTextBlockFormat* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextBlockFormat_indent(const QTextBlockFormat* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextBlockFormat_isValid(const QTextBlockFormat* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextBlockFormat_leftMargin(const QTextBlockFormat* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextBlockFormat_lineHeightType(const QTextBlockFormat* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextBlockFormat_lineHeight_no_args(const QTextBlockFormat* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextBlockFormat_lineHeight_scriptLineHeight_scaling(const QTextBlockFormat* this_ptr, double scriptLineHeight, double scaling);
QT_GUI_C_EXPORT bool qt_gui_c_QTextBlockFormat_nonBreakableLines(const QTextBlockFormat* this_ptr);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QTextBlockFormat_pageBreakPolicy(const QTextBlockFormat* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextBlockFormat_rightMargin(const QTextBlockFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlockFormat_setBottomMargin(QTextBlockFormat* this_ptr, double margin);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlockFormat_setIndent(QTextBlockFormat* this_ptr, int indent);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlockFormat_setLeftMargin(QTextBlockFormat* this_ptr, double margin);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlockFormat_setLineHeight(QTextBlockFormat* this_ptr, double height, int heightType);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlockFormat_setNonBreakableLines(QTextBlockFormat* this_ptr, bool b);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlockFormat_setPageBreakPolicy(QTextBlockFormat* this_ptr, unsigned int flags);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlockFormat_setRightMargin(QTextBlockFormat* this_ptr, double margin);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlockFormat_setTabPositions(QTextBlockFormat* this_ptr, const QList< QTextOption::Tab >* tabs);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlockFormat_setTextIndent(QTextBlockFormat* this_ptr, double aindent);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlockFormat_setTopMargin(QTextBlockFormat* this_ptr, double margin);
QT_GUI_C_EXPORT void qt_gui_c_QTextBlockFormat_tabPositions_to_output(const QTextBlockFormat* this_ptr, QList< QTextOption::Tab >* output);
QT_GUI_C_EXPORT double qt_gui_c_QTextBlockFormat_textIndent(const QTextBlockFormat* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextBlockFormat_topMargin(const QTextBlockFormat* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QTEXTBLOCKFORMAT_H
