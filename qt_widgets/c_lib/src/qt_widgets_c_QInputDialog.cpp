#include "qt_widgets_c_QInputDialog.h"

QInputDialog* qt_widgets_c_QInputDialog_G_dynamic_cast_QInputDialog_ptr_QDialog(QDialog* ptr) {
  return dynamic_cast<QInputDialog*>(ptr);
}

QInputDialog* qt_widgets_c_QInputDialog_G_dynamic_cast_QInputDialog_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QInputDialog*>(ptr);
}

QDialog* qt_widgets_c_QInputDialog_G_static_cast_QDialog_ptr(QInputDialog* ptr) {
  return static_cast<QDialog*>(ptr);
}

QInputDialog* qt_widgets_c_QInputDialog_G_static_cast_QInputDialog_ptr_QDialog(QDialog* ptr) {
  return static_cast<QInputDialog*>(ptr);
}

QInputDialog* qt_widgets_c_QInputDialog_G_static_cast_QInputDialog_ptr_QObject(QObject* ptr) {
  return static_cast<QInputDialog*>(ptr);
}

QInputDialog* qt_widgets_c_QInputDialog_G_static_cast_QInputDialog_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QInputDialog*>(ptr);
}

QInputDialog* qt_widgets_c_QInputDialog_G_static_cast_QInputDialog_ptr_QWidget(QWidget* ptr) {
  return static_cast<QInputDialog*>(ptr);
}

