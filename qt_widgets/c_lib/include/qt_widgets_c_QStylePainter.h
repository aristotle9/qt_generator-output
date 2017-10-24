#ifndef QT_WIDGETS_C_QSTYLEPAINTER_H
#define QT_WIDGETS_C_QSTYLEPAINTER_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QPainter* qt_widgets_c_QStylePainter_G_static_cast_QPainter_ptr(QStylePainter* ptr);
QT_WIDGETS_C_EXPORT QStylePainter* qt_widgets_c_QStylePainter_G_static_cast_QStylePainter_ptr(QPainter* ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QStylePainter_begin_pd_w(QStylePainter* this_ptr, QPaintDevice* pd, QWidget* w);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QStylePainter_begin_w(QStylePainter* this_ptr, QWidget* w);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStylePainter_constructor_no_args(QStylePainter* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStylePainter_constructor_pd_w(QPaintDevice* pd, QWidget* w, QStylePainter* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStylePainter_constructor_w(QWidget* w, QStylePainter* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStylePainter_destructor(QStylePainter* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStylePainter_drawComplexControl(QStylePainter* this_ptr, const QStyle::ComplexControl* cc, const QStyleOptionComplex* opt);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStylePainter_drawControl(QStylePainter* this_ptr, const QStyle::ControlElement* ce, const QStyleOption* opt);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStylePainter_drawItemPixmap(QStylePainter* this_ptr, const QRect* r, int flags, const QPixmap* pixmap);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStylePainter_drawItemText_r_flags_pal_enabled_text(QStylePainter* this_ptr, const QRect* r, int flags, const QPalette* pal, bool enabled, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStylePainter_drawItemText_r_flags_pal_enabled_text_textRole(QStylePainter* this_ptr, const QRect* r, int flags, const QPalette* pal, bool enabled, const QString* text, const QPalette::ColorRole* textRole);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStylePainter_drawPrimitive(QStylePainter* this_ptr, const QStyle::PrimitiveElement* pe, const QStyleOption* opt);
QT_WIDGETS_C_EXPORT QStyle* qt_widgets_c_QStylePainter_style(const QStylePainter* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QSTYLEPAINTER_H
