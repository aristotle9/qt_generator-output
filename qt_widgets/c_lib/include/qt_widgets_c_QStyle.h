#ifndef QT_WIDGETS_C_QSTYLE_H
#define QT_WIDGETS_C_QSTYLE_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QStyle_G_static_cast_QObject_ptr(QStyle* ptr);
QT_WIDGETS_C_EXPORT QStyle* qt_widgets_c_QStyle_G_static_cast_QStyle_ptr(QObject* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_delete(QStyle* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_drawComplexControl_cc_opt_p(const QStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QPainter* p);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_drawComplexControl_cc_opt_p_widget(const QStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QPainter* p, const QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_drawControl_element_opt_p(const QStyle* this_ptr, QStyle::ControlElement element, const QStyleOption* opt, QPainter* p);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_drawControl_element_opt_p_w(const QStyle* this_ptr, QStyle::ControlElement element, const QStyleOption* opt, QPainter* p, const QWidget* w);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_drawItemPixmap(const QStyle* this_ptr, QPainter* painter, const QRect* rect, int alignment, const QPixmap* pixmap);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_drawItemText_painter_rect_flags_pal_enabled_text(const QStyle* this_ptr, QPainter* painter, const QRect* rect, int flags, const QPalette* pal, bool enabled, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_drawItemText_painter_rect_flags_pal_enabled_text_textRole(const QStyle* this_ptr, QPainter* painter, const QRect* rect, int flags, const QPalette* pal, bool enabled, const QString* text, const QPalette::ColorRole* textRole);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_drawPrimitive_pe_opt_p(const QStyle* this_ptr, QStyle::PrimitiveElement pe, const QStyleOption* opt, QPainter* p);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_drawPrimitive_pe_opt_p_w(const QStyle* this_ptr, QStyle::PrimitiveElement pe, const QStyleOption* opt, QPainter* p, const QWidget* w);
QT_WIDGETS_C_EXPORT QPixmap* qt_widgets_c_QStyle_generatedIconPixmap_as_ptr(const QStyle* this_ptr, const QIcon::Mode* iconMode, const QPixmap* pixmap, const QStyleOption* opt);
QT_WIDGETS_C_EXPORT QStyle::SubControl qt_widgets_c_QStyle_hitTestComplexControl_cc_opt_pt(const QStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, const QPoint* pt);
QT_WIDGETS_C_EXPORT QStyle::SubControl qt_widgets_c_QStyle_hitTestComplexControl_cc_opt_pt_widget(const QStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, const QPoint* pt, const QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_itemPixmapRect_to_output(const QStyle* this_ptr, const QRect* r, int flags, const QPixmap* pixmap, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_itemTextRect_to_output(const QStyle* this_ptr, const QFontMetrics* fm, const QRect* r, int flags, bool enabled, const QString* text, QRect* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyle_layoutSpacing_control1_control2_orientation(const QStyle* this_ptr, const QSizePolicy::ControlType* control1, const QSizePolicy::ControlType* control2, const Qt::Orientation* orientation);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyle_layoutSpacing_control1_control2_orientation_option(const QStyle* this_ptr, const QSizePolicy::ControlType* control1, const QSizePolicy::ControlType* control2, const Qt::Orientation* orientation, const QStyleOption* option);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyle_layoutSpacing_control1_control2_orientation_option_widget(const QStyle* this_ptr, const QSizePolicy::ControlType* control1, const QSizePolicy::ControlType* control2, const Qt::Orientation* orientation, const QStyleOption* option, const QWidget* widget);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QStyle_metaObject(const QStyle* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyle_pixelMetric_metric(const QStyle* this_ptr, QStyle::PixelMetric metric);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyle_pixelMetric_metric_option(const QStyle* this_ptr, QStyle::PixelMetric metric, const QStyleOption* option);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyle_pixelMetric_metric_option_widget(const QStyle* this_ptr, QStyle::PixelMetric metric, const QStyleOption* option, const QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_polish_application(QStyle* this_ptr, QApplication* application);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_polish_palette(QStyle* this_ptr, QPalette* palette);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_polish_widget(QStyle* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT const QStyle* qt_widgets_c_QStyle_proxy(const QStyle* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyle_qt_metacall(QStyle* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QStyle_qt_metacast(QStyle* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_sizeFromContents_to_output_ct_opt_contentsSize(const QStyle* this_ptr, QStyle::ContentsType ct, const QStyleOption* opt, const QSize* contentsSize, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_sizeFromContents_to_output_ct_opt_contentsSize_w(const QStyle* this_ptr, QStyle::ContentsType ct, const QStyleOption* opt, const QSize* contentsSize, const QWidget* w, QSize* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyle_sliderPositionFromValue_min_max_val_space(int min, int max, int val, int space);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyle_sliderPositionFromValue_min_max_val_space_upsideDown(int min, int max, int val, int space, bool upsideDown);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyle_sliderValueFromPosition_min_max_pos_space(int min, int max, int pos, int space);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyle_sliderValueFromPosition_min_max_pos_space_upsideDown(int min, int max, int pos, int space, bool upsideDown);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_standardIcon_to_output_standardIcon(const QStyle* this_ptr, QStyle::StandardPixmap standardIcon, QIcon* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_standardIcon_to_output_standardIcon_option(const QStyle* this_ptr, QStyle::StandardPixmap standardIcon, const QStyleOption* option, QIcon* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_standardIcon_to_output_standardIcon_option_widget(const QStyle* this_ptr, QStyle::StandardPixmap standardIcon, const QStyleOption* option, const QWidget* widget, QIcon* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_standardPalette_to_output(const QStyle* this_ptr, QPalette* output);
QT_WIDGETS_C_EXPORT QPixmap* qt_widgets_c_QStyle_standardPixmap_as_ptr_standardPixmap(const QStyle* this_ptr, QStyle::StandardPixmap standardPixmap);
QT_WIDGETS_C_EXPORT QPixmap* qt_widgets_c_QStyle_standardPixmap_as_ptr_standardPixmap_opt(const QStyle* this_ptr, QStyle::StandardPixmap standardPixmap, const QStyleOption* opt);
QT_WIDGETS_C_EXPORT QPixmap* qt_widgets_c_QStyle_standardPixmap_as_ptr_standardPixmap_opt_widget(const QStyle* this_ptr, QStyle::StandardPixmap standardPixmap, const QStyleOption* opt, const QWidget* widget);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyle_styleHint_stylehint(const QStyle* this_ptr, QStyle::StyleHint stylehint);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyle_styleHint_stylehint_opt(const QStyle* this_ptr, QStyle::StyleHint stylehint, const QStyleOption* opt);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyle_styleHint_stylehint_opt_widget(const QStyle* this_ptr, QStyle::StyleHint stylehint, const QStyleOption* opt, const QWidget* widget);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyle_styleHint_stylehint_opt_widget_returnData(const QStyle* this_ptr, QStyle::StyleHint stylehint, const QStyleOption* opt, const QWidget* widget, QStyleHintReturn* returnData);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_subControlRect_to_output_cc_opt_sc(const QStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QStyle::SubControl sc, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_subControlRect_to_output_cc_opt_sc_widget(const QStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QStyle::SubControl sc, const QWidget* widget, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_subElementRect_to_output_subElement_option(const QStyle* this_ptr, QStyle::SubElement subElement, const QStyleOption* option, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_subElementRect_to_output_subElement_option_widget(const QStyle* this_ptr, QStyle::SubElement subElement, const QStyleOption* option, const QWidget* widget, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_unpolish_application(QStyle* this_ptr, QApplication* application);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_unpolish_widget(QStyle* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_visualPos_to_output(const Qt::LayoutDirection* direction, const QRect* boundingRect, const QPoint* logicalPos, QPoint* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyle_visualRect_to_output(const Qt::LayoutDirection* direction, const QRect* boundingRect, const QRect* logicalRect, QRect* output);

} // extern "C"

#endif // QT_WIDGETS_C_QSTYLE_H
