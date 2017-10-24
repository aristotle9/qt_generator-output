#ifndef QT_CORE_C_QLATIN1CHAR_H
#define QT_CORE_C_QLATIN1CHAR_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QLatin1Char_constructor(char c, QLatin1Char* output);
QT_CORE_C_EXPORT void qt_core_c_QLatin1Char_destructor(QLatin1Char* this_ptr);
QT_CORE_C_EXPORT char qt_core_c_QLatin1Char_toLatin1(const QLatin1Char* this_ptr);
QT_CORE_C_EXPORT unsigned short qt_core_c_QLatin1Char_unicode(const QLatin1Char* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QLATIN1CHAR_H
