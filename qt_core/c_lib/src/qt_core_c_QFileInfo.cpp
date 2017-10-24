#include "qt_core_c_QFileInfo.h"

void qt_core_c_QFileInfo_G_swap(QFileInfo* value1, QFileInfo* value2) {
  swap(*value1, *value2);
}

void qt_core_c_QFileInfo_absoluteDir_to_output(const QFileInfo* this_ptr, QDir* output) {
  new(output) QDir(this_ptr->absoluteDir());
}

void qt_core_c_QFileInfo_absoluteFilePath_to_output(const QFileInfo* this_ptr, QString* output) {
  new(output) QString(this_ptr->absoluteFilePath());
}

void qt_core_c_QFileInfo_absolutePath_to_output(const QFileInfo* this_ptr, QString* output) {
  new(output) QString(this_ptr->absolutePath());
}

void qt_core_c_QFileInfo_baseName_to_output(const QFileInfo* this_ptr, QString* output) {
  new(output) QString(this_ptr->baseName());
}

void qt_core_c_QFileInfo_bundleName_to_output(const QFileInfo* this_ptr, QString* output) {
  new(output) QString(this_ptr->bundleName());
}

bool qt_core_c_QFileInfo_caching(const QFileInfo* this_ptr) {
  return this_ptr->caching();
}

void qt_core_c_QFileInfo_canonicalFilePath_to_output(const QFileInfo* this_ptr, QString* output) {
  new(output) QString(this_ptr->canonicalFilePath());
}

void qt_core_c_QFileInfo_canonicalPath_to_output(const QFileInfo* this_ptr, QString* output) {
  new(output) QString(this_ptr->canonicalPath());
}

void qt_core_c_QFileInfo_completeBaseName_to_output(const QFileInfo* this_ptr, QString* output) {
  new(output) QString(this_ptr->completeBaseName());
}

void qt_core_c_QFileInfo_completeSuffix_to_output(const QFileInfo* this_ptr, QString* output) {
  new(output) QString(this_ptr->completeSuffix());
}

void qt_core_c_QFileInfo_constructor_QDir_QString(const QDir* dir, const QString* file, QFileInfo* output) {
  new(output) QFileInfo(*dir, *file);
}

void qt_core_c_QFileInfo_constructor_QFile(const QFile* file, QFileInfo* output) {
  new(output) QFileInfo(*file);
}

void qt_core_c_QFileInfo_constructor_QFileInfo(const QFileInfo* fileinfo, QFileInfo* output) {
  new(output) QFileInfo(*fileinfo);
}

void qt_core_c_QFileInfo_constructor_QString(const QString* file, QFileInfo* output) {
  new(output) QFileInfo(*file);
}

void qt_core_c_QFileInfo_constructor_no_args(QFileInfo* output) {
  new(output) QFileInfo();
}

void qt_core_c_QFileInfo_created_to_output(const QFileInfo* this_ptr, QDateTime* output) {
  new(output) QDateTime(this_ptr->created());
}

