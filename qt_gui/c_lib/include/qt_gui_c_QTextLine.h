#ifndef QT_GUI_C_QTEXTLINE_H
#define QT_GUI_C_QTEXTLINE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT double qt_gui_c_QTextLine_ascent(const QTextLine* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextLine_constructor(QTextLine* output);
QT_GUI_C_EXPORT double qt_gui_c_QTextLine_cursorToX_int(const QTextLine* this_ptr, int cursorPos);
QT_GUI_C_EXPORT double qt_gui_c_QTextLine_cursorToX_int_QTextLine_Edge(const QTextLine* this_ptr, int cursorPos, QTextLine::Edge edge);
QT_GUI_C_EXPORT double qt_gui_c_QTextLine_cursorToX_int_ptr(const QTextLine* this_ptr, int* cursorPos);
QT_GUI_C_EXPORT double qt_gui_c_QTextLine_cursorToX_int_ptr_QTextLine_Edge(const QTextLine* this_ptr, int* cursorPos, QTextLine::Edge edge);
QT_GUI_C_EXPORT double qt_gui_c_QTextLine_descent(const QTextLine* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextLine_destructor(QTextLine* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextLine_draw_p_point(const QTextLine* this_ptr, QPainter* p, const QPointF* point);
QT_GUI_C_EXPORT void qt_gui_c_QTextLine_draw_p_point_selection(const QTextLine* this_ptr, QPainter* p, const QPointF* point, const QTextLayout::FormatRange* selection);
QT_GUI_C_EXPORT void qt_gui_c_QTextLine_glyphRuns_to_output_from(const QTextLine* this_ptr, int from, QList< QGlyphRun >* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextLine_glyphRuns_to_output_from_length(const QTextLine* this_ptr, int from, int length, QList< QGlyphRun >* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextLine_glyphRuns_to_output_no_args(const QTextLine* this_ptr, QList< QGlyphRun >* output);
QT_GUI_C_EXPORT double qt_gui_c_QTextLine_height(const QTextLine* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextLine_horizontalAdvance(const QTextLine* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextLine_isValid(const QTextLine* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextLine_leading(const QTextLine* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextLine_leadingIncluded(const QTextLine* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextLine_lineNumber(const QTextLine* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextLine_naturalTextRect_to_output(const QTextLine* this_ptr, QRectF* output);
QT_GUI_C_EXPORT double qt_gui_c_QTextLine_naturalTextWidth(const QTextLine* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextLine_position_to_output(const QTextLine* this_ptr, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextLine_rect_to_output(const QTextLine* this_ptr, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextLine_setLeadingIncluded(QTextLine* this_ptr, bool included);
QT_GUI_C_EXPORT void qt_gui_c_QTextLine_setLineWidth(QTextLine* this_ptr, double width);
QT_GUI_C_EXPORT void qt_gui_c_QTextLine_setNumColumns_columns(QTextLine* this_ptr, int columns);
QT_GUI_C_EXPORT void qt_gui_c_QTextLine_setNumColumns_columns_alignmentWidth(QTextLine* this_ptr, int columns, double alignmentWidth);
QT_GUI_C_EXPORT void qt_gui_c_QTextLine_setPosition(QTextLine* this_ptr, const QPointF* pos);
QT_GUI_C_EXPORT int qt_gui_c_QTextLine_textLength(const QTextLine* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextLine_textStart(const QTextLine* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextLine_width(const QTextLine* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextLine_x(const QTextLine* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextLine_xToCursor_x(const QTextLine* this_ptr, double x);
QT_GUI_C_EXPORT int qt_gui_c_QTextLine_xToCursor_x_arg2(const QTextLine* this_ptr, double x, QTextLine::CursorPosition arg2);
QT_GUI_C_EXPORT double qt_gui_c_QTextLine_y(const QTextLine* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QTEXTLINE_H
