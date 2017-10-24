#ifndef QT_GUI_C_QTEXTLISTFORMAT_H
#define QT_GUI_C_QTEXTLISTFORMAT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QTextFormat* qt_gui_c_QTextListFormat_G_static_cast_QTextFormat_ptr(QTextListFormat* ptr);
QT_GUI_C_EXPORT QTextListFormat* qt_gui_c_QTextListFormat_G_static_cast_QTextListFormat_ptr(QTextFormat* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextListFormat_constructor(QTextListFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextListFormat_destructor(QTextListFormat* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextListFormat_indent(const QTextListFormat* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextListFormat_isValid(const QTextListFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextListFormat_numberPrefix_to_output(const QTextListFormat* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextListFormat_numberSuffix_to_output(const QTextListFormat* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextListFormat_setIndent(QTextListFormat* this_ptr, int indent);
QT_GUI_C_EXPORT void qt_gui_c_QTextListFormat_setNumberPrefix(QTextListFormat* this_ptr, const QString* numberPrefix);
QT_GUI_C_EXPORT void qt_gui_c_QTextListFormat_setNumberSuffix(QTextListFormat* this_ptr, const QString* numberSuffix);
QT_GUI_C_EXPORT void qt_gui_c_QTextListFormat_setStyle(QTextListFormat* this_ptr, QTextListFormat::Style style);
QT_GUI_C_EXPORT QTextListFormat::Style qt_gui_c_QTextListFormat_style(const QTextListFormat* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QTEXTLISTFORMAT_H
