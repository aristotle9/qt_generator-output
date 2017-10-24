#include "qt_widgets_c_QStyleOptionTab.h"

QStyleOptionTab* qt_widgets_c_QStyleOptionTab_G_static_cast_QStyleOptionTab_ptr(QStyleOption* ptr) {
  return static_cast<QStyleOptionTab*>(ptr);
}

QStyleOption* qt_widgets_c_QStyleOptionTab_G_static_cast_QStyleOption_ptr(QStyleOptionTab* ptr) {
  return static_cast<QStyleOption*>(ptr);
}

unsigned int qt_widgets_c_QStyleOptionTab_cornerWidgets(const QStyleOptionTab* this_ptr) {
  return uint(this_ptr->cornerWidgets);
}

void qt_widgets_c_QStyleOptionTab_delete(QStyleOptionTab* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QStyleOptionTab_documentMode(const QStyleOptionTab* this_ptr) {
  return this_ptr->documentMode;
}

unsigned int qt_widgets_c_QStyleOptionTab_features(const QStyleOptionTab* this_ptr) {
  return uint(this_ptr->features);
}

const QIcon* qt_widgets_c_QStyleOptionTab_icon(const QStyleOptionTab* this_ptr) {
  return &this_ptr->icon;
}

const QSize* qt_widgets_c_QStyleOptionTab_iconSize(const QStyleOptionTab* this_ptr) {
  return &this_ptr->iconSize;
}

QSize* qt_widgets_c_QStyleOptionTab_iconSize_mut(QStyleOptionTab* this_ptr) {
  return &this_ptr->iconSize;
}

QIcon* qt_widgets_c_QStyleOptionTab_icon_mut(QStyleOptionTab* this_ptr) {
  return &this_ptr->icon;
}

const QSize* qt_widgets_c_QStyleOptionTab_leftButtonSize(const QStyleOptionTab* this_ptr) {
  return &this_ptr->leftButtonSize;
}

QSize* qt_widgets_c_QStyleOptionTab_leftButtonSize_mut(QStyleOptionTab* this_ptr) {
  return &this_ptr->leftButtonSize;
}

QStyleOptionTab* qt_widgets_c_QStyleOptionTab_new_no_args() {
  return new QStyleOptionTab();
}

QStyleOptionTab* qt_widgets_c_QStyleOptionTab_new_other(const QStyleOptionTab* other) {
  return new QStyleOptionTab(*other);
}

QStyleOptionTab::TabPosition qt_widgets_c_QStyleOptionTab_position(const QStyleOptionTab* this_ptr) {
  return this_ptr->position;
}

const QSize* qt_widgets_c_QStyleOptionTab_rightButtonSize(const QStyleOptionTab* this_ptr) {
  return &this_ptr->rightButtonSize;
}

QSize* qt_widgets_c_QStyleOptionTab_rightButtonSize_mut(QStyleOptionTab* this_ptr) {
  return &this_ptr->rightButtonSize;
}

int qt_widgets_c_QStyleOptionTab_row(const QStyleOptionTab* this_ptr) {
  return this_ptr->row;
}

QStyleOptionTab::SelectedPosition qt_widgets_c_QStyleOptionTab_selectedPosition(const QStyleOptionTab* this_ptr) {
  return this_ptr->selectedPosition;
}

void qt_widgets_c_QStyleOptionTab_set_cornerWidgets(QStyleOptionTab* this_ptr, unsigned int value) {
  this_ptr->cornerWidgets = QFlags< QStyleOptionTab::CornerWidget >(value);
}

void qt_widgets_c_QStyleOptionTab_set_documentMode(QStyleOptionTab* this_ptr, bool value) {
  this_ptr->documentMode = value;
}

void qt_widgets_c_QStyleOptionTab_set_features(QStyleOptionTab* this_ptr, unsigned int value) {
  this_ptr->features = QFlags< QStyleOptionTab::TabFeature >(value);
}

void qt_widgets_c_QStyleOptionTab_set_icon(QStyleOptionTab* this_ptr, const QIcon* value) {
  this_ptr->icon = *value;
}

void qt_widgets_c_QStyleOptionTab_set_iconSize(QStyleOptionTab* this_ptr, const QSize* value) {
  this_ptr->iconSize = *value;
}

void qt_widgets_c_QStyleOptionTab_set_leftButtonSize(QStyleOptionTab* this_ptr, const QSize* value) {
  this_ptr->leftButtonSize = *value;
}

void qt_widgets_c_QStyleOptionTab_set_position(QStyleOptionTab* this_ptr, QStyleOptionTab::TabPosition value) {
  this_ptr->position = value;
}

void qt_widgets_c_QStyleOptionTab_set_rightButtonSize(QStyleOptionTab* this_ptr, const QSize* value) {
  this_ptr->rightButtonSize = *value;
}

void qt_widgets_c_QStyleOptionTab_set_row(QStyleOptionTab* this_ptr, int value) {
  this_ptr->row = value;
}

void qt_widgets_c_QStyleOptionTab_set_selectedPosition(QStyleOptionTab* this_ptr, QStyleOptionTab::SelectedPosition value) {
  this_ptr->selectedPosition = value;
}

void qt_widgets_c_QStyleOptionTab_set_shape(QStyleOptionTab* this_ptr, const QTabBar::Shape* value) {
  this_ptr->shape = *value;
}

void qt_widgets_c_QStyleOptionTab_set_text(QStyleOptionTab* this_ptr, const QString* value) {
  this_ptr->text = *value;
}

const QTabBar::Shape* qt_widgets_c_QStyleOptionTab_shape(const QStyleOptionTab* this_ptr) {
  return &this_ptr->shape;
}

QTabBar::Shape* qt_widgets_c_QStyleOptionTab_shape_mut(QStyleOptionTab* this_ptr) {
  return &this_ptr->shape;
}

const QString* qt_widgets_c_QStyleOptionTab_text(const QStyleOptionTab* this_ptr) {
  return &this_ptr->text;
}

QString* qt_widgets_c_QStyleOptionTab_text_mut(QStyleOptionTab* this_ptr) {
  return &this_ptr->text;
}

