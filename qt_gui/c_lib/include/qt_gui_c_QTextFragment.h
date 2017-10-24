#ifndef QT_GUI_C_QTEXTFRAGMENT_H
#define QT_GUI_C_QTEXTFRAGMENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT int qt_gui_c_QTextFragment_charFormatIndex(const QTextFragment* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextFragment_charFormat_to_output(const QTextFragment* this_ptr, QTextCharFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFragment_constructor_no_args(QTextFragment* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFragment_constructor_o(const QTextFragment* o, QTextFragment* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTextFragment_contains(const QTextFragment* this_ptr, int position);
QT_GUI_C_EXPORT void qt_gui_c_QTextFragment_destructor(QTextFragment* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextFragment_glyphRuns_to_output_from(const QTextFragment* this_ptr, int from, QList< QGlyphRun >* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFragment_glyphRuns_to_output_from_length(const QTextFragment* this_ptr, int from, int length, QList< QGlyphRun >* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFragment_glyphRuns_to_output_no_args(const QTextFragment* this_ptr, QList< QGlyphRun >* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTextFragment_isValid(const QTextFragment* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextFragment_length(const QTextFragment* this_ptr);
QT_GUI_C_EXPORT QTextFragment* qt_gui_c_QTextFragment_operator_assign(QTextFragment* this_ptr, const QTextFragment* o);
QT_GUI_C_EXPORT bool qt_gui_c_QTextFragment_operator_eq(const QTextFragment* this_ptr, const QTextFragment* o);
QT_GUI_C_EXPORT bool qt_gui_c_QTextFragment_operator_lt(const QTextFragment* this_ptr, const QTextFragment* o);
QT_GUI_C_EXPORT bool qt_gui_c_QTextFragment_operator_neq(const QTextFragment* this_ptr, const QTextFragment* o);
QT_GUI_C_EXPORT int qt_gui_c_QTextFragment_position(const QTextFragment* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextFragment_text_to_output(const QTextFragment* this_ptr, QString* output);

} // extern "C"

#endif // QT_GUI_C_QTEXTFRAGMENT_H
