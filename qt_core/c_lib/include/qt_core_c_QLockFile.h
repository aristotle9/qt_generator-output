#ifndef QT_CORE_C_QLOCKFILE_H
#define QT_CORE_C_QLOCKFILE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QLockFile_constructor(const QString* fileName, QLockFile* output);
QT_CORE_C_EXPORT void qt_core_c_QLockFile_destructor(QLockFile* this_ptr);
QT_CORE_C_EXPORT QLockFile::LockError qt_core_c_QLockFile_error(const QLockFile* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QLockFile_getLockInfo(const QLockFile* this_ptr, qint64* pid, QString* hostname, QString* appname);
QT_CORE_C_EXPORT bool qt_core_c_QLockFile_isLocked(const QLockFile* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QLockFile_lock(QLockFile* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QLockFile_removeStaleLockFile(QLockFile* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QLockFile_setStaleLockTime(QLockFile* this_ptr, int arg1);
QT_CORE_C_EXPORT int qt_core_c_QLockFile_staleLockTime(const QLockFile* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QLockFile_tryLock_no_args(QLockFile* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QLockFile_tryLock_timeout(QLockFile* this_ptr, int timeout);
QT_CORE_C_EXPORT void qt_core_c_QLockFile_unlock(QLockFile* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QLOCKFILE_H
