#ifndef QT_GUI_C_QGRADIENT_H
#define QT_GUI_C_QGRADIENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QGradient::CoordinateMode qt_gui_c_QGradient_coordinateMode(const QGradient* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QGradient_delete(QGradient* this_ptr);
QT_GUI_C_EXPORT QGradient::InterpolationMode qt_gui_c_QGradient_interpolationMode(const QGradient* this_ptr);
QT_GUI_C_EXPORT QGradient* qt_gui_c_QGradient_new();
QT_GUI_C_EXPORT bool qt_gui_c_QGradient_operator_eq(const QGradient* this_ptr, const QGradient* gradient);
QT_GUI_C_EXPORT bool qt_gui_c_QGradient_operator_neq(const QGradient* this_ptr, const QGradient* other);
QT_GUI_C_EXPORT void qt_gui_c_QGradient_setColorAt(QGradient* this_ptr, double pos, const QColor* color);
QT_GUI_C_EXPORT void qt_gui_c_QGradient_setCoordinateMode(QGradient* this_ptr, QGradient::CoordinateMode mode);
QT_GUI_C_EXPORT void qt_gui_c_QGradient_setInterpolationMode(QGradient* this_ptr, QGradient::InterpolationMode mode);
QT_GUI_C_EXPORT void qt_gui_c_QGradient_setSpread(QGradient* this_ptr, QGradient::Spread spread);
QT_GUI_C_EXPORT void qt_gui_c_QGradient_setStops(QGradient* this_ptr, const QVector< QPair< double, QColor > >* stops);
QT_GUI_C_EXPORT QGradient::Spread qt_gui_c_QGradient_spread(const QGradient* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QGradient_stops_to_output(const QGradient* this_ptr, QVector< QPair< double, QColor > >* output);
QT_GUI_C_EXPORT QGradient::Type qt_gui_c_QGradient_type(const QGradient* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QGRADIENT_H
