#include "qt_widgets_c_QAbstractButton.h"

QAbstractButton* qt_widgets_c_QAbstractButton_G_dynamic_cast_QAbstractButton_ptr(QWidget* ptr) {
  return dynamic_cast<QAbstractButton*>(ptr);
}

QAbstractButton* qt_widgets_c_QAbstractButton_G_static_cast_QAbstractButton_ptr_QObject(QObject* ptr) {
  return static_cast<QAbstractButton*>(ptr);
}

QAbstractButton* qt_widgets_c_QAbstractButton_G_static_cast_QAbstractButton_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QAbstractButton*>(ptr);
}

QAbstractButton* qt_widgets_c_QAbstractButton_G_static_cast_QAbstractButton_ptr_QWidget(QWidget* ptr) {
  return static_cast<QAbstractButton*>(ptr);
}

QObject* qt_widgets_c_QAbstractButton_G_static_cast_QObject_ptr(QAbstractButton* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QAbstractButton_G_static_cast_QPaintDevice_ptr(QAbstractButton* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QAbstractButton_G_static_cast_QWidget_ptr(QAbstractButton* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QAbstractButton_animateClick_msec(QAbstractButton* this_ptr, int msec) {
  this_ptr->animateClick(msec);
}

void qt_widgets_c_QAbstractButton_animateClick_no_args(QAbstractButton* this_ptr) {
  this_ptr->animateClick();
}

bool qt_widgets_c_QAbstractButton_autoExclusive(const QAbstractButton* this_ptr) {
  return this_ptr->autoExclusive();
}

bool qt_widgets_c_QAbstractButton_autoRepeat(const QAbstractButton* this_ptr) {
  return this_ptr->autoRepeat();
}

int qt_widgets_c_QAbstractButton_autoRepeatDelay(const QAbstractButton* this_ptr) {
  return this_ptr->autoRepeatDelay();
}

int qt_widgets_c_QAbstractButton_autoRepeatInterval(const QAbstractButton* this_ptr) {
  return this_ptr->autoRepeatInterval();
}

void qt_widgets_c_QAbstractButton_click(QAbstractButton* this_ptr) {
  this_ptr->click();
}

void qt_widgets_c_QAbstractButton_delete(QAbstractButton* this_ptr) {
  delete this_ptr;
}

QButtonGroup* qt_widgets_c_QAbstractButton_group(const QAbstractButton* this_ptr) {
  return this_ptr->group();
}

void qt_widgets_c_QAbstractButton_iconSize_to_output(const QAbstractButton* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->iconSize());
}

void qt_widgets_c_QAbstractButton_icon_to_output(const QAbstractButton* this_ptr, QIcon* output) {
  new(output) QIcon(this_ptr->icon());
}

bool qt_widgets_c_QAbstractButton_isCheckable(const QAbstractButton* this_ptr) {
  return this_ptr->isCheckable();
}

bool qt_widgets_c_QAbstractButton_isChecked(const QAbstractButton* this_ptr) {
  return this_ptr->isChecked();
}

bool qt_widgets_c_QAbstractButton_isDown(const QAbstractButton* this_ptr) {
  return this_ptr->isDown();
}

const QMetaObject* qt_widgets_c_QAbstractButton_metaObject(const QAbstractButton* this_ptr) {
  return this_ptr->metaObject();
}

int qt_widgets_c_QAbstractButton_qt_metacall(QAbstractButton* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QAbstractButton_qt_metacast(QAbstractButton* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QAbstractButton_setAutoExclusive(QAbstractButton* this_ptr, bool arg1) {
  this_ptr->setAutoExclusive(arg1);
}

void qt_widgets_c_QAbstractButton_setAutoRepeat(QAbstractButton* this_ptr, bool arg1) {
  this_ptr->setAutoRepeat(arg1);
}

void qt_widgets_c_QAbstractButton_setAutoRepeatDelay(QAbstractButton* this_ptr, int arg1) {
  this_ptr->setAutoRepeatDelay(arg1);
}

void qt_widgets_c_QAbstractButton_setAutoRepeatInterval(QAbstractButton* this_ptr, int arg1) {
  this_ptr->setAutoRepeatInterval(arg1);
}

void qt_widgets_c_QAbstractButton_setCheckable(QAbstractButton* this_ptr, bool arg1) {
  this_ptr->setCheckable(arg1);
}

void qt_widgets_c_QAbstractButton_setChecked(QAbstractButton* this_ptr, bool arg1) {
  this_ptr->setChecked(arg1);
}

void qt_widgets_c_QAbstractButton_setDown(QAbstractButton* this_ptr, bool arg1) {
  this_ptr->setDown(arg1);
}

void qt_widgets_c_QAbstractButton_setIcon(QAbstractButton* this_ptr, const QIcon* icon) {
  this_ptr->setIcon(*icon);
}

void qt_widgets_c_QAbstractButton_setIconSize(QAbstractButton* this_ptr, const QSize* size) {
  this_ptr->setIconSize(*size);
}

void qt_widgets_c_QAbstractButton_setShortcut(QAbstractButton* this_ptr, const QKeySequence* key) {
  this_ptr->setShortcut(*key);
}

void qt_widgets_c_QAbstractButton_setText(QAbstractButton* this_ptr, const QString* text) {
  this_ptr->setText(*text);
}

void qt_widgets_c_QAbstractButton_shortcut_to_output(const QAbstractButton* this_ptr, QKeySequence* output) {
  new(output) QKeySequence(this_ptr->shortcut());
}

void qt_widgets_c_QAbstractButton_text_to_output(const QAbstractButton* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

void qt_widgets_c_QAbstractButton_toggle(QAbstractButton* this_ptr) {
  this_ptr->toggle();
}

void qt_widgets_c_QAbstractButton_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractButton::trUtf8(s, c, n));
}

void qt_widgets_c_QAbstractButton_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractButton::tr(s, c, n));
}