QObject* qt_widgets_c_QInputDialog_G_static_cast_QObject_ptr(QInputDialog* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QInputDialog_G_static_cast_QPaintDevice_ptr(QInputDialog* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QInputDialog_G_static_cast_QWidget_ptr(QInputDialog* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QInputDialog_cancelButtonText_to_output(const QInputDialog* this_ptr, QString* output) {
  new(output) QString(this_ptr->cancelButtonText());
}

void qt_widgets_c_QInputDialog_comboBoxItems_to_output(const QInputDialog* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->comboBoxItems());
}

void qt_widgets_c_QInputDialog_delete(QInputDialog* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QInputDialog_done(QInputDialog* this_ptr, int result) {
  this_ptr->done(result);
}

int qt_widgets_c_QInputDialog_doubleDecimals(const QInputDialog* this_ptr) {
  return this_ptr->doubleDecimals();
}

double qt_widgets_c_QInputDialog_doubleMaximum(const QInputDialog* this_ptr) {
  return this_ptr->doubleMaximum();
}

double qt_widgets_c_QInputDialog_doubleMinimum(const QInputDialog* this_ptr) {
  return this_ptr->doubleMinimum();
}

double qt_widgets_c_QInputDialog_doubleValue(const QInputDialog* this_ptr) {
  return this_ptr->doubleValue();
}

QInputDialog::InputMode qt_widgets_c_QInputDialog_inputMode(const QInputDialog* this_ptr) {
  return this_ptr->inputMode();
}

int qt_widgets_c_QInputDialog_intMaximum(const QInputDialog* this_ptr) {
  return this_ptr->intMaximum();
}

int qt_widgets_c_QInputDialog_intMinimum(const QInputDialog* this_ptr) {
  return this_ptr->intMinimum();
}

int qt_widgets_c_QInputDialog_intStep(const QInputDialog* this_ptr) {
  return this_ptr->intStep();
}

int qt_widgets_c_QInputDialog_intValue(const QInputDialog* this_ptr) {
  return this_ptr->intValue();
}

bool qt_widgets_c_QInputDialog_isComboBoxEditable(const QInputDialog* this_ptr) {
  return this_ptr->isComboBoxEditable();
}

void qt_widgets_c_QInputDialog_labelText_to_output(const QInputDialog* this_ptr, QString* output) {
  new(output) QString(this_ptr->labelText());
}

const QMetaObject* qt_widgets_c_QInputDialog_metaObject(const QInputDialog* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QInputDialog_minimumSizeHint_to_output(const QInputDialog* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSizeHint());
}

void qt_widgets_c_QInputDialog_okButtonText_to_output(const QInputDialog* this_ptr, QString* output) {
  new(output) QString(this_ptr->okButtonText());
}

void qt_widgets_c_QInputDialog_open(QInputDialog* this_ptr, QObject* receiver, const char* member) {
  this_ptr->open(receiver, member);
}

unsigned int qt_widgets_c_QInputDialog_options(const QInputDialog* this_ptr) {
  return uint(this_ptr->options());
}

int qt_widgets_c_QInputDialog_qt_metacall(QInputDialog* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QInputDialog_qt_metacast(QInputDialog* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QInputDialog_setCancelButtonText(QInputDialog* this_ptr, const QString* text) {
  this_ptr->setCancelButtonText(*text);
}

void qt_widgets_c_QInputDialog_setComboBoxEditable(QInputDialog* this_ptr, bool editable) {
  this_ptr->setComboBoxEditable(editable);
}

void qt_widgets_c_QInputDialog_setComboBoxItems(QInputDialog* this_ptr, const QStringList* items) {
  this_ptr->setComboBoxItems(*items);
}

void qt_widgets_c_QInputDialog_setDoubleDecimals(QInputDialog* this_ptr, int decimals) {
  this_ptr->setDoubleDecimals(decimals);
}

void qt_widgets_c_QInputDialog_setDoubleMaximum(QInputDialog* this_ptr, double max) {
  this_ptr->setDoubleMaximum(max);
}

void qt_widgets_c_QInputDialog_setDoubleMinimum(QInputDialog* this_ptr, double min) {
  this_ptr->setDoubleMinimum(min);
}

void qt_widgets_c_QInputDialog_setDoubleRange(QInputDialog* this_ptr, double min, double max) {
  this_ptr->setDoubleRange(min, max);
}

void qt_widgets_c_QInputDialog_setDoubleValue(QInputDialog* this_ptr, double value) {
  this_ptr->setDoubleValue(value);
}

void qt_widgets_c_QInputDialog_setInputMode(QInputDialog* this_ptr, QInputDialog::InputMode mode) {
  this_ptr->setInputMode(mode);
}

void qt_widgets_c_QInputDialog_setIntMaximum(QInputDialog* this_ptr, int max) {
  this_ptr->setIntMaximum(max);
}

void qt_widgets_c_QInputDialog_setIntMinimum(QInputDialog* this_ptr, int min) {
  this_ptr->setIntMinimum(min);
}

void qt_widgets_c_QInputDialog_setIntRange(QInputDialog* this_ptr, int min, int max) {
  this_ptr->setIntRange(min, max);
}

void qt_widgets_c_QInputDialog_setIntStep(QInputDialog* this_ptr, int step) {
  this_ptr->setIntStep(step);
}

void qt_widgets_c_QInputDialog_setIntValue(QInputDialog* this_ptr, int value) {
  this_ptr->setIntValue(value);
}

void qt_widgets_c_QInputDialog_setLabelText(QInputDialog* this_ptr, const QString* text) {
  this_ptr->setLabelText(*text);
}

void qt_widgets_c_QInputDialog_setOkButtonText(QInputDialog* this_ptr, const QString* text) {
  this_ptr->setOkButtonText(*text);
}

void qt_widgets_c_QInputDialog_setOption_option(QInputDialog* this_ptr, QInputDialog::InputDialogOption option) {
  this_ptr->setOption(option);
}

void qt_widgets_c_QInputDialog_setOption_option_on(QInputDialog* this_ptr, QInputDialog::InputDialogOption option, bool on) {
  this_ptr->setOption(option, on);
}

void qt_widgets_c_QInputDialog_setOptions(QInputDialog* this_ptr, unsigned int options) {
  this_ptr->setOptions(QFlags< QInputDialog::InputDialogOption >(options));
}

void qt_widgets_c_QInputDialog_setTextEchoMode(QInputDialog* this_ptr, const QLineEdit::EchoMode* mode) {
  this_ptr->setTextEchoMode(*mode);
}

void qt_widgets_c_QInputDialog_setTextValue(QInputDialog* this_ptr, const QString* text) {
  this_ptr->setTextValue(*text);
}

void qt_widgets_c_QInputDialog_setVisible(QInputDialog* this_ptr, bool visible) {
  this_ptr->setVisible(visible);
}

void qt_widgets_c_QInputDialog_sizeHint_to_output(const QInputDialog* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

bool qt_widgets_c_QInputDialog_testOption(const QInputDialog* this_ptr, QInputDialog::InputDialogOption option) {
  return this_ptr->testOption(option);
}

void qt_widgets_c_QInputDialog_textValue_to_output(const QInputDialog* this_ptr, QString* output) {
  new(output) QString(this_ptr->textValue());
}

void qt_widgets_c_QInputDialog_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QInputDialog::trUtf8(s, c, n));
}

void qt_widgets_c_QInputDialog_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QInputDialog::tr(s, c, n));
}

