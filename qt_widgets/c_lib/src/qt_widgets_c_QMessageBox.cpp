#include "qt_widgets_c_QMessageBox.h"

QMessageBox* qt_widgets_c_QMessageBox_G_dynamic_cast_QMessageBox_ptr_QDialog(QDialog* ptr) {
  return dynamic_cast<QMessageBox*>(ptr);
}

QMessageBox* qt_widgets_c_QMessageBox_G_dynamic_cast_QMessageBox_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QMessageBox*>(ptr);
}

QDialog* qt_widgets_c_QMessageBox_G_static_cast_QDialog_ptr(QMessageBox* ptr) {
  return static_cast<QDialog*>(ptr);
}

QMessageBox* qt_widgets_c_QMessageBox_G_static_cast_QMessageBox_ptr_QDialog(QDialog* ptr) {
  return static_cast<QMessageBox*>(ptr);
}

QMessageBox* qt_widgets_c_QMessageBox_G_static_cast_QMessageBox_ptr_QObject(QObject* ptr) {
  return static_cast<QMessageBox*>(ptr);
}

QMessageBox* qt_widgets_c_QMessageBox_G_static_cast_QMessageBox_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QMessageBox*>(ptr);
}

QMessageBox* qt_widgets_c_QMessageBox_G_static_cast_QMessageBox_ptr_QWidget(QWidget* ptr) {
  return static_cast<QMessageBox*>(ptr);
}

QObject* qt_widgets_c_QMessageBox_G_static_cast_QObject_ptr(QMessageBox* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QMessageBox_G_static_cast_QPaintDevice_ptr(QMessageBox* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QMessageBox_G_static_cast_QWidget_ptr(QMessageBox* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QMessageBox_about(QWidget* parent, const QString* title, const QString* text) {
  QMessageBox::about(parent, *title, *text);
}

void qt_widgets_c_QMessageBox_aboutQt_parent(QWidget* parent) {
  QMessageBox::aboutQt(parent);
}

void qt_widgets_c_QMessageBox_aboutQt_parent_title(QWidget* parent, const QString* title) {
  QMessageBox::aboutQt(parent, *title);
}

QPushButton* qt_widgets_c_QMessageBox_addButton_button(QMessageBox* this_ptr, QMessageBox::StandardButton button) {
  return this_ptr->addButton(button);
}

void qt_widgets_c_QMessageBox_addButton_button_role(QMessageBox* this_ptr, QAbstractButton* button, QMessageBox::ButtonRole role) {
  this_ptr->addButton(button, role);
}

QPushButton* qt_widgets_c_QMessageBox_addButton_text_role(QMessageBox* this_ptr, const QString* text, QMessageBox::ButtonRole role) {
  return this_ptr->addButton(*text, role);
}

QAbstractButton* qt_widgets_c_QMessageBox_button(const QMessageBox* this_ptr, QMessageBox::StandardButton which) {
  return this_ptr->button(which);
}

QMessageBox::ButtonRole qt_widgets_c_QMessageBox_buttonRole(const QMessageBox* this_ptr, QAbstractButton* button) {
  return this_ptr->buttonRole(button);
}

void qt_widgets_c_QMessageBox_buttonText_to_output(const QMessageBox* this_ptr, int button, QString* output) {
  new(output) QString(this_ptr->buttonText(button));
}

void qt_widgets_c_QMessageBox_buttons_to_output(const QMessageBox* this_ptr, QList< QAbstractButton* >* output) {
  new(output) QList< QAbstractButton* >(this_ptr->buttons());
}

QCheckBox* qt_widgets_c_QMessageBox_checkBox(const QMessageBox* this_ptr) {
  return this_ptr->checkBox();
}

QAbstractButton* qt_widgets_c_QMessageBox_clickedButton(const QMessageBox* this_ptr) {
  return this_ptr->clickedButton();
}

QMessageBox::StandardButton qt_widgets_c_QMessageBox_critical_QWidget_QString_QString(QWidget* parent, const QString* title, const QString* text) {
  return QMessageBox::critical(parent, *title, *text);
}

QMessageBox::StandardButton qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QFlags_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, unsigned int buttons) {
  return QMessageBox::critical(parent, *title, *text, QFlags< QMessageBox::StandardButton >(buttons));
}

QMessageBox::StandardButton qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QFlags_QMessageBox_StandardButton_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, unsigned int buttons, QMessageBox::StandardButton defaultButton) {
  return QMessageBox::critical(parent, *title, *text, QFlags< QMessageBox::StandardButton >(buttons), defaultButton);
}

int qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QMessageBox_StandardButton_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, QMessageBox::StandardButton button0, QMessageBox::StandardButton button1) {
  return QMessageBox::critical(parent, *title, *text, button0, button1);
}

int qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text) {
  return QMessageBox::critical(parent, *title, *text, *button0Text);
}

int qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text) {
  return QMessageBox::critical(parent, *title, *text, *button0Text, *button1Text);
}

int qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text) {
  return QMessageBox::critical(parent, *title, *text, *button0Text, *button1Text, *button2Text);
}

int qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QString_QString_QString_int(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text, int defaultButtonNumber) {
  return QMessageBox::critical(parent, *title, *text, *button0Text, *button1Text, *button2Text, defaultButtonNumber);
}

int qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QString_QString_QString_int_int(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text, int defaultButtonNumber, int escapeButtonNumber) {
  return QMessageBox::critical(parent, *title, *text, *button0Text, *button1Text, *button2Text, defaultButtonNumber, escapeButtonNumber);
}

int qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_int_int(QWidget* parent, const QString* title, const QString* text, int button0, int button1) {
  return QMessageBox::critical(parent, *title, *text, button0, button1);
}

int qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_int_int_int(QWidget* parent, const QString* title, const QString* text, int button0, int button1, int button2) {
  return QMessageBox::critical(parent, *title, *text, button0, button1, button2);
}

QPushButton* qt_widgets_c_QMessageBox_defaultButton(const QMessageBox* this_ptr) {
  return this_ptr->defaultButton();
}

void qt_widgets_c_QMessageBox_delete(QMessageBox* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QMessageBox_detailedText_to_output(const QMessageBox* this_ptr, QString* output) {
  new(output) QString(this_ptr->detailedText());
}

QAbstractButton* qt_widgets_c_QMessageBox_escapeButton(const QMessageBox* this_ptr) {
  return this_ptr->escapeButton();
}

QMessageBox::Icon qt_widgets_c_QMessageBox_icon(const QMessageBox* this_ptr) {
  return this_ptr->icon();
}

QPixmap* qt_widgets_c_QMessageBox_iconPixmap_as_ptr(const QMessageBox* this_ptr) {
  return new QPixmap(this_ptr->iconPixmap());
}

QMessageBox::StandardButton qt_widgets_c_QMessageBox_information_QWidget_QString_QString(QWidget* parent, const QString* title, const QString* text) {
  return QMessageBox::information(parent, *title, *text);
}

QMessageBox::StandardButton qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QFlags_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, unsigned int buttons) {
  return QMessageBox::information(parent, *title, *text, QFlags< QMessageBox::StandardButton >(buttons));
}

QMessageBox::StandardButton qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QFlags_QMessageBox_StandardButton_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, unsigned int buttons, QMessageBox::StandardButton defaultButton) {
  return QMessageBox::information(parent, *title, *text, QFlags< QMessageBox::StandardButton >(buttons), defaultButton);
}

QMessageBox::StandardButton qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, QMessageBox::StandardButton button0) {
  return QMessageBox::information(parent, *title, *text, button0);
}

QMessageBox::StandardButton qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QMessageBox_StandardButton_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, QMessageBox::StandardButton button0, QMessageBox::StandardButton button1) {
  return QMessageBox::information(parent, *title, *text, button0, button1);
}

int qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text) {
  return QMessageBox::information(parent, *title, *text, *button0Text);
}

int qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text) {
  return QMessageBox::information(parent, *title, *text, *button0Text, *button1Text);
}

int qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text) {
  return QMessageBox::information(parent, *title, *text, *button0Text, *button1Text, *button2Text);
}

int qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QString_QString_QString_int(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text, int defaultButtonNumber) {
  return QMessageBox::information(parent, *title, *text, *button0Text, *button1Text, *button2Text, defaultButtonNumber);
}

int qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QString_QString_QString_int_int(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text, int defaultButtonNumber, int escapeButtonNumber) {
  return QMessageBox::information(parent, *title, *text, *button0Text, *button1Text, *button2Text, defaultButtonNumber, escapeButtonNumber);
}

int qt_widgets_c_QMessageBox_information_QWidget_QString_QString_int(QWidget* parent, const QString* title, const QString* text, int button0) {
  return QMessageBox::information(parent, *title, *text, button0);
}

int qt_widgets_c_QMessageBox_information_QWidget_QString_QString_int_int(QWidget* parent, const QString* title, const QString* text, int button0, int button1) {
  return QMessageBox::information(parent, *title, *text, button0, button1);
}

int qt_widgets_c_QMessageBox_information_QWidget_QString_QString_int_int_int(QWidget* parent, const QString* title, const QString* text, int button0, int button1, int button2) {
  return QMessageBox::information(parent, *title, *text, button0, button1, button2);
}

