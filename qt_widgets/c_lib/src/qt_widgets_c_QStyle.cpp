#include "qt_widgets_c_QStyle.h"

QObject* qt_widgets_c_QStyle_G_static_cast_QObject_ptr(QStyle* ptr) {
  return static_cast<QObject*>(ptr);
}

QStyle* qt_widgets_c_QStyle_G_static_cast_QStyle_ptr(QObject* ptr) {
  return static_cast<QStyle*>(ptr);
}

void qt_widgets_c_QStyle_delete(QStyle* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QStyle_drawComplexControl_cc_opt_p(const QStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QPainter* p) {
  this_ptr->drawComplexControl(cc, opt, p);
}

void qt_widgets_c_QStyle_drawComplexControl_cc_opt_p_widget(const QStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QPainter* p, const QWidget* widget) {
  this_ptr->drawComplexControl(cc, opt, p, widget);
}

void qt_widgets_c_QStyle_drawControl_element_opt_p(const QStyle* this_ptr, QStyle::ControlElement element, const QStyleOption* opt, QPainter* p) {
  this_ptr->drawControl(element, opt, p);
}

void qt_widgets_c_QStyle_drawControl_element_opt_p_w(const QStyle* this_ptr, QStyle::ControlElement element, const QStyleOption* opt, QPainter* p, const QWidget* w) {
  this_ptr->drawControl(element, opt, p, w);
}

void qt_widgets_c_QStyle_drawItemPixmap(const QStyle* this_ptr, QPainter* painter, const QRect* rect, int alignment, const QPixmap* pixmap) {
  this_ptr->drawItemPixmap(painter, *rect, alignment, *pixmap);
}

void qt_widgets_c_QStyle_drawItemText_painter_rect_flags_pal_enabled_text(const QStyle* this_ptr, QPainter* painter, const QRect* rect, int flags, const QPalette* pal, bool enabled, const QString* text) {
  this_ptr->drawItemText(painter, *rect, flags, *pal, enabled, *text);
}

void qt_widgets_c_QStyle_drawItemText_painter_rect_flags_pal_enabled_text_textRole(const QStyle* this_ptr, QPainter* painter, const QRect* rect, int flags, const QPalette* pal, bool enabled, const QString* text, const QPalette::ColorRole* textRole) {
  this_ptr->drawItemText(painter, *rect, flags, *pal, enabled, *text, *textRole);
}

void qt_widgets_c_QStyle_drawPrimitive_pe_opt_p(const QStyle* this_ptr, QStyle::PrimitiveElement pe, const QStyleOption* opt, QPainter* p) {
  this_ptr->drawPrimitive(pe, opt, p);
}

void qt_widgets_c_QStyle_drawPrimitive_pe_opt_p_w(const QStyle* this_ptr, QStyle::PrimitiveElement pe, const QStyleOption* opt, QPainter* p, const QWidget* w) {
  this_ptr->drawPrimitive(pe, opt, p, w);
}

QPixmap* qt_widgets_c_QStyle_generatedIconPixmap_as_ptr(const QStyle* this_ptr, const QIcon::Mode* iconMode, const QPixmap* pixmap, const QStyleOption* opt) {
  return new QPixmap(this_ptr->generatedIconPixmap(*iconMode, *pixmap, opt));
}

QStyle::SubControl qt_widgets_c_QStyle_hitTestComplexControl_cc_opt_pt(const QStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, const QPoint* pt) {
  return this_ptr->hitTestComplexControl(cc, opt, *pt);
}

QStyle::SubControl qt_widgets_c_QStyle_hitTestComplexControl_cc_opt_pt_widget(const QStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, const QPoint* pt, const QWidget* widget) {
  return this_ptr->hitTestComplexControl(cc, opt, *pt, widget);
}

void qt_widgets_c_QStyle_itemPixmapRect_to_output(const QStyle* this_ptr, const QRect* r, int flags, const QPixmap* pixmap, QRect* output) {
  new(output) QRect(this_ptr->itemPixmapRect(*r, flags, *pixmap));
}

void qt_widgets_c_QStyle_itemTextRect_to_output(const QStyle* this_ptr, const QFontMetrics* fm, const QRect* r, int flags, bool enabled, const QString* text, QRect* output) {
  new(output) QRect(this_ptr->itemTextRect(*fm, *r, flags, enabled, *text));
}

int qt_widgets_c_QStyle_layoutSpacing_control1_control2_orientation(const QStyle* this_ptr, const QSizePolicy::ControlType* control1, const QSizePolicy::ControlType* control2, const Qt::Orientation* orientation) {
  return this_ptr->layoutSpacing(*control1, *control2, *orientation);
}

int qt_widgets_c_QStyle_layoutSpacing_control1_control2_orientation_option(const QStyle* this_ptr, const QSizePolicy::ControlType* control1, const QSizePolicy::ControlType* control2, const Qt::Orientation* orientation, const QStyleOption* option) {
  return this_ptr->layoutSpacing(*control1, *control2, *orientation, option);
}

int qt_widgets_c_QStyle_layoutSpacing_control1_control2_orientation_option_widget(const QStyle* this_ptr, const QSizePolicy::ControlType* control1, const QSizePolicy::ControlType* control2, const Qt::Orientation* orientation, const QStyleOption* option, const QWidget* widget) {
  return this_ptr->layoutSpacing(*control1, *control2, *orientation, option, widget);
}

const QMetaObject* qt_widgets_c_QStyle_metaObject(const QStyle* this_ptr) {
  return this_ptr->metaObject();
}

int qt_widgets_c_QStyle_pixelMetric_metric(const QStyle* this_ptr, QStyle::PixelMetric metric) {
  return this_ptr->pixelMetric(metric);
}

int qt_widgets_c_QStyle_pixelMetric_metric_option(const QStyle* this_ptr, QStyle::PixelMetric metric, const QStyleOption* option) {
  return this_ptr->pixelMetric(metric, option);
}

int qt_widgets_c_QStyle_pixelMetric_metric_option_widget(const QStyle* this_ptr, QStyle::PixelMetric metric, const QStyleOption* option, const QWidget* widget) {
  return this_ptr->pixelMetric(metric, option, widget);
}

void qt_widgets_c_QStyle_polish_application(QStyle* this_ptr, QApplication* application) {
  this_ptr->polish(application);
}

void qt_widgets_c_QStyle_polish_palette(QStyle* this_ptr, QPalette* palette) {
  this_ptr->polish(*palette);
}

void qt_widgets_c_QStyle_polish_widget(QStyle* this_ptr, QWidget* widget) {
  this_ptr->polish(widget);
}

const QStyle* qt_widgets_c_QStyle_proxy(const QStyle* this_ptr) {
  return this_ptr->proxy();
}

int qt_widgets_c_QStyle_qt_metacall(QStyle* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QStyle_qt_metacast(QStyle* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QStyle_sizeFromContents_to_output_ct_opt_contentsSize(const QStyle* this_ptr, QStyle::ContentsType ct, const QStyleOption* opt, const QSize* contentsSize, QSize* output) {
  new(output) QSize(this_ptr->sizeFromContents(ct, opt, *contentsSize));
}

void qt_widgets_c_QStyle_sizeFromContents_to_output_ct_opt_contentsSize_w(const QStyle* this_ptr, QStyle::ContentsType ct, const QStyleOption* opt, const QSize* contentsSize, const QWidget* w, QSize* output) {
  new(output) QSize(this_ptr->sizeFromContents(ct, opt, *contentsSize, w));
}

int qt_widgets_c_QStyle_sliderPositionFromValue_min_max_val_space(int min, int max, int val, int space) {
  return QStyle::sliderPositionFromValue(min, max, val, space);
}

int qt_widgets_c_QStyle_sliderPositionFromValue_min_max_val_space_upsideDown(int min, int max, int val, int space, bool upsideDown) {
  return QStyle::sliderPositionFromValue(min, max, val, space, upsideDown);
}

int qt_widgets_c_QStyle_sliderValueFromPosition_min_max_pos_space(int min, int max, int pos, int space) {
  return QStyle::sliderValueFromPosition(min, max, pos, space);
}

int qt_widgets_c_QStyle_sliderValueFromPosition_min_max_pos_space_upsideDown(int min, int max, int pos, int space, bool upsideDown) {
  return QStyle::sliderValueFromPosition(min, max, pos, space, upsideDown);
}

void qt_widgets_c_QStyle_standardIcon_to_output_standardIcon(const QStyle* this_ptr, QStyle::StandardPixmap standardIcon, QIcon* output) {
  new(output) QIcon(this_ptr->standardIcon(standardIcon));
}

void qt_widgets_c_QStyle_standardIcon_to_output_standardIcon_option(const QStyle* this_ptr, QStyle::StandardPixmap standardIcon, const QStyleOption* option, QIcon* output) {
  new(output) QIcon(this_ptr->standardIcon(standardIcon, option));
}

void qt_widgets_c_QStyle_standardIcon_to_output_standardIcon_option_widget(const QStyle* this_ptr, QStyle::StandardPixmap standardIcon, const QStyleOption* option, const QWidget* widget, QIcon* output) {
  new(output) QIcon(this_ptr->standardIcon(standardIcon, option, widget));
}

void qt_widgets_c_QStyle_standardPalette_to_output(const QStyle* this_ptr, QPalette* output) {
  new(output) QPalette(this_ptr->standardPalette());
}

QPixmap* qt_widgets_c_QStyle_standardPixmap_as_ptr_standardPixmap(const QStyle* this_ptr, QStyle::StandardPixmap standardPixmap) {
  return new QPixmap(this_ptr->standardPixmap(standardPixmap));
}

QPixmap* qt_widgets_c_QStyle_standardPixmap_as_ptr_standardPixmap_opt(const QStyle* this_ptr, QStyle::StandardPixmap standardPixmap, const QStyleOption* opt) {
  return new QPixmap(this_ptr->standardPixmap(standardPixmap, opt));
}

QPixmap* qt_widgets_c_QStyle_standardPixmap_as_ptr_standardPixmap_opt_widget(const QStyle* this_ptr, QStyle::StandardPixmap standardPixmap, const QStyleOption* opt, const QWidget* widget) {
  return new QPixmap(this_ptr->standardPixmap(standardPixmap, opt, widget));
}

int qt_widgets_c_QStyle_styleHint_stylehint(const QStyle* this_ptr, QStyle::StyleHint stylehint) {
  return this_ptr->styleHint(stylehint);
}

int qt_widgets_c_QStyle_styleHint_stylehint_opt(const QStyle* this_ptr, QStyle::StyleHint stylehint, const QStyleOption* opt) {
  return this_ptr->styleHint(stylehint, opt);
}

int qt_widgets_c_QStyle_styleHint_stylehint_opt_widget(const QStyle* this_ptr, QStyle::StyleHint stylehint, const QStyleOption* opt, const QWidget* widget) {
  return this_ptr->styleHint(stylehint, opt, widget);
}

int qt_widgets_c_QStyle_styleHint_stylehint_opt_widget_returnData(const QStyle* this_ptr, QStyle::StyleHint stylehint, const QStyleOption* opt, const QWidget* widget, QStyleHintReturn* returnData) {
  return this_ptr->styleHint(stylehint, opt, widget, returnData);
}

void qt_widgets_c_QStyle_subControlRect_to_output_cc_opt_sc(const QStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QStyle::SubControl sc, QRect* output) {
  new(output) QRect(this_ptr->subControlRect(cc, opt, sc));
}

void qt_widgets_c_QStyle_subControlRect_to_output_cc_opt_sc_widget(const QStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QStyle::SubControl sc, const QWidget* widget, QRect* output) {
  new(output) QRect(this_ptr->subControlRect(cc, opt, sc, widget));
}

void qt_widgets_c_QStyle_subElementRect_to_output_subElement_option(const QStyle* this_ptr, QStyle::SubElement subElement, const QStyleOption* option, QRect* output) {
  new(output) QRect(this_ptr->subElementRect(subElement, option));
}

void qt_widgets_c_QStyle_subElementRect_to_output_subElement_option_widget(const QStyle* this_ptr, QStyle::SubElement subElement, const QStyleOption* option, const QWidget* widget, QRect* output) {
  new(output) QRect(this_ptr->subElementRect(subElement, option, widget));
}

void qt_widgets_c_QStyle_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QStyle::trUtf8(s, c, n));
}

void qt_widgets_c_QStyle_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QStyle::tr(s, c, n));
}

void qt_widgets_c_QStyle_unpolish_application(QStyle* this_ptr, QApplication* application) {
  this_ptr->unpolish(application);
}

void qt_widgets_c_QStyle_unpolish_widget(QStyle* this_ptr, QWidget* widget) {
  this_ptr->unpolish(widget);
}

void qt_widgets_c_QStyle_visualPos_to_output(const Qt::LayoutDirection* direction, const QRect* boundingRect, const QPoint* logicalPos, QPoint* output) {
  new(output) QPoint(QStyle::visualPos(*direction, *boundingRect, *logicalPos));
}

void qt_widgets_c_QStyle_visualRect_to_output(const Qt::LayoutDirection* direction, const QRect* boundingRect, const QRect* logicalRect, QRect* output) {
  new(output) QRect(QStyle::visualRect(*direction, *boundingRect, *logicalRect));
}

