#include "qt_widgets_c_QAbstractSpinBox.h"

QAbstractSpinBox* qt_widgets_c_QAbstractSpinBox_G_dynamic_cast_QAbstractSpinBox_ptr(QWidget* ptr) {
  return dynamic_cast<QAbstractSpinBox*>(ptr);
}

QAbstractSpinBox* qt_widgets_c_QAbstractSpinBox_G_static_cast_QAbstractSpinBox_ptr_QObject(QObject* ptr) {
  return static_cast<QAbstractSpinBox*>(ptr);
}

QAbstractSpinBox* qt_widgets_c_QAbstractSpinBox_G_static_cast_QAbstractSpinBox_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QAbstractSpinBox*>(ptr);
}

QAbstractSpinBox* qt_widgets_c_QAbstractSpinBox_G_static_cast_QAbstractSpinBox_ptr_QWidget(QWidget* ptr) {
  return static_cast<QAbstractSpinBox*>(ptr);
}

QObject* qt_widgets_c_QAbstractSpinBox_G_static_cast_QObject_ptr(QAbstractSpinBox* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QAbstractSpinBox_G_static_cast_QPaintDevice_ptr(QAbstractSpinBox* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QAbstractSpinBox_G_static_cast_QWidget_ptr(QAbstractSpinBox* ptr) {
  return static_cast<QWidget*>(ptr);
}

QAbstractSpinBox::ButtonSymbols qt_widgets_c_QAbstractSpinBox_buttonSymbols(const QAbstractSpinBox* this_ptr) {
  return this_ptr->buttonSymbols();
}

void qt_widgets_c_QAbstractSpinBox_clear(QAbstractSpinBox* this_ptr) {
  this_ptr->clear();
}

QAbstractSpinBox::CorrectionMode qt_widgets_c_QAbstractSpinBox_correctionMode(const QAbstractSpinBox* this_ptr) {
  return this_ptr->correctionMode();
}

void qt_widgets_c_QAbstractSpinBox_delete(QAbstractSpinBox* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QAbstractSpinBox_event(QAbstractSpinBox* this_ptr, QEvent* event) {
  return this_ptr->event(event);
}

void qt_widgets_c_QAbstractSpinBox_fixup(const QAbstractSpinBox* this_ptr, QString* input) {
  this_ptr->fixup(*input);
}

bool qt_widgets_c_QAbstractSpinBox_hasAcceptableInput(const QAbstractSpinBox* this_ptr) {
  return this_ptr->hasAcceptableInput();
}

bool qt_widgets_c_QAbstractSpinBox_hasFrame(const QAbstractSpinBox* this_ptr) {
  return this_ptr->hasFrame();
}

void qt_widgets_c_QAbstractSpinBox_inputMethodQuery_to_output(const QAbstractSpinBox* this_ptr, const Qt::InputMethodQuery* arg1, QVariant* output) {
  new(output) QVariant(this_ptr->inputMethodQuery(*arg1));
}

void qt_widgets_c_QAbstractSpinBox_interpretText(QAbstractSpinBox* this_ptr) {
  this_ptr->interpretText();
}

bool qt_widgets_c_QAbstractSpinBox_isAccelerated(const QAbstractSpinBox* this_ptr) {
  return this_ptr->isAccelerated();
}

bool qt_widgets_c_QAbstractSpinBox_isGroupSeparatorShown(const QAbstractSpinBox* this_ptr) {
  return this_ptr->isGroupSeparatorShown();
}

bool qt_widgets_c_QAbstractSpinBox_isReadOnly(const QAbstractSpinBox* this_ptr) {
  return this_ptr->isReadOnly();
}

bool qt_widgets_c_QAbstractSpinBox_keyboardTracking(const QAbstractSpinBox* this_ptr) {
  return this_ptr->keyboardTracking();
}

const QMetaObject* qt_widgets_c_QAbstractSpinBox_metaObject(const QAbstractSpinBox* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QAbstractSpinBox_minimumSizeHint_to_output(const QAbstractSpinBox* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSizeHint());
}

QAbstractSpinBox* qt_widgets_c_QAbstractSpinBox_new_no_args() {
  return new QAbstractSpinBox();
}

QAbstractSpinBox* qt_widgets_c_QAbstractSpinBox_new_parent(QWidget* parent) {
  return new QAbstractSpinBox(parent);
}

int qt_widgets_c_QAbstractSpinBox_qt_metacall(QAbstractSpinBox* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QAbstractSpinBox_qt_metacast(QAbstractSpinBox* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QAbstractSpinBox_selectAll(QAbstractSpinBox* this_ptr) {
  this_ptr->selectAll();
}

void qt_widgets_c_QAbstractSpinBox_setAccelerated(QAbstractSpinBox* this_ptr, bool on) {
  this_ptr->setAccelerated(on);
}

void qt_widgets_c_QAbstractSpinBox_setButtonSymbols(QAbstractSpinBox* this_ptr, QAbstractSpinBox::ButtonSymbols bs) {
  this_ptr->setButtonSymbols(bs);
}

void qt_widgets_c_QAbstractSpinBox_setCorrectionMode(QAbstractSpinBox* this_ptr, QAbstractSpinBox::CorrectionMode cm) {
  this_ptr->setCorrectionMode(cm);
}

void qt_widgets_c_QAbstractSpinBox_setFrame(QAbstractSpinBox* this_ptr, bool arg1) {
  this_ptr->setFrame(arg1);
}

void qt_widgets_c_QAbstractSpinBox_setGroupSeparatorShown(QAbstractSpinBox* this_ptr, bool shown) {
  this_ptr->setGroupSeparatorShown(shown);
}

void qt_widgets_c_QAbstractSpinBox_setKeyboardTracking(QAbstractSpinBox* this_ptr, bool kt) {
  this_ptr->setKeyboardTracking(kt);
}

void qt_widgets_c_QAbstractSpinBox_setReadOnly(QAbstractSpinBox* this_ptr, bool r) {
  this_ptr->setReadOnly(r);
}

void qt_widgets_c_QAbstractSpinBox_setSpecialValueText(QAbstractSpinBox* this_ptr, const QString* txt) {
  this_ptr->setSpecialValueText(*txt);
}

void qt_widgets_c_QAbstractSpinBox_setWrapping(QAbstractSpinBox* this_ptr, bool w) {
  this_ptr->setWrapping(w);
}

void qt_widgets_c_QAbstractSpinBox_sizeHint_to_output(const QAbstractSpinBox* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QAbstractSpinBox_specialValueText_to_output(const QAbstractSpinBox* this_ptr, QString* output) {
  new(output) QString(this_ptr->specialValueText());
}

void qt_widgets_c_QAbstractSpinBox_stepBy(QAbstractSpinBox* this_ptr, int steps) {
  this_ptr->stepBy(steps);
}

void qt_widgets_c_QAbstractSpinBox_stepDown(QAbstractSpinBox* this_ptr) {
  this_ptr->stepDown();
}

void qt_widgets_c_QAbstractSpinBox_stepUp(QAbstractSpinBox* this_ptr) {
  this_ptr->stepUp();
}

void qt_widgets_c_QAbstractSpinBox_text_to_output(const QAbstractSpinBox* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

void qt_widgets_c_QAbstractSpinBox_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractSpinBox::trUtf8(s, c, n));
}

void qt_widgets_c_QAbstractSpinBox_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractSpinBox::tr(s, c, n));
}

bool qt_widgets_c_QAbstractSpinBox_wrapping(const QAbstractSpinBox* this_ptr) {
  return this_ptr->wrapping();
}

