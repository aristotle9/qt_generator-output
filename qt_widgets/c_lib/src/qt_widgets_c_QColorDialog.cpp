#include "qt_widgets_c_QColorDialog.h"

QColorDialog* qt_widgets_c_QColorDialog_G_dynamic_cast_QColorDialog_ptr_QDialog(QDialog* ptr) {
  return dynamic_cast<QColorDialog*>(ptr);
}

QColorDialog* qt_widgets_c_QColorDialog_G_dynamic_cast_QColorDialog_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QColorDialog*>(ptr);
}

QColorDialog* qt_widgets_c_QColorDialog_G_static_cast_QColorDialog_ptr_QDialog(QDialog* ptr) {
  return static_cast<QColorDialog*>(ptr);
}

QColorDialog* qt_widgets_c_QColorDialog_G_static_cast_QColorDialog_ptr_QObject(QObject* ptr) {
  return static_cast<QColorDialog*>(ptr);
}

QColorDialog* qt_widgets_c_QColorDialog_G_static_cast_QColorDialog_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QColorDialog*>(ptr);
}

QColorDialog* qt_widgets_c_QColorDialog_G_static_cast_QColorDialog_ptr_QWidget(QWidget* ptr) {
  return static_cast<QColorDialog*>(ptr);
}

QDialog* qt_widgets_c_QColorDialog_G_static_cast_QDialog_ptr(QColorDialog* ptr) {
  return static_cast<QDialog*>(ptr);
}

QObject* qt_widgets_c_QColorDialog_G_static_cast_QObject_ptr(QColorDialog* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QColorDialog_G_static_cast_QPaintDevice_ptr(QColorDialog* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QColorDialog_G_static_cast_QWidget_ptr(QColorDialog* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QColorDialog_currentColor_to_output(const QColorDialog* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->currentColor());
}

void qt_widgets_c_QColorDialog_customColor_to_output(int index, QColor* output) {
  new(output) QColor(QColorDialog::customColor(index));
}

int qt_widgets_c_QColorDialog_customCount() {
  return QColorDialog::customCount();
}

void qt_widgets_c_QColorDialog_delete(QColorDialog* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QColorDialog_getColor_to_output_initial(const QColor* initial, QColor* output) {
  new(output) QColor(QColorDialog::getColor(*initial));
}

void qt_widgets_c_QColorDialog_getColor_to_output_initial_parent(const QColor* initial, QWidget* parent, QColor* output) {
  new(output) QColor(QColorDialog::getColor(*initial, parent));
}

void qt_widgets_c_QColorDialog_getColor_to_output_initial_parent_title(const QColor* initial, QWidget* parent, const QString* title, QColor* output) {
  new(output) QColor(QColorDialog::getColor(*initial, parent, *title));
}

void qt_widgets_c_QColorDialog_getColor_to_output_initial_parent_title_options(const QColor* initial, QWidget* parent, const QString* title, unsigned int options, QColor* output) {
  new(output) QColor(QColorDialog::getColor(*initial, parent, *title, QFlags< QColorDialog::ColorDialogOption >(options)));
}

void qt_widgets_c_QColorDialog_getColor_to_output_no_args(QColor* output) {
  new(output) QColor(QColorDialog::getColor());
}

unsigned int qt_widgets_c_QColorDialog_getRgba_no_args() {
  return QColorDialog::getRgba();
}

unsigned int qt_widgets_c_QColorDialog_getRgba_rgba(unsigned int rgba) {
  return QColorDialog::getRgba(rgba);
}

unsigned int qt_widgets_c_QColorDialog_getRgba_rgba_ok(unsigned int rgba, bool* ok) {
  return QColorDialog::getRgba(rgba, ok);
}

unsigned int qt_widgets_c_QColorDialog_getRgba_rgba_ok_parent(unsigned int rgba, bool* ok, QWidget* parent) {
  return QColorDialog::getRgba(rgba, ok, parent);
}

const QMetaObject* qt_widgets_c_QColorDialog_metaObject(const QColorDialog* this_ptr) {
  return this_ptr->metaObject();
}

QColorDialog* qt_widgets_c_QColorDialog_new_initial(const QColor* initial) {
  return new QColorDialog(*initial);
}

QColorDialog* qt_widgets_c_QColorDialog_new_initial_parent(const QColor* initial, QWidget* parent) {
  return new QColorDialog(*initial, parent);
}

QColorDialog* qt_widgets_c_QColorDialog_new_no_args() {
  return new QColorDialog();
}

QColorDialog* qt_widgets_c_QColorDialog_new_parent(QWidget* parent) {
  return new QColorDialog(parent);
}

void qt_widgets_c_QColorDialog_open(QColorDialog* this_ptr, QObject* receiver, const char* member) {
  this_ptr->open(receiver, member);
}

unsigned int qt_widgets_c_QColorDialog_options(const QColorDialog* this_ptr) {
  return uint(this_ptr->options());
}

int qt_widgets_c_QColorDialog_qt_metacall(QColorDialog* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QColorDialog_qt_metacast(QColorDialog* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QColorDialog_selectedColor_to_output(const QColorDialog* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->selectedColor());
}

void qt_widgets_c_QColorDialog_setCurrentColor(QColorDialog* this_ptr, const QColor* color) {
  this_ptr->setCurrentColor(*color);
}

void qt_widgets_c_QColorDialog_setCustomColor(int index, const QColor* color) {
  QColorDialog::setCustomColor(index, *color);
}

void qt_widgets_c_QColorDialog_setOption_option(QColorDialog* this_ptr, QColorDialog::ColorDialogOption option) {
  this_ptr->setOption(option);
}

void qt_widgets_c_QColorDialog_setOption_option_on(QColorDialog* this_ptr, QColorDialog::ColorDialogOption option, bool on) {
  this_ptr->setOption(option, on);
}

void qt_widgets_c_QColorDialog_setOptions(QColorDialog* this_ptr, unsigned int options) {
  this_ptr->setOptions(QFlags< QColorDialog::ColorDialogOption >(options));
}

void qt_widgets_c_QColorDialog_setStandardColor(int index, const QColor* color) {
  QColorDialog::setStandardColor(index, *color);
}

void qt_widgets_c_QColorDialog_setVisible(QColorDialog* this_ptr, bool visible) {
  this_ptr->setVisible(visible);
}

void qt_widgets_c_QColorDialog_standardColor_to_output(int index, QColor* output) {
  new(output) QColor(QColorDialog::standardColor(index));
}

bool qt_widgets_c_QColorDialog_testOption(const QColorDialog* this_ptr, QColorDialog::ColorDialogOption option) {
  return this_ptr->testOption(option);
}

void qt_widgets_c_QColorDialog_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QColorDialog::trUtf8(s, c, n));
}

void qt_widgets_c_QColorDialog_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QColorDialog::tr(s, c, n));
}

