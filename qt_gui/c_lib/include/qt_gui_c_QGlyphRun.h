#ifndef QT_GUI_C_QGLYPHRUN_H
#define QT_GUI_C_QGLYPHRUN_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QGlyphRun_G_swap(QGlyphRun* value1, QGlyphRun* value2);
QT_GUI_C_EXPORT void qt_gui_c_QGlyphRun_boundingRect_to_output(const QGlyphRun* this_ptr, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QGlyphRun_clear(QGlyphRun* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QGlyphRun_constructor_no_args(QGlyphRun* output);
QT_GUI_C_EXPORT void qt_gui_c_QGlyphRun_constructor_other(const QGlyphRun* other, QGlyphRun* output);
QT_GUI_C_EXPORT void qt_gui_c_QGlyphRun_destructor(QGlyphRun* this_ptr);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QGlyphRun_flags(const QGlyphRun* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QGlyphRun_glyphIndexes_to_output(const QGlyphRun* this_ptr, QVector< quint32 >* output);
QT_GUI_C_EXPORT bool qt_gui_c_QGlyphRun_isEmpty(const QGlyphRun* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QGlyphRun_isRightToLeft(const QGlyphRun* this_ptr);
QT_GUI_C_EXPORT QGlyphRun* qt_gui_c_QGlyphRun_operator_assign(QGlyphRun* this_ptr, const QGlyphRun* other);
QT_GUI_C_EXPORT bool qt_gui_c_QGlyphRun_operator_eq(const QGlyphRun* this_ptr, const QGlyphRun* other);
QT_GUI_C_EXPORT bool qt_gui_c_QGlyphRun_operator_neq(const QGlyphRun* this_ptr, const QGlyphRun* other);
QT_GUI_C_EXPORT bool qt_gui_c_QGlyphRun_overline(const QGlyphRun* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QGlyphRun_positions_to_output(const QGlyphRun* this_ptr, QVector< QPointF >* output);
QT_GUI_C_EXPORT void qt_gui_c_QGlyphRun_rawFont_to_output(const QGlyphRun* this_ptr, QRawFont* output);
QT_GUI_C_EXPORT void qt_gui_c_QGlyphRun_setBoundingRect(QGlyphRun* this_ptr, const QRectF* boundingRect);
QT_GUI_C_EXPORT void qt_gui_c_QGlyphRun_setFlag_flag(QGlyphRun* this_ptr, QGlyphRun::GlyphRunFlag flag);
QT_GUI_C_EXPORT void qt_gui_c_QGlyphRun_setFlag_flag_enabled(QGlyphRun* this_ptr, QGlyphRun::GlyphRunFlag flag, bool enabled);
QT_GUI_C_EXPORT void qt_gui_c_QGlyphRun_setFlags(QGlyphRun* this_ptr, unsigned int flags);
QT_GUI_C_EXPORT void qt_gui_c_QGlyphRun_setGlyphIndexes(QGlyphRun* this_ptr, const QVector< quint32 >* glyphIndexes);
QT_GUI_C_EXPORT void qt_gui_c_QGlyphRun_setOverline(QGlyphRun* this_ptr, bool overline);
QT_GUI_C_EXPORT void qt_gui_c_QGlyphRun_setPositions(QGlyphRun* this_ptr, const QVector< QPointF >* positions);
QT_GUI_C_EXPORT void qt_gui_c_QGlyphRun_setRawData(QGlyphRun* this_ptr, const quint32* glyphIndexArray, const QPointF* glyphPositionArray, int size);
QT_GUI_C_EXPORT void qt_gui_c_QGlyphRun_setRawFont(QGlyphRun* this_ptr, const QRawFont* rawFont);
QT_GUI_C_EXPORT void qt_gui_c_QGlyphRun_setRightToLeft(QGlyphRun* this_ptr, bool on);
QT_GUI_C_EXPORT void qt_gui_c_QGlyphRun_setStrikeOut(QGlyphRun* this_ptr, bool strikeOut);
QT_GUI_C_EXPORT void qt_gui_c_QGlyphRun_setUnderline(QGlyphRun* this_ptr, bool underline);
QT_GUI_C_EXPORT bool qt_gui_c_QGlyphRun_strikeOut(const QGlyphRun* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QGlyphRun_swap(QGlyphRun* this_ptr, QGlyphRun* other);
QT_GUI_C_EXPORT bool qt_gui_c_QGlyphRun_underline(const QGlyphRun* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QGLYPHRUN_H
