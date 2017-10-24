#include "qt_gui_c_QPainterPath.h"

void qt_gui_c_QPainterPath_Element_convert_to_QPointF_to_output(const QPainterPath::Element* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->operator QPointF());
}

void qt_gui_c_QPainterPath_Element_destructor(QPainterPath::Element* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QPainterPath_Element_isCurveTo(const QPainterPath::Element* this_ptr) {
  return this_ptr->isCurveTo();
}

bool qt_gui_c_QPainterPath_Element_isLineTo(const QPainterPath::Element* this_ptr) {
  return this_ptr->isLineTo();
}

bool qt_gui_c_QPainterPath_Element_isMoveTo(const QPainterPath::Element* this_ptr) {
  return this_ptr->isMoveTo();
}

bool qt_gui_c_QPainterPath_Element_operator_eq(const QPainterPath::Element* this_ptr, const QPainterPath::Element* e) {
  return this_ptr->operator==(*e);
}

bool qt_gui_c_QPainterPath_Element_operator_neq(const QPainterPath::Element* this_ptr, const QPainterPath::Element* e) {
  return this_ptr->operator!=(*e);
}

void qt_gui_c_QPainterPath_Element_set_type(QPainterPath::Element* this_ptr, QPainterPath::ElementType value) {
  this_ptr->type = value;
}

void qt_gui_c_QPainterPath_Element_set_x(QPainterPath::Element* this_ptr, double value) {
  this_ptr->x = value;
}

void qt_gui_c_QPainterPath_Element_set_y(QPainterPath::Element* this_ptr, double value) {
  this_ptr->y = value;
}

QPainterPath::ElementType qt_gui_c_QPainterPath_Element_type(const QPainterPath::Element* this_ptr) {
  return this_ptr->type;
}

double qt_gui_c_QPainterPath_Element_x(const QPainterPath::Element* this_ptr) {
  return this_ptr->x;
}

double qt_gui_c_QPainterPath_Element_y(const QPainterPath::Element* this_ptr) {
  return this_ptr->y;
}

void qt_gui_c_QPainterPath_G_operator_shl_to_output(const QDebug* arg1, const QPainterPath* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

void qt_gui_c_QPainterPath_G_swap(QPainterPath* value1, QPainterPath* value2) {
  swap(*value1, *value2);
}

void qt_gui_c_QPainterPath_addEllipse_center_rx_ry(QPainterPath* this_ptr, const QPointF* center, double rx, double ry) {
  this_ptr->addEllipse(*center, rx, ry);
}

void qt_gui_c_QPainterPath_addEllipse_rect(QPainterPath* this_ptr, const QRectF* rect) {
  this_ptr->addEllipse(*rect);
}

void qt_gui_c_QPainterPath_addEllipse_x_y_w_h(QPainterPath* this_ptr, double x, double y, double w, double h) {
  this_ptr->addEllipse(x, y, w, h);
}

void qt_gui_c_QPainterPath_addPath(QPainterPath* this_ptr, const QPainterPath* path) {
  this_ptr->addPath(*path);
}

void qt_gui_c_QPainterPath_addPolygon(QPainterPath* this_ptr, const QPolygonF* polygon) {
  this_ptr->addPolygon(*polygon);
}

void qt_gui_c_QPainterPath_addRect_rect(QPainterPath* this_ptr, const QRectF* rect) {
  this_ptr->addRect(*rect);
}

void qt_gui_c_QPainterPath_addRect_x_y_w_h(QPainterPath* this_ptr, double x, double y, double w, double h) {
  this_ptr->addRect(x, y, w, h);
}

void qt_gui_c_QPainterPath_addRegion(QPainterPath* this_ptr, const QRegion* region) {
  this_ptr->addRegion(*region);
}

void qt_gui_c_QPainterPath_addRoundRect_rect_roundness(QPainterPath* this_ptr, const QRectF* rect, int roundness) {
  this_ptr->addRoundRect(*rect, roundness);
}

void qt_gui_c_QPainterPath_addRoundRect_rect_xRnd_yRnd(QPainterPath* this_ptr, const QRectF* rect, int xRnd, int yRnd) {
  this_ptr->addRoundRect(*rect, xRnd, yRnd);
}

void qt_gui_c_QPainterPath_addRoundRect_x_y_w_h_roundness(QPainterPath* this_ptr, double x, double y, double w, double h, int roundness) {
  this_ptr->addRoundRect(x, y, w, h, roundness);
}

void qt_gui_c_QPainterPath_addRoundRect_x_y_w_h_xRnd_yRnd(QPainterPath* this_ptr, double x, double y, double w, double h, int xRnd, int yRnd) {
  this_ptr->addRoundRect(x, y, w, h, xRnd, yRnd);
}

void qt_gui_c_QPainterPath_addRoundedRect_rect_xRadius_yRadius(QPainterPath* this_ptr, const QRectF* rect, double xRadius, double yRadius) {
  this_ptr->addRoundedRect(*rect, xRadius, yRadius);
}

void qt_gui_c_QPainterPath_addRoundedRect_rect_xRadius_yRadius_mode(QPainterPath* this_ptr, const QRectF* rect, double xRadius, double yRadius, const Qt::SizeMode* mode) {
  this_ptr->addRoundedRect(*rect, xRadius, yRadius, *mode);
}

void qt_gui_c_QPainterPath_addRoundedRect_x_y_w_h_xRadius_yRadius(QPainterPath* this_ptr, double x, double y, double w, double h, double xRadius, double yRadius) {
  this_ptr->addRoundedRect(x, y, w, h, xRadius, yRadius);
}

void qt_gui_c_QPainterPath_addRoundedRect_x_y_w_h_xRadius_yRadius_mode(QPainterPath* this_ptr, double x, double y, double w, double h, double xRadius, double yRadius, const Qt::SizeMode* mode) {
  this_ptr->addRoundedRect(x, y, w, h, xRadius, yRadius, *mode);
}

void qt_gui_c_QPainterPath_addText_point_f_text(QPainterPath* this_ptr, const QPointF* point, const QFont* f, const QString* text) {
  this_ptr->addText(*point, *f, *text);
}

void qt_gui_c_QPainterPath_addText_x_y_f_text(QPainterPath* this_ptr, double x, double y, const QFont* f, const QString* text) {
  this_ptr->addText(x, y, *f, *text);
}

double qt_gui_c_QPainterPath_angleAtPercent(const QPainterPath* this_ptr, double t) {
  return this_ptr->angleAtPercent(t);
}

void qt_gui_c_QPainterPath_arcMoveTo_rect_angle(QPainterPath* this_ptr, const QRectF* rect, double angle) {
  this_ptr->arcMoveTo(*rect, angle);
}

void qt_gui_c_QPainterPath_arcMoveTo_x_y_w_h_angle(QPainterPath* this_ptr, double x, double y, double w, double h, double angle) {
  this_ptr->arcMoveTo(x, y, w, h, angle);
}

void qt_gui_c_QPainterPath_arcTo_rect_startAngle_arcLength(QPainterPath* this_ptr, const QRectF* rect, double startAngle, double arcLength) {
  this_ptr->arcTo(*rect, startAngle, arcLength);
}

void qt_gui_c_QPainterPath_arcTo_x_y_w_h_startAngle_arcLength(QPainterPath* this_ptr, double x, double y, double w, double h, double startAngle, double arcLength) {
  this_ptr->arcTo(x, y, w, h, startAngle, arcLength);
}

void qt_gui_c_QPainterPath_boundingRect_to_output(const QPainterPath* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect());
}

void qt_gui_c_QPainterPath_closeSubpath(QPainterPath* this_ptr) {
  this_ptr->closeSubpath();
}

void qt_gui_c_QPainterPath_connectPath(QPainterPath* this_ptr, const QPainterPath* path) {
  this_ptr->connectPath(*path);
}

void qt_gui_c_QPainterPath_constructor_no_args(QPainterPath* output) {
  new(output) QPainterPath();
}

void qt_gui_c_QPainterPath_constructor_other(const QPainterPath* other, QPainterPath* output) {
  new(output) QPainterPath(*other);
}

void qt_gui_c_QPainterPath_constructor_startPoint(const QPointF* startPoint, QPainterPath* output) {
  new(output) QPainterPath(*startPoint);
}

bool qt_gui_c_QPainterPath_contains_p(const QPainterPath* this_ptr, const QPainterPath* p) {
  return this_ptr->contains(*p);
}

bool qt_gui_c_QPainterPath_contains_pt(const QPainterPath* this_ptr, const QPointF* pt) {
  return this_ptr->contains(*pt);
}

bool qt_gui_c_QPainterPath_contains_rect(const QPainterPath* this_ptr, const QRectF* rect) {
  return this_ptr->contains(*rect);
}

void qt_gui_c_QPainterPath_controlPointRect_to_output(const QPainterPath* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->controlPointRect());
}

