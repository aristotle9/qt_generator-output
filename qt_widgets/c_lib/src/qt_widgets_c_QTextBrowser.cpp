#include "qt_widgets_c_QTextBrowser.h"

QTextBrowser* qt_widgets_c_QTextBrowser_G_dynamic_cast_QTextBrowser_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return dynamic_cast<QTextBrowser*>(ptr);
}

QTextBrowser* qt_widgets_c_QTextBrowser_G_dynamic_cast_QTextBrowser_ptr_QFrame(QFrame* ptr) {
  return dynamic_cast<QTextBrowser*>(ptr);
}

QTextBrowser* qt_widgets_c_QTextBrowser_G_dynamic_cast_QTextBrowser_ptr_QTextEdit(QTextEdit* ptr) {
  return dynamic_cast<QTextBrowser*>(ptr);
}

QTextBrowser* qt_widgets_c_QTextBrowser_G_dynamic_cast_QTextBrowser_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QTextBrowser*>(ptr);
}

QAbstractScrollArea* qt_widgets_c_QTextBrowser_G_static_cast_QAbstractScrollArea_ptr(QTextBrowser* ptr) {
  return static_cast<QAbstractScrollArea*>(ptr);
}

QFrame* qt_widgets_c_QTextBrowser_G_static_cast_QFrame_ptr(QTextBrowser* ptr) {
  return static_cast<QFrame*>(ptr);
}

QObject* qt_widgets_c_QTextBrowser_G_static_cast_QObject_ptr(QTextBrowser* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QTextBrowser_G_static_cast_QPaintDevice_ptr(QTextBrowser* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QTextBrowser* qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return static_cast<QTextBrowser*>(ptr);
}

QTextBrowser* qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QFrame(QFrame* ptr) {
  return static_cast<QTextBrowser*>(ptr);
}

QTextBrowser* qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QObject(QObject* ptr) {
  return static_cast<QTextBrowser*>(ptr);
}

QTextBrowser* qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QTextBrowser*>(ptr);
}

QTextBrowser* qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QTextEdit(QTextEdit* ptr) {
  return static_cast<QTextBrowser*>(ptr);
}

QTextBrowser* qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QWidget(QWidget* ptr) {
  return static_cast<QTextBrowser*>(ptr);
}

QTextEdit* qt_widgets_c_QTextBrowser_G_static_cast_QTextEdit_ptr(QTextBrowser* ptr) {
  return static_cast<QTextEdit*>(ptr);
}

QWidget* qt_widgets_c_QTextBrowser_G_static_cast_QWidget_ptr(QTextBrowser* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QTextBrowser_backward(QTextBrowser* this_ptr) {
  this_ptr->backward();
}

int qt_widgets_c_QTextBrowser_backwardHistoryCount(const QTextBrowser* this_ptr) {
  return this_ptr->backwardHistoryCount();
}

void qt_widgets_c_QTextBrowser_clearHistory(QTextBrowser* this_ptr) {
  this_ptr->clearHistory();
}

void qt_widgets_c_QTextBrowser_delete(QTextBrowser* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QTextBrowser_forward(QTextBrowser* this_ptr) {
  this_ptr->forward();
}

int qt_widgets_c_QTextBrowser_forwardHistoryCount(const QTextBrowser* this_ptr) {
  return this_ptr->forwardHistoryCount();
}

void qt_widgets_c_QTextBrowser_historyTitle_to_output(const QTextBrowser* this_ptr, int arg1, QString* output) {
  new(output) QString(this_ptr->historyTitle(arg1));
}

void qt_widgets_c_QTextBrowser_historyUrl_to_output(const QTextBrowser* this_ptr, int arg1, QUrl* output) {
  new(output) QUrl(this_ptr->historyUrl(arg1));
}

void qt_widgets_c_QTextBrowser_home(QTextBrowser* this_ptr) {
  this_ptr->home();
}

bool qt_widgets_c_QTextBrowser_isBackwardAvailable(const QTextBrowser* this_ptr) {
  return this_ptr->isBackwardAvailable();
}

bool qt_widgets_c_QTextBrowser_isForwardAvailable(const QTextBrowser* this_ptr) {
  return this_ptr->isForwardAvailable();
}

void qt_widgets_c_QTextBrowser_loadResource_to_output(QTextBrowser* this_ptr, int type, const QUrl* name, QVariant* output) {
  new(output) QVariant(this_ptr->loadResource(type, *name));
}

const QMetaObject* qt_widgets_c_QTextBrowser_metaObject(const QTextBrowser* this_ptr) {
  return this_ptr->metaObject();
}

QTextBrowser* qt_widgets_c_QTextBrowser_new_no_args() {
  return new QTextBrowser();
}

QTextBrowser* qt_widgets_c_QTextBrowser_new_parent(QWidget* parent) {
  return new QTextBrowser(parent);
}

bool qt_widgets_c_QTextBrowser_openExternalLinks(const QTextBrowser* this_ptr) {
  return this_ptr->openExternalLinks();
}

bool qt_widgets_c_QTextBrowser_openLinks(const QTextBrowser* this_ptr) {
  return this_ptr->openLinks();
}

int qt_widgets_c_QTextBrowser_qt_metacall(QTextBrowser* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QTextBrowser_qt_metacast(QTextBrowser* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QTextBrowser_reload(QTextBrowser* this_ptr) {
  this_ptr->reload();
}

void qt_widgets_c_QTextBrowser_searchPaths_to_output(const QTextBrowser* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->searchPaths());
}

void qt_widgets_c_QTextBrowser_setOpenExternalLinks(QTextBrowser* this_ptr, bool open) {
  this_ptr->setOpenExternalLinks(open);
}

void qt_widgets_c_QTextBrowser_setOpenLinks(QTextBrowser* this_ptr, bool open) {
  this_ptr->setOpenLinks(open);
}

void qt_widgets_c_QTextBrowser_setSearchPaths(QTextBrowser* this_ptr, const QStringList* paths) {
  this_ptr->setSearchPaths(*paths);
}

void qt_widgets_c_QTextBrowser_setSource(QTextBrowser* this_ptr, const QUrl* name) {
  this_ptr->setSource(*name);
}

void qt_widgets_c_QTextBrowser_source_to_output(const QTextBrowser* this_ptr, QUrl* output) {
  new(output) QUrl(this_ptr->source());
}

void qt_widgets_c_QTextBrowser_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTextBrowser::trUtf8(s, c, n));
}

void qt_widgets_c_QTextBrowser_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTextBrowser::tr(s, c, n));
}

