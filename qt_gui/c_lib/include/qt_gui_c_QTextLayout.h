#ifndef QT_GUI_C_QTEXTLAYOUT_H
#define QT_GUI_C_QTEXTLAYOUT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_FormatRange_delete(QTextLayout::FormatRange* this_ptr);
QT_GUI_C_EXPORT const QTextCharFormat* qt_gui_c_QTextLayout_FormatRange_format(const QTextLayout::FormatRange* this_ptr);
QT_GUI_C_EXPORT QTextCharFormat* qt_gui_c_QTextLayout_FormatRange_format_mut(QTextLayout::FormatRange* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextLayout_FormatRange_length(const QTextLayout::FormatRange* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_FormatRange_set_format(QTextLayout::FormatRange* this_ptr, const QTextCharFormat* value);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_FormatRange_set_length(QTextLayout::FormatRange* this_ptr, int value);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_FormatRange_set_start(QTextLayout::FormatRange* this_ptr, int value);
QT_GUI_C_EXPORT int qt_gui_c_QTextLayout_FormatRange_start(const QTextLayout::FormatRange* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_additionalFormats_to_output(const QTextLayout* this_ptr, QList< QTextLayout::FormatRange >* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_beginLayout(QTextLayout* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_boundingRect_to_output(const QTextLayout* this_ptr, QRectF* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTextLayout_cacheEnabled(const QTextLayout* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_clearAdditionalFormats(QTextLayout* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_clearFormats(QTextLayout* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_clearLayout(QTextLayout* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_createLine_to_output(QTextLayout* this_ptr, QTextLine* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_delete(QTextLayout* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_drawCursor_p_pos_cursorPosition(const QTextLayout* this_ptr, QPainter* p, const QPointF* pos, int cursorPosition);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_drawCursor_p_pos_cursorPosition_width(const QTextLayout* this_ptr, QPainter* p, const QPointF* pos, int cursorPosition, int width);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_draw_p_pos(const QTextLayout* this_ptr, QPainter* p, const QPointF* pos);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_draw_p_pos_selections(const QTextLayout* this_ptr, QPainter* p, const QPointF* pos, const QVector< QTextLayout::FormatRange >* selections);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_draw_p_pos_selections_clip(const QTextLayout* this_ptr, QPainter* p, const QPointF* pos, const QVector< QTextLayout::FormatRange >* selections, const QRectF* clip);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_endLayout(QTextLayout* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_font_to_output(const QTextLayout* this_ptr, QFont* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_formats_to_output(const QTextLayout* this_ptr, QVector< QTextLayout::FormatRange >* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_glyphRuns_to_output_from(const QTextLayout* this_ptr, int from, QList< QGlyphRun >* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_glyphRuns_to_output_from_length(const QTextLayout* this_ptr, int from, int length, QList< QGlyphRun >* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_glyphRuns_to_output_no_args(const QTextLayout* this_ptr, QList< QGlyphRun >* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTextLayout_isValidCursorPosition(const QTextLayout* this_ptr, int pos);
QT_GUI_C_EXPORT int qt_gui_c_QTextLayout_leftCursorPosition(const QTextLayout* this_ptr, int oldPos);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_lineAt_to_output(const QTextLayout* this_ptr, int i, QTextLine* output);
QT_GUI_C_EXPORT int qt_gui_c_QTextLayout_lineCount(const QTextLayout* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_lineForTextPosition_to_output(const QTextLayout* this_ptr, int pos, QTextLine* output);
QT_GUI_C_EXPORT double qt_gui_c_QTextLayout_maximumWidth(const QTextLayout* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextLayout_minimumWidth(const QTextLayout* this_ptr);
QT_GUI_C_EXPORT QTextLayout* qt_gui_c_QTextLayout_new_b(const QTextBlock* b);
QT_GUI_C_EXPORT QTextLayout* qt_gui_c_QTextLayout_new_no_args();
QT_GUI_C_EXPORT QTextLayout* qt_gui_c_QTextLayout_new_text(const QString* text);
QT_GUI_C_EXPORT QTextLayout* qt_gui_c_QTextLayout_new_text_font(const QString* text, const QFont* font);
QT_GUI_C_EXPORT QTextLayout* qt_gui_c_QTextLayout_new_text_font_paintdevice(const QString* text, const QFont* font, QPaintDevice* paintdevice);
QT_GUI_C_EXPORT int qt_gui_c_QTextLayout_nextCursorPosition_oldPos(const QTextLayout* this_ptr, int oldPos);
QT_GUI_C_EXPORT int qt_gui_c_QTextLayout_nextCursorPosition_oldPos_mode(const QTextLayout* this_ptr, int oldPos, QTextLayout::CursorMode mode);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_position_to_output(const QTextLayout* this_ptr, QPointF* output);
QT_GUI_C_EXPORT int qt_gui_c_QTextLayout_preeditAreaPosition(const QTextLayout* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_preeditAreaText_to_output(const QTextLayout* this_ptr, QString* output);
QT_GUI_C_EXPORT int qt_gui_c_QTextLayout_previousCursorPosition_oldPos(const QTextLayout* this_ptr, int oldPos);
QT_GUI_C_EXPORT int qt_gui_c_QTextLayout_previousCursorPosition_oldPos_mode(const QTextLayout* this_ptr, int oldPos, QTextLayout::CursorMode mode);
QT_GUI_C_EXPORT int qt_gui_c_QTextLayout_rightCursorPosition(const QTextLayout* this_ptr, int oldPos);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_setAdditionalFormats(QTextLayout* this_ptr, const QList< QTextLayout::FormatRange >* overrides);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_setCacheEnabled(QTextLayout* this_ptr, bool enable);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_setCursorMoveStyle(QTextLayout* this_ptr, const Qt::CursorMoveStyle* style);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_setFlags(QTextLayout* this_ptr, int flags);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_setFont(QTextLayout* this_ptr, const QFont* f);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_setFormats(QTextLayout* this_ptr, const QVector< QTextLayout::FormatRange >* overrides);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_setPosition(QTextLayout* this_ptr, const QPointF* p);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_setPreeditArea(QTextLayout* this_ptr, int position, const QString* text);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_setRawFont(QTextLayout* this_ptr, const QRawFont* rawFont);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_setText(QTextLayout* this_ptr, const QString* string);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_setTextOption(QTextLayout* this_ptr, const QTextOption* option);
QT_GUI_C_EXPORT const QTextOption* qt_gui_c_QTextLayout_textOption(const QTextLayout* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextLayout_text_to_output(const QTextLayout* this_ptr, QString* output);

} // extern "C"

#endif // QT_GUI_C_QTEXTLAYOUT_H
