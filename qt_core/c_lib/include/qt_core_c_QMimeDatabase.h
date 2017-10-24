#ifndef QT_CORE_C_QMIMEDATABASE_H
#define QT_CORE_C_QMIMEDATABASE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QMimeDatabase_allMimeTypes_to_output(const QMimeDatabase* this_ptr, QList< QMimeType >* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeDatabase_constructor(QMimeDatabase* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeDatabase_destructor(QMimeDatabase* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMimeDatabase_mimeTypeForData_to_output_data(const QMimeDatabase* this_ptr, const QByteArray* data, QMimeType* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeDatabase_mimeTypeForData_to_output_device(const QMimeDatabase* this_ptr, QIODevice* device, QMimeType* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeDatabase_mimeTypeForFileNameAndData_to_output_fileName_data(const QMimeDatabase* this_ptr, const QString* fileName, const QByteArray* data, QMimeType* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeDatabase_mimeTypeForFileNameAndData_to_output_fileName_device(const QMimeDatabase* this_ptr, const QString* fileName, QIODevice* device, QMimeType* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeDatabase_mimeTypeForFile_to_output_fileInfo(const QMimeDatabase* this_ptr, const QFileInfo* fileInfo, QMimeType* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeDatabase_mimeTypeForFile_to_output_fileInfo_mode(const QMimeDatabase* this_ptr, const QFileInfo* fileInfo, QMimeDatabase::MatchMode mode, QMimeType* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeDatabase_mimeTypeForFile_to_output_fileName(const QMimeDatabase* this_ptr, const QString* fileName, QMimeType* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeDatabase_mimeTypeForFile_to_output_fileName_mode(const QMimeDatabase* this_ptr, const QString* fileName, QMimeDatabase::MatchMode mode, QMimeType* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeDatabase_mimeTypeForName_to_output(const QMimeDatabase* this_ptr, const QString* nameOrAlias, QMimeType* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeDatabase_mimeTypeForUrl_to_output(const QMimeDatabase* this_ptr, const QUrl* url, QMimeType* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeDatabase_mimeTypesForFileName_to_output(const QMimeDatabase* this_ptr, const QString* fileName, QList< QMimeType >* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeDatabase_suffixForFileName_to_output(const QMimeDatabase* this_ptr, const QString* fileName, QString* output);

} // extern "C"

#endif // QT_CORE_C_QMIMEDATABASE_H
