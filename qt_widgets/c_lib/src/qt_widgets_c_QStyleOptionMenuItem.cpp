#include "qt_widgets_c_QStyleOptionMenuItem.h"

QStyleOptionMenuItem* qt_widgets_c_QStyleOptionMenuItem_G_static_cast_QStyleOptionMenuItem_ptr(QStyleOption* ptr) {
  return static_cast<QStyleOptionMenuItem*>(ptr);
}

QStyleOption* qt_widgets_c_QStyleOptionMenuItem_G_static_cast_QStyleOption_ptr(QStyleOptionMenuItem* ptr) {
  return static_cast<QStyleOption*>(ptr);
}

QStyleOptionMenuItem::CheckType qt_widgets_c_QStyleOptionMenuItem_checkType(const QStyleOptionMenuItem* this_ptr) {
  return this_ptr->checkType;
}

bool qt_widgets_c_QStyleOptionMenuItem_checked(const QStyleOptionMenuItem* this_ptr) {
  return this_ptr->checked;
}

void qt_widgets_c_QStyleOptionMenuItem_delete(QStyleOptionMenuItem* this_ptr) {
  delete this_ptr;
}

const QFont* qt_widgets_c_QStyleOptionMenuItem_font(const QStyleOptionMenuItem* this_ptr) {
  return &this_ptr->font;
}

QFont* qt_widgets_c_QStyleOptionMenuItem_font_mut(QStyleOptionMenuItem* this_ptr) {
  return &this_ptr->font;
}

const QIcon* qt_widgets_c_QStyleOptionMenuItem_icon(const QStyleOptionMenuItem* this_ptr) {
  return &this_ptr->icon;
}

QIcon* qt_widgets_c_QStyleOptionMenuItem_icon_mut(QStyleOptionMenuItem* this_ptr) {
  return &this_ptr->icon;
}

int qt_widgets_c_QStyleOptionMenuItem_maxIconWidth(const QStyleOptionMenuItem* this_ptr) {
  return this_ptr->maxIconWidth;
}

bool qt_widgets_c_QStyleOptionMenuItem_menuHasCheckableItems(const QStyleOptionMenuItem* this_ptr) {
  return this_ptr->menuHasCheckableItems;
}

QStyleOptionMenuItem::MenuItemType qt_widgets_c_QStyleOptionMenuItem_menuItemType(const QStyleOptionMenuItem* this_ptr) {
  return this_ptr->menuItemType;
}

const QRect* qt_widgets_c_QStyleOptionMenuItem_menuRect(const QStyleOptionMenuItem* this_ptr) {
  return &this_ptr->menuRect;
}

QRect* qt_widgets_c_QStyleOptionMenuItem_menuRect_mut(QStyleOptionMenuItem* this_ptr) {
  return &this_ptr->menuRect;
}

QStyleOptionMenuItem* qt_widgets_c_QStyleOptionMenuItem_new_no_args() {
  return new QStyleOptionMenuItem();
}

QStyleOptionMenuItem* qt_widgets_c_QStyleOptionMenuItem_new_other(const QStyleOptionMenuItem* other) {
  return new QStyleOptionMenuItem(*other);
}

void qt_widgets_c_QStyleOptionMenuItem_set_checkType(QStyleOptionMenuItem* this_ptr, QStyleOptionMenuItem::CheckType value) {
  this_ptr->checkType = value;
}

void qt_widgets_c_QStyleOptionMenuItem_set_checked(QStyleOptionMenuItem* this_ptr, bool value) {
  this_ptr->checked = value;
}

void qt_widgets_c_QStyleOptionMenuItem_set_font(QStyleOptionMenuItem* this_ptr, const QFont* value) {
  this_ptr->font = *value;
}

void qt_widgets_c_QStyleOptionMenuItem_set_icon(QStyleOptionMenuItem* this_ptr, const QIcon* value) {
  this_ptr->icon = *value;
}

void qt_widgets_c_QStyleOptionMenuItem_set_maxIconWidth(QStyleOptionMenuItem* this_ptr, int value) {
  this_ptr->maxIconWidth = value;
}

void qt_widgets_c_QStyleOptionMenuItem_set_menuHasCheckableItems(QStyleOptionMenuItem* this_ptr, bool value) {
  this_ptr->menuHasCheckableItems = value;
}

void qt_widgets_c_QStyleOptionMenuItem_set_menuItemType(QStyleOptionMenuItem* this_ptr, QStyleOptionMenuItem::MenuItemType value) {
  this_ptr->menuItemType = value;
}

void qt_widgets_c_QStyleOptionMenuItem_set_menuRect(QStyleOptionMenuItem* this_ptr, const QRect* value) {
  this_ptr->menuRect = *value;
}

void qt_widgets_c_QStyleOptionMenuItem_set_tabWidth(QStyleOptionMenuItem* this_ptr, int value) {
  this_ptr->tabWidth = value;
}

void qt_widgets_c_QStyleOptionMenuItem_set_text(QStyleOptionMenuItem* this_ptr, const QString* value) {
  this_ptr->text = *value;
}

int qt_widgets_c_QStyleOptionMenuItem_tabWidth(const QStyleOptionMenuItem* this_ptr) {
  return this_ptr->tabWidth;
}

const QString* qt_widgets_c_QStyleOptionMenuItem_text(const QStyleOptionMenuItem* this_ptr) {
  return &this_ptr->text;
}

QString* qt_widgets_c_QStyleOptionMenuItem_text_mut(QStyleOptionMenuItem* this_ptr) {
  return &this_ptr->text;
}