void qt_gui_c_QPainterPath_cubicTo_ctrlPt1_ctrlPt2_endPt(QPainterPath* this_ptr, const QPointF* ctrlPt1, const QPointF* ctrlPt2, const QPointF* endPt) {
  this_ptr->cubicTo(*ctrlPt1, *ctrlPt2, *endPt);
}

void qt_gui_c_QPainterPath_cubicTo_ctrlPt1x_ctrlPt1y_ctrlPt2x_ctrlPt2y_endPtx_endPty(QPainterPath* this_ptr, double ctrlPt1x, double ctrlPt1y, double ctrlPt2x, double ctrlPt2y, double endPtx, double endPty) {
  this_ptr->cubicTo(ctrlPt1x, ctrlPt1y, ctrlPt2x, ctrlPt2y, endPtx, endPty);
}

void qt_gui_c_QPainterPath_currentPosition_to_output(const QPainterPath* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->currentPosition());
}

void qt_gui_c_QPainterPath_destructor(QPainterPath* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QPainterPath_elementAt_to_output(const QPainterPath* this_ptr, int i, QPainterPath::Element* output) {
  new(output) QPainterPath::Element(this_ptr->elementAt(i));
}

int qt_gui_c_QPainterPath_elementCount(const QPainterPath* this_ptr) {
  return this_ptr->elementCount();
}

void qt_gui_c_QPainterPath_intersected_to_output(const QPainterPath* this_ptr, const QPainterPath* r, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->intersected(*r));
}

