#include "qt_core_c_QDirIterator.h"

void qt_core_c_QDirIterator_constructor_dir(const QDir* dir, QDirIterator* output) {
  new(output) QDirIterator(*dir);
}

void qt_core_c_QDirIterator_constructor_dir_flags(const QDir* dir, unsigned int flags, QDirIterator* output) {
  new(output) QDirIterator(*dir, QFlags< QDirIterator::IteratorFlag >(flags));
}

void qt_core_c_QDirIterator_constructor_path(const QString* path, QDirIterator* output) {
  new(output) QDirIterator(*path);
}

void qt_core_c_QDirIterator_constructor_path_flags(const QString* path, unsigned int flags, QDirIterator* output) {
  new(output) QDirIterator(*path, QFlags< QDirIterator::IteratorFlag >(flags));
}

void qt_core_c_QDirIterator_destructor(QDirIterator* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QDirIterator_fileInfo_to_output(const QDirIterator* this_ptr, QFileInfo* output) {
  new(output) QFileInfo(this_ptr->fileInfo());
}

void qt_core_c_QDirIterator_fileName_to_output(const QDirIterator* this_ptr, QString* output) {
  new(output) QString(this_ptr->fileName());
}

void qt_core_c_QDirIterator_filePath_to_output(const QDirIterator* this_ptr, QString* output) {
  new(output) QString(this_ptr->filePath());
}

bool qt_core_c_QDirIterator_hasNext(const QDirIterator* this_ptr) {
  return this_ptr->hasNext();
}

void qt_core_c_QDirIterator_next_to_output(QDirIterator* this_ptr, QString* output) {
  new(output) QString(this_ptr->next());
}

void qt_core_c_QDirIterator_path_to_output(const QDirIterator* this_ptr, QString* output) {
  new(output) QString(this_ptr->path());
}

