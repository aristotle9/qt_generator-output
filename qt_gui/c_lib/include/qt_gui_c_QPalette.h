#ifndef QT_GUI_C_QPALETTE_H
#define QT_GUI_C_QPALETTE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QPalette_G_operator_shl_to_output(const QDebug* arg1, const QPalette* arg2, QDebug* output);
QT_GUI_C_EXPORT QDataStream* qt_gui_c_QPalette_G_operator_shr(QDataStream* ds, QPalette* p);
QT_GUI_C_EXPORT void qt_gui_c_QPalette_G_swap(QPalette* value1, QPalette* value2);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPalette_alternateBase(const QPalette* this_ptr);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPalette_background(const QPalette* this_ptr);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPalette_base(const QPalette* this_ptr);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPalette_brightText(const QPalette* this_ptr);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPalette_brush_cg_cr(const QPalette* this_ptr, QPalette::ColorGroup cg, QPalette::ColorRole cr);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPalette_brush_cr(const QPalette* this_ptr, QPalette::ColorRole cr);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPalette_button(const QPalette* this_ptr);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPalette_buttonText(const QPalette* this_ptr);
QT_GUI_C_EXPORT qint64 qt_gui_c_QPalette_cacheKey(const QPalette* this_ptr);
QT_GUI_C_EXPORT const QColor* qt_gui_c_QPalette_color_cg_cr(const QPalette* this_ptr, QPalette::ColorGroup cg, QPalette::ColorRole cr);
QT_GUI_C_EXPORT const QColor* qt_gui_c_QPalette_color_cr(const QPalette* this_ptr, QPalette::ColorRole cr);
QT_GUI_C_EXPORT void qt_gui_c_QPalette_constructor_QBrush_QBrush_QBrush_QBrush_QBrush_QBrush_QBrush_QBrush_QBrush(const QBrush* windowText, const QBrush* button, const QBrush* light, const QBrush* dark, const QBrush* mid, const QBrush* text, const QBrush* bright_text, const QBrush* base, const QBrush* window, QPalette* output);
QT_GUI_C_EXPORT void qt_gui_c_QPalette_constructor_QColor(const QColor* button, QPalette* output);
QT_GUI_C_EXPORT void qt_gui_c_QPalette_constructor_QColor_QColor(const QColor* button, const QColor* window, QPalette* output);
QT_GUI_C_EXPORT void qt_gui_c_QPalette_constructor_QColor_QColor_QColor_QColor_QColor_QColor_QColor(const QColor* windowText, const QColor* window, const QColor* light, const QColor* dark, const QColor* mid, const QColor* text, const QColor* base, QPalette* output);
QT_GUI_C_EXPORT void qt_gui_c_QPalette_constructor_QPalette(const QPalette* palette, QPalette* output);
QT_GUI_C_EXPORT void qt_gui_c_QPalette_constructor_Qt_GlobalColor(const Qt::GlobalColor* button, QPalette* output);
QT_GUI_C_EXPORT void qt_gui_c_QPalette_constructor_no_args(QPalette* output);
QT_GUI_C_EXPORT void qt_gui_c_QPalette_convert_to_QVariant_to_output(const QPalette* this_ptr, QVariant* output);
QT_GUI_C_EXPORT QPalette::ColorGroup qt_gui_c_QPalette_currentColorGroup(const QPalette* this_ptr);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPalette_dark(const QPalette* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPalette_destructor(QPalette* this_ptr);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPalette_foreground(const QPalette* this_ptr);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPalette_highlight(const QPalette* this_ptr);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPalette_highlightedText(const QPalette* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPalette_isBrushSet(const QPalette* this_ptr, QPalette::ColorGroup cg, QPalette::ColorRole cr);
QT_GUI_C_EXPORT bool qt_gui_c_QPalette_isCopyOf(const QPalette* this_ptr, const QPalette* p);
QT_GUI_C_EXPORT bool qt_gui_c_QPalette_isEqual(const QPalette* this_ptr, QPalette::ColorGroup cr1, QPalette::ColorGroup cr2);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPalette_light(const QPalette* this_ptr);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPalette_link(const QPalette* this_ptr);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPalette_linkVisited(const QPalette* this_ptr);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPalette_mid(const QPalette* this_ptr);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPalette_midlight(const QPalette* this_ptr);
QT_GUI_C_EXPORT QPalette* qt_gui_c_QPalette_operator_assign(QPalette* this_ptr, const QPalette* palette);
QT_GUI_C_EXPORT bool qt_gui_c_QPalette_operator_eq(const QPalette* this_ptr, const QPalette* p);
QT_GUI_C_EXPORT bool qt_gui_c_QPalette_operator_neq(const QPalette* this_ptr, const QPalette* p);
QT_GUI_C_EXPORT void qt_gui_c_QPalette_resolve_mask(QPalette* this_ptr, unsigned int mask);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QPalette_resolve_no_args(const QPalette* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPalette_resolve_to_output(const QPalette* this_ptr, const QPalette* arg1, QPalette* output);
QT_GUI_C_EXPORT void qt_gui_c_QPalette_setBrush_cg_cr_brush(QPalette* this_ptr, QPalette::ColorGroup cg, QPalette::ColorRole cr, const QBrush* brush);
QT_GUI_C_EXPORT void qt_gui_c_QPalette_setBrush_cr_brush(QPalette* this_ptr, QPalette::ColorRole cr, const QBrush* brush);
QT_GUI_C_EXPORT void qt_gui_c_QPalette_setColorGroup(QPalette* this_ptr, QPalette::ColorGroup cr, const QBrush* windowText, const QBrush* button, const QBrush* light, const QBrush* dark, const QBrush* mid, const QBrush* text, const QBrush* bright_text, const QBrush* base, const QBrush* window);
QT_GUI_C_EXPORT void qt_gui_c_QPalette_setColor_cg_cr_color(QPalette* this_ptr, QPalette::ColorGroup cg, QPalette::ColorRole cr, const QColor* color);
QT_GUI_C_EXPORT void qt_gui_c_QPalette_setColor_cr_color(QPalette* this_ptr, QPalette::ColorRole cr, const QColor* color);
QT_GUI_C_EXPORT void qt_gui_c_QPalette_setCurrentColorGroup(QPalette* this_ptr, QPalette::ColorGroup cg);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPalette_shadow(const QPalette* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPalette_swap(QPalette* this_ptr, QPalette* other);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPalette_text(const QPalette* this_ptr);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPalette_toolTipBase(const QPalette* this_ptr);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPalette_toolTipText(const QPalette* this_ptr);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPalette_window(const QPalette* this_ptr);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPalette_windowText(const QPalette* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QPALETTE_H