void qt_core_c_QFileInfo_destructor(QFileInfo* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QFileInfo_dir_to_output(const QFileInfo* this_ptr, QDir* output) {
  new(output) QDir(this_ptr->dir());
}

bool qt_core_c_QFileInfo_exists_file(const QString* file) {
  return QFileInfo::exists(*file);
}

bool qt_core_c_QFileInfo_exists_no_args(const QFileInfo* this_ptr) {
  return this_ptr->exists();
}

void qt_core_c_QFileInfo_fileName_to_output(const QFileInfo* this_ptr, QString* output) {
  new(output) QString(this_ptr->fileName());
}

void qt_core_c_QFileInfo_filePath_to_output(const QFileInfo* this_ptr, QString* output) {
  new(output) QString(this_ptr->filePath());
}

unsigned int qt_core_c_QFileInfo_groupId(const QFileInfo* this_ptr) {
  return this_ptr->groupId();
}

void qt_core_c_QFileInfo_group_to_output(const QFileInfo* this_ptr, QString* output) {
  new(output) QString(this_ptr->group());
}

bool qt_core_c_QFileInfo_isAbsolute(const QFileInfo* this_ptr) {
  return this_ptr->isAbsolute();
}

bool qt_core_c_QFileInfo_isBundle(const QFileInfo* this_ptr) {
  return this_ptr->isBundle();
}

bool qt_core_c_QFileInfo_isDir(const QFileInfo* this_ptr) {
  return this_ptr->isDir();
}

bool qt_core_c_QFileInfo_isExecutable(const QFileInfo* this_ptr) {
  return this_ptr->isExecutable();
}

bool qt_core_c_QFileInfo_isFile(const QFileInfo* this_ptr) {
  return this_ptr->isFile();
}

bool qt_core_c_QFileInfo_isHidden(const QFileInfo* this_ptr) {
  return this_ptr->isHidden();
}

bool qt_core_c_QFileInfo_isNativePath(const QFileInfo* this_ptr) {
  return this_ptr->isNativePath();
}

bool qt_core_c_QFileInfo_isReadable(const QFileInfo* this_ptr) {
  return this_ptr->isReadable();
}

bool qt_core_c_QFileInfo_isRelative(const QFileInfo* this_ptr) {
  return this_ptr->isRelative();
}

bool qt_core_c_QFileInfo_isRoot(const QFileInfo* this_ptr) {
  return this_ptr->isRoot();
}

bool qt_core_c_QFileInfo_isSymLink(const QFileInfo* this_ptr) {
  return this_ptr->isSymLink();
}

bool qt_core_c_QFileInfo_isWritable(const QFileInfo* this_ptr) {
  return this_ptr->isWritable();
}

void qt_core_c_QFileInfo_lastModified_to_output(const QFileInfo* this_ptr, QDateTime* output) {
  new(output) QDateTime(this_ptr->lastModified());
}

void qt_core_c_QFileInfo_lastRead_to_output(const QFileInfo* this_ptr, QDateTime* output) {
  new(output) QDateTime(this_ptr->lastRead());
}

bool qt_core_c_QFileInfo_makeAbsolute(QFileInfo* this_ptr) {
  return this_ptr->makeAbsolute();
}

QFileInfo* qt_core_c_QFileInfo_operator_assign(QFileInfo* this_ptr, const QFileInfo* fileinfo) {
  return &this_ptr->operator=(*fileinfo);
}

bool qt_core_c_QFileInfo_operator_eq(const QFileInfo* this_ptr, const QFileInfo* fileinfo) {
  return this_ptr->operator==(*fileinfo);
}

bool qt_core_c_QFileInfo_operator_neq(const QFileInfo* this_ptr, const QFileInfo* fileinfo) {
  return this_ptr->operator!=(*fileinfo);
}

unsigned int qt_core_c_QFileInfo_ownerId(const QFileInfo* this_ptr) {
  return this_ptr->ownerId();
}

void qt_core_c_QFileInfo_owner_to_output(const QFileInfo* this_ptr, QString* output) {
  new(output) QString(this_ptr->owner());
}

void qt_core_c_QFileInfo_path_to_output(const QFileInfo* this_ptr, QString* output) {
  new(output) QString(this_ptr->path());
}

void qt_core_c_QFileInfo_readLink_to_output(const QFileInfo* this_ptr, QString* output) {
  new(output) QString(this_ptr->readLink());
}

void qt_core_c_QFileInfo_refresh(QFileInfo* this_ptr) {
  this_ptr->refresh();
}

void qt_core_c_QFileInfo_setCaching(QFileInfo* this_ptr, bool on) {
  this_ptr->setCaching(on);
}

void qt_core_c_QFileInfo_setFile_QDir_QString(QFileInfo* this_ptr, const QDir* dir, const QString* file) {
  this_ptr->setFile(*dir, *file);
}

void qt_core_c_QFileInfo_setFile_QFile(QFileInfo* this_ptr, const QFile* file) {
  this_ptr->setFile(*file);
}

void qt_core_c_QFileInfo_setFile_QString(QFileInfo* this_ptr, const QString* file) {
  this_ptr->setFile(*file);
}

qint64 qt_core_c_QFileInfo_size(const QFileInfo* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QFileInfo_suffix_to_output(const QFileInfo* this_ptr, QString* output) {
  new(output) QString(this_ptr->suffix());
}

void qt_core_c_QFileInfo_swap(QFileInfo* this_ptr, QFileInfo* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QFileInfo_symLinkTarget_to_output(const QFileInfo* this_ptr, QString* output) {
  new(output) QString(this_ptr->symLinkTarget());
}

