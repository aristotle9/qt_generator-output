#include "qt_widgets_c_QStyleOptionViewItem.h"

QStyleOptionViewItem* qt_widgets_c_QStyleOptionViewItem_G_static_cast_QStyleOptionViewItem_ptr(QStyleOption* ptr) {
  return static_cast<QStyleOptionViewItem*>(ptr);
}

QStyleOption* qt_widgets_c_QStyleOptionViewItem_G_static_cast_QStyleOption_ptr(QStyleOptionViewItem* ptr) {
  return static_cast<QStyleOption*>(ptr);
}

const QBrush* qt_widgets_c_QStyleOptionViewItem_backgroundBrush(const QStyleOptionViewItem* this_ptr) {
  return &this_ptr->backgroundBrush;
}

QBrush* qt_widgets_c_QStyleOptionViewItem_backgroundBrush_mut(QStyleOptionViewItem* this_ptr) {
  return &this_ptr->backgroundBrush;
}

const Qt::CheckState* qt_widgets_c_QStyleOptionViewItem_checkState(const QStyleOptionViewItem* this_ptr) {
  return &this_ptr->checkState;
}

Qt::CheckState* qt_widgets_c_QStyleOptionViewItem_checkState_mut(QStyleOptionViewItem* this_ptr) {
  return &this_ptr->checkState;
}

QStyleOptionViewItem::Position qt_widgets_c_QStyleOptionViewItem_decorationPosition(const QStyleOptionViewItem* this_ptr) {
  return this_ptr->decorationPosition;
}

const QSize* qt_widgets_c_QStyleOptionViewItem_decorationSize(const QStyleOptionViewItem* this_ptr) {
  return &this_ptr->decorationSize;
}

QSize* qt_widgets_c_QStyleOptionViewItem_decorationSize_mut(QStyleOptionViewItem* this_ptr) {
  return &this_ptr->decorationSize;
}

void qt_widgets_c_QStyleOptionViewItem_delete(QStyleOptionViewItem* this_ptr) {
  delete this_ptr;
}

unsigned int qt_widgets_c_QStyleOptionViewItem_features(const QStyleOptionViewItem* this_ptr) {
  return uint(this_ptr->features);
}

const QFont* qt_widgets_c_QStyleOptionViewItem_font(const QStyleOptionViewItem* this_ptr) {
  return &this_ptr->font;
}

QFont* qt_widgets_c_QStyleOptionViewItem_font_mut(QStyleOptionViewItem* this_ptr) {
  return &this_ptr->font;
}

const QIcon* qt_widgets_c_QStyleOptionViewItem_icon(const QStyleOptionViewItem* this_ptr) {
  return &this_ptr->icon;
}

QIcon* qt_widgets_c_QStyleOptionViewItem_icon_mut(QStyleOptionViewItem* this_ptr) {
  return &this_ptr->icon;
}

const QModelIndex* qt_widgets_c_QStyleOptionViewItem_index(const QStyleOptionViewItem* this_ptr) {
  return &this_ptr->index;
}

QModelIndex* qt_widgets_c_QStyleOptionViewItem_index_mut(QStyleOptionViewItem* this_ptr) {
  return &this_ptr->index;
}

const QLocale* qt_widgets_c_QStyleOptionViewItem_locale(const QStyleOptionViewItem* this_ptr) {
  return &this_ptr->locale;
}

QLocale* qt_widgets_c_QStyleOptionViewItem_locale_mut(QStyleOptionViewItem* this_ptr) {
  return &this_ptr->locale;
}

QStyleOptionViewItem* qt_widgets_c_QStyleOptionViewItem_new_no_args() {
  return new QStyleOptionViewItem();
}

QStyleOptionViewItem* qt_widgets_c_QStyleOptionViewItem_new_other(const QStyleOptionViewItem* other) {
  return new QStyleOptionViewItem(*other);
}

