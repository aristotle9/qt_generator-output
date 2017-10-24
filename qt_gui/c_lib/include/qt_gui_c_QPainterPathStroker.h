#ifndef QT_GUI_C_QPAINTERPATHSTROKER_H
#define QT_GUI_C_QPAINTERPATHSTROKER_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QPainterPathStroker_constructor_no_args(QPainterPathStroker* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPathStroker_constructor_pen(const QPen* pen, QPainterPathStroker* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPathStroker_createStroke_to_output(const QPainterPathStroker* this_ptr, const QPainterPath* path, QPainterPath* output);
QT_GUI_C_EXPORT double qt_gui_c_QPainterPathStroker_curveThreshold(const QPainterPathStroker* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QPainterPathStroker_dashOffset(const QPainterPathStroker* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPathStroker_dashPattern_to_output(const QPainterPathStroker* this_ptr, QVector< double >* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPathStroker_destructor(QPainterPathStroker* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QPainterPathStroker_miterLimit(const QPainterPathStroker* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPathStroker_setCapStyle(QPainterPathStroker* this_ptr, const Qt::PenCapStyle* style);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPathStroker_setCurveThreshold(QPainterPathStroker* this_ptr, double threshold);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPathStroker_setDashOffset(QPainterPathStroker* this_ptr, double offset);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPathStroker_setDashPattern_arg1(QPainterPathStroker* this_ptr, const Qt::PenStyle* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPathStroker_setDashPattern_dashPattern(QPainterPathStroker* this_ptr, const QVector< double >* dashPattern);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPathStroker_setJoinStyle(QPainterPathStroker* this_ptr, const Qt::PenJoinStyle* style);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPathStroker_setMiterLimit(QPainterPathStroker* this_ptr, double length);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPathStroker_setWidth(QPainterPathStroker* this_ptr, double width);
QT_GUI_C_EXPORT double qt_gui_c_QPainterPathStroker_width(const QPainterPathStroker* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QPAINTERPATHSTROKER_H
