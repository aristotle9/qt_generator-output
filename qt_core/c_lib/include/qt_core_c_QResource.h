#ifndef QT_CORE_C_QRESOURCE_H
#define QT_CORE_C_QRESOURCE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QResource_absoluteFilePath_to_output(const QResource* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QResource_addSearchPath(const QString* path);
QT_CORE_C_EXPORT void qt_core_c_QResource_constructor_file(const QString* file, QResource* output);
QT_CORE_C_EXPORT void qt_core_c_QResource_constructor_file_locale(const QString* file, const QLocale* locale, QResource* output);
QT_CORE_C_EXPORT void qt_core_c_QResource_constructor_no_args(QResource* output);
QT_CORE_C_EXPORT const unsigned char* qt_core_c_QResource_data(const QResource* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QResource_destructor(QResource* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QResource_fileName_to_output(const QResource* this_ptr, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QResource_isCompressed(const QResource* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QResource_isValid(const QResource* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QResource_lastModified_to_output(const QResource* this_ptr, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QResource_locale_to_output(const QResource* this_ptr, QLocale* output);
QT_CORE_C_EXPORT bool qt_core_c_QResource_registerResource_rccData(const unsigned char* rccData);
QT_CORE_C_EXPORT bool qt_core_c_QResource_registerResource_rccData_resourceRoot(const unsigned char* rccData, const QString* resourceRoot);
QT_CORE_C_EXPORT bool qt_core_c_QResource_registerResource_rccFilename(const QString* rccFilename);
QT_CORE_C_EXPORT bool qt_core_c_QResource_registerResource_rccFilename_resourceRoot(const QString* rccFilename, const QString* resourceRoot);
QT_CORE_C_EXPORT void qt_core_c_QResource_searchPaths_to_output(QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QResource_setFileName(QResource* this_ptr, const QString* file);
QT_CORE_C_EXPORT void qt_core_c_QResource_setLocale(QResource* this_ptr, const QLocale* locale);
QT_CORE_C_EXPORT qint64 qt_core_c_QResource_size(const QResource* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QResource_unregisterResource_rccData(const unsigned char* rccData);
QT_CORE_C_EXPORT bool qt_core_c_QResource_unregisterResource_rccData_resourceRoot(const unsigned char* rccData, const QString* resourceRoot);
QT_CORE_C_EXPORT bool qt_core_c_QResource_unregisterResource_rccFilename(const QString* rccFilename);
QT_CORE_C_EXPORT bool qt_core_c_QResource_unregisterResource_rccFilename_resourceRoot(const QString* rccFilename, const QString* resourceRoot);

} // extern "C"

#endif // QT_CORE_C_QRESOURCE_H