bool qt_gui_c_QPainterPath_intersects_p(const QPainterPath* this_ptr, const QPainterPath* p) {
  return this_ptr->intersects(*p);
}

bool qt_gui_c_QPainterPath_intersects_rect(const QPainterPath* this_ptr, const QRectF* rect) {
  return this_ptr->intersects(*rect);
}

bool qt_gui_c_QPainterPath_isEmpty(const QPainterPath* this_ptr) {
  return this_ptr->isEmpty();
}

double qt_gui_c_QPainterPath_length(const QPainterPath* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QPainterPath_lineTo_p(QPainterPath* this_ptr, const QPointF* p) {
  this_ptr->lineTo(*p);
}

void qt_gui_c_QPainterPath_lineTo_x_y(QPainterPath* this_ptr, double x, double y) {
  this_ptr->lineTo(x, y);
}

void qt_gui_c_QPainterPath_moveTo_p(QPainterPath* this_ptr, const QPointF* p) {
  this_ptr->moveTo(*p);
}

void qt_gui_c_QPainterPath_moveTo_x_y(QPainterPath* this_ptr, double x, double y) {
  this_ptr->moveTo(x, y);
}

QPainterPath* qt_gui_c_QPainterPath_operator_add_assign(QPainterPath* this_ptr, const QPainterPath* other) {
  return &this_ptr->operator+=(*other);
}

void qt_gui_c_QPainterPath_operator_add_to_output(const QPainterPath* this_ptr, const QPainterPath* other, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->operator+(*other));
}

QPainterPath* qt_gui_c_QPainterPath_operator_assign(QPainterPath* this_ptr, const QPainterPath* other) {
  return &this_ptr->operator=(*other);
}

QPainterPath* qt_gui_c_QPainterPath_operator_bit_and_assign(QPainterPath* this_ptr, const QPainterPath* other) {
  return &this_ptr->operator&=(*other);
}

void qt_gui_c_QPainterPath_operator_bit_and_to_output(const QPainterPath* this_ptr, const QPainterPath* other, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->operator&(*other));
}

QPainterPath* qt_gui_c_QPainterPath_operator_bit_or_assign(QPainterPath* this_ptr, const QPainterPath* other) {
  return &this_ptr->operator|=(*other);
}

void qt_gui_c_QPainterPath_operator_bit_or_to_output(const QPainterPath* this_ptr, const QPainterPath* other, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->operator|(*other));
}

bool qt_gui_c_QPainterPath_operator_eq(const QPainterPath* this_ptr, const QPainterPath* other) {
  return this_ptr->operator==(*other);
}

bool qt_gui_c_QPainterPath_operator_neq(const QPainterPath* this_ptr, const QPainterPath* other) {
  return this_ptr->operator!=(*other);
}

QPainterPath* qt_gui_c_QPainterPath_operator_sub_assign(QPainterPath* this_ptr, const QPainterPath* other) {
  return &this_ptr->operator-=(*other);
}

void qt_gui_c_QPainterPath_operator_sub_to_output(const QPainterPath* this_ptr, const QPainterPath* other, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->operator-(*other));
}

double qt_gui_c_QPainterPath_percentAtLength(const QPainterPath* this_ptr, double t) {
  return this_ptr->percentAtLength(t);
}

void qt_gui_c_QPainterPath_pointAtPercent_to_output(const QPainterPath* this_ptr, double t, QPointF* output) {
  new(output) QPointF(this_ptr->pointAtPercent(t));
}

