#include "qt_ui_tools_c_QUiLoader.h"

QObject* qt_ui_tools_c_QUiLoader_G_static_cast_QObject_ptr(QUiLoader* ptr) {
  return static_cast<QObject*>(ptr);
}

QUiLoader* qt_ui_tools_c_QUiLoader_G_static_cast_QUiLoader_ptr(QObject* ptr) {
  return static_cast<QUiLoader*>(ptr);
}

void qt_ui_tools_c_QUiLoader_addPluginPath(QUiLoader* this_ptr, const QString* path) {
  this_ptr->addPluginPath(*path);
}

void qt_ui_tools_c_QUiLoader_availableLayouts_to_output(const QUiLoader* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->availableLayouts());
}

void qt_ui_tools_c_QUiLoader_availableWidgets_to_output(const QUiLoader* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->availableWidgets());
}

void qt_ui_tools_c_QUiLoader_clearPluginPaths(QUiLoader* this_ptr) {
  this_ptr->clearPluginPaths();
}

QActionGroup* qt_ui_tools_c_QUiLoader_createActionGroup_no_args(QUiLoader* this_ptr) {
  return this_ptr->createActionGroup();
}

QActionGroup* qt_ui_tools_c_QUiLoader_createActionGroup_parent(QUiLoader* this_ptr, QObject* parent) {
  return this_ptr->createActionGroup(parent);
}

QActionGroup* qt_ui_tools_c_QUiLoader_createActionGroup_parent_name(QUiLoader* this_ptr, QObject* parent, const QString* name) {
  return this_ptr->createActionGroup(parent, *name);
}

QAction* qt_ui_tools_c_QUiLoader_createAction_no_args(QUiLoader* this_ptr) {
  return this_ptr->createAction();
}

QAction* qt_ui_tools_c_QUiLoader_createAction_parent(QUiLoader* this_ptr, QObject* parent) {
  return this_ptr->createAction(parent);
}

QAction* qt_ui_tools_c_QUiLoader_createAction_parent_name(QUiLoader* this_ptr, QObject* parent, const QString* name) {
  return this_ptr->createAction(parent, *name);
}

QLayout* qt_ui_tools_c_QUiLoader_createLayout_className(QUiLoader* this_ptr, const QString* className) {
  return this_ptr->createLayout(*className);
}

QLayout* qt_ui_tools_c_QUiLoader_createLayout_className_parent(QUiLoader* this_ptr, const QString* className, QObject* parent) {
  return this_ptr->createLayout(*className, parent);
}

QLayout* qt_ui_tools_c_QUiLoader_createLayout_className_parent_name(QUiLoader* this_ptr, const QString* className, QObject* parent, const QString* name) {
  return this_ptr->createLayout(*className, parent, *name);
}

QWidget* qt_ui_tools_c_QUiLoader_createWidget_className(QUiLoader* this_ptr, const QString* className) {
  return this_ptr->createWidget(*className);
}

QWidget* qt_ui_tools_c_QUiLoader_createWidget_className_parent(QUiLoader* this_ptr, const QString* className, QWidget* parent) {
  return this_ptr->createWidget(*className, parent);
}

QWidget* qt_ui_tools_c_QUiLoader_createWidget_className_parent_name(QUiLoader* this_ptr, const QString* className, QWidget* parent, const QString* name) {
  return this_ptr->createWidget(*className, parent, *name);
}

void qt_ui_tools_c_QUiLoader_delete(QUiLoader* this_ptr) {
  delete this_ptr;
}

void qt_ui_tools_c_QUiLoader_errorString_to_output(const QUiLoader* this_ptr, QString* output) {
  new(output) QString(this_ptr->errorString());
}

bool qt_ui_tools_c_QUiLoader_isLanguageChangeEnabled(const QUiLoader* this_ptr) {
  return this_ptr->isLanguageChangeEnabled();
}

bool qt_ui_tools_c_QUiLoader_isTranslationEnabled(const QUiLoader* this_ptr) {
  return this_ptr->isTranslationEnabled();
}

QWidget* qt_ui_tools_c_QUiLoader_load_device(QUiLoader* this_ptr, QIODevice* device) {
  return this_ptr->load(device);
}

QWidget* qt_ui_tools_c_QUiLoader_load_device_parentWidget(QUiLoader* this_ptr, QIODevice* device, QWidget* parentWidget) {
  return this_ptr->load(device, parentWidget);
}

const QMetaObject* qt_ui_tools_c_QUiLoader_metaObject(const QUiLoader* this_ptr) {
  return this_ptr->metaObject();
}

QUiLoader* qt_ui_tools_c_QUiLoader_new_no_args() {
  return new QUiLoader();
}

QUiLoader* qt_ui_tools_c_QUiLoader_new_parent(QObject* parent) {
  return new QUiLoader(parent);
}

void qt_ui_tools_c_QUiLoader_pluginPaths_to_output(const QUiLoader* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->pluginPaths());
}

int qt_ui_tools_c_QUiLoader_qt_metacall(QUiLoader* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_ui_tools_c_QUiLoader_qt_metacast(QUiLoader* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_ui_tools_c_QUiLoader_setLanguageChangeEnabled(QUiLoader* this_ptr, bool enabled) {
  this_ptr->setLanguageChangeEnabled(enabled);
}

void qt_ui_tools_c_QUiLoader_setTranslationEnabled(QUiLoader* this_ptr, bool enabled) {
  this_ptr->setTranslationEnabled(enabled);
}

void qt_ui_tools_c_QUiLoader_setWorkingDirectory(QUiLoader* this_ptr, const QDir* dir) {
  this_ptr->setWorkingDirectory(*dir);
}

void qt_ui_tools_c_QUiLoader_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QUiLoader::trUtf8(s, c, n));
}

void qt_ui_tools_c_QUiLoader_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QUiLoader::tr(s, c, n));
}

void qt_ui_tools_c_QUiLoader_workingDirectory_to_output(const QUiLoader* this_ptr, QDir* output) {
  new(output) QDir(this_ptr->workingDirectory());
}

