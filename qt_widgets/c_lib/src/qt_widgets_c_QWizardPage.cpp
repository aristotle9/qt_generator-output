#include "qt_widgets_c_QWizardPage.h"

QWizardPage* qt_widgets_c_QWizardPage_G_dynamic_cast_QWizardPage_ptr(QWidget* ptr) {
  return dynamic_cast<QWizardPage*>(ptr);
}

QObject* qt_widgets_c_QWizardPage_G_static_cast_QObject_ptr(QWizardPage* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QWizardPage_G_static_cast_QPaintDevice_ptr(QWizardPage* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QWizardPage_G_static_cast_QWidget_ptr(QWizardPage* ptr) {
  return static_cast<QWidget*>(ptr);
}

QWizardPage* qt_widgets_c_QWizardPage_G_static_cast_QWizardPage_ptr_QObject(QObject* ptr) {
  return static_cast<QWizardPage*>(ptr);
}

QWizardPage* qt_widgets_c_QWizardPage_G_static_cast_QWizardPage_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QWizardPage*>(ptr);
}

QWizardPage* qt_widgets_c_QWizardPage_G_static_cast_QWizardPage_ptr_QWidget(QWidget* ptr) {
  return static_cast<QWizardPage*>(ptr);
}

void qt_widgets_c_QWizardPage_buttonText_to_output(const QWizardPage* this_ptr, const QWizard::WizardButton* which, QString* output) {
  new(output) QString(this_ptr->buttonText(*which));
}

void qt_widgets_c_QWizardPage_cleanupPage(QWizardPage* this_ptr) {
  this_ptr->cleanupPage();
}

void qt_widgets_c_QWizardPage_delete(QWizardPage* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QWizardPage_initializePage(QWizardPage* this_ptr) {
  this_ptr->initializePage();
}

bool qt_widgets_c_QWizardPage_isCommitPage(const QWizardPage* this_ptr) {
  return this_ptr->isCommitPage();
}

bool qt_widgets_c_QWizardPage_isComplete(const QWizardPage* this_ptr) {
  return this_ptr->isComplete();
}

bool qt_widgets_c_QWizardPage_isFinalPage(const QWizardPage* this_ptr) {
  return this_ptr->isFinalPage();
}

const QMetaObject* qt_widgets_c_QWizardPage_metaObject(const QWizardPage* this_ptr) {
  return this_ptr->metaObject();
}

QWizardPage* qt_widgets_c_QWizardPage_new_no_args() {
  return new QWizardPage();
}

QWizardPage* qt_widgets_c_QWizardPage_new_parent(QWidget* parent) {
  return new QWizardPage(parent);
}

int qt_widgets_c_QWizardPage_nextId(const QWizardPage* this_ptr) {
  return this_ptr->nextId();
}

QPixmap* qt_widgets_c_QWizardPage_pixmap_as_ptr(const QWizardPage* this_ptr, const QWizard::WizardPixmap* which) {
  return new QPixmap(this_ptr->pixmap(*which));
}

int qt_widgets_c_QWizardPage_qt_metacall(QWizardPage* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QWizardPage_qt_metacast(QWizardPage* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QWizardPage_setButtonText(QWizardPage* this_ptr, const QWizard::WizardButton* which, const QString* text) {
  this_ptr->setButtonText(*which, *text);
}

void qt_widgets_c_QWizardPage_setCommitPage(QWizardPage* this_ptr, bool commitPage) {
  this_ptr->setCommitPage(commitPage);
}

void qt_widgets_c_QWizardPage_setFinalPage(QWizardPage* this_ptr, bool finalPage) {
  this_ptr->setFinalPage(finalPage);
}

void qt_widgets_c_QWizardPage_setPixmap(QWizardPage* this_ptr, const QWizard::WizardPixmap* which, const QPixmap* pixmap) {
  this_ptr->setPixmap(*which, *pixmap);
}

void qt_widgets_c_QWizardPage_setSubTitle(QWizardPage* this_ptr, const QString* subTitle) {
  this_ptr->setSubTitle(*subTitle);
}

void qt_widgets_c_QWizardPage_setTitle(QWizardPage* this_ptr, const QString* title) {
  this_ptr->setTitle(*title);
}

void qt_widgets_c_QWizardPage_subTitle_to_output(const QWizardPage* this_ptr, QString* output) {
  new(output) QString(this_ptr->subTitle());
}

void qt_widgets_c_QWizardPage_title_to_output(const QWizardPage* this_ptr, QString* output) {
  new(output) QString(this_ptr->title());
}

void qt_widgets_c_QWizardPage_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QWizardPage::trUtf8(s, c, n));
}

void qt_widgets_c_QWizardPage_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QWizardPage::tr(s, c, n));
}

bool qt_widgets_c_QWizardPage_validatePage(QWizardPage* this_ptr) {
  return this_ptr->validatePage();
}