void qt_widgets_c_QMessageBox_informativeText_to_output(const QMessageBox* this_ptr, QString* output) {
  new(output) QString(this_ptr->informativeText());
}

const QMetaObject* qt_widgets_c_QMessageBox_metaObject(const QMessageBox* this_ptr) {
  return this_ptr->metaObject();
}

QMessageBox* qt_widgets_c_QMessageBox_new_no_args() {
  return new QMessageBox();
}

QMessageBox* qt_widgets_c_QMessageBox_new_parent(QWidget* parent) {
  return new QMessageBox(parent);
}

void qt_widgets_c_QMessageBox_open(QMessageBox* this_ptr, QObject* receiver, const char* member) {
  this_ptr->open(receiver, member);
}

int qt_widgets_c_QMessageBox_qt_metacall(QMessageBox* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QMessageBox_qt_metacast(QMessageBox* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

QMessageBox::StandardButton qt_widgets_c_QMessageBox_question_QWidget_QString_QString(QWidget* parent, const QString* title, const QString* text) {
  return QMessageBox::question(parent, *title, *text);
}

QMessageBox::StandardButton qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QFlags_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, unsigned int buttons) {
  return QMessageBox::question(parent, *title, *text, QFlags< QMessageBox::StandardButton >(buttons));
}

QMessageBox::StandardButton qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QFlags_QMessageBox_StandardButton_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, unsigned int buttons, QMessageBox::StandardButton defaultButton) {
  return QMessageBox::question(parent, *title, *text, QFlags< QMessageBox::StandardButton >(buttons), defaultButton);
}

int qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QMessageBox_StandardButton_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, QMessageBox::StandardButton button0, QMessageBox::StandardButton button1) {
  return QMessageBox::question(parent, *title, *text, button0, button1);
}

int qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text) {
  return QMessageBox::question(parent, *title, *text, *button0Text);
}

int qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text) {
  return QMessageBox::question(parent, *title, *text, *button0Text, *button1Text);
}

int qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text) {
  return QMessageBox::question(parent, *title, *text, *button0Text, *button1Text, *button2Text);
}

int qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QString_QString_QString_int(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text, int defaultButtonNumber) {
  return QMessageBox::question(parent, *title, *text, *button0Text, *button1Text, *button2Text, defaultButtonNumber);
}

int qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QString_QString_QString_int_int(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text, int defaultButtonNumber, int escapeButtonNumber) {
  return QMessageBox::question(parent, *title, *text, *button0Text, *button1Text, *button2Text, defaultButtonNumber, escapeButtonNumber);
}

int qt_widgets_c_QMessageBox_question_QWidget_QString_QString_int(QWidget* parent, const QString* title, const QString* text, int button0) {
  return QMessageBox::question(parent, *title, *text, button0);
}

int qt_widgets_c_QMessageBox_question_QWidget_QString_QString_int_int(QWidget* parent, const QString* title, const QString* text, int button0, int button1) {
  return QMessageBox::question(parent, *title, *text, button0, button1);
}

int qt_widgets_c_QMessageBox_question_QWidget_QString_QString_int_int_int(QWidget* parent, const QString* title, const QString* text, int button0, int button1, int button2) {
  return QMessageBox::question(parent, *title, *text, button0, button1, button2);
}

void qt_widgets_c_QMessageBox_removeButton(QMessageBox* this_ptr, QAbstractButton* button) {
  this_ptr->removeButton(button);
}

void qt_widgets_c_QMessageBox_setButtonText(QMessageBox* this_ptr, int button, const QString* text) {
  this_ptr->setButtonText(button, *text);
}

void qt_widgets_c_QMessageBox_setCheckBox(QMessageBox* this_ptr, QCheckBox* cb) {
  this_ptr->setCheckBox(cb);
}

void qt_widgets_c_QMessageBox_setDefaultButton_QMessageBox_StandardButton(QMessageBox* this_ptr, QMessageBox::StandardButton button) {
  this_ptr->setDefaultButton(button);
}

void qt_widgets_c_QMessageBox_setDefaultButton_QPushButton(QMessageBox* this_ptr, QPushButton* button) {
  this_ptr->setDefaultButton(button);
}

void qt_widgets_c_QMessageBox_setDetailedText(QMessageBox* this_ptr, const QString* text) {
  this_ptr->setDetailedText(*text);
}

void qt_widgets_c_QMessageBox_setEscapeButton_QAbstractButton(QMessageBox* this_ptr, QAbstractButton* button) {
  this_ptr->setEscapeButton(button);
}

