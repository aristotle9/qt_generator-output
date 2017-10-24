#include "qt_gui_c_QPalette.h"

void qt_gui_c_QPalette_G_operator_shl_to_output(const QDebug* arg1, const QPalette* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

QDataStream* qt_gui_c_QPalette_G_operator_shr(QDataStream* ds, QPalette* p) {
  return &operator>>(*ds, *p);
}

void qt_gui_c_QPalette_G_swap(QPalette* value1, QPalette* value2) {
  swap(*value1, *value2);
}

const QBrush* qt_gui_c_QPalette_alternateBase(const QPalette* this_ptr) {
  return &this_ptr->alternateBase();
}

const QBrush* qt_gui_c_QPalette_background(const QPalette* this_ptr) {
  return &this_ptr->background();
}

const QBrush* qt_gui_c_QPalette_base(const QPalette* this_ptr) {
  return &this_ptr->base();
}

const QBrush* qt_gui_c_QPalette_brightText(const QPalette* this_ptr) {
  return &this_ptr->brightText();
}

const QBrush* qt_gui_c_QPalette_brush_cg_cr(const QPalette* this_ptr, QPalette::ColorGroup cg, QPalette::ColorRole cr) {
  return &this_ptr->brush(cg, cr);
}

const QBrush* qt_gui_c_QPalette_brush_cr(const QPalette* this_ptr, QPalette::ColorRole cr) {
  return &this_ptr->brush(cr);
}

const QBrush* qt_gui_c_QPalette_button(const QPalette* this_ptr) {
  return &this_ptr->button();
}

const QBrush* qt_gui_c_QPalette_buttonText(const QPalette* this_ptr) {
  return &this_ptr->buttonText();
}

qint64 qt_gui_c_QPalette_cacheKey(const QPalette* this_ptr) {
  return this_ptr->cacheKey();
}

const QColor* qt_gui_c_QPalette_color_cg_cr(const QPalette* this_ptr, QPalette::ColorGroup cg, QPalette::ColorRole cr) {
  return &this_ptr->color(cg, cr);
}

const QColor* qt_gui_c_QPalette_color_cr(const QPalette* this_ptr, QPalette::ColorRole cr) {
  return &this_ptr->color(cr);
}

void qt_gui_c_QPalette_constructor_QBrush_QBrush_QBrush_QBrush_QBrush_QBrush_QBrush_QBrush_QBrush(const QBrush* windowText, const QBrush* button, const QBrush* light, const QBrush* dark, const QBrush* mid, const QBrush* text, const QBrush* bright_text, const QBrush* base, const QBrush* window, QPalette* output) {
  new(output) QPalette(*windowText, *button, *light, *dark, *mid, *text, *bright_text, *base, *window);
}

void qt_gui_c_QPalette_constructor_QColor(const QColor* button, QPalette* output) {
  new(output) QPalette(*button);
}

void qt_gui_c_QPalette_constructor_QColor_QColor(const QColor* button, const QColor* window, QPalette* output) {
  new(output) QPalette(*button, *window);
}

void qt_gui_c_QPalette_constructor_QColor_QColor_QColor_QColor_QColor_QColor_QColor(const QColor* windowText, const QColor* window, const QColor* light, const QColor* dark, const QColor* mid, const QColor* text, const QColor* base, QPalette* output) {
  new(output) QPalette(*windowText, *window, *light, *dark, *mid, *text, *base);
}

void qt_gui_c_QPalette_constructor_QPalette(const QPalette* palette, QPalette* output) {
  new(output) QPalette(*palette);
}

void qt_gui_c_QPalette_constructor_Qt_GlobalColor(const Qt::GlobalColor* button, QPalette* output) {
  new(output) QPalette(*button);
}

void qt_gui_c_QPalette_constructor_no_args(QPalette* output) {
  new(output) QPalette();
}

void qt_gui_c_QPalette_convert_to_QVariant_to_output(const QPalette* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

QPalette::ColorGroup qt_gui_c_QPalette_currentColorGroup(const QPalette* this_ptr) {
  return this_ptr->currentColorGroup();
}

const QBrush* qt_gui_c_QPalette_dark(const QPalette* this_ptr) {
  return &this_ptr->dark();
}

void qt_gui_c_QPalette_destructor(QPalette* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

const QBrush* qt_gui_c_QPalette_foreground(const QPalette* this_ptr) {
  return &this_ptr->foreground();
}

const QBrush* qt_gui_c_QPalette_highlight(const QPalette* this_ptr) {
  return &this_ptr->highlight();
}

const QBrush* qt_gui_c_QPalette_highlightedText(const QPalette* this_ptr) {
  return &this_ptr->highlightedText();
}

bool qt_gui_c_QPalette_isBrushSet(const QPalette* this_ptr, QPalette::ColorGroup cg, QPalette::ColorRole cr) {
  return this_ptr->isBrushSet(cg, cr);
}

bool qt_gui_c_QPalette_isCopyOf(const QPalette* this_ptr, const QPalette* p) {
  return this_ptr->isCopyOf(*p);
}

bool qt_gui_c_QPalette_isEqual(const QPalette* this_ptr, QPalette::ColorGroup cr1, QPalette::ColorGroup cr2) {
  return this_ptr->isEqual(cr1, cr2);
}

const QBrush* qt_gui_c_QPalette_light(const QPalette* this_ptr) {
  return &this_ptr->light();
}

const QBrush* qt_gui_c_QPalette_link(const QPalette* this_ptr) {
  return &this_ptr->link();
}

const QBrush* qt_gui_c_QPalette_linkVisited(const QPalette* this_ptr) {
  return &this_ptr->linkVisited();
}

const QBrush* qt_gui_c_QPalette_mid(const QPalette* this_ptr) {
  return &this_ptr->mid();
}

const QBrush* qt_gui_c_QPalette_midlight(const QPalette* this_ptr) {
  return &this_ptr->midlight();
}

QPalette* qt_gui_c_QPalette_operator_assign(QPalette* this_ptr, const QPalette* palette) {
  return &this_ptr->operator=(*palette);
}

bool qt_gui_c_QPalette_operator_eq(const QPalette* this_ptr, const QPalette* p) {
  return this_ptr->operator==(*p);
}

bool qt_gui_c_QPalette_operator_neq(const QPalette* this_ptr, const QPalette* p) {
  return this_ptr->operator!=(*p);
}

void qt_gui_c_QPalette_resolve_mask(QPalette* this_ptr, unsigned int mask) {
  this_ptr->resolve(mask);
}

unsigned int qt_gui_c_QPalette_resolve_no_args(const QPalette* this_ptr) {
  return this_ptr->resolve();
}

void qt_gui_c_QPalette_resolve_to_output(const QPalette* this_ptr, const QPalette* arg1, QPalette* output) {
  new(output) QPalette(this_ptr->resolve(*arg1));
}

void qt_gui_c_QPalette_setBrush_cg_cr_brush(QPalette* this_ptr, QPalette::ColorGroup cg, QPalette::ColorRole cr, const QBrush* brush) {
  this_ptr->setBrush(cg, cr, *brush);
}

void qt_gui_c_QPalette_setBrush_cr_brush(QPalette* this_ptr, QPalette::ColorRole cr, const QBrush* brush) {
  this_ptr->setBrush(cr, *brush);
}

void qt_gui_c_QPalette_setColorGroup(QPalette* this_ptr, QPalette::ColorGroup cr, const QBrush* windowText, const QBrush* button, const QBrush* light, const QBrush* dark, const QBrush* mid, const QBrush* text, const QBrush* bright_text, const QBrush* base, const QBrush* window) {
  this_ptr->setColorGroup(cr, *windowText, *button, *light, *dark, *mid, *text, *bright_text, *base, *window);
}

void qt_gui_c_QPalette_setColor_cg_cr_color(QPalette* this_ptr, QPalette::ColorGroup cg, QPalette::ColorRole cr, const QColor* color) {
  this_ptr->setColor(cg, cr, *color);
}

void qt_gui_c_QPalette_setColor_cr_color(QPalette* this_ptr, QPalette::ColorRole cr, const QColor* color) {
  this_ptr->setColor(cr, *color);
}

void qt_gui_c_QPalette_setCurrentColorGroup(QPalette* this_ptr, QPalette::ColorGroup cg) {
  this_ptr->setCurrentColorGroup(cg);
}

const QBrush* qt_gui_c_QPalette_shadow(const QPalette* this_ptr) {
  return &this_ptr->shadow();
}

void qt_gui_c_QPalette_swap(QPalette* this_ptr, QPalette* other) {
  this_ptr->swap(*other);
}

const QBrush* qt_gui_c_QPalette_text(const QPalette* this_ptr) {
  return &this_ptr->text();
}

const QBrush* qt_gui_c_QPalette_toolTipBase(const QPalette* this_ptr) {
  return &this_ptr->toolTipBase();
}

const QBrush* qt_gui_c_QPalette_toolTipText(const QPalette* this_ptr) {
  return &this_ptr->toolTipText();
}

const QBrush* qt_gui_c_QPalette_window(const QPalette* this_ptr) {
  return &this_ptr->window();
}

const QBrush* qt_gui_c_QPalette_windowText(const QPalette* this_ptr) {
  return &this_ptr->windowText();
}

