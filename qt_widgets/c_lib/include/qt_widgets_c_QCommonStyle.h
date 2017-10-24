#ifndef QT_WIDGETS_C_QCOMMONSTYLE_H
#define QT_WIDGETS_C_QCOMMONSTYLE_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QCommonStyle* qt_widgets_c_QCommonStyle_G_dynamic_cast_QCommonStyle_ptr(QStyle* ptr);
QT_WIDGETS_C_EXPORT QCommonStyle* qt_widgets_c_QCommonStyle_G_static_cast_QCommonStyle_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QCommonStyle* qt_widgets_c_QCommonStyle_G_static_cast_QCommonStyle_ptr_QStyle(QStyle* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QCommonStyle_G_static_cast_QObject_ptr(QCommonStyle* ptr);
QT_WIDGETS_C_EXPORT QStyle* qt_widgets_c_QCommonStyle_G_static_cast_QStyle_ptr(QCommonStyle* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCommonStyle_delete(QCommonStyle* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCommonStyle_drawComplexControl_cc_opt_p(const QCommonStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QPainter* p);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCommonStyle_drawComplexControl_cc_opt_p_w(const QCommonStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QPainter* p, const QWidget* w);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCommonStyle_drawControl_element_opt_p(const QCommonStyle* this_ptr, QStyle::ControlElement element, const QStyleOption* opt, QPainter* p);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCommonStyle_drawControl_element_opt_p_w(const QCommonStyle* this_ptr, QStyle::ControlElement element, const QStyleOption* opt, QPainter* p, const QWidget* w);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCommonStyle_drawPrimitive_pe_opt_p(const QCommonStyle* this_ptr, QStyle::PrimitiveElement pe, const QStyleOption* opt, QPainter* p);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCommonStyle_drawPrimitive_pe_opt_p_w(const QCommonStyle* this_ptr, QStyle::PrimitiveElement pe, const QStyleOption* opt, QPainter* p, const QWidget* w);
QT_WIDGETS_C_EXPORT QPixmap* qt_widgets_c_QCommonStyle_generatedIconPixmap_as_ptr(const QCommonStyle* this_ptr, const QIcon::Mode* iconMode, const QPixmap* pixmap, const QStyleOption* opt);
QT_WIDGETS_C_EXPORT QStyle::SubControl qt_widgets_c_QCommonStyle_hitTestComplexControl_cc_opt_pt(const QCommonStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, const QPoint* pt);
QT_WIDGETS_C_EXPORT QStyle::SubControl qt_widgets_c_QCommonStyle_hitTestComplexControl_cc_opt_pt_w(const QCommonStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, const QPoint* pt, const QWidget* w);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QCommonStyle_layoutSpacing_control1_control2_orientation(const QCommonStyle* this_ptr, const QSizePolicy::ControlType* control1, const QSizePolicy::ControlType* control2, const Qt::Orientation* orientation);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QCommonStyle_layoutSpacing_control1_control2_orientation_option(const QCommonStyle* this_ptr, const QSizePolicy::ControlType* control1, const QSizePolicy::ControlType* control2, const Qt::Orientation* orientation, const QStyleOption* option);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QCommonStyle_layoutSpacing_control1_control2_orientation_option_widget(const QCommonStyle* this_ptr, const QSizePolicy::ControlType* control1, const QSizePolicy::ControlType* control2, const Qt::Orientation* orientation, const QStyleOption* option, const QWidget* widget);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QCommonStyle_metaObject(const QCommonStyle* this_ptr);
QT_WIDGETS_C_EXPORT QCommonStyle* qt_widgets_c_QCommonStyle_new();
QT_WIDGETS_C_EXPORT int qt_widgets_c_QCommonStyle_pixelMetric_m(const QCommonStyle* this_ptr, QStyle::PixelMetric m);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QCommonStyle_pixelMetric_m_opt(const QCommonStyle* this_ptr, QStyle::PixelMetric m, const QStyleOption* opt);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QCommonStyle_pixelMetric_m_opt_widget(const QCommonStyle* this_ptr, QStyle::PixelMetric m, const QStyleOption* opt, const QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCommonStyle_polish_app(QCommonStyle* this_ptr, QApplication* app);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCommonStyle_polish_arg1(QCommonStyle* this_ptr, QPalette* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCommonStyle_polish_widget(QCommonStyle* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QCommonStyle_qt_metacall(QCommonStyle* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QCommonStyle_qt_metacast(QCommonStyle* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCommonStyle_sizeFromContents_to_output_ct_opt_contentsSize(const QCommonStyle* this_ptr, QStyle::ContentsType ct, const QStyleOption* opt, const QSize* contentsSize, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCommonStyle_sizeFromContents_to_output_ct_opt_contentsSize_widget(const QCommonStyle* this_ptr, QStyle::ContentsType ct, const QStyleOption* opt, const QSize* contentsSize, const QWidget* widget, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCommonStyle_standardIcon_to_output_standardIcon(const QCommonStyle* this_ptr, QStyle::StandardPixmap standardIcon, QIcon* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCommonStyle_standardIcon_to_output_standardIcon_opt(const QCommonStyle* this_ptr, QStyle::StandardPixmap standardIcon, const QStyleOption* opt, QIcon* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCommonStyle_standardIcon_to_output_standardIcon_opt_widget(const QCommonStyle* this_ptr, QStyle::StandardPixmap standardIcon, const QStyleOption* opt, const QWidget* widget, QIcon* output);
QT_WIDGETS_C_EXPORT QPixmap* qt_widgets_c_QCommonStyle_standardPixmap_as_ptr_sp(const QCommonStyle* this_ptr, QStyle::StandardPixmap sp);
QT_WIDGETS_C_EXPORT QPixmap* qt_widgets_c_QCommonStyle_standardPixmap_as_ptr_sp_opt(const QCommonStyle* this_ptr, QStyle::StandardPixmap sp, const QStyleOption* opt);
QT_WIDGETS_C_EXPORT QPixmap* qt_widgets_c_QCommonStyle_standardPixmap_as_ptr_sp_opt_widget(const QCommonStyle* this_ptr, QStyle::StandardPixmap sp, const QStyleOption* opt, const QWidget* widget);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QCommonStyle_styleHint_sh(const QCommonStyle* this_ptr, QStyle::StyleHint sh);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QCommonStyle_styleHint_sh_opt(const QCommonStyle* this_ptr, QStyle::StyleHint sh, const QStyleOption* opt);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QCommonStyle_styleHint_sh_opt_w(const QCommonStyle* this_ptr, QStyle::StyleHint sh, const QStyleOption* opt, const QWidget* w);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QCommonStyle_styleHint_sh_opt_w_shret(const QCommonStyle* this_ptr, QStyle::StyleHint sh, const QStyleOption* opt, const QWidget* w, QStyleHintReturn* shret);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCommonStyle_subControlRect_to_output_cc_opt_sc(const QCommonStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QStyle::SubControl sc, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCommonStyle_subControlRect_to_output_cc_opt_sc_w(const QCommonStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QStyle::SubControl sc, const QWidget* w, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCommonStyle_subElementRect_to_output_r_opt(const QCommonStyle* this_ptr, QStyle::SubElement r, const QStyleOption* opt, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCommonStyle_subElementRect_to_output_r_opt_widget(const QCommonStyle* this_ptr, QStyle::SubElement r, const QStyleOption* opt, const QWidget* widget, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCommonStyle_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCommonStyle_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCommonStyle_unpolish_application(QCommonStyle* this_ptr, QApplication* application);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCommonStyle_unpolish_widget(QCommonStyle* this_ptr, QWidget* widget);

} // extern "C"

#endif // QT_WIDGETS_C_QCOMMONSTYLE_H
