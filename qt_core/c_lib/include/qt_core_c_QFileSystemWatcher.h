#ifndef QT_CORE_C_QFILESYSTEMWATCHER_H
#define QT_CORE_C_QFILESYSTEMWATCHER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QFileSystemWatcher* qt_core_c_QFileSystemWatcher_G_dynamic_cast_QFileSystemWatcher_ptr(QObject* ptr);
QT_CORE_C_EXPORT QFileSystemWatcher* qt_core_c_QFileSystemWatcher_G_static_cast_QFileSystemWatcher_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QFileSystemWatcher_G_static_cast_QObject_ptr(QFileSystemWatcher* ptr);
QT_CORE_C_EXPORT bool qt_core_c_QFileSystemWatcher_addPath(QFileSystemWatcher* this_ptr, const QString* file);
QT_CORE_C_EXPORT void qt_core_c_QFileSystemWatcher_addPaths_to_output(QFileSystemWatcher* this_ptr, const QStringList* files, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QFileSystemWatcher_delete(QFileSystemWatcher* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QFileSystemWatcher_directories_to_output(const QFileSystemWatcher* this_ptr, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QFileSystemWatcher_files_to_output(const QFileSystemWatcher* this_ptr, QStringList* output);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QFileSystemWatcher_metaObject(const QFileSystemWatcher* this_ptr);
QT_CORE_C_EXPORT QFileSystemWatcher* qt_core_c_QFileSystemWatcher_new_no_args();
QT_CORE_C_EXPORT QFileSystemWatcher* qt_core_c_QFileSystemWatcher_new_parent(QObject* parent);
QT_CORE_C_EXPORT QFileSystemWatcher* qt_core_c_QFileSystemWatcher_new_paths(const QStringList* paths);
QT_CORE_C_EXPORT QFileSystemWatcher* qt_core_c_QFileSystemWatcher_new_paths_parent(const QStringList* paths, QObject* parent);
QT_CORE_C_EXPORT bool qt_core_c_QFileSystemWatcher_removePath(QFileSystemWatcher* this_ptr, const QString* file);
QT_CORE_C_EXPORT void qt_core_c_QFileSystemWatcher_removePaths_to_output(QFileSystemWatcher* this_ptr, const QStringList* files, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QFileSystemWatcher_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QFileSystemWatcher_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QFILESYSTEMWATCHER_H
