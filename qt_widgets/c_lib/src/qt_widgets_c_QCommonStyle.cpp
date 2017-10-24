#include "qt_widgets_c_QCommonStyle.h"

QCommonStyle* qt_widgets_c_QCommonStyle_G_dynamic_cast_QCommonStyle_ptr(QStyle* ptr) {
  return dynamic_cast<QCommonStyle*>(ptr);
}

QCommonStyle* qt_widgets_c_QCommonStyle_G_static_cast_QCommonStyle_ptr_QObject(QObject* ptr) {
  return static_cast<QCommonStyle*>(ptr);
}

QCommonStyle* qt_widgets_c_QCommonStyle_G_static_cast_QCommonStyle_ptr_QStyle(QStyle* ptr) {
  return static_cast<QCommonStyle*>(ptr);
}

QObject* qt_widgets_c_QCommonStyle_G_static_cast_QObject_ptr(QCommonStyle* ptr) {
  return static_cast<QObject*>(ptr);
}

QStyle* qt_widgets_c_QCommonStyle_G_static_cast_QStyle_ptr(QCommonStyle* ptr) {
  return static_cast<QStyle*>(ptr);
}

void qt_widgets_c_QCommonStyle_delete(QCommonStyle* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QCommonStyle_drawComplexControl_cc_opt_p(const QCommonStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QPainter* p) {
  this_ptr->drawComplexControl(cc, opt, p);
}

void qt_widgets_c_QCommonStyle_drawComplexControl_cc_opt_p_w(const QCommonStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QPainter* p, const QWidget* w) {
  this_ptr->drawComplexControl(cc, opt, p, w);
}

void qt_widgets_c_QCommonStyle_drawControl_element_opt_p(const QCommonStyle* this_ptr, QStyle::ControlElement element, const QStyleOption* opt, QPainter* p) {
  this_ptr->drawControl(element, opt, p);
}

void qt_widgets_c_QCommonStyle_drawControl_element_opt_p_w(const QCommonStyle* this_ptr, QStyle::ControlElement element, const QStyleOption* opt, QPainter* p, const QWidget* w) {
  this_ptr->drawControl(element, opt, p, w);
}

void qt_widgets_c_QCommonStyle_drawPrimitive_pe_opt_p(const QCommonStyle* this_ptr, QStyle::PrimitiveElement pe, const QStyleOption* opt, QPainter* p) {
  this_ptr->drawPrimitive(pe, opt, p);
}

void qt_widgets_c_QCommonStyle_drawPrimitive_pe_opt_p_w(const QCommonStyle* this_ptr, QStyle::PrimitiveElement pe, const QStyleOption* opt, QPainter* p, const QWidget* w) {
  this_ptr->drawPrimitive(pe, opt, p, w);
}

QPixmap* qt_widgets_c_QCommonStyle_generatedIconPixmap_as_ptr(const QCommonStyle* this_ptr, const QIcon::Mode* iconMode, const QPixmap* pixmap, const QStyleOption* opt) {
  return new QPixmap(this_ptr->generatedIconPixmap(*iconMode, *pixmap, opt));
}

QStyle::SubControl qt_widgets_c_QCommonStyle_hitTestComplexControl_cc_opt_pt(const QCommonStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, const QPoint* pt) {
  return this_ptr->hitTestComplexControl(cc, opt, *pt);
}

QStyle::SubControl qt_widgets_c_QCommonStyle_hitTestComplexControl_cc_opt_pt_w(const QCommonStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, const QPoint* pt, const QWidget* w) {
  return this_ptr->hitTestComplexControl(cc, opt, *pt, w);
}

int qt_widgets_c_QCommonStyle_layoutSpacing_control1_control2_orientation(const QCommonStyle* this_ptr, const QSizePolicy::ControlType* control1, const QSizePolicy::ControlType* control2, const Qt::Orientation* orientation) {
  return this_ptr->layoutSpacing(*control1, *control2, *orientation);
}

int qt_widgets_c_QCommonStyle_layoutSpacing_control1_control2_orientation_option(const QCommonStyle* this_ptr, const QSizePolicy::ControlType* control1, const QSizePolicy::ControlType* control2, const Qt::Orientation* orientation, const QStyleOption* option) {
  return this_ptr->layoutSpacing(*control1, *control2, *orientation, option);
}

int qt_widgets_c_QCommonStyle_layoutSpacing_control1_control2_orientation_option_widget(const QCommonStyle* this_ptr, const QSizePolicy::ControlType* control1, const QSizePolicy::ControlType* control2, const Qt::Orientation* orientation, const QStyleOption* option, const QWidget* widget) {
  return this_ptr->layoutSpacing(*control1, *control2, *orientation, option, widget);
}

const QMetaObject* qt_widgets_c_QCommonStyle_metaObject(const QCommonStyle* this_ptr) {
  return this_ptr->metaObject();
}

QCommonStyle* qt_widgets_c_QCommonStyle_new() {
  return new QCommonStyle();
}

int qt_widgets_c_QCommonStyle_pixelMetric_m(const QCommonStyle* this_ptr, QStyle::PixelMetric m) {
  return this_ptr->pixelMetric(m);
}

int qt_widgets_c_QCommonStyle_pixelMetric_m_opt(const QCommonStyle* this_ptr, QStyle::PixelMetric m, const QStyleOption* opt) {
  return this_ptr->pixelMetric(m, opt);
}

int qt_widgets_c_QCommonStyle_pixelMetric_m_opt_widget(const QCommonStyle* this_ptr, QStyle::PixelMetric m, const QStyleOption* opt, const QWidget* widget) {
  return this_ptr->pixelMetric(m, opt, widget);
}

void qt_widgets_c_QCommonStyle_polish_app(QCommonStyle* this_ptr, QApplication* app) {
  this_ptr->polish(app);
}

void qt_widgets_c_QCommonStyle_polish_arg1(QCommonStyle* this_ptr, QPalette* arg1) {
  this_ptr->polish(*arg1);
}

void qt_widgets_c_QCommonStyle_polish_widget(QCommonStyle* this_ptr, QWidget* widget) {
  this_ptr->polish(widget);
}

int qt_widgets_c_QCommonStyle_qt_metacall(QCommonStyle* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QCommonStyle_qt_metacast(QCommonStyle* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QCommonStyle_sizeFromContents_to_output_ct_opt_contentsSize(const QCommonStyle* this_ptr, QStyle::ContentsType ct, const QStyleOption* opt, const QSize* contentsSize, QSize* output) {
  new(output) QSize(this_ptr->sizeFromContents(ct, opt, *contentsSize));
}

void qt_widgets_c_QCommonStyle_sizeFromContents_to_output_ct_opt_contentsSize_widget(const QCommonStyle* this_ptr, QStyle::ContentsType ct, const QStyleOption* opt, const QSize* contentsSize, const QWidget* widget, QSize* output) {
  new(output) QSize(this_ptr->sizeFromContents(ct, opt, *contentsSize, widget));
}

void qt_widgets_c_QCommonStyle_standardIcon_to_output_standardIcon(const QCommonStyle* this_ptr, QStyle::StandardPixmap standardIcon, QIcon* output) {
  new(output) QIcon(this_ptr->standardIcon(standardIcon));
}

void qt_widgets_c_QCommonStyle_standardIcon_to_output_standardIcon_opt(const QCommonStyle* this_ptr, QStyle::StandardPixmap standardIcon, const QStyleOption* opt, QIcon* output) {
  new(output) QIcon(this_ptr->standardIcon(standardIcon, opt));
}

void qt_widgets_c_QCommonStyle_standardIcon_to_output_standardIcon_opt_widget(const QCommonStyle* this_ptr, QStyle::StandardPixmap standardIcon, const QStyleOption* opt, const QWidget* widget, QIcon* output) {
  new(output) QIcon(this_ptr->standardIcon(standardIcon, opt, widget));
}

QPixmap* qt_widgets_c_QCommonStyle_standardPixmap_as_ptr_sp(const QCommonStyle* this_ptr, QStyle::StandardPixmap sp) {
  return new QPixmap(this_ptr->standardPixmap(sp));
}

QPixmap* qt_widgets_c_QCommonStyle_standardPixmap_as_ptr_sp_opt(const QCommonStyle* this_ptr, QStyle::StandardPixmap sp, const QStyleOption* opt) {
  return new QPixmap(this_ptr->standardPixmap(sp, opt));
}

QPixmap* qt_widgets_c_QCommonStyle_standardPixmap_as_ptr_sp_opt_widget(const QCommonStyle* this_ptr, QStyle::StandardPixmap sp, const QStyleOption* opt, const QWidget* widget) {
  return new QPixmap(this_ptr->standardPixmap(sp, opt, widget));
}

int qt_widgets_c_QCommonStyle_styleHint_sh(const QCommonStyle* this_ptr, QStyle::StyleHint sh) {
  return this_ptr->styleHint(sh);
}

int qt_widgets_c_QCommonStyle_styleHint_sh_opt(const QCommonStyle* this_ptr, QStyle::StyleHint sh, const QStyleOption* opt) {
  return this_ptr->styleHint(sh, opt);
}

int qt_widgets_c_QCommonStyle_styleHint_sh_opt_w(const QCommonStyle* this_ptr, QStyle::StyleHint sh, const QStyleOption* opt, const QWidget* w) {
  return this_ptr->styleHint(sh, opt, w);
}

int qt_widgets_c_QCommonStyle_styleHint_sh_opt_w_shret(const QCommonStyle* this_ptr, QStyle::StyleHint sh, const QStyleOption* opt, const QWidget* w, QStyleHintReturn* shret) {
  return this_ptr->styleHint(sh, opt, w, shret);
}

void qt_widgets_c_QCommonStyle_subControlRect_to_output_cc_opt_sc(const QCommonStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QStyle::SubControl sc, QRect* output) {
  new(output) QRect(this_ptr->subControlRect(cc, opt, sc));
}

void qt_widgets_c_QCommonStyle_subControlRect_to_output_cc_opt_sc_w(const QCommonStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QStyle::SubControl sc, const QWidget* w, QRect* output) {
  new(output) QRect(this_ptr->subControlRect(cc, opt, sc, w));
}

void qt_widgets_c_QCommonStyle_subElementRect_to_output_r_opt(const QCommonStyle* this_ptr, QStyle::SubElement r, const QStyleOption* opt, QRect* output) {
  new(output) QRect(this_ptr->subElementRect(r, opt));
}

void qt_widgets_c_QCommonStyle_subElementRect_to_output_r_opt_widget(const QCommonStyle* this_ptr, QStyle::SubElement r, const QStyleOption* opt, const QWidget* widget, QRect* output) {
  new(output) QRect(this_ptr->subElementRect(r, opt, widget));
}

void qt_widgets_c_QCommonStyle_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QCommonStyle::trUtf8(s, c, n));
}

void qt_widgets_c_QCommonStyle_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QCommonStyle::tr(s, c, n));
}

void qt_widgets_c_QCommonStyle_unpolish_application(QCommonStyle* this_ptr, QApplication* application) {
  this_ptr->unpolish(application);
}

void qt_widgets_c_QCommonStyle_unpolish_widget(QCommonStyle* this_ptr, QWidget* widget) {
  this_ptr->unpolish(widget);
}