void qt_widgets_c_QStyleOptionViewItem_set_backgroundBrush(QStyleOptionViewItem* this_ptr, const QBrush* value) {
  this_ptr->backgroundBrush = *value;
}

void qt_widgets_c_QStyleOptionViewItem_set_checkState(QStyleOptionViewItem* this_ptr, const Qt::CheckState* value) {
  this_ptr->checkState = *value;
}

void qt_widgets_c_QStyleOptionViewItem_set_decorationPosition(QStyleOptionViewItem* this_ptr, QStyleOptionViewItem::Position value) {
  this_ptr->decorationPosition = value;
}

void qt_widgets_c_QStyleOptionViewItem_set_decorationSize(QStyleOptionViewItem* this_ptr, const QSize* value) {
  this_ptr->decorationSize = *value;
}

void qt_widgets_c_QStyleOptionViewItem_set_features(QStyleOptionViewItem* this_ptr, unsigned int value) {
  this_ptr->features = QFlags< QStyleOptionViewItem::ViewItemFeature >(value);
}

void qt_widgets_c_QStyleOptionViewItem_set_font(QStyleOptionViewItem* this_ptr, const QFont* value) {
  this_ptr->font = *value;
}

void qt_widgets_c_QStyleOptionViewItem_set_icon(QStyleOptionViewItem* this_ptr, const QIcon* value) {
  this_ptr->icon = *value;
}

void qt_widgets_c_QStyleOptionViewItem_set_index(QStyleOptionViewItem* this_ptr, const QModelIndex* value) {
  this_ptr->index = *value;
}

void qt_widgets_c_QStyleOptionViewItem_set_locale(QStyleOptionViewItem* this_ptr, const QLocale* value) {
  this_ptr->locale = *value;
}

void qt_widgets_c_QStyleOptionViewItem_set_showDecorationSelected(QStyleOptionViewItem* this_ptr, bool value) {
  this_ptr->showDecorationSelected = value;
}

void qt_widgets_c_QStyleOptionViewItem_set_text(QStyleOptionViewItem* this_ptr, const QString* value) {
  this_ptr->text = *value;
}

void qt_widgets_c_QStyleOptionViewItem_set_textElideMode(QStyleOptionViewItem* this_ptr, const Qt::TextElideMode* value) {
  this_ptr->textElideMode = *value;
}

void qt_widgets_c_QStyleOptionViewItem_set_viewItemPosition(QStyleOptionViewItem* this_ptr, QStyleOptionViewItem::ViewItemPosition value) {
  this_ptr->viewItemPosition = value;
}

void qt_widgets_c_QStyleOptionViewItem_set_widget(QStyleOptionViewItem* this_ptr, const QWidget* value) {
  this_ptr->widget = value;
}

bool qt_widgets_c_QStyleOptionViewItem_showDecorationSelected(const QStyleOptionViewItem* this_ptr) {
  return this_ptr->showDecorationSelected;
}

const QString* qt_widgets_c_QStyleOptionViewItem_text(const QStyleOptionViewItem* this_ptr) {
  return &this_ptr->text;
}

const Qt::TextElideMode* qt_widgets_c_QStyleOptionViewItem_textElideMode(const QStyleOptionViewItem* this_ptr) {
  return &this_ptr->textElideMode;
}

Qt::TextElideMode* qt_widgets_c_QStyleOptionViewItem_textElideMode_mut(QStyleOptionViewItem* this_ptr) {
  return &this_ptr->textElideMode;
}

QString* qt_widgets_c_QStyleOptionViewItem_text_mut(QStyleOptionViewItem* this_ptr) {
  return &this_ptr->text;
}

QStyleOptionViewItem::ViewItemPosition qt_widgets_c_QStyleOptionViewItem_viewItemPosition(const QStyleOptionViewItem* this_ptr) {
  return this_ptr->viewItemPosition;
}

const QWidget* qt_widgets_c_QStyleOptionViewItem_widget(const QStyleOptionViewItem* this_ptr) {
  return this_ptr->widget;
}

