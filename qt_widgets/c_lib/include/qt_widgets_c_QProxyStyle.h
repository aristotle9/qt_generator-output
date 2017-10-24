#ifndef QT_WIDGETS_C_QPROXYSTYLE_H
#define QT_WIDGETS_C_QPROXYSTYLE_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QProxyStyle* qt_widgets_c_QProxyStyle_G_dynamic_cast_QProxyStyle_ptr_QCommonStyle(QCommonStyle* ptr);
QT_WIDGETS_C_EXPORT QProxyStyle* qt_widgets_c_QProxyStyle_G_dynamic_cast_QProxyStyle_ptr_QStyle(QStyle* ptr);
QT_WIDGETS_C_EXPORT QCommonStyle* qt_widgets_c_QProxyStyle_G_static_cast_QCommonStyle_ptr(QProxyStyle* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QProxyStyle_G_static_cast_QObject_ptr(QProxyStyle* ptr);
QT_WIDGETS_C_EXPORT QProxyStyle* qt_widgets_c_QProxyStyle_G_static_cast_QProxyStyle_ptr_QCommonStyle(QCommonStyle* ptr);
QT_WIDGETS_C_EXPORT QProxyStyle* qt_widgets_c_QProxyStyle_G_static_cast_QProxyStyle_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QProxyStyle* qt_widgets_c_QProxyStyle_G_static_cast_QProxyStyle_ptr_QStyle(QStyle* ptr);
QT_WIDGETS_C_EXPORT QStyle* qt_widgets_c_QProxyStyle_G_static_cast_QStyle_ptr(QProxyStyle* ptr);
QT_WIDGETS_C_EXPORT QStyle* qt_widgets_c_QProxyStyle_baseStyle(const QProxyStyle* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_delete(QProxyStyle* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_drawComplexControl_control_option_painter(const QProxyStyle* this_ptr, QStyle::ComplexControl control, const QStyleOptionComplex* option, QPainter* painter);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_drawComplexControl_control_option_painter_widget(const QProxyStyle* this_ptr, QStyle::ComplexControl control, const QStyleOptionComplex* option, QPainter* painter, const QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_drawControl_element_option_painter(const QProxyStyle* this_ptr, QStyle::ControlElement element, const QStyleOption* option, QPainter* painter);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_drawControl_element_option_painter_widget(const QProxyStyle* this_ptr, QStyle::ControlElement element, const QStyleOption* option, QPainter* painter, const QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_drawItemPixmap(const QProxyStyle* this_ptr, QPainter* painter, const QRect* rect, int alignment, const QPixmap* pixmap);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_drawItemText_painter_rect_flags_pal_enabled_text(const QProxyStyle* this_ptr, QPainter* painter, const QRect* rect, int flags, const QPalette* pal, bool enabled, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_drawItemText_painter_rect_flags_pal_enabled_text_textRole(const QProxyStyle* this_ptr, QPainter* painter, const QRect* rect, int flags, const QPalette* pal, bool enabled, const QString* text, const QPalette::ColorRole* textRole);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_drawPrimitive_element_option_painter(const QProxyStyle* this_ptr, QStyle::PrimitiveElement element, const QStyleOption* option, QPainter* painter);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_drawPrimitive_element_option_painter_widget(const QProxyStyle* this_ptr, QStyle::PrimitiveElement element, const QStyleOption* option, QPainter* painter, const QWidget* widget);
QT_WIDGETS_C_EXPORT QPixmap* qt_widgets_c_QProxyStyle_generatedIconPixmap_as_ptr(const QProxyStyle* this_ptr, const QIcon::Mode* iconMode, const QPixmap* pixmap, const QStyleOption* opt);
QT_WIDGETS_C_EXPORT QStyle::SubControl qt_widgets_c_QProxyStyle_hitTestComplexControl_control_option_pos(const QProxyStyle* this_ptr, QStyle::ComplexControl control, const QStyleOptionComplex* option, const QPoint* pos);
QT_WIDGETS_C_EXPORT QStyle::SubControl qt_widgets_c_QProxyStyle_hitTestComplexControl_control_option_pos_widget(const QProxyStyle* this_ptr, QStyle::ComplexControl control, const QStyleOptionComplex* option, const QPoint* pos, const QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_itemPixmapRect_to_output(const QProxyStyle* this_ptr, const QRect* r, int flags, const QPixmap* pixmap, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_itemTextRect_to_output(const QProxyStyle* this_ptr, const QFontMetrics* fm, const QRect* r, int flags, bool enabled, const QString* text, QRect* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QProxyStyle_layoutSpacing_control1_control2_orientation(const QProxyStyle* this_ptr, const QSizePolicy::ControlType* control1, const QSizePolicy::ControlType* control2, const Qt::Orientation* orientation);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QProxyStyle_layoutSpacing_control1_control2_orientation_option(const QProxyStyle* this_ptr, const QSizePolicy::ControlType* control1, const QSizePolicy::ControlType* control2, const Qt::Orientation* orientation, const QStyleOption* option);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QProxyStyle_layoutSpacing_control1_control2_orientation_option_widget(const QProxyStyle* this_ptr, const QSizePolicy::ControlType* control1, const QSizePolicy::ControlType* control2, const Qt::Orientation* orientation, const QStyleOption* option, const QWidget* widget);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QProxyStyle_metaObject(const QProxyStyle* this_ptr);
QT_WIDGETS_C_EXPORT QProxyStyle* qt_widgets_c_QProxyStyle_new_key(const QString* key);
QT_WIDGETS_C_EXPORT QProxyStyle* qt_widgets_c_QProxyStyle_new_no_args();
QT_WIDGETS_C_EXPORT QProxyStyle* qt_widgets_c_QProxyStyle_new_style(QStyle* style);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QProxyStyle_pixelMetric_metric(const QProxyStyle* this_ptr, QStyle::PixelMetric metric);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QProxyStyle_pixelMetric_metric_option(const QProxyStyle* this_ptr, QStyle::PixelMetric metric, const QStyleOption* option);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QProxyStyle_pixelMetric_metric_option_widget(const QProxyStyle* this_ptr, QStyle::PixelMetric metric, const QStyleOption* option, const QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_polish_app(QProxyStyle* this_ptr, QApplication* app);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_polish_pal(QProxyStyle* this_ptr, QPalette* pal);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_polish_widget(QProxyStyle* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QProxyStyle_qt_metacall(QProxyStyle* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QProxyStyle_qt_metacast(QProxyStyle* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_setBaseStyle(QProxyStyle* this_ptr, QStyle* style);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_sizeFromContents_to_output(const QProxyStyle* this_ptr, QStyle::ContentsType type, const QStyleOption* option, const QSize* size, const QWidget* widget, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_standardIcon_to_output_standardIcon(const QProxyStyle* this_ptr, QStyle::StandardPixmap standardIcon, QIcon* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_standardIcon_to_output_standardIcon_option(const QProxyStyle* this_ptr, QStyle::StandardPixmap standardIcon, const QStyleOption* option, QIcon* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_standardIcon_to_output_standardIcon_option_widget(const QProxyStyle* this_ptr, QStyle::StandardPixmap standardIcon, const QStyleOption* option, const QWidget* widget, QIcon* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_standardPalette_to_output(const QProxyStyle* this_ptr, QPalette* output);
QT_WIDGETS_C_EXPORT QPixmap* qt_widgets_c_QProxyStyle_standardPixmap_as_ptr_standardPixmap_opt(const QProxyStyle* this_ptr, QStyle::StandardPixmap standardPixmap, const QStyleOption* opt);
QT_WIDGETS_C_EXPORT QPixmap* qt_widgets_c_QProxyStyle_standardPixmap_as_ptr_standardPixmap_opt_widget(const QProxyStyle* this_ptr, QStyle::StandardPixmap standardPixmap, const QStyleOption* opt, const QWidget* widget);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QProxyStyle_styleHint_hint(const QProxyStyle* this_ptr, QStyle::StyleHint hint);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QProxyStyle_styleHint_hint_option(const QProxyStyle* this_ptr, QStyle::StyleHint hint, const QStyleOption* option);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QProxyStyle_styleHint_hint_option_widget(const QProxyStyle* this_ptr, QStyle::StyleHint hint, const QStyleOption* option, const QWidget* widget);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QProxyStyle_styleHint_hint_option_widget_returnData(const QProxyStyle* this_ptr, QStyle::StyleHint hint, const QStyleOption* option, const QWidget* widget, QStyleHintReturn* returnData);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_subControlRect_to_output(const QProxyStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QStyle::SubControl sc, const QWidget* widget, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_subElementRect_to_output(const QProxyStyle* this_ptr, QStyle::SubElement element, const QStyleOption* option, const QWidget* widget, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_unpolish_app(QProxyStyle* this_ptr, QApplication* app);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProxyStyle_unpolish_widget(QProxyStyle* this_ptr, QWidget* widget);

} // extern "C"

#endif // QT_WIDGETS_C_QPROXYSTYLE_H
