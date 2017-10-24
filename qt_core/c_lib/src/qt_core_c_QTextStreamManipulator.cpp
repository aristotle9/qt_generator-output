#include "qt_core_c_QTextStreamManipulator.h"

void qt_core_c_QTextStreamManipulator_destructor(QTextStreamManipulator* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QTextStreamManipulator_exec(QTextStreamManipulator* this_ptr, QTextStream* s) {
  this_ptr->exec(*s);
}

