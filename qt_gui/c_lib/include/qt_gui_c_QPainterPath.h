#ifndef QT_GUI_C_QPAINTERPATH_H
#define QT_GUI_C_QPAINTERPATH_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_Element_convert_to_QPointF_to_output(const QPainterPath::Element* this_ptr, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_Element_destructor(QPainterPath::Element* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPainterPath_Element_isCurveTo(const QPainterPath::Element* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPainterPath_Element_isLineTo(const QPainterPath::Element* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPainterPath_Element_isMoveTo(const QPainterPath::Element* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPainterPath_Element_operator_eq(const QPainterPath::Element* this_ptr, const QPainterPath::Element* e);
QT_GUI_C_EXPORT bool qt_gui_c_QPainterPath_Element_operator_neq(const QPainterPath::Element* this_ptr, const QPainterPath::Element* e);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_Element_set_type(QPainterPath::Element* this_ptr, QPainterPath::ElementType value);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_Element_set_x(QPainterPath::Element* this_ptr, double value);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_Element_set_y(QPainterPath::Element* this_ptr, double value);
QT_GUI_C_EXPORT QPainterPath::ElementType qt_gui_c_QPainterPath_Element_type(const QPainterPath::Element* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QPainterPath_Element_x(const QPainterPath::Element* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QPainterPath_Element_y(const QPainterPath::Element* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_G_operator_shl_to_output(const QDebug* arg1, const QPainterPath* arg2, QDebug* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_G_swap(QPainterPath* value1, QPainterPath* value2);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_addEllipse_center_rx_ry(QPainterPath* this_ptr, const QPointF* center, double rx, double ry);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_addEllipse_rect(QPainterPath* this_ptr, const QRectF* rect);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_addEllipse_x_y_w_h(QPainterPath* this_ptr, double x, double y, double w, double h);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_addPath(QPainterPath* this_ptr, const QPainterPath* path);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_addPolygon(QPainterPath* this_ptr, const QPolygonF* polygon);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_addRect_rect(QPainterPath* this_ptr, const QRectF* rect);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_addRect_x_y_w_h(QPainterPath* this_ptr, double x, double y, double w, double h);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_addRegion(QPainterPath* this_ptr, const QRegion* region);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_addRoundRect_rect_roundness(QPainterPath* this_ptr, const QRectF* rect, int roundness);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_addRoundRect_rect_xRnd_yRnd(QPainterPath* this_ptr, const QRectF* rect, int xRnd, int yRnd);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_addRoundRect_x_y_w_h_roundness(QPainterPath* this_ptr, double x, double y, double w, double h, int roundness);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_addRoundRect_x_y_w_h_xRnd_yRnd(QPainterPath* this_ptr, double x, double y, double w, double h, int xRnd, int yRnd);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_addRoundedRect_rect_xRadius_yRadius(QPainterPath* this_ptr, const QRectF* rect, double xRadius, double yRadius);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_addRoundedRect_rect_xRadius_yRadius_mode(QPainterPath* this_ptr, const QRectF* rect, double xRadius, double yRadius, const Qt::SizeMode* mode);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_addRoundedRect_x_y_w_h_xRadius_yRadius(QPainterPath* this_ptr, double x, double y, double w, double h, double xRadius, double yRadius);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_addRoundedRect_x_y_w_h_xRadius_yRadius_mode(QPainterPath* this_ptr, double x, double y, double w, double h, double xRadius, double yRadius, const Qt::SizeMode* mode);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_addText_point_f_text(QPainterPath* this_ptr, const QPointF* point, const QFont* f, const QString* text);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_addText_x_y_f_text(QPainterPath* this_ptr, double x, double y, const QFont* f, const QString* text);
QT_GUI_C_EXPORT double qt_gui_c_QPainterPath_angleAtPercent(const QPainterPath* this_ptr, double t);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_arcMoveTo_rect_angle(QPainterPath* this_ptr, const QRectF* rect, double angle);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_arcMoveTo_x_y_w_h_angle(QPainterPath* this_ptr, double x, double y, double w, double h, double angle);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_arcTo_rect_startAngle_arcLength(QPainterPath* this_ptr, const QRectF* rect, double startAngle, double arcLength);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_arcTo_x_y_w_h_startAngle_arcLength(QPainterPath* this_ptr, double x, double y, double w, double h, double startAngle, double arcLength);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_boundingRect_to_output(const QPainterPath* this_ptr, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_closeSubpath(QPainterPath* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_connectPath(QPainterPath* this_ptr, const QPainterPath* path);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_constructor_no_args(QPainterPath* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_constructor_other(const QPainterPath* other, QPainterPath* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_constructor_startPoint(const QPointF* startPoint, QPainterPath* output);
QT_GUI_C_EXPORT bool qt_gui_c_QPainterPath_contains_p(const QPainterPath* this_ptr, const QPainterPath* p);
QT_GUI_C_EXPORT bool qt_gui_c_QPainterPath_contains_pt(const QPainterPath* this_ptr, const QPointF* pt);
QT_GUI_C_EXPORT bool qt_gui_c_QPainterPath_contains_rect(const QPainterPath* this_ptr, const QRectF* rect);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_controlPointRect_to_output(const QPainterPath* this_ptr, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_cubicTo_ctrlPt1_ctrlPt2_endPt(QPainterPath* this_ptr, const QPointF* ctrlPt1, const QPointF* ctrlPt2, const QPointF* endPt);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_cubicTo_ctrlPt1x_ctrlPt1y_ctrlPt2x_ctrlPt2y_endPtx_endPty(QPainterPath* this_ptr, double ctrlPt1x, double ctrlPt1y, double ctrlPt2x, double ctrlPt2y, double endPtx, double endPty);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_currentPosition_to_output(const QPainterPath* this_ptr, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_destructor(QPainterPath* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_elementAt_to_output(const QPainterPath* this_ptr, int i, QPainterPath::Element* output);
QT_GUI_C_EXPORT int qt_gui_c_QPainterPath_elementCount(const QPainterPath* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_intersected_to_output(const QPainterPath* this_ptr, const QPainterPath* r, QPainterPath* output);
QT_GUI_C_EXPORT bool qt_gui_c_QPainterPath_intersects_p(const QPainterPath* this_ptr, const QPainterPath* p);
QT_GUI_C_EXPORT bool qt_gui_c_QPainterPath_intersects_rect(const QPainterPath* this_ptr, const QRectF* rect);
QT_GUI_C_EXPORT bool qt_gui_c_QPainterPath_isEmpty(const QPainterPath* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QPainterPath_length(const QPainterPath* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_lineTo_p(QPainterPath* this_ptr, const QPointF* p);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_lineTo_x_y(QPainterPath* this_ptr, double x, double y);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_moveTo_p(QPainterPath* this_ptr, const QPointF* p);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_moveTo_x_y(QPainterPath* this_ptr, double x, double y);
QT_GUI_C_EXPORT QPainterPath* qt_gui_c_QPainterPath_operator_add_assign(QPainterPath* this_ptr, const QPainterPath* other);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_operator_add_to_output(const QPainterPath* this_ptr, const QPainterPath* other, QPainterPath* output);
QT_GUI_C_EXPORT QPainterPath* qt_gui_c_QPainterPath_operator_assign(QPainterPath* this_ptr, const QPainterPath* other);
QT_GUI_C_EXPORT QPainterPath* qt_gui_c_QPainterPath_operator_bit_and_assign(QPainterPath* this_ptr, const QPainterPath* other);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_operator_bit_and_to_output(const QPainterPath* this_ptr, const QPainterPath* other, QPainterPath* output);
QT_GUI_C_EXPORT QPainterPath* qt_gui_c_QPainterPath_operator_bit_or_assign(QPainterPath* this_ptr, const QPainterPath* other);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_operator_bit_or_to_output(const QPainterPath* this_ptr, const QPainterPath* other, QPainterPath* output);
QT_GUI_C_EXPORT bool qt_gui_c_QPainterPath_operator_eq(const QPainterPath* this_ptr, const QPainterPath* other);
QT_GUI_C_EXPORT bool qt_gui_c_QPainterPath_operator_neq(const QPainterPath* this_ptr, const QPainterPath* other);
QT_GUI_C_EXPORT QPainterPath* qt_gui_c_QPainterPath_operator_sub_assign(QPainterPath* this_ptr, const QPainterPath* other);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_operator_sub_to_output(const QPainterPath* this_ptr, const QPainterPath* other, QPainterPath* output);
QT_GUI_C_EXPORT double qt_gui_c_QPainterPath_percentAtLength(const QPainterPath* this_ptr, double t);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_pointAtPercent_to_output(const QPainterPath* this_ptr, double t, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_quadTo_ctrlPt_endPt(QPainterPath* this_ptr, const QPointF* ctrlPt, const QPointF* endPt);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_quadTo_ctrlPtx_ctrlPty_endPtx_endPty(QPainterPath* this_ptr, double ctrlPtx, double ctrlPty, double endPtx, double endPty);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_setElementPositionAt(QPainterPath* this_ptr, int i, double x, double y);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_setFillRule(QPainterPath* this_ptr, const Qt::FillRule* fillRule);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_simplified_to_output(const QPainterPath* this_ptr, QPainterPath* output);
QT_GUI_C_EXPORT double qt_gui_c_QPainterPath_slopeAtPercent(const QPainterPath* this_ptr, double t);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_subtractedInverted_to_output(const QPainterPath* this_ptr, const QPainterPath* r, QPainterPath* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_subtracted_to_output(const QPainterPath* this_ptr, const QPainterPath* r, QPainterPath* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_swap(QPainterPath* this_ptr, QPainterPath* other);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_toFillPolygon_to_output_QMatrix(const QPainterPath* this_ptr, const QMatrix* matrix, QPolygonF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_toFillPolygon_to_output_QTransform(const QPainterPath* this_ptr, const QTransform* matrix, QPolygonF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_toFillPolygon_to_output_no_args(const QPainterPath* this_ptr, QPolygonF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_toFillPolygons_to_output_QMatrix(const QPainterPath* this_ptr, const QMatrix* matrix, QList< QPolygonF >* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_toFillPolygons_to_output_QTransform(const QPainterPath* this_ptr, const QTransform* matrix, QList< QPolygonF >* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_toFillPolygons_to_output_no_args(const QPainterPath* this_ptr, QList< QPolygonF >* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_toReversed_to_output(const QPainterPath* this_ptr, QPainterPath* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_toSubpathPolygons_to_output_QMatrix(const QPainterPath* this_ptr, const QMatrix* matrix, QList< QPolygonF >* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_toSubpathPolygons_to_output_QTransform(const QPainterPath* this_ptr, const QTransform* matrix, QList< QPolygonF >* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_toSubpathPolygons_to_output_no_args(const QPainterPath* this_ptr, QList< QPolygonF >* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_translate_dx_dy(QPainterPath* this_ptr, double dx, double dy);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_translate_offset(QPainterPath* this_ptr, const QPointF* offset);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_translated_to_output_dx_dy(const QPainterPath* this_ptr, double dx, double dy, QPainterPath* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_translated_to_output_offset(const QPainterPath* this_ptr, const QPointF* offset, QPainterPath* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainterPath_united_to_output(const QPainterPath* this_ptr, const QPainterPath* r, QPainterPath* output);

} // extern "C"

#endif // QT_GUI_C_QPAINTERPATH_H
