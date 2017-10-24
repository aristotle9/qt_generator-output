#include "qt_widgets_c_QStyleOptionFrame.h"

QStyleOptionFrame* qt_widgets_c_QStyleOptionFrame_G_static_cast_QStyleOptionFrame_ptr(QStyleOption* ptr) {
  return static_cast<QStyleOptionFrame*>(ptr);
}

QStyleOption* qt_widgets_c_QStyleOptionFrame_G_static_cast_QStyleOption_ptr(QStyleOptionFrame* ptr) {
  return static_cast<QStyleOption*>(ptr);
}

void qt_widgets_c_QStyleOptionFrame_delete(QStyleOptionFrame* this_ptr) {
  delete this_ptr;
}

unsigned int qt_widgets_c_QStyleOptionFrame_features(const QStyleOptionFrame* this_ptr) {
  return uint(this_ptr->features);
}

const QFrame::Shape* qt_widgets_c_QStyleOptionFrame_frameShape(const QStyleOptionFrame* this_ptr) {
  return &this_ptr->frameShape;
}

QFrame::Shape* qt_widgets_c_QStyleOptionFrame_frameShape_mut(QStyleOptionFrame* this_ptr) {
  return &this_ptr->frameShape;
}

int qt_widgets_c_QStyleOptionFrame_lineWidth(const QStyleOptionFrame* this_ptr) {
  return this_ptr->lineWidth;
}

int qt_widgets_c_QStyleOptionFrame_midLineWidth(const QStyleOptionFrame* this_ptr) {
  return this_ptr->midLineWidth;
}

QStyleOptionFrame* qt_widgets_c_QStyleOptionFrame_new_no_args() {
  return new QStyleOptionFrame();
}

QStyleOptionFrame* qt_widgets_c_QStyleOptionFrame_new_other(const QStyleOptionFrame* other) {
  return new QStyleOptionFrame(*other);
}

void qt_widgets_c_QStyleOptionFrame_set_features(QStyleOptionFrame* this_ptr, unsigned int value) {
  this_ptr->features = QFlags< QStyleOptionFrame::FrameFeature >(value);
}

void qt_widgets_c_QStyleOptionFrame_set_frameShape(QStyleOptionFrame* this_ptr, const QFrame::Shape* value) {
  this_ptr->frameShape = *value;
}

void qt_widgets_c_QStyleOptionFrame_set_lineWidth(QStyleOptionFrame* this_ptr, int value) {
  this_ptr->lineWidth = value;
}

void qt_widgets_c_QStyleOptionFrame_set_midLineWidth(QStyleOptionFrame* this_ptr, int value) {
  this_ptr->midLineWidth = value;
}

