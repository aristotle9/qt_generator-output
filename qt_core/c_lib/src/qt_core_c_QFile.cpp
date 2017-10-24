#include "qt_core_c_QFile.h"

QFile* qt_core_c_QFile_G_dynamic_cast_QFile_ptr_QFileDevice(QFileDevice* ptr) {
  return dynamic_cast<QFile*>(ptr);
}

QFile* qt_core_c_QFile_G_dynamic_cast_QFile_ptr_QIODevice(QIODevice* ptr) {
  return dynamic_cast<QFile*>(ptr);
}

QFile* qt_core_c_QFile_G_dynamic_cast_QFile_ptr_QObject(QObject* ptr) {
  return dynamic_cast<QFile*>(ptr);
}

QFileDevice* qt_core_c_QFile_G_static_cast_QFileDevice_ptr(QFile* ptr) {
  return static_cast<QFileDevice*>(ptr);
}

QFile* qt_core_c_QFile_G_static_cast_QFile_ptr_QFileDevice(QFileDevice* ptr) {
  return static_cast<QFile*>(ptr);
}

QFile* qt_core_c_QFile_G_static_cast_QFile_ptr_QIODevice(QIODevice* ptr) {
  return static_cast<QFile*>(ptr);
}

QFile* qt_core_c_QFile_G_static_cast_QFile_ptr_QObject(QObject* ptr) {
  return static_cast<QFile*>(ptr);
}

QIODevice* qt_core_c_QFile_G_static_cast_QIODevice_ptr(QFile* ptr) {
  return static_cast<QIODevice*>(ptr);
}

QObject* qt_core_c_QFile_G_static_cast_QObject_ptr(QFile* ptr) {
  return static_cast<QObject*>(ptr);
}

bool qt_core_c_QFile_copy_fileName_newName(const QString* fileName, const QString* newName) {
  return QFile::copy(*fileName, *newName);
}

bool qt_core_c_QFile_copy_newName(QFile* this_ptr, const QString* newName) {
  return this_ptr->copy(*newName);
}

void qt_core_c_QFile_decodeName_to_output_QByteArray(const QByteArray* localFileName, QString* output) {
  new(output) QString(QFile::decodeName(*localFileName));
}

void qt_core_c_QFile_decodeName_to_output_char(const char* localFileName, QString* output) {
  new(output) QString(QFile::decodeName(localFileName));
}

void qt_core_c_QFile_delete(QFile* this_ptr) {
  delete this_ptr;
}

void qt_core_c_QFile_encodeName_to_output(const QString* fileName, QByteArray* output) {
  new(output) QByteArray(QFile::encodeName(*fileName));
}

bool qt_core_c_QFile_exists_fileName(const QString* fileName) {
  return QFile::exists(*fileName);
}

bool qt_core_c_QFile_exists_no_args(const QFile* this_ptr) {
  return this_ptr->exists();
}

void qt_core_c_QFile_fileName_to_output(const QFile* this_ptr, QString* output) {
  new(output) QString(this_ptr->fileName());
}

bool qt_core_c_QFile_link_newName(QFile* this_ptr, const QString* newName) {
  return this_ptr->link(*newName);
}

bool qt_core_c_QFile_link_oldname_newName(const QString* oldname, const QString* newName) {
  return QFile::link(*oldname, *newName);
}

const QMetaObject* qt_core_c_QFile_metaObject(const QFile* this_ptr) {
  return this_ptr->metaObject();
}

QFile* qt_core_c_QFile_new_name(const QString* name) {
  return new QFile(*name);
}

QFile* qt_core_c_QFile_new_name_parent(const QString* name, QObject* parent) {
  return new QFile(*name, parent);
}

QFile* qt_core_c_QFile_new_no_args() {
  return new QFile();
}

QFile* qt_core_c_QFile_new_parent(QObject* parent) {
  return new QFile(parent);
}

bool qt_core_c_QFile_open_fd_ioFlags(QFile* this_ptr, int fd, unsigned int ioFlags) {
  return this_ptr->open(fd, QFlags< QIODevice::OpenModeFlag >(ioFlags));
}

bool qt_core_c_QFile_open_fd_ioFlags_handleFlags(QFile* this_ptr, int fd, unsigned int ioFlags, unsigned int handleFlags) {
  return this_ptr->open(fd, QFlags< QIODevice::OpenModeFlag >(ioFlags), QFlags< QFileDevice::FileHandleFlag >(handleFlags));
}

bool qt_core_c_QFile_open_flags(QFile* this_ptr, unsigned int flags) {
  return this_ptr->open(QFlags< QIODevice::OpenModeFlag >(flags));
}

unsigned int qt_core_c_QFile_permissions_filename(const QString* filename) {
  return uint(QFile::permissions(*filename));
}

unsigned int qt_core_c_QFile_permissions_no_args(const QFile* this_ptr) {
  return uint(this_ptr->permissions());
}

void qt_core_c_QFile_readLink_to_output_fileName(const QString* fileName, QString* output) {
  new(output) QString(QFile::readLink(*fileName));
}

void qt_core_c_QFile_readLink_to_output_no_args(const QFile* this_ptr, QString* output) {
  new(output) QString(this_ptr->readLink());
}

bool qt_core_c_QFile_remove_fileName(const QString* fileName) {
  return QFile::remove(*fileName);
}

bool qt_core_c_QFile_remove_no_args(QFile* this_ptr) {
  return this_ptr->remove();
}

bool qt_core_c_QFile_rename_newName(QFile* this_ptr, const QString* newName) {
  return this_ptr->rename(*newName);
}

bool qt_core_c_QFile_rename_oldName_newName(const QString* oldName, const QString* newName) {
  return QFile::rename(*oldName, *newName);
}

bool qt_core_c_QFile_resize_filename_sz(const QString* filename, qint64 sz) {
  return QFile::resize(*filename, sz);
}

bool qt_core_c_QFile_resize_sz(QFile* this_ptr, qint64 sz) {
  return this_ptr->resize(sz);
}

void qt_core_c_QFile_setFileName(QFile* this_ptr, const QString* name) {
  this_ptr->setFileName(*name);
}

bool qt_core_c_QFile_setPermissions_filename_permissionSpec(const QString* filename, unsigned int permissionSpec) {
  return QFile::setPermissions(*filename, QFlags< QFileDevice::Permission >(permissionSpec));
}

bool qt_core_c_QFile_setPermissions_permissionSpec(QFile* this_ptr, unsigned int permissionSpec) {
  return this_ptr->setPermissions(QFlags< QFileDevice::Permission >(permissionSpec));
}

qint64 qt_core_c_QFile_size(const QFile* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QFile_symLinkTarget_to_output_fileName(const QString* fileName, QString* output) {
  new(output) QString(QFile::symLinkTarget(*fileName));
}

void qt_core_c_QFile_symLinkTarget_to_output_no_args(const QFile* this_ptr, QString* output) {
  new(output) QString(this_ptr->symLinkTarget());
}

void qt_core_c_QFile_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFile::trUtf8(s, c, n));
}

void qt_core_c_QFile_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFile::tr(s, c, n));
}

