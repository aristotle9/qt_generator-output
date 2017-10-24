#include "qt_widgets_c_QProxyStyle.h"

QProxyStyle* qt_widgets_c_QProxyStyle_G_dynamic_cast_QProxyStyle_ptr_QCommonStyle(QCommonStyle* ptr) {
  return dynamic_cast<QProxyStyle*>(ptr);
}

QProxyStyle* qt_widgets_c_QProxyStyle_G_dynamic_cast_QProxyStyle_ptr_QStyle(QStyle* ptr) {
  return dynamic_cast<QProxyStyle*>(ptr);
}

QCommonStyle* qt_widgets_c_QProxyStyle_G_static_cast_QCommonStyle_ptr(QProxyStyle* ptr) {
  return static_cast<QCommonStyle*>(ptr);
}

QObject* qt_widgets_c_QProxyStyle_G_static_cast_QObject_ptr(QProxyStyle* ptr) {
  return static_cast<QObject*>(ptr);
}

QProxyStyle* qt_widgets_c_QProxyStyle_G_static_cast_QProxyStyle_ptr_QCommonStyle(QCommonStyle* ptr) {
  return static_cast<QProxyStyle*>(ptr);
}

QProxyStyle* qt_widgets_c_QProxyStyle_G_static_cast_QProxyStyle_ptr_QObject(QObject* ptr) {
  return static_cast<QProxyStyle*>(ptr);
}

QProxyStyle* qt_widgets_c_QProxyStyle_G_static_cast_QProxyStyle_ptr_QStyle(QStyle* ptr) {
  return static_cast<QProxyStyle*>(ptr);
}

QStyle* qt_widgets_c_QProxyStyle_G_static_cast_QStyle_ptr(QProxyStyle* ptr) {
  return static_cast<QStyle*>(ptr);
}

QStyle* qt_widgets_c_QProxyStyle_baseStyle(const QProxyStyle* this_ptr) {
  return this_ptr->baseStyle();
}

