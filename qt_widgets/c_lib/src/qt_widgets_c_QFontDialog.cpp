#include "qt_widgets_c_QFontDialog.h"

QFontDialog* qt_widgets_c_QFontDialog_G_dynamic_cast_QFontDialog_ptr_QDialog(QDialog* ptr) {
  return dynamic_cast<QFontDialog*>(ptr);
}

QFontDialog* qt_widgets_c_QFontDialog_G_dynamic_cast_QFontDialog_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QFontDialog*>(ptr);
}

QDialog* qt_widgets_c_QFontDialog_G_static_cast_QDialog_ptr(QFontDialog* ptr) {
  return static_cast<QDialog*>(ptr);
}

QFontDialog* qt_widgets_c_QFontDialog_G_static_cast_QFontDialog_ptr_QDialog(QDialog* ptr) {
  return static_cast<QFontDialog*>(ptr);
}

QFontDialog* qt_widgets_c_QFontDialog_G_static_cast_QFontDialog_ptr_QObject(QObject* ptr) {
  return static_cast<QFontDialog*>(ptr);
}

QFontDialog* qt_widgets_c_QFontDialog_G_static_cast_QFontDialog_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QFontDialog*>(ptr);
}

QFontDialog* qt_widgets_c_QFontDialog_G_static_cast_QFontDialog_ptr_QWidget(QWidget* ptr) {
  return static_cast<QFontDialog*>(ptr);
}

QObject* qt_widgets_c_QFontDialog_G_static_cast_QObject_ptr(QFontDialog* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QFontDialog_G_static_cast_QPaintDevice_ptr(QFontDialog* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QFontDialog_G_static_cast_QWidget_ptr(QFontDialog* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QFontDialog_currentFont_to_output(const QFontDialog* this_ptr, QFont* output) {
  new(output) QFont(this_ptr->currentFont());
}

void qt_widgets_c_QFontDialog_delete(QFontDialog* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QFontDialog_getFont_to_output_ok(bool* ok, QFont* output) {
  new(output) QFont(QFontDialog::getFont(ok));
}

void qt_widgets_c_QFontDialog_getFont_to_output_ok_initial(bool* ok, const QFont* initial, QFont* output) {
  new(output) QFont(QFontDialog::getFont(ok, *initial));
}

void qt_widgets_c_QFontDialog_getFont_to_output_ok_initial_parent(bool* ok, const QFont* initial, QWidget* parent, QFont* output) {
  new(output) QFont(QFontDialog::getFont(ok, *initial, parent));
}

void qt_widgets_c_QFontDialog_getFont_to_output_ok_initial_parent_title(bool* ok, const QFont* initial, QWidget* parent, const QString* title, QFont* output) {
  new(output) QFont(QFontDialog::getFont(ok, *initial, parent, *title));
}

void qt_widgets_c_QFontDialog_getFont_to_output_ok_initial_parent_title_options(bool* ok, const QFont* initial, QWidget* parent, const QString* title, unsigned int options, QFont* output) {
  new(output) QFont(QFontDialog::getFont(ok, *initial, parent, *title, QFlags< QFontDialog::FontDialogOption >(options)));
}

void qt_widgets_c_QFontDialog_getFont_to_output_ok_parent(bool* ok, QWidget* parent, QFont* output) {
  new(output) QFont(QFontDialog::getFont(ok, parent));
}

const QMetaObject* qt_widgets_c_QFontDialog_metaObject(const QFontDialog* this_ptr) {
  return this_ptr->metaObject();
}

QFontDialog* qt_widgets_c_QFontDialog_new_initial(const QFont* initial) {
  return new QFontDialog(*initial);
}

QFontDialog* qt_widgets_c_QFontDialog_new_initial_parent(const QFont* initial, QWidget* parent) {
  return new QFontDialog(*initial, parent);
}

QFontDialog* qt_widgets_c_QFontDialog_new_no_args() {
  return new QFontDialog();
}

QFontDialog* qt_widgets_c_QFontDialog_new_parent(QWidget* parent) {
  return new QFontDialog(parent);
}

void qt_widgets_c_QFontDialog_open(QFontDialog* this_ptr, QObject* receiver, const char* member) {
  this_ptr->open(receiver, member);
}

unsigned int qt_widgets_c_QFontDialog_options(const QFontDialog* this_ptr) {
  return uint(this_ptr->options());
}

int qt_widgets_c_QFontDialog_qt_metacall(QFontDialog* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QFontDialog_qt_metacast(QFontDialog* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QFontDialog_selectedFont_to_output(const QFontDialog* this_ptr, QFont* output) {
  new(output) QFont(this_ptr->selectedFont());
}

void qt_widgets_c_QFontDialog_setCurrentFont(QFontDialog* this_ptr, const QFont* font) {
  this_ptr->setCurrentFont(*font);
}

void qt_widgets_c_QFontDialog_setOption_option(QFontDialog* this_ptr, QFontDialog::FontDialogOption option) {
  this_ptr->setOption(option);
}

void qt_widgets_c_QFontDialog_setOption_option_on(QFontDialog* this_ptr, QFontDialog::FontDialogOption option, bool on) {
  this_ptr->setOption(option, on);
}

void qt_widgets_c_QFontDialog_setOptions(QFontDialog* this_ptr, unsigned int options) {
  this_ptr->setOptions(QFlags< QFontDialog::FontDialogOption >(options));
}

void qt_widgets_c_QFontDialog_setVisible(QFontDialog* this_ptr, bool visible) {
  this_ptr->setVisible(visible);
}

bool qt_widgets_c_QFontDialog_testOption(const QFontDialog* this_ptr, QFontDialog::FontDialogOption option) {
  return this_ptr->testOption(option);
}

void qt_widgets_c_QFontDialog_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFontDialog::trUtf8(s, c, n));
}

void qt_widgets_c_QFontDialog_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFontDialog::tr(s, c, n));
}

