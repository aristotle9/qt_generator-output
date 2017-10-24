#ifndef QT_CORE_C_QLIBRARY_H
#define QT_CORE_C_QLIBRARY_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QLibrary* qt_core_c_QLibrary_G_dynamic_cast_QLibrary_ptr(QObject* ptr);
QT_CORE_C_EXPORT QLibrary* qt_core_c_QLibrary_G_static_cast_QLibrary_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QLibrary_G_static_cast_QObject_ptr(QLibrary* ptr);
QT_CORE_C_EXPORT void qt_core_c_QLibrary_delete(QLibrary* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QLibrary_errorString_to_output(const QLibrary* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QLibrary_fileName_to_output(const QLibrary* this_ptr, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QLibrary_isLibrary(const QString* fileName);
QT_CORE_C_EXPORT bool qt_core_c_QLibrary_isLoaded(const QLibrary* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QLibrary_load(QLibrary* this_ptr);
QT_CORE_C_EXPORT unsigned int qt_core_c_QLibrary_loadHints(const QLibrary* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QLibrary_metaObject(const QLibrary* this_ptr);
QT_CORE_C_EXPORT QLibrary* qt_core_c_QLibrary_new_fileName(const QString* fileName);
QT_CORE_C_EXPORT QLibrary* qt_core_c_QLibrary_new_fileName_parent(const QString* fileName, QObject* parent);
QT_CORE_C_EXPORT QLibrary* qt_core_c_QLibrary_new_fileName_verNum(const QString* fileName, int verNum);
QT_CORE_C_EXPORT QLibrary* qt_core_c_QLibrary_new_fileName_verNum_parent(const QString* fileName, int verNum, QObject* parent);
QT_CORE_C_EXPORT QLibrary* qt_core_c_QLibrary_new_fileName_version(const QString* fileName, const QString* version);
QT_CORE_C_EXPORT QLibrary* qt_core_c_QLibrary_new_fileName_version_parent(const QString* fileName, const QString* version, QObject* parent);
QT_CORE_C_EXPORT QLibrary* qt_core_c_QLibrary_new_no_args();
QT_CORE_C_EXPORT QLibrary* qt_core_c_QLibrary_new_parent(QObject* parent);
QT_CORE_C_EXPORT void (*qt_core_c_QLibrary_resolve_fileName_symbol(const QString* fileName, const char* symbol))();
QT_CORE_C_EXPORT void (*qt_core_c_QLibrary_resolve_fileName_verNum_symbol(const QString* fileName, int verNum, const char* symbol))();
QT_CORE_C_EXPORT void (*qt_core_c_QLibrary_resolve_fileName_version_symbol(const QString* fileName, const QString* version, const char* symbol))();
QT_CORE_C_EXPORT void (*qt_core_c_QLibrary_resolve_symbol(QLibrary* this_ptr, const char* symbol))();
QT_CORE_C_EXPORT void qt_core_c_QLibrary_setFileName(QLibrary* this_ptr, const QString* fileName);
QT_CORE_C_EXPORT void qt_core_c_QLibrary_setFileNameAndVersion_fileName_verNum(QLibrary* this_ptr, const QString* fileName, int verNum);
QT_CORE_C_EXPORT void qt_core_c_QLibrary_setFileNameAndVersion_fileName_version(QLibrary* this_ptr, const QString* fileName, const QString* version);
QT_CORE_C_EXPORT void qt_core_c_QLibrary_setLoadHints(QLibrary* this_ptr, unsigned int hints);
QT_CORE_C_EXPORT void qt_core_c_QLibrary_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QLibrary_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QLibrary_unload(QLibrary* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QLIBRARY_H