void qt_widgets_c_QProxyStyle_delete(QProxyStyle* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QProxyStyle_drawComplexControl_control_option_painter(const QProxyStyle* this_ptr, QStyle::ComplexControl control, const QStyleOptionComplex* option, QPainter* painter) {
  this_ptr->drawComplexControl(control, option, painter);
}

void qt_widgets_c_QProxyStyle_drawComplexControl_control_option_painter_widget(const QProxyStyle* this_ptr, QStyle::ComplexControl control, const QStyleOptionComplex* option, QPainter* painter, const QWidget* widget) {
  this_ptr->drawComplexControl(control, option, painter, widget);
}

void qt_widgets_c_QProxyStyle_drawControl_element_option_painter(const QProxyStyle* this_ptr, QStyle::ControlElement element, const QStyleOption* option, QPainter* painter) {
  this_ptr->drawControl(element, option, painter);
}

void qt_widgets_c_QProxyStyle_drawControl_element_option_painter_widget(const QProxyStyle* this_ptr, QStyle::ControlElement element, const QStyleOption* option, QPainter* painter, const QWidget* widget) {
  this_ptr->drawControl(element, option, painter, widget);
}

void qt_widgets_c_QProxyStyle_drawItemPixmap(const QProxyStyle* this_ptr, QPainter* painter, const QRect* rect, int alignment, const QPixmap* pixmap) {
  this_ptr->drawItemPixmap(painter, *rect, alignment, *pixmap);
}

void qt_widgets_c_QProxyStyle_drawItemText_painter_rect_flags_pal_enabled_text(const QProxyStyle* this_ptr, QPainter* painter, const QRect* rect, int flags, const QPalette* pal, bool enabled, const QString* text) {
  this_ptr->drawItemText(painter, *rect, flags, *pal, enabled, *text);
}

void qt_widgets_c_QProxyStyle_drawItemText_painter_rect_flags_pal_enabled_text_textRole(const QProxyStyle* this_ptr, QPainter* painter, const QRect* rect, int flags, const QPalette* pal, bool enabled, const QString* text, const QPalette::ColorRole* textRole) {
  this_ptr->drawItemText(painter, *rect, flags, *pal, enabled, *text, *textRole);
}

void qt_widgets_c_QProxyStyle_drawPrimitive_element_option_painter(const QProxyStyle* this_ptr, QStyle::PrimitiveElement element, const QStyleOption* option, QPainter* painter) {
  this_ptr->drawPrimitive(element, option, painter);
}

void qt_widgets_c_QProxyStyle_drawPrimitive_element_option_painter_widget(const QProxyStyle* this_ptr, QStyle::PrimitiveElement element, const QStyleOption* option, QPainter* painter, const QWidget* widget) {
  this_ptr->drawPrimitive(element, option, painter, widget);
}

QPixmap* qt_widgets_c_QProxyStyle_generatedIconPixmap_as_ptr(const QProxyStyle* this_ptr, const QIcon::Mode* iconMode, const QPixmap* pixmap, const QStyleOption* opt) {
  return new QPixmap(this_ptr->generatedIconPixmap(*iconMode, *pixmap, opt));
}

QStyle::SubControl qt_widgets_c_QProxyStyle_hitTestComplexControl_control_option_pos(const QProxyStyle* this_ptr, QStyle::ComplexControl control, const QStyleOptionComplex* option, const QPoint* pos) {
  return this_ptr->hitTestComplexControl(control, option, *pos);
}

QStyle::SubControl qt_widgets_c_QProxyStyle_hitTestComplexControl_control_option_pos_widget(const QProxyStyle* this_ptr, QStyle::ComplexControl control, const QStyleOptionComplex* option, const QPoint* pos, const QWidget* widget) {
  return this_ptr->hitTestComplexControl(control, option, *pos, widget);
}

void qt_widgets_c_QProxyStyle_itemPixmapRect_to_output(const QProxyStyle* this_ptr, const QRect* r, int flags, const QPixmap* pixmap, QRect* output) {
  new(output) QRect(this_ptr->itemPixmapRect(*r, flags, *pixmap));
}

void qt_widgets_c_QProxyStyle_itemTextRect_to_output(const QProxyStyle* this_ptr, const QFontMetrics* fm, const QRect* r, int flags, bool enabled, const QString* text, QRect* output) {
  new(output) QRect(this_ptr->itemTextRect(*fm, *r, flags, enabled, *text));
}

int qt_widgets_c_QProxyStyle_layoutSpacing_control1_control2_orientation(const QProxyStyle* this_ptr, const QSizePolicy::ControlType* control1, const QSizePolicy::ControlType* control2, const Qt::Orientation* orientation) {
  return this_ptr->layoutSpacing(*control1, *control2, *orientation);
}

int qt_widgets_c_QProxyStyle_layoutSpacing_control1_control2_orientation_option(const QProxyStyle* this_ptr, const QSizePolicy::ControlType* control1, const QSizePolicy::ControlType* control2, const Qt::Orientation* orientation, const QStyleOption* option) {
  return this_ptr->layoutSpacing(*control1, *control2, *orientation, option);
}

int qt_widgets_c_QProxyStyle_layoutSpacing_control1_control2_orientation_option_widget(const QProxyStyle* this_ptr, const QSizePolicy::ControlType* control1, const QSizePolicy::ControlType* control2, const Qt::Orientation* orientation, const QStyleOption* option, const QWidget* widget) {
  return this_ptr->layoutSpacing(*control1, *control2, *orientation, option, widget);
}

const QMetaObject* qt_widgets_c_QProxyStyle_metaObject(const QProxyStyle* this_ptr) {
  return this_ptr->metaObject();
}

QProxyStyle* qt_widgets_c_QProxyStyle_new_key(const QString* key) {
  return new QProxyStyle(*key);
}

QProxyStyle* qt_widgets_c_QProxyStyle_new_no_args() {
  return new QProxyStyle();
}

QProxyStyle* qt_widgets_c_QProxyStyle_new_style(QStyle* style) {
  return new QProxyStyle(style);
}

int qt_widgets_c_QProxyStyle_pixelMetric_metric(const QProxyStyle* this_ptr, QStyle::PixelMetric metric) {
  return this_ptr->pixelMetric(metric);
}

int qt_widgets_c_QProxyStyle_pixelMetric_metric_option(const QProxyStyle* this_ptr, QStyle::PixelMetric metric, const QStyleOption* option) {
  return this_ptr->pixelMetric(metric, option);
}

int qt_widgets_c_QProxyStyle_pixelMetric_metric_option_widget(const QProxyStyle* this_ptr, QStyle::PixelMetric metric, const QStyleOption* option, const QWidget* widget) {
  return this_ptr->pixelMetric(metric, option, widget);
}

void qt_widgets_c_QProxyStyle_polish_app(QProxyStyle* this_ptr, QApplication* app) {
  this_ptr->polish(app);
}

void qt_widgets_c_QProxyStyle_polish_pal(QProxyStyle* this_ptr, QPalette* pal) {
  this_ptr->polish(*pal);
}

void qt_widgets_c_QProxyStyle_polish_widget(QProxyStyle* this_ptr, QWidget* widget) {
  this_ptr->polish(widget);
}

int qt_widgets_c_QProxyStyle_qt_metacall(QProxyStyle* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QProxyStyle_qt_metacast(QProxyStyle* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QProxyStyle_setBaseStyle(QProxyStyle* this_ptr, QStyle* style) {
  this_ptr->setBaseStyle(style);
}

void qt_widgets_c_QProxyStyle_sizeFromContents_to_output(const QProxyStyle* this_ptr, QStyle::ContentsType type, const QStyleOption* option, const QSize* size, const QWidget* widget, QSize* output) {
  new(output) QSize(this_ptr->sizeFromContents(type, option, *size, widget));
}

void qt_widgets_c_QProxyStyle_standardIcon_to_output_standardIcon(const QProxyStyle* this_ptr, QStyle::StandardPixmap standardIcon, QIcon* output) {
  new(output) QIcon(this_ptr->standardIcon(standardIcon));
}

void qt_widgets_c_QProxyStyle_standardIcon_to_output_standardIcon_option(const QProxyStyle* this_ptr, QStyle::StandardPixmap standardIcon, const QStyleOption* option, QIcon* output) {
  new(output) QIcon(this_ptr->standardIcon(standardIcon, option));
}

void qt_widgets_c_QProxyStyle_standardIcon_to_output_standardIcon_option_widget(const QProxyStyle* this_ptr, QStyle::StandardPixmap standardIcon, const QStyleOption* option, const QWidget* widget, QIcon* output) {
  new(output) QIcon(this_ptr->standardIcon(standardIcon, option, widget));
}

void qt_widgets_c_QProxyStyle_standardPalette_to_output(const QProxyStyle* this_ptr, QPalette* output) {
  new(output) QPalette(this_ptr->standardPalette());
}

QPixmap* qt_widgets_c_QProxyStyle_standardPixmap_as_ptr_standardPixmap_opt(const QProxyStyle* this_ptr, QStyle::StandardPixmap standardPixmap, const QStyleOption* opt) {
  return new QPixmap(this_ptr->standardPixmap(standardPixmap, opt));
}

QPixmap* qt_widgets_c_QProxyStyle_standardPixmap_as_ptr_standardPixmap_opt_widget(const QProxyStyle* this_ptr, QStyle::StandardPixmap standardPixmap, const QStyleOption* opt, const QWidget* widget) {
  return new QPixmap(this_ptr->standardPixmap(standardPixmap, opt, widget));
}

int qt_widgets_c_QProxyStyle_styleHint_hint(const QProxyStyle* this_ptr, QStyle::StyleHint hint) {
  return this_ptr->styleHint(hint);
}

int qt_widgets_c_QProxyStyle_styleHint_hint_option(const QProxyStyle* this_ptr, QStyle::StyleHint hint, const QStyleOption* option) {
  return this_ptr->styleHint(hint, option);
}

int qt_widgets_c_QProxyStyle_styleHint_hint_option_widget(const QProxyStyle* this_ptr, QStyle::StyleHint hint, const QStyleOption* option, const QWidget* widget) {
  return this_ptr->styleHint(hint, option, widget);
}

int qt_widgets_c_QProxyStyle_styleHint_hint_option_widget_returnData(const QProxyStyle* this_ptr, QStyle::StyleHint hint, const QStyleOption* option, const QWidget* widget, QStyleHintReturn* returnData) {
  return this_ptr->styleHint(hint, option, widget, returnData);
}

void qt_widgets_c_QProxyStyle_subControlRect_to_output(const QProxyStyle* this_ptr, QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QStyle::SubControl sc, const QWidget* widget, QRect* output) {
  new(output) QRect(this_ptr->subControlRect(cc, opt, sc, widget));
}

void qt_widgets_c_QProxyStyle_subElementRect_to_output(const QProxyStyle* this_ptr, QStyle::SubElement element, const QStyleOption* option, const QWidget* widget, QRect* output) {
  new(output) QRect(this_ptr->subElementRect(element, option, widget));
}

void qt_widgets_c_QProxyStyle_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QProxyStyle::trUtf8(s, c, n));
}

void qt_widgets_c_QProxyStyle_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QProxyStyle::tr(s, c, n));
}

void qt_widgets_c_QProxyStyle_unpolish_app(QProxyStyle* this_ptr, QApplication* app) {
  this_ptr->unpolish(app);
}

void qt_widgets_c_QProxyStyle_unpolish_widget(QProxyStyle* this_ptr, QWidget* widget) {
  this_ptr->unpolish(widget);
}

