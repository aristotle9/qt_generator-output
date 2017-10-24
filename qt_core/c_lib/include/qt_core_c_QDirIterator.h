#ifndef QT_CORE_C_QDIRITERATOR_H
#define QT_CORE_C_QDIRITERATOR_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QDirIterator_constructor_dir(const QDir* dir, QDirIterator* output);
QT_CORE_C_EXPORT void qt_core_c_QDirIterator_constructor_dir_flags(const QDir* dir, unsigned int flags, QDirIterator* output);
QT_CORE_C_EXPORT void qt_core_c_QDirIterator_constructor_path(const QString* path, QDirIterator* output);
QT_CORE_C_EXPORT void qt_core_c_QDirIterator_constructor_path_flags(const QString* path, unsigned int flags, QDirIterator* output);
QT_CORE_C_EXPORT void qt_core_c_QDirIterator_destructor(QDirIterator* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDirIterator_fileInfo_to_output(const QDirIterator* this_ptr, QFileInfo* output);
QT_CORE_C_EXPORT void qt_core_c_QDirIterator_fileName_to_output(const QDirIterator* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDirIterator_filePath_to_output(const QDirIterator* this_ptr, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QDirIterator_hasNext(const QDirIterator* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDirIterator_next_to_output(QDirIterator* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDirIterator_path_to_output(const QDirIterator* this_ptr, QString* output);

} // extern "C"

#endif // QT_CORE_C_QDIRITERATOR_H
