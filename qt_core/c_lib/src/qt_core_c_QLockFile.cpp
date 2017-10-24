#include "qt_core_c_QLockFile.h"

void qt_core_c_QLockFile_constructor(const QString* fileName, QLockFile* output) {
  new(output) QLockFile(*fileName);
}

void qt_core_c_QLockFile_destructor(QLockFile* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

QLockFile::LockError qt_core_c_QLockFile_error(const QLockFile* this_ptr) {
  return this_ptr->error();
}

bool qt_core_c_QLockFile_getLockInfo(const QLockFile* this_ptr, qint64* pid, QString* hostname, QString* appname) {
  return this_ptr->getLockInfo(pid, hostname, appname);
}

bool qt_core_c_QLockFile_isLocked(const QLockFile* this_ptr) {
  return this_ptr->isLocked();
}

bool qt_core_c_QLockFile_lock(QLockFile* this_ptr) {
  return this_ptr->lock();
}

bool qt_core_c_QLockFile_removeStaleLockFile(QLockFile* this_ptr) {
  return this_ptr->removeStaleLockFile();
}

void qt_core_c_QLockFile_setStaleLockTime(QLockFile* this_ptr, int arg1) {
  this_ptr->setStaleLockTime(arg1);
}

int qt_core_c_QLockFile_staleLockTime(const QLockFile* this_ptr) {
  return this_ptr->staleLockTime();
}

bool qt_core_c_QLockFile_tryLock_no_args(QLockFile* this_ptr) {
  return this_ptr->tryLock();
}

bool qt_core_c_QLockFile_tryLock_timeout(QLockFile* this_ptr, int timeout) {
  return this_ptr->tryLock(timeout);
}

void qt_core_c_QLockFile_unlock(QLockFile* this_ptr) {
  this_ptr->unlock();
}

