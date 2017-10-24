#ifndef QT_CORE_C_QTEMPORARYFILE_H
#define QT_CORE_C_QTEMPORARYFILE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QTemporaryFile* qt_core_c_QTemporaryFile_G_dynamic_cast_QTemporaryFile_ptr_QFile(QFile* ptr);
QT_CORE_C_EXPORT QTemporaryFile* qt_core_c_QTemporaryFile_G_dynamic_cast_QTemporaryFile_ptr_QFileDevice(QFileDevice* ptr);
QT_CORE_C_EXPORT QTemporaryFile* qt_core_c_QTemporaryFile_G_dynamic_cast_QTemporaryFile_ptr_QIODevice(QIODevice* ptr);
QT_CORE_C_EXPORT QTemporaryFile* qt_core_c_QTemporaryFile_G_dynamic_cast_QTemporaryFile_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QFileDevice* qt_core_c_QTemporaryFile_G_static_cast_QFileDevice_ptr(QTemporaryFile* ptr);
QT_CORE_C_EXPORT QFile* qt_core_c_QTemporaryFile_G_static_cast_QFile_ptr(QTemporaryFile* ptr);
QT_CORE_C_EXPORT QIODevice* qt_core_c_QTemporaryFile_G_static_cast_QIODevice_ptr(QTemporaryFile* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QTemporaryFile_G_static_cast_QObject_ptr(QTemporaryFile* ptr);
QT_CORE_C_EXPORT QTemporaryFile* qt_core_c_QTemporaryFile_G_static_cast_QTemporaryFile_ptr_QFile(QFile* ptr);
QT_CORE_C_EXPORT QTemporaryFile* qt_core_c_QTemporaryFile_G_static_cast_QTemporaryFile_ptr_QFileDevice(QFileDevice* ptr);
QT_CORE_C_EXPORT QTemporaryFile* qt_core_c_QTemporaryFile_G_static_cast_QTemporaryFile_ptr_QIODevice(QIODevice* ptr);
QT_CORE_C_EXPORT QTemporaryFile* qt_core_c_QTemporaryFile_G_static_cast_QTemporaryFile_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT bool qt_core_c_QTemporaryFile_autoRemove(const QTemporaryFile* this_ptr);
QT_CORE_C_EXPORT QTemporaryFile* qt_core_c_QTemporaryFile_createLocalFile_file(QFile* file);
QT_CORE_C_EXPORT QTemporaryFile* qt_core_c_QTemporaryFile_createLocalFile_fileName(const QString* fileName);
QT_CORE_C_EXPORT QTemporaryFile* qt_core_c_QTemporaryFile_createNativeFile_file(QFile* file);
QT_CORE_C_EXPORT QTemporaryFile* qt_core_c_QTemporaryFile_createNativeFile_fileName(const QString* fileName);
QT_CORE_C_EXPORT void qt_core_c_QTemporaryFile_delete(QTemporaryFile* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTemporaryFile_fileName_to_output(const QTemporaryFile* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTemporaryFile_fileTemplate_to_output(const QTemporaryFile* this_ptr, QString* output);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QTemporaryFile_metaObject(const QTemporaryFile* this_ptr);
QT_CORE_C_EXPORT QTemporaryFile* qt_core_c_QTemporaryFile_new_no_args();
QT_CORE_C_EXPORT QTemporaryFile* qt_core_c_QTemporaryFile_new_parent(QObject* parent);
QT_CORE_C_EXPORT QTemporaryFile* qt_core_c_QTemporaryFile_new_templateName(const QString* templateName);
QT_CORE_C_EXPORT QTemporaryFile* qt_core_c_QTemporaryFile_new_templateName_parent(const QString* templateName, QObject* parent);
QT_CORE_C_EXPORT bool qt_core_c_QTemporaryFile_open(QTemporaryFile* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTemporaryFile_setAutoRemove(QTemporaryFile* this_ptr, bool b);
QT_CORE_C_EXPORT void qt_core_c_QTemporaryFile_setFileTemplate(QTemporaryFile* this_ptr, const QString* name);
QT_CORE_C_EXPORT void qt_core_c_QTemporaryFile_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTemporaryFile_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QTEMPORARYFILE_H
