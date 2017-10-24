#include "qt_core_c_QFileSystemWatcher.h"

QFileSystemWatcher* qt_core_c_QFileSystemWatcher_G_dynamic_cast_QFileSystemWatcher_ptr(QObject* ptr) {
  return dynamic_cast<QFileSystemWatcher*>(ptr);
}

QFileSystemWatcher* qt_core_c_QFileSystemWatcher_G_static_cast_QFileSystemWatcher_ptr(QObject* ptr) {
  return static_cast<QFileSystemWatcher*>(ptr);
}

QObject* qt_core_c_QFileSystemWatcher_G_static_cast_QObject_ptr(QFileSystemWatcher* ptr) {
  return static_cast<QObject*>(ptr);
}

bool qt_core_c_QFileSystemWatcher_addPath(QFileSystemWatcher* this_ptr, const QString* file) {
  return this_ptr->addPath(*file);
}

void qt_core_c_QFileSystemWatcher_addPaths_to_output(QFileSystemWatcher* this_ptr, const QStringList* files, QStringList* output) {
  new(output) QStringList(this_ptr->addPaths(*files));
}

void qt_core_c_QFileSystemWatcher_delete(QFileSystemWatcher* this_ptr) {
  delete this_ptr;
}

void qt_core_c_QFileSystemWatcher_directories_to_output(const QFileSystemWatcher* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->directories());
}

void qt_core_c_QFileSystemWatcher_files_to_output(const QFileSystemWatcher* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->files());
}

const QMetaObject* qt_core_c_QFileSystemWatcher_metaObject(const QFileSystemWatcher* this_ptr) {
  return this_ptr->metaObject();
}

QFileSystemWatcher* qt_core_c_QFileSystemWatcher_new_no_args() {
  return new QFileSystemWatcher();
}

QFileSystemWatcher* qt_core_c_QFileSystemWatcher_new_parent(QObject* parent) {
  return new QFileSystemWatcher(parent);
}

QFileSystemWatcher* qt_core_c_QFileSystemWatcher_new_paths(const QStringList* paths) {
  return new QFileSystemWatcher(*paths);
}

QFileSystemWatcher* qt_core_c_QFileSystemWatcher_new_paths_parent(const QStringList* paths, QObject* parent) {
  return new QFileSystemWatcher(*paths, parent);
}

bool qt_core_c_QFileSystemWatcher_removePath(QFileSystemWatcher* this_ptr, const QString* file) {
  return this_ptr->removePath(*file);
}

void qt_core_c_QFileSystemWatcher_removePaths_to_output(QFileSystemWatcher* this_ptr, const QStringList* files, QStringList* output) {
  new(output) QStringList(this_ptr->removePaths(*files));
}

void qt_core_c_QFileSystemWatcher_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFileSystemWatcher::trUtf8(s, c, n));
}

void qt_core_c_QFileSystemWatcher_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFileSystemWatcher::tr(s, c, n));
}

