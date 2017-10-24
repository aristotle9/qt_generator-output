#include "qt_widgets_c_QLCDNumber.h"

QLCDNumber* qt_widgets_c_QLCDNumber_G_dynamic_cast_QLCDNumber_ptr_QFrame(QFrame* ptr) {
  return dynamic_cast<QLCDNumber*>(ptr);
}

QLCDNumber* qt_widgets_c_QLCDNumber_G_dynamic_cast_QLCDNumber_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QLCDNumber*>(ptr);
}

QFrame* qt_widgets_c_QLCDNumber_G_static_cast_QFrame_ptr(QLCDNumber* ptr) {
  return static_cast<QFrame*>(ptr);
}

QLCDNumber* qt_widgets_c_QLCDNumber_G_static_cast_QLCDNumber_ptr_QFrame(QFrame* ptr) {
  return static_cast<QLCDNumber*>(ptr);
}

QLCDNumber* qt_widgets_c_QLCDNumber_G_static_cast_QLCDNumber_ptr_QObject(QObject* ptr) {
  return static_cast<QLCDNumber*>(ptr);
}

QLCDNumber* qt_widgets_c_QLCDNumber_G_static_cast_QLCDNumber_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QLCDNumber*>(ptr);
}

QLCDNumber* qt_widgets_c_QLCDNumber_G_static_cast_QLCDNumber_ptr_QWidget(QWidget* ptr) {
  return static_cast<QLCDNumber*>(ptr);
}

QObject* qt_widgets_c_QLCDNumber_G_static_cast_QObject_ptr(QLCDNumber* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QLCDNumber_G_static_cast_QPaintDevice_ptr(QLCDNumber* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QLCDNumber_G_static_cast_QWidget_ptr(QLCDNumber* ptr) {
  return static_cast<QWidget*>(ptr);
}

bool qt_widgets_c_QLCDNumber_checkOverflow_double(const QLCDNumber* this_ptr, double num) {
  return this_ptr->checkOverflow(num);
}

bool qt_widgets_c_QLCDNumber_checkOverflow_int(const QLCDNumber* this_ptr, int num) {
  return this_ptr->checkOverflow(num);
}

void qt_widgets_c_QLCDNumber_delete(QLCDNumber* this_ptr) {
  delete this_ptr;
}

int qt_widgets_c_QLCDNumber_digitCount(const QLCDNumber* this_ptr) {
  return this_ptr->digitCount();
}

void qt_widgets_c_QLCDNumber_display_QString(QLCDNumber* this_ptr, const QString* str) {
  this_ptr->display(*str);
}

void qt_widgets_c_QLCDNumber_display_double(QLCDNumber* this_ptr, double num) {
  this_ptr->display(num);
}

void qt_widgets_c_QLCDNumber_display_int(QLCDNumber* this_ptr, int num) {
  this_ptr->display(num);
}

int qt_widgets_c_QLCDNumber_intValue(const QLCDNumber* this_ptr) {
  return this_ptr->intValue();
}

const QMetaObject* qt_widgets_c_QLCDNumber_metaObject(const QLCDNumber* this_ptr) {
  return this_ptr->metaObject();
}

QLCDNumber::Mode qt_widgets_c_QLCDNumber_mode(const QLCDNumber* this_ptr) {
  return this_ptr->mode();
}

QLCDNumber* qt_widgets_c_QLCDNumber_new_no_args() {
  return new QLCDNumber();
}

QLCDNumber* qt_widgets_c_QLCDNumber_new_numDigits(unsigned int numDigits) {
  return new QLCDNumber(numDigits);
}

QLCDNumber* qt_widgets_c_QLCDNumber_new_numDigits_parent(unsigned int numDigits, QWidget* parent) {
  return new QLCDNumber(numDigits, parent);
}

QLCDNumber* qt_widgets_c_QLCDNumber_new_parent(QWidget* parent) {
  return new QLCDNumber(parent);
}

int qt_widgets_c_QLCDNumber_qt_metacall(QLCDNumber* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QLCDNumber_qt_metacast(QLCDNumber* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

QLCDNumber::SegmentStyle qt_widgets_c_QLCDNumber_segmentStyle(const QLCDNumber* this_ptr) {
  return this_ptr->segmentStyle();
}

void qt_widgets_c_QLCDNumber_setBinMode(QLCDNumber* this_ptr) {
  this_ptr->setBinMode();
}

void qt_widgets_c_QLCDNumber_setDecMode(QLCDNumber* this_ptr) {
  this_ptr->setDecMode();
}

void qt_widgets_c_QLCDNumber_setDigitCount(QLCDNumber* this_ptr, int nDigits) {
  this_ptr->setDigitCount(nDigits);
}

void qt_widgets_c_QLCDNumber_setHexMode(QLCDNumber* this_ptr) {
  this_ptr->setHexMode();
}

void qt_widgets_c_QLCDNumber_setMode(QLCDNumber* this_ptr, QLCDNumber::Mode arg1) {
  this_ptr->setMode(arg1);
}

void qt_widgets_c_QLCDNumber_setOctMode(QLCDNumber* this_ptr) {
  this_ptr->setOctMode();
}

void qt_widgets_c_QLCDNumber_setSegmentStyle(QLCDNumber* this_ptr, QLCDNumber::SegmentStyle arg1) {
  this_ptr->setSegmentStyle(arg1);
}

void qt_widgets_c_QLCDNumber_setSmallDecimalPoint(QLCDNumber* this_ptr, bool arg1) {
  this_ptr->setSmallDecimalPoint(arg1);
}

void qt_widgets_c_QLCDNumber_sizeHint_to_output(const QLCDNumber* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

bool qt_widgets_c_QLCDNumber_smallDecimalPoint(const QLCDNumber* this_ptr) {
  return this_ptr->smallDecimalPoint();
}

void qt_widgets_c_QLCDNumber_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QLCDNumber::trUtf8(s, c, n));
}

void qt_widgets_c_QLCDNumber_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QLCDNumber::tr(s, c, n));
}

double qt_widgets_c_QLCDNumber_value(const QLCDNumber* this_ptr) {
  return this_ptr->value();
}