void qt_gui_c_QPainterPath_quadTo_ctrlPt_endPt(QPainterPath* this_ptr, const QPointF* ctrlPt, const QPointF* endPt) {
  this_ptr->quadTo(*ctrlPt, *endPt);
}

void qt_gui_c_QPainterPath_quadTo_ctrlPtx_ctrlPty_endPtx_endPty(QPainterPath* this_ptr, double ctrlPtx, double ctrlPty, double endPtx, double endPty) {
  this_ptr->quadTo(ctrlPtx, ctrlPty, endPtx, endPty);
}

void qt_gui_c_QPainterPath_setElementPositionAt(QPainterPath* this_ptr, int i, double x, double y) {
  this_ptr->setElementPositionAt(i, x, y);
}

void qt_gui_c_QPainterPath_setFillRule(QPainterPath* this_ptr, const Qt::FillRule* fillRule) {
  this_ptr->setFillRule(*fillRule);
}

void qt_gui_c_QPainterPath_simplified_to_output(const QPainterPath* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->simplified());
}

double qt_gui_c_QPainterPath_slopeAtPercent(const QPainterPath* this_ptr, double t) {
  return this_ptr->slopeAtPercent(t);
}

void qt_gui_c_QPainterPath_subtractedInverted_to_output(const QPainterPath* this_ptr, const QPainterPath* r, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->subtractedInverted(*r));
}

void qt_gui_c_QPainterPath_subtracted_to_output(const QPainterPath* this_ptr, const QPainterPath* r, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->subtracted(*r));
}

void qt_gui_c_QPainterPath_swap(QPainterPath* this_ptr, QPainterPath* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QPainterPath_toFillPolygon_to_output_QMatrix(const QPainterPath* this_ptr, const QMatrix* matrix, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->toFillPolygon(*matrix));
}

void qt_gui_c_QPainterPath_toFillPolygon_to_output_QTransform(const QPainterPath* this_ptr, const QTransform* matrix, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->toFillPolygon(*matrix));
}

void qt_gui_c_QPainterPath_toFillPolygon_to_output_no_args(const QPainterPath* this_ptr, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->toFillPolygon());
}

void qt_gui_c_QPainterPath_toFillPolygons_to_output_QMatrix(const QPainterPath* this_ptr, const QMatrix* matrix, QList< QPolygonF >* output) {
  new(output) QList< QPolygonF >(this_ptr->toFillPolygons(*matrix));
}

void qt_gui_c_QPainterPath_toFillPolygons_to_output_QTransform(const QPainterPath* this_ptr, const QTransform* matrix, QList< QPolygonF >* output) {
  new(output) QList< QPolygonF >(this_ptr->toFillPolygons(*matrix));
}

void qt_gui_c_QPainterPath_toFillPolygons_to_output_no_args(const QPainterPath* this_ptr, QList< QPolygonF >* output) {
  new(output) QList< QPolygonF >(this_ptr->toFillPolygons());
}

void qt_gui_c_QPainterPath_toReversed_to_output(const QPainterPath* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->toReversed());
}

void qt_gui_c_QPainterPath_toSubpathPolygons_to_output_QMatrix(const QPainterPath* this_ptr, const QMatrix* matrix, QList< QPolygonF >* output) {
  new(output) QList< QPolygonF >(this_ptr->toSubpathPolygons(*matrix));
}

void qt_gui_c_QPainterPath_toSubpathPolygons_to_output_QTransform(const QPainterPath* this_ptr, const QTransform* matrix, QList< QPolygonF >* output) {
  new(output) QList< QPolygonF >(this_ptr->toSubpathPolygons(*matrix));
}

void qt_gui_c_QPainterPath_toSubpathPolygons_to_output_no_args(const QPainterPath* this_ptr, QList< QPolygonF >* output) {
  new(output) QList< QPolygonF >(this_ptr->toSubpathPolygons());
}

void qt_gui_c_QPainterPath_translate_dx_dy(QPainterPath* this_ptr, double dx, double dy) {
  this_ptr->translate(dx, dy);
}

void qt_gui_c_QPainterPath_translate_offset(QPainterPath* this_ptr, const QPointF* offset) {
  this_ptr->translate(*offset);
}

void qt_gui_c_QPainterPath_translated_to_output_dx_dy(const QPainterPath* this_ptr, double dx, double dy, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->translated(dx, dy));
}

void qt_gui_c_QPainterPath_translated_to_output_offset(const QPainterPath* this_ptr, const QPointF* offset, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->translated(*offset));
}

void qt_gui_c_QPainterPath_united_to_output(const QPainterPath* this_ptr, const QPainterPath* r, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->united(*r));
}

