#include "qt_widgets_c_QStylePainter.h"

QPainter* qt_widgets_c_QStylePainter_G_static_cast_QPainter_ptr(QStylePainter* ptr) {
  return static_cast<QPainter*>(ptr);
}

QStylePainter* qt_widgets_c_QStylePainter_G_static_cast_QStylePainter_ptr(QPainter* ptr) {
  return static_cast<QStylePainter*>(ptr);
}

bool qt_widgets_c_QStylePainter_begin_pd_w(QStylePainter* this_ptr, QPaintDevice* pd, QWidget* w) {
  return this_ptr->begin(pd, w);
}

bool qt_widgets_c_QStylePainter_begin_w(QStylePainter* this_ptr, QWidget* w) {
  return this_ptr->begin(w);
}

void qt_widgets_c_QStylePainter_constructor_no_args(QStylePainter* output) {
  new(output) QStylePainter();
}

void qt_widgets_c_QStylePainter_constructor_pd_w(QPaintDevice* pd, QWidget* w, QStylePainter* output) {
  new(output) QStylePainter(pd, w);
}

void qt_widgets_c_QStylePainter_constructor_w(QWidget* w, QStylePainter* output) {
  new(output) QStylePainter(w);
}

void qt_widgets_c_QStylePainter_destructor(QStylePainter* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

void qt_widgets_c_QStylePainter_drawComplexControl(QStylePainter* this_ptr, const QStyle::ComplexControl* cc, const QStyleOptionComplex* opt) {
  this_ptr->drawComplexControl(*cc, *opt);
}

void qt_widgets_c_QStylePainter_drawControl(QStylePainter* this_ptr, const QStyle::ControlElement* ce, const QStyleOption* opt) {
  this_ptr->drawControl(*ce, *opt);
}

void qt_widgets_c_QStylePainter_drawItemPixmap(QStylePainter* this_ptr, const QRect* r, int flags, const QPixmap* pixmap) {
  this_ptr->drawItemPixmap(*r, flags, *pixmap);
}

void qt_widgets_c_QStylePainter_drawItemText_r_flags_pal_enabled_text(QStylePainter* this_ptr, const QRect* r, int flags, const QPalette* pal, bool enabled, const QString* text) {
  this_ptr->drawItemText(*r, flags, *pal, enabled, *text);
}

void qt_widgets_c_QStylePainter_drawItemText_r_flags_pal_enabled_text_textRole(QStylePainter* this_ptr, const QRect* r, int flags, const QPalette* pal, bool enabled, const QString* text, const QPalette::ColorRole* textRole) {
  this_ptr->drawItemText(*r, flags, *pal, enabled, *text, *textRole);
}

void qt_widgets_c_QStylePainter_drawPrimitive(QStylePainter* this_ptr, const QStyle::PrimitiveElement* pe, const QStyleOption* opt) {
  this_ptr->drawPrimitive(*pe, *opt);
}

QStyle* qt_widgets_c_QStylePainter_style(const QStylePainter* this_ptr) {
  return this_ptr->style();
}

