#include "qt_widgets_c_QStyleOptionToolButton.h"

QStyleOptionComplex* qt_widgets_c_QStyleOptionToolButton_G_static_cast_QStyleOptionComplex_ptr(QStyleOptionToolButton* ptr) {
  return static_cast<QStyleOptionComplex*>(ptr);
}

QStyleOptionToolButton* qt_widgets_c_QStyleOptionToolButton_G_static_cast_QStyleOptionToolButton_ptr_QStyleOption(QStyleOption* ptr) {
  return static_cast<QStyleOptionToolButton*>(ptr);
}

QStyleOptionToolButton* qt_widgets_c_QStyleOptionToolButton_G_static_cast_QStyleOptionToolButton_ptr_QStyleOptionComplex(QStyleOptionComplex* ptr) {
  return static_cast<QStyleOptionToolButton*>(ptr);
}

QStyleOption* qt_widgets_c_QStyleOptionToolButton_G_static_cast_QStyleOption_ptr(QStyleOptionToolButton* ptr) {
  return static_cast<QStyleOption*>(ptr);
}

const Qt::ArrowType* qt_widgets_c_QStyleOptionToolButton_arrowType(const QStyleOptionToolButton* this_ptr) {
  return &this_ptr->arrowType;
}

Qt::ArrowType* qt_widgets_c_QStyleOptionToolButton_arrowType_mut(QStyleOptionToolButton* this_ptr) {
  return &this_ptr->arrowType;
}

void qt_widgets_c_QStyleOptionToolButton_delete(QStyleOptionToolButton* this_ptr) {
  delete this_ptr;
}

unsigned int qt_widgets_c_QStyleOptionToolButton_features(const QStyleOptionToolButton* this_ptr) {
  return uint(this_ptr->features);
}

const QFont* qt_widgets_c_QStyleOptionToolButton_font(const QStyleOptionToolButton* this_ptr) {
  return &this_ptr->font;
}

QFont* qt_widgets_c_QStyleOptionToolButton_font_mut(QStyleOptionToolButton* this_ptr) {
  return &this_ptr->font;
}

const QIcon* qt_widgets_c_QStyleOptionToolButton_icon(const QStyleOptionToolButton* this_ptr) {
  return &this_ptr->icon;
}

const QSize* qt_widgets_c_QStyleOptionToolButton_iconSize(const QStyleOptionToolButton* this_ptr) {
  return &this_ptr->iconSize;
}

QSize* qt_widgets_c_QStyleOptionToolButton_iconSize_mut(QStyleOptionToolButton* this_ptr) {
  return &this_ptr->iconSize;
}

QIcon* qt_widgets_c_QStyleOptionToolButton_icon_mut(QStyleOptionToolButton* this_ptr) {
  return &this_ptr->icon;
}

QStyleOptionToolButton* qt_widgets_c_QStyleOptionToolButton_new_no_args() {
  return new QStyleOptionToolButton();
}

QStyleOptionToolButton* qt_widgets_c_QStyleOptionToolButton_new_other(const QStyleOptionToolButton* other) {
  return new QStyleOptionToolButton(*other);
}

const QPoint* qt_widgets_c_QStyleOptionToolButton_pos(const QStyleOptionToolButton* this_ptr) {
  return &this_ptr->pos;
}

QPoint* qt_widgets_c_QStyleOptionToolButton_pos_mut(QStyleOptionToolButton* this_ptr) {
  return &this_ptr->pos;
}

void qt_widgets_c_QStyleOptionToolButton_set_arrowType(QStyleOptionToolButton* this_ptr, const Qt::ArrowType* value) {
  this_ptr->arrowType = *value;
}

void qt_widgets_c_QStyleOptionToolButton_set_features(QStyleOptionToolButton* this_ptr, unsigned int value) {
  this_ptr->features = QFlags< QStyleOptionToolButton::ToolButtonFeature >(value);
}

void qt_widgets_c_QStyleOptionToolButton_set_font(QStyleOptionToolButton* this_ptr, const QFont* value) {
  this_ptr->font = *value;
}

void qt_widgets_c_QStyleOptionToolButton_set_icon(QStyleOptionToolButton* this_ptr, const QIcon* value) {
  this_ptr->icon = *value;
}

void qt_widgets_c_QStyleOptionToolButton_set_iconSize(QStyleOptionToolButton* this_ptr, const QSize* value) {
  this_ptr->iconSize = *value;
}

void qt_widgets_c_QStyleOptionToolButton_set_pos(QStyleOptionToolButton* this_ptr, const QPoint* value) {
  this_ptr->pos = *value;
}

void qt_widgets_c_QStyleOptionToolButton_set_text(QStyleOptionToolButton* this_ptr, const QString* value) {
  this_ptr->text = *value;
}

void qt_widgets_c_QStyleOptionToolButton_set_toolButtonStyle(QStyleOptionToolButton* this_ptr, const Qt::ToolButtonStyle* value) {
  this_ptr->toolButtonStyle = *value;
}

const QString* qt_widgets_c_QStyleOptionToolButton_text(const QStyleOptionToolButton* this_ptr) {
  return &this_ptr->text;
}

QString* qt_widgets_c_QStyleOptionToolButton_text_mut(QStyleOptionToolButton* this_ptr) {
  return &this_ptr->text;
}

const Qt::ToolButtonStyle* qt_widgets_c_QStyleOptionToolButton_toolButtonStyle(const QStyleOptionToolButton* this_ptr) {
  return &this_ptr->toolButtonStyle;
}

Qt::ToolButtonStyle* qt_widgets_c_QStyleOptionToolButton_toolButtonStyle_mut(QStyleOptionToolButton* this_ptr) {
  return &this_ptr->toolButtonStyle;
}

