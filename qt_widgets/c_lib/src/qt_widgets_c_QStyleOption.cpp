#include "qt_widgets_c_QStyleOption.h"

void qt_widgets_c_QStyleOption_G_operator_shl_to_output_debug_option(const QDebug* debug, const QStyleOption* option, QDebug* output) {
  new(output) QDebug(operator<<(*debug, *option));
}

void qt_widgets_c_QStyleOption_G_operator_shl_to_output_debug_optionType(const QDebug* debug, const QStyleOption::OptionType* optionType, QDebug* output) {
  new(output) QDebug(operator<<(*debug, *optionType));
}

void qt_widgets_c_QStyleOption_delete(QStyleOption* this_ptr) {
  delete this_ptr;
}

const Qt::LayoutDirection* qt_widgets_c_QStyleOption_direction(const QStyleOption* this_ptr) {
  return &this_ptr->direction;
}

Qt::LayoutDirection* qt_widgets_c_QStyleOption_direction_mut(QStyleOption* this_ptr) {
  return &this_ptr->direction;
}

const QFontMetrics* qt_widgets_c_QStyleOption_fontMetrics(const QStyleOption* this_ptr) {
  return &this_ptr->fontMetrics;
}

QFontMetrics* qt_widgets_c_QStyleOption_fontMetrics_mut(QStyleOption* this_ptr) {
  return &this_ptr->fontMetrics;
}

void qt_widgets_c_QStyleOption_init(QStyleOption* this_ptr, const QWidget* w) {
  this_ptr->init(w);
}

void qt_widgets_c_QStyleOption_initFrom(QStyleOption* this_ptr, const QWidget* w) {
  this_ptr->initFrom(w);
}

QStyleOption* qt_widgets_c_QStyleOption_new_no_args() {
  return new QStyleOption();
}

QStyleOption* qt_widgets_c_QStyleOption_new_other(const QStyleOption* other) {
  return new QStyleOption(*other);
}

QStyleOption* qt_widgets_c_QStyleOption_new_version(int version) {
  return new QStyleOption(version);
}

QStyleOption* qt_widgets_c_QStyleOption_new_version_type(int version, int type) {
  return new QStyleOption(version, type);
}

QStyleOption* qt_widgets_c_QStyleOption_operator_assign(QStyleOption* this_ptr, const QStyleOption* other) {
  return &this_ptr->operator=(*other);
}

const QPalette* qt_widgets_c_QStyleOption_palette(const QStyleOption* this_ptr) {
  return &this_ptr->palette;
}

QPalette* qt_widgets_c_QStyleOption_palette_mut(QStyleOption* this_ptr) {
  return &this_ptr->palette;
}

const QRect* qt_widgets_c_QStyleOption_rect(const QStyleOption* this_ptr) {
  return &this_ptr->rect;
}

QRect* qt_widgets_c_QStyleOption_rect_mut(QStyleOption* this_ptr) {
  return &this_ptr->rect;
}

void qt_widgets_c_QStyleOption_set_direction(QStyleOption* this_ptr, const Qt::LayoutDirection* value) {
  this_ptr->direction = *value;
}

void qt_widgets_c_QStyleOption_set_fontMetrics(QStyleOption* this_ptr, const QFontMetrics* value) {
  this_ptr->fontMetrics = *value;
}

void qt_widgets_c_QStyleOption_set_palette(QStyleOption* this_ptr, const QPalette* value) {
  this_ptr->palette = *value;
}

void qt_widgets_c_QStyleOption_set_rect(QStyleOption* this_ptr, const QRect* value) {
  this_ptr->rect = *value;
}

void qt_widgets_c_QStyleOption_set_styleObject(QStyleOption* this_ptr, QObject* value) {
  this_ptr->styleObject = value;
}

void qt_widgets_c_QStyleOption_set_type(QStyleOption* this_ptr, int value) {
  this_ptr->type = value;
}

void qt_widgets_c_QStyleOption_set_version(QStyleOption* this_ptr, int value) {
  this_ptr->version = value;
}

QObject* qt_widgets_c_QStyleOption_styleObject(const QStyleOption* this_ptr) {
  return this_ptr->styleObject;
}

int qt_widgets_c_QStyleOption_type(const QStyleOption* this_ptr) {
  return this_ptr->type;
}

int qt_widgets_c_QStyleOption_version(const QStyleOption* this_ptr) {
  return this_ptr->version;
}

