#include "qt_core_c_QTemporaryDir.h"

bool qt_core_c_QTemporaryDir_autoRemove(const QTemporaryDir* this_ptr) {
  return this_ptr->autoRemove();
}

void qt_core_c_QTemporaryDir_constructor_no_args(QTemporaryDir* output) {
  new(output) QTemporaryDir();
}

void qt_core_c_QTemporaryDir_constructor_templateName(const QString* templateName, QTemporaryDir* output) {
  new(output) QTemporaryDir(*templateName);
}

void qt_core_c_QTemporaryDir_destructor(QTemporaryDir* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QTemporaryDir_errorString_to_output(const QTemporaryDir* this_ptr, QString* output) {
  new(output) QString(this_ptr->errorString());
}

void qt_core_c_QTemporaryDir_filePath_to_output(const QTemporaryDir* this_ptr, const QString* fileName, QString* output) {
  new(output) QString(this_ptr->filePath(*fileName));
}

bool qt_core_c_QTemporaryDir_isValid(const QTemporaryDir* this_ptr) {
  return this_ptr->isValid();
}

void qt_core_c_QTemporaryDir_path_to_output(const QTemporaryDir* this_ptr, QString* output) {
  new(output) QString(this_ptr->path());
}

bool qt_core_c_QTemporaryDir_remove(QTemporaryDir* this_ptr) {
  return this_ptr->remove();
}

void qt_core_c_QTemporaryDir_setAutoRemove(QTemporaryDir* this_ptr, bool b) {
  this_ptr->setAutoRemove(b);
}

