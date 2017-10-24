#include "qt_gui_c_QPaintEngineState.h"

void qt_gui_c_QPaintEngineState_backgroundBrush_to_output(const QPaintEngineState* this_ptr, QBrush* output) {
  new(output) QBrush(this_ptr->backgroundBrush());
}

bool qt_gui_c_QPaintEngineState_brushNeedsResolving(const QPaintEngineState* this_ptr) {
  return this_ptr->brushNeedsResolving();
}

void qt_gui_c_QPaintEngineState_brushOrigin_to_output(const QPaintEngineState* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->brushOrigin());
}

void qt_gui_c_QPaintEngineState_brush_to_output(const QPaintEngineState* this_ptr, QBrush* output) {
  new(output) QBrush(this_ptr->brush());
}

void qt_gui_c_QPaintEngineState_clipPath_to_output(const QPaintEngineState* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->clipPath());
}

QRegion* qt_gui_c_QPaintEngineState_clipRegion_as_ptr(const QPaintEngineState* this_ptr) {
  return new QRegion(this_ptr->clipRegion());
}

void qt_gui_c_QPaintEngineState_destructor(QPaintEngineState* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QPaintEngineState_font_to_output(const QPaintEngineState* this_ptr, QFont* output) {
  new(output) QFont(this_ptr->font());
}

bool qt_gui_c_QPaintEngineState_isClipEnabled(const QPaintEngineState* this_ptr) {
  return this_ptr->isClipEnabled();
}

void qt_gui_c_QPaintEngineState_matrix_to_output(const QPaintEngineState* this_ptr, QMatrix* output) {
  new(output) QMatrix(this_ptr->matrix());
}

double qt_gui_c_QPaintEngineState_opacity(const QPaintEngineState* this_ptr) {
  return this_ptr->opacity();
}

QPainter* qt_gui_c_QPaintEngineState_painter(const QPaintEngineState* this_ptr) {
  return this_ptr->painter();
}

bool qt_gui_c_QPaintEngineState_penNeedsResolving(const QPaintEngineState* this_ptr) {
  return this_ptr->penNeedsResolving();
}

void qt_gui_c_QPaintEngineState_pen_to_output(const QPaintEngineState* this_ptr, QPen* output) {
  new(output) QPen(this_ptr->pen());
}

void qt_gui_c_QPaintEngineState_transform_to_output(const QPaintEngineState* this_ptr, QTransform* output) {
  new(output) QTransform(this_ptr->transform());
}

