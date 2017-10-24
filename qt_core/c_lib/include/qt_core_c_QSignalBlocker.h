#ifndef QT_CORE_C_QSIGNALBLOCKER_H
#define QT_CORE_C_QSIGNALBLOCKER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QSignalBlocker_constructor_QObject_ptr(QObject* o, QSignalBlocker* output);
QT_CORE_C_EXPORT void qt_core_c_QSignalBlocker_constructor_QObject_ref(QObject* o, QSignalBlocker* output);
QT_CORE_C_EXPORT void qt_core_c_QSignalBlocker_destructor(QSignalBlocker* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSignalBlocker_reblock(QSignalBlocker* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSignalBlocker_unblock(QSignalBlocker* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QSIGNALBLOCKER_H
