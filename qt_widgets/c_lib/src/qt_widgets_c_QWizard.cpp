#include "qt_widgets_c_QWizard.h"

QWizard* qt_widgets_c_QWizard_G_dynamic_cast_QWizard_ptr_QDialog(QDialog* ptr) {
  return dynamic_cast<QWizard*>(ptr);
}

QWizard* qt_widgets_c_QWizard_G_dynamic_cast_QWizard_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QWizard*>(ptr);
}

QDialog* qt_widgets_c_QWizard_G_static_cast_QDialog_ptr(QWizard* ptr) {
  return static_cast<QDialog*>(ptr);
}

QObject* qt_widgets_c_QWizard_G_static_cast_QObject_ptr(QWizard* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QWizard_G_static_cast_QPaintDevice_ptr(QWizard* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QWizard_G_static_cast_QWidget_ptr(QWizard* ptr) {
  return static_cast<QWidget*>(ptr);
}

QWizard* qt_widgets_c_QWizard_G_static_cast_QWizard_ptr_QDialog(QDialog* ptr) {
  return static_cast<QWizard*>(ptr);
}

QWizard* qt_widgets_c_QWizard_G_static_cast_QWizard_ptr_QObject(QObject* ptr) {
  return static_cast<QWizard*>(ptr);
}

QWizard* qt_widgets_c_QWizard_G_static_cast_QWizard_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QWizard*>(ptr);
}

QWizard* qt_widgets_c_QWizard_G_static_cast_QWizard_ptr_QWidget(QWidget* ptr) {
  return static_cast<QWizard*>(ptr);
}

int qt_widgets_c_QWizard_addPage(QWizard* this_ptr, QWizardPage* page) {
  return this_ptr->addPage(page);
}

void qt_widgets_c_QWizard_back(QWizard* this_ptr) {
  this_ptr->back();
}

QAbstractButton* qt_widgets_c_QWizard_button(const QWizard* this_ptr, QWizard::WizardButton which) {
  return this_ptr->button(which);
}

void qt_widgets_c_QWizard_buttonText_to_output(const QWizard* this_ptr, QWizard::WizardButton which, QString* output) {
  new(output) QString(this_ptr->buttonText(which));
}

int qt_widgets_c_QWizard_currentId(const QWizard* this_ptr) {
  return this_ptr->currentId();
}

QWizardPage* qt_widgets_c_QWizard_currentPage(const QWizard* this_ptr) {
  return this_ptr->currentPage();
}

void qt_widgets_c_QWizard_delete(QWizard* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QWizard_field_to_output(const QWizard* this_ptr, const QString* name, QVariant* output) {
  new(output) QVariant(this_ptr->field(*name));
}

bool qt_widgets_c_QWizard_hasVisitedPage(const QWizard* this_ptr, int id) {
  return this_ptr->hasVisitedPage(id);
}

const QMetaObject* qt_widgets_c_QWizard_metaObject(const QWizard* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QWizard_next(QWizard* this_ptr) {
  this_ptr->next();
}

int qt_widgets_c_QWizard_nextId(const QWizard* this_ptr) {
  return this_ptr->nextId();
}

unsigned int qt_widgets_c_QWizard_options(const QWizard* this_ptr) {
  return uint(this_ptr->options());
}

QWizardPage* qt_widgets_c_QWizard_page(const QWizard* this_ptr, int id) {
  return this_ptr->page(id);
}

void qt_widgets_c_QWizard_pageIds_to_output(const QWizard* this_ptr, QList< int >* output) {
  new(output) QList< int >(this_ptr->pageIds());
}

QPixmap* qt_widgets_c_QWizard_pixmap_as_ptr(const QWizard* this_ptr, QWizard::WizardPixmap which) {
  return new QPixmap(this_ptr->pixmap(which));
}

int qt_widgets_c_QWizard_qt_metacall(QWizard* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QWizard_qt_metacast(QWizard* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QWizard_removePage(QWizard* this_ptr, int id) {
  this_ptr->removePage(id);
}

void qt_widgets_c_QWizard_restart(QWizard* this_ptr) {
  this_ptr->restart();
}

void qt_widgets_c_QWizard_setButton(QWizard* this_ptr, QWizard::WizardButton which, QAbstractButton* button) {
  this_ptr->setButton(which, button);
}

void qt_widgets_c_QWizard_setButtonLayout(QWizard* this_ptr, const QList< QWizard::WizardButton >* layout) {
  this_ptr->setButtonLayout(*layout);
}

void qt_widgets_c_QWizard_setButtonText(QWizard* this_ptr, QWizard::WizardButton which, const QString* text) {
  this_ptr->setButtonText(which, *text);
}

void qt_widgets_c_QWizard_setDefaultProperty(QWizard* this_ptr, const char* className, const char* property, const char* changedSignal) {
  this_ptr->setDefaultProperty(className, property, changedSignal);
}

void qt_widgets_c_QWizard_setField(QWizard* this_ptr, const QString* name, const QVariant* value) {
  this_ptr->setField(*name, *value);
}

void qt_widgets_c_QWizard_setOption_option(QWizard* this_ptr, QWizard::WizardOption option) {
  this_ptr->setOption(option);
}

void qt_widgets_c_QWizard_setOption_option_on(QWizard* this_ptr, QWizard::WizardOption option, bool on) {
  this_ptr->setOption(option, on);
}

void qt_widgets_c_QWizard_setOptions(QWizard* this_ptr, unsigned int options) {
  this_ptr->setOptions(QFlags< QWizard::WizardOption >(options));
}

void qt_widgets_c_QWizard_setPage(QWizard* this_ptr, int id, QWizardPage* page) {
  this_ptr->setPage(id, page);
}

void qt_widgets_c_QWizard_setPixmap(QWizard* this_ptr, QWizard::WizardPixmap which, const QPixmap* pixmap) {
  this_ptr->setPixmap(which, *pixmap);
}

void qt_widgets_c_QWizard_setSideWidget(QWizard* this_ptr, QWidget* widget) {
  this_ptr->setSideWidget(widget);
}

void qt_widgets_c_QWizard_setStartId(QWizard* this_ptr, int id) {
  this_ptr->setStartId(id);
}

void qt_widgets_c_QWizard_setSubTitleFormat(QWizard* this_ptr, const Qt::TextFormat* format) {
  this_ptr->setSubTitleFormat(*format);
}

void qt_widgets_c_QWizard_setTitleFormat(QWizard* this_ptr, const Qt::TextFormat* format) {
  this_ptr->setTitleFormat(*format);
}

void qt_widgets_c_QWizard_setVisible(QWizard* this_ptr, bool visible) {
  this_ptr->setVisible(visible);
}

void qt_widgets_c_QWizard_setWizardStyle(QWizard* this_ptr, QWizard::WizardStyle style) {
  this_ptr->setWizardStyle(style);
}

QWidget* qt_widgets_c_QWizard_sideWidget(const QWizard* this_ptr) {
  return this_ptr->sideWidget();
}

void qt_widgets_c_QWizard_sizeHint_to_output(const QWizard* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

int qt_widgets_c_QWizard_startId(const QWizard* this_ptr) {
  return this_ptr->startId();
}

bool qt_widgets_c_QWizard_testOption(const QWizard* this_ptr, QWizard::WizardOption option) {
  return this_ptr->testOption(option);
}

void qt_widgets_c_QWizard_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QWizard::trUtf8(s, c, n));
}

void qt_widgets_c_QWizard_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QWizard::tr(s, c, n));
}

bool qt_widgets_c_QWizard_validateCurrentPage(QWizard* this_ptr) {
  return this_ptr->validateCurrentPage();
}

void qt_widgets_c_QWizard_visitedPages_to_output(const QWizard* this_ptr, QList< int >* output) {
  new(output) QList< int >(this_ptr->visitedPages());
}

QWizard::WizardStyle qt_widgets_c_QWizard_wizardStyle(const QWizard* this_ptr) {
  return this_ptr->wizardStyle();
}

