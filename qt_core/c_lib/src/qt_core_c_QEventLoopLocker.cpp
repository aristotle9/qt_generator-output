#include "qt_core_c_QEventLoopLocker.h"

void qt_core_c_QEventLoopLocker_constructor_loop(QEventLoop* loop, QEventLoopLocker* output) {
  new(output) QEventLoopLocker(loop);
}

void qt_core_c_QEventLoopLocker_constructor_no_args(QEventLoopLocker* output) {
  new(output) QEventLoopLocker();
}

void qt_core_c_QEventLoopLocker_constructor_thread(QThread* thread, QEventLoopLocker* output) {
  new(output) QEventLoopLocker(thread);
}

void qt_core_c_QEventLoopLocker_destructor(QEventLoopLocker* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

