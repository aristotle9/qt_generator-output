#ifndef QT_CORE_C_QSAVEFILE_H
#define QT_CORE_C_QSAVEFILE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QSaveFile* qt_core_c_QSaveFile_G_dynamic_cast_QSaveFile_ptr_QFileDevice(QFileDevice* ptr);
QT_CORE_C_EXPORT QSaveFile* qt_core_c_QSaveFile_G_dynamic_cast_QSaveFile_ptr_QIODevice(QIODevice* ptr);
QT_CORE_C_EXPORT QSaveFile* qt_core_c_QSaveFile_G_dynamic_cast_QSaveFile_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QFileDevice* qt_core_c_QSaveFile_G_static_cast_QFileDevice_ptr(QSaveFile* ptr);
QT_CORE_C_EXPORT QIODevice* qt_core_c_QSaveFile_G_static_cast_QIODevice_ptr(QSaveFile* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QSaveFile_G_static_cast_QObject_ptr(QSaveFile* ptr);
QT_CORE_C_EXPORT QSaveFile* qt_core_c_QSaveFile_G_static_cast_QSaveFile_ptr_QFileDevice(QFileDevice* ptr);
QT_CORE_C_EXPORT QSaveFile* qt_core_c_QSaveFile_G_static_cast_QSaveFile_ptr_QIODevice(QIODevice* ptr);
QT_CORE_C_EXPORT QSaveFile* qt_core_c_QSaveFile_G_static_cast_QSaveFile_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT void qt_core_c_QSaveFile_cancelWriting(QSaveFile* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QSaveFile_commit(QSaveFile* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSaveFile_delete(QSaveFile* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QSaveFile_directWriteFallback(const QSaveFile* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSaveFile_fileName_to_output(const QSaveFile* this_ptr, QString* output);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QSaveFile_metaObject(const QSaveFile* this_ptr);
QT_CORE_C_EXPORT QSaveFile* qt_core_c_QSaveFile_new_name(const QString* name);
QT_CORE_C_EXPORT QSaveFile* qt_core_c_QSaveFile_new_name_parent(const QString* name, QObject* parent);
QT_CORE_C_EXPORT QSaveFile* qt_core_c_QSaveFile_new_no_args();
QT_CORE_C_EXPORT QSaveFile* qt_core_c_QSaveFile_new_parent(QObject* parent);
QT_CORE_C_EXPORT bool qt_core_c_QSaveFile_open(QSaveFile* this_ptr, unsigned int flags);
QT_CORE_C_EXPORT void qt_core_c_QSaveFile_setDirectWriteFallback(QSaveFile* this_ptr, bool enabled);
QT_CORE_C_EXPORT void qt_core_c_QSaveFile_setFileName(QSaveFile* this_ptr, const QString* name);
QT_CORE_C_EXPORT void qt_core_c_QSaveFile_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QSaveFile_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QSAVEFILE_H
