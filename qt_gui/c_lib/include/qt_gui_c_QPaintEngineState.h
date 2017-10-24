#ifndef QT_GUI_C_QPAINTENGINESTATE_H
#define QT_GUI_C_QPAINTENGINESTATE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QPaintEngineState_backgroundBrush_to_output(const QPaintEngineState* this_ptr, QBrush* output);
QT_GUI_C_EXPORT bool qt_gui_c_QPaintEngineState_brushNeedsResolving(const QPaintEngineState* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngineState_brushOrigin_to_output(const QPaintEngineState* this_ptr, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngineState_brush_to_output(const QPaintEngineState* this_ptr, QBrush* output);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngineState_clipPath_to_output(const QPaintEngineState* this_ptr, QPainterPath* output);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QPaintEngineState_clipRegion_as_ptr(const QPaintEngineState* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngineState_destructor(QPaintEngineState* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngineState_font_to_output(const QPaintEngineState* this_ptr, QFont* output);
QT_GUI_C_EXPORT bool qt_gui_c_QPaintEngineState_isClipEnabled(const QPaintEngineState* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngineState_matrix_to_output(const QPaintEngineState* this_ptr, QMatrix* output);
QT_GUI_C_EXPORT double qt_gui_c_QPaintEngineState_opacity(const QPaintEngineState* this_ptr);
QT_GUI_C_EXPORT QPainter* qt_gui_c_QPaintEngineState_painter(const QPaintEngineState* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPaintEngineState_penNeedsResolving(const QPaintEngineState* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngineState_pen_to_output(const QPaintEngineState* this_ptr, QPen* output);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngineState_transform_to_output(const QPaintEngineState* this_ptr, QTransform* output);

} // extern "C"

#endif // QT_GUI_C_QPAINTENGINESTATE_H
