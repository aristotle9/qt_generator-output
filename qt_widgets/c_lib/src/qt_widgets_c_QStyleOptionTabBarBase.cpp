#include "qt_widgets_c_QStyleOptionTabBarBase.h"

QStyleOptionTabBarBase* qt_widgets_c_QStyleOptionTabBarBase_G_static_cast_QStyleOptionTabBarBase_ptr(QStyleOption* ptr) {
  return static_cast<QStyleOptionTabBarBase*>(ptr);
}

QStyleOption* qt_widgets_c_QStyleOptionTabBarBase_G_static_cast_QStyleOption_ptr(QStyleOptionTabBarBase* ptr) {
  return static_cast<QStyleOption*>(ptr);
}

void qt_widgets_c_QStyleOptionTabBarBase_constructor_no_args(QStyleOptionTabBarBase* output) {
  new(output) QStyleOptionTabBarBase();
}

void qt_widgets_c_QStyleOptionTabBarBase_constructor_other(const QStyleOptionTabBarBase* other, QStyleOptionTabBarBase* output) {
  new(output) QStyleOptionTabBarBase(*other);
}

void qt_widgets_c_QStyleOptionTabBarBase_destructor(QStyleOptionTabBarBase* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QStyleOptionTabBarBase_documentMode(const QStyleOptionTabBarBase* this_ptr) {
  return this_ptr->documentMode;
}

const QRect* qt_widgets_c_QStyleOptionTabBarBase_selectedTabRect(const QStyleOptionTabBarBase* this_ptr) {
  return &this_ptr->selectedTabRect;
}

QRect* qt_widgets_c_QStyleOptionTabBarBase_selectedTabRect_mut(QStyleOptionTabBarBase* this_ptr) {
  return &this_ptr->selectedTabRect;
}

void qt_widgets_c_QStyleOptionTabBarBase_set_documentMode(QStyleOptionTabBarBase* this_ptr, bool value) {
  this_ptr->documentMode = value;
}

void qt_widgets_c_QStyleOptionTabBarBase_set_selectedTabRect(QStyleOptionTabBarBase* this_ptr, const QRect* value) {
  this_ptr->selectedTabRect = *value;
}

void qt_widgets_c_QStyleOptionTabBarBase_set_shape(QStyleOptionTabBarBase* this_ptr, const QTabBar::Shape* value) {
  this_ptr->shape = *value;
}

void qt_widgets_c_QStyleOptionTabBarBase_set_tabBarRect(QStyleOptionTabBarBase* this_ptr, const QRect* value) {
  this_ptr->tabBarRect = *value;
}

const QTabBar::Shape* qt_widgets_c_QStyleOptionTabBarBase_shape(const QStyleOptionTabBarBase* this_ptr) {
  return &this_ptr->shape;
}

QTabBar::Shape* qt_widgets_c_QStyleOptionTabBarBase_shape_mut(QStyleOptionTabBarBase* this_ptr) {
  return &this_ptr->shape;
}

const QRect* qt_widgets_c_QStyleOptionTabBarBase_tabBarRect(const QStyleOptionTabBarBase* this_ptr) {
  return &this_ptr->tabBarRect;
}

QRect* qt_widgets_c_QStyleOptionTabBarBase_tabBarRect_mut(QStyleOptionTabBarBase* this_ptr) {
  return &this_ptr->tabBarRect;
}

