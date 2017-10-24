#include "qt_widgets_c_QColormap.h"

void qt_widgets_c_QColormap_cleanup() {
  QColormap::cleanup();
}

void qt_widgets_c_QColormap_colorAt_to_output(const QColormap* this_ptr, unsigned int pixel, QColor* output) {
  new(output) QColor(this_ptr->colorAt(pixel));
}

void qt_widgets_c_QColormap_colormap_to_output(const QColormap* this_ptr, QVector< QColor >* output) {
  new(output) QVector< QColor >(this_ptr->colormap());
}

void qt_widgets_c_QColormap_constructor(const QColormap* colormap, QColormap* output) {
  new(output) QColormap(*colormap);
}

int qt_widgets_c_QColormap_depth(const QColormap* this_ptr) {
  return this_ptr->depth();
}

void qt_widgets_c_QColormap_destructor(QColormap* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

void qt_widgets_c_QColormap_initialize() {
  QColormap::initialize();
}

void qt_widgets_c_QColormap_instance_to_output_no_args(QColormap* output) {
  new(output) QColormap(QColormap::instance());
}

void qt_widgets_c_QColormap_instance_to_output_screen(int screen, QColormap* output) {
  new(output) QColormap(QColormap::instance(screen));
}

QColormap::Mode qt_widgets_c_QColormap_mode(const QColormap* this_ptr) {
  return this_ptr->mode();
}

QColormap* qt_widgets_c_QColormap_operator_assign(QColormap* this_ptr, const QColormap* colormap) {
  return &this_ptr->operator=(*colormap);
}

unsigned int qt_widgets_c_QColormap_pixel(const QColormap* this_ptr, const QColor* color) {
  return this_ptr->pixel(*color);
}

int qt_widgets_c_QColormap_size(const QColormap* this_ptr) {
  return this_ptr->size();
}

