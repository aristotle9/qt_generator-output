#include "qt_core_c_QMimeDatabase.h"

void qt_core_c_QMimeDatabase_allMimeTypes_to_output(const QMimeDatabase* this_ptr, QList< QMimeType >* output) {
  new(output) QList< QMimeType >(this_ptr->allMimeTypes());
}

void qt_core_c_QMimeDatabase_constructor(QMimeDatabase* output) {
  new(output) QMimeDatabase();
}

void qt_core_c_QMimeDatabase_destructor(QMimeDatabase* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QMimeDatabase_mimeTypeForData_to_output_data(const QMimeDatabase* this_ptr, const QByteArray* data, QMimeType* output) {
  new(output) QMimeType(this_ptr->mimeTypeForData(*data));
}

void qt_core_c_QMimeDatabase_mimeTypeForData_to_output_device(const QMimeDatabase* this_ptr, QIODevice* device, QMimeType* output) {
  new(output) QMimeType(this_ptr->mimeTypeForData(device));
}

void qt_core_c_QMimeDatabase_mimeTypeForFileNameAndData_to_output_fileName_data(const QMimeDatabase* this_ptr, const QString* fileName, const QByteArray* data, QMimeType* output) {
  new(output) QMimeType(this_ptr->mimeTypeForFileNameAndData(*fileName, *data));
}

void qt_core_c_QMimeDatabase_mimeTypeForFileNameAndData_to_output_fileName_device(const QMimeDatabase* this_ptr, const QString* fileName, QIODevice* device, QMimeType* output) {
  new(output) QMimeType(this_ptr->mimeTypeForFileNameAndData(*fileName, device));
}

void qt_core_c_QMimeDatabase_mimeTypeForFile_to_output_fileInfo(const QMimeDatabase* this_ptr, const QFileInfo* fileInfo, QMimeType* output) {
  new(output) QMimeType(this_ptr->mimeTypeForFile(*fileInfo));
}

void qt_core_c_QMimeDatabase_mimeTypeForFile_to_output_fileInfo_mode(const QMimeDatabase* this_ptr, const QFileInfo* fileInfo, QMimeDatabase::MatchMode mode, QMimeType* output) {
  new(output) QMimeType(this_ptr->mimeTypeForFile(*fileInfo, mode));
}

void qt_core_c_QMimeDatabase_mimeTypeForFile_to_output_fileName(const QMimeDatabase* this_ptr, const QString* fileName, QMimeType* output) {
  new(output) QMimeType(this_ptr->mimeTypeForFile(*fileName));
}

void qt_core_c_QMimeDatabase_mimeTypeForFile_to_output_fileName_mode(const QMimeDatabase* this_ptr, const QString* fileName, QMimeDatabase::MatchMode mode, QMimeType* output) {
  new(output) QMimeType(this_ptr->mimeTypeForFile(*fileName, mode));
}

void qt_core_c_QMimeDatabase_mimeTypeForName_to_output(const QMimeDatabase* this_ptr, const QString* nameOrAlias, QMimeType* output) {
  new(output) QMimeType(this_ptr->mimeTypeForName(*nameOrAlias));
}

void qt_core_c_QMimeDatabase_mimeTypeForUrl_to_output(const QMimeDatabase* this_ptr, const QUrl* url, QMimeType* output) {
  new(output) QMimeType(this_ptr->mimeTypeForUrl(*url));
}

void qt_core_c_QMimeDatabase_mimeTypesForFileName_to_output(const QMimeDatabase* this_ptr, const QString* fileName, QList< QMimeType >* output) {
  new(output) QList< QMimeType >(this_ptr->mimeTypesForFileName(*fileName));
}

void qt_core_c_QMimeDatabase_suffixForFileName_to_output(const QMimeDatabase* this_ptr, const QString* fileName, QString* output) {
  new(output) QString(this_ptr->suffixForFileName(*fileName));
}