void qt_widgets_c_QMessageBox_setEscapeButton_QMessageBox_StandardButton(QMessageBox* this_ptr, QMessageBox::StandardButton button) {
  this_ptr->setEscapeButton(button);
}

void qt_widgets_c_QMessageBox_setIcon(QMessageBox* this_ptr, QMessageBox::Icon arg1) {
  this_ptr->setIcon(arg1);
}

void qt_widgets_c_QMessageBox_setIconPixmap(QMessageBox* this_ptr, const QPixmap* pixmap) {
  this_ptr->setIconPixmap(*pixmap);
}

void qt_widgets_c_QMessageBox_setInformativeText(QMessageBox* this_ptr, const QString* text) {
  this_ptr->setInformativeText(*text);
}

void qt_widgets_c_QMessageBox_setStandardButtons(QMessageBox* this_ptr, unsigned int buttons) {
  this_ptr->setStandardButtons(QFlags< QMessageBox::StandardButton >(buttons));
}

void qt_widgets_c_QMessageBox_setText(QMessageBox* this_ptr, const QString* text) {
  this_ptr->setText(*text);
}

void qt_widgets_c_QMessageBox_setTextFormat(QMessageBox* this_ptr, const Qt::TextFormat* format) {
  this_ptr->setTextFormat(*format);
}

void qt_widgets_c_QMessageBox_setWindowModality(QMessageBox* this_ptr, const Qt::WindowModality* windowModality) {
  this_ptr->setWindowModality(*windowModality);
}

void qt_widgets_c_QMessageBox_setWindowTitle(QMessageBox* this_ptr, const QString* title) {
  this_ptr->setWindowTitle(*title);
}

QMessageBox::StandardButton qt_widgets_c_QMessageBox_standardButton(const QMessageBox* this_ptr, QAbstractButton* button) {
  return this_ptr->standardButton(button);
}

unsigned int qt_widgets_c_QMessageBox_standardButtons(const QMessageBox* this_ptr) {
  return uint(this_ptr->standardButtons());
}

QPixmap* qt_widgets_c_QMessageBox_standardIcon_as_ptr(QMessageBox::Icon icon) {
  return new QPixmap(QMessageBox::standardIcon(icon));
}

void qt_widgets_c_QMessageBox_text_to_output(const QMessageBox* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

void qt_widgets_c_QMessageBox_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QMessageBox::trUtf8(s, c, n));
}

void qt_widgets_c_QMessageBox_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QMessageBox::tr(s, c, n));
}

QMessageBox::StandardButton qt_widgets_c_QMessageBox_warning_QWidget_QString_QString(QWidget* parent, const QString* title, const QString* text) {
  return QMessageBox::warning(parent, *title, *text);
}

QMessageBox::StandardButton qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QFlags_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, unsigned int buttons) {
  return QMessageBox::warning(parent, *title, *text, QFlags< QMessageBox::StandardButton >(buttons));
}

QMessageBox::StandardButton qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QFlags_QMessageBox_StandardButton_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, unsigned int buttons, QMessageBox::StandardButton defaultButton) {
  return QMessageBox::warning(parent, *title, *text, QFlags< QMessageBox::StandardButton >(buttons), defaultButton);
}

int qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QMessageBox_StandardButton_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, QMessageBox::StandardButton button0, QMessageBox::StandardButton button1) {
  return QMessageBox::warning(parent, *title, *text, button0, button1);
}

int qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text) {
  return QMessageBox::warning(parent, *title, *text, *button0Text);
}

int qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text) {
  return QMessageBox::warning(parent, *title, *text, *button0Text, *button1Text);
}

int qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text) {
  return QMessageBox::warning(parent, *title, *text, *button0Text, *button1Text, *button2Text);
}

int qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QString_QString_QString_int(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text, int defaultButtonNumber) {
  return QMessageBox::warning(parent, *title, *text, *button0Text, *button1Text, *button2Text, defaultButtonNumber);
}

int qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QString_QString_QString_int_int(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text, int defaultButtonNumber, int escapeButtonNumber) {
  return QMessageBox::warning(parent, *title, *text, *button0Text, *button1Text, *button2Text, defaultButtonNumber, escapeButtonNumber);
}

int qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_int_int(QWidget* parent, const QString* title, const QString* text, int button0, int button1) {
  return QMessageBox::warning(parent, *title, *text, button0, button1);
}

int qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_int_int_int(QWidget* parent, const QString* title, const QString* text, int button0, int button1, int button2) {
  return QMessageBox::warning(parent, *title, *text, button0, button1, button2);
}

