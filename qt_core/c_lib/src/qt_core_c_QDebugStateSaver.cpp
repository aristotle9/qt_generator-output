#include "qt_core_c_QDebugStateSaver.h"

void qt_core_c_QDebugStateSaver_constructor(QDebug* dbg, QDebugStateSaver* output) {
  new(output) QDebugStateSaver(*dbg);
}

void qt_core_c_QDebugStateSaver_destructor(QDebugStateSaver* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

