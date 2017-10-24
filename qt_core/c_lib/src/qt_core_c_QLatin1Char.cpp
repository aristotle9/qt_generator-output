#include "qt_core_c_QLatin1Char.h"

void qt_core_c_QLatin1Char_constructor(char c, QLatin1Char* output) {
  new(output) QLatin1Char(c);
}

void qt_core_c_QLatin1Char_destructor(QLatin1Char* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

char qt_core_c_QLatin1Char_toLatin1(const QLatin1Char* this_ptr) {
  return this_ptr->toLatin1();
}

unsigned short qt_core_c_QLatin1Char_unicode(const QLatin1Char* this_ptr) {
  return this_ptr->unicode();
}

