#include "qt_gui_c_QSurface.h"

void qt_gui_c_QSurface_delete(QSurface* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QSurface_format_to_output(const QSurface* this_ptr, QSurfaceFormat* output) {
  new(output) QSurfaceFormat(this_ptr->format());
}

void qt_gui_c_QSurface_size_to_output(const QSurface* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->size());
}

bool qt_gui_c_QSurface_supportsOpenGL(const QSurface* this_ptr) {
  return this_ptr->supportsOpenGL();
}

QSurface::SurfaceClass qt_gui_c_QSurface_surfaceClass(const QSurface* this_ptr) {
  return this_ptr->surfaceClass();
}

QSurface::SurfaceType qt_gui_c_QSurface_surfaceType(const QSurface* this_ptr) {
  return this_ptr->surfaceType();
}

