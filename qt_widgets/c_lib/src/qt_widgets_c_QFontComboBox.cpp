#include "qt_widgets_c_QFontComboBox.h"

QFontComboBox* qt_widgets_c_QFontComboBox_G_dynamic_cast_QFontComboBox_ptr_QComboBox(QComboBox* ptr) {
  return dynamic_cast<QFontComboBox*>(ptr);
}

QFontComboBox* qt_widgets_c_QFontComboBox_G_dynamic_cast_QFontComboBox_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QFontComboBox*>(ptr);
}

QComboBox* qt_widgets_c_QFontComboBox_G_static_cast_QComboBox_ptr(QFontComboBox* ptr) {
  return static_cast<QComboBox*>(ptr);
}

QFontComboBox* qt_widgets_c_QFontComboBox_G_static_cast_QFontComboBox_ptr_QComboBox(QComboBox* ptr) {
  return static_cast<QFontComboBox*>(ptr);
}

QFontComboBox* qt_widgets_c_QFontComboBox_G_static_cast_QFontComboBox_ptr_QObject(QObject* ptr) {
  return static_cast<QFontComboBox*>(ptr);
}

QFontComboBox* qt_widgets_c_QFontComboBox_G_static_cast_QFontComboBox_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QFontComboBox*>(ptr);
}

QFontComboBox* qt_widgets_c_QFontComboBox_G_static_cast_QFontComboBox_ptr_QWidget(QWidget* ptr) {
  return static_cast<QFontComboBox*>(ptr);
}

QObject* qt_widgets_c_QFontComboBox_G_static_cast_QObject_ptr(QFontComboBox* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QFontComboBox_G_static_cast_QPaintDevice_ptr(QFontComboBox* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QFontComboBox_G_static_cast_QWidget_ptr(QFontComboBox* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QFontComboBox_currentFont_to_output(const QFontComboBox* this_ptr, QFont* output) {
  new(output) QFont(this_ptr->currentFont());
}

void qt_widgets_c_QFontComboBox_delete(QFontComboBox* this_ptr) {
  delete this_ptr;
}

unsigned int qt_widgets_c_QFontComboBox_fontFilters(const QFontComboBox* this_ptr) {
  return uint(this_ptr->fontFilters());
}

const QMetaObject* qt_widgets_c_QFontComboBox_metaObject(const QFontComboBox* this_ptr) {
  return this_ptr->metaObject();
}

QFontComboBox* qt_widgets_c_QFontComboBox_new_no_args() {
  return new QFontComboBox();
}

QFontComboBox* qt_widgets_c_QFontComboBox_new_parent(QWidget* parent) {
  return new QFontComboBox(parent);
}

int qt_widgets_c_QFontComboBox_qt_metacall(QFontComboBox* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QFontComboBox_qt_metacast(QFontComboBox* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QFontComboBox_setCurrentFont(QFontComboBox* this_ptr, const QFont* f) {
  this_ptr->setCurrentFont(*f);
}

void qt_widgets_c_QFontComboBox_setFontFilters(QFontComboBox* this_ptr, unsigned int filters) {
  this_ptr->setFontFilters(QFlags< QFontComboBox::FontFilter >(filters));
}

void qt_widgets_c_QFontComboBox_setWritingSystem(QFontComboBox* this_ptr, const QFontDatabase::WritingSystem* arg1) {
  this_ptr->setWritingSystem(*arg1);
}

void qt_widgets_c_QFontComboBox_sizeHint_to_output(const QFontComboBox* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QFontComboBox_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFontComboBox::trUtf8(s, c, n));
}

void qt_widgets_c_QFontComboBox_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFontComboBox::tr(s, c, n));
}

