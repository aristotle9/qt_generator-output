#include "qt_gui_c_QOpenGLVersionFunctions.h"

void qt_gui_c_QAbstractOpenGLFunctions_delete(QAbstractOpenGLFunctions* this_ptr) {
  delete this_ptr;
}

bool qt_gui_c_QAbstractOpenGLFunctions_initializeOpenGLFunctions(QAbstractOpenGLFunctions* this_ptr) {
  return this_ptr->initializeOpenGLFunctions();
}

