#include "qt_gui_c_QPainter.h"

QPainter::PixmapFragment* qt_gui_c_QPainter_PixmapFragment_create_as_ptr_pos_sourceRect(const QPointF* pos, const QRectF* sourceRect) {
  return new QPainter::PixmapFragment(QPainter::PixmapFragment::create(*pos, *sourceRect));
}

QPainter::PixmapFragment* qt_gui_c_QPainter_PixmapFragment_create_as_ptr_pos_sourceRect_scaleX(const QPointF* pos, const QRectF* sourceRect, double scaleX) {
  return new QPainter::PixmapFragment(QPainter::PixmapFragment::create(*pos, *sourceRect, scaleX));
}

QPainter::PixmapFragment* qt_gui_c_QPainter_PixmapFragment_create_as_ptr_pos_sourceRect_scaleX_scaleY(const QPointF* pos, const QRectF* sourceRect, double scaleX, double scaleY) {
  return new QPainter::PixmapFragment(QPainter::PixmapFragment::create(*pos, *sourceRect, scaleX, scaleY));
}

QPainter::PixmapFragment* qt_gui_c_QPainter_PixmapFragment_create_as_ptr_pos_sourceRect_scaleX_scaleY_rotation(const QPointF* pos, const QRectF* sourceRect, double scaleX, double scaleY, double rotation) {
  return new QPainter::PixmapFragment(QPainter::PixmapFragment::create(*pos, *sourceRect, scaleX, scaleY, rotation));
}

QPainter::PixmapFragment* qt_gui_c_QPainter_PixmapFragment_create_as_ptr_pos_sourceRect_scaleX_scaleY_rotation_opacity(const QPointF* pos, const QRectF* sourceRect, double scaleX, double scaleY, double rotation, double opacity) {
  return new QPainter::PixmapFragment(QPainter::PixmapFragment::create(*pos, *sourceRect, scaleX, scaleY, rotation, opacity));
}

void qt_gui_c_QPainter_PixmapFragment_delete(QPainter::PixmapFragment* this_ptr) {
  delete this_ptr;
}

double qt_gui_c_QPainter_PixmapFragment_height(const QPainter::PixmapFragment* this_ptr) {
  return this_ptr->height;
}

double qt_gui_c_QPainter_PixmapFragment_opacity(const QPainter::PixmapFragment* this_ptr) {
  return this_ptr->opacity;
}

double qt_gui_c_QPainter_PixmapFragment_rotation(const QPainter::PixmapFragment* this_ptr) {
  return this_ptr->rotation;
}

double qt_gui_c_QPainter_PixmapFragment_scaleX(const QPainter::PixmapFragment* this_ptr) {
  return this_ptr->scaleX;
}

double qt_gui_c_QPainter_PixmapFragment_scaleY(const QPainter::PixmapFragment* this_ptr) {
  return this_ptr->scaleY;
}

void qt_gui_c_QPainter_PixmapFragment_set_height(QPainter::PixmapFragment* this_ptr, double value) {
  this_ptr->height = value;
}

void qt_gui_c_QPainter_PixmapFragment_set_opacity(QPainter::PixmapFragment* this_ptr, double value) {
  this_ptr->opacity = value;
}

void qt_gui_c_QPainter_PixmapFragment_set_rotation(QPainter::PixmapFragment* this_ptr, double value) {
  this_ptr->rotation = value;
}

void qt_gui_c_QPainter_PixmapFragment_set_scaleX(QPainter::PixmapFragment* this_ptr, double value) {
  this_ptr->scaleX = value;
}

void qt_gui_c_QPainter_PixmapFragment_set_scaleY(QPainter::PixmapFragment* this_ptr, double value) {
  this_ptr->scaleY = value;
}

void qt_gui_c_QPainter_PixmapFragment_set_sourceLeft(QPainter::PixmapFragment* this_ptr, double value) {
  this_ptr->sourceLeft = value;
}

void qt_gui_c_QPainter_PixmapFragment_set_sourceTop(QPainter::PixmapFragment* this_ptr, double value) {
  this_ptr->sourceTop = value;
}

void qt_gui_c_QPainter_PixmapFragment_set_width(QPainter::PixmapFragment* this_ptr, double value) {
  this_ptr->width = value;
}

void qt_gui_c_QPainter_PixmapFragment_set_x(QPainter::PixmapFragment* this_ptr, double value) {
  this_ptr->x = value;
}

void qt_gui_c_QPainter_PixmapFragment_set_y(QPainter::PixmapFragment* this_ptr, double value) {
  this_ptr->y = value;
}

double qt_gui_c_QPainter_PixmapFragment_sourceLeft(const QPainter::PixmapFragment* this_ptr) {
  return this_ptr->sourceLeft;
}

double qt_gui_c_QPainter_PixmapFragment_sourceTop(const QPainter::PixmapFragment* this_ptr) {
  return this_ptr->sourceTop;
}

double qt_gui_c_QPainter_PixmapFragment_width(const QPainter::PixmapFragment* this_ptr) {
  return this_ptr->width;
}

double qt_gui_c_QPainter_PixmapFragment_x(const QPainter::PixmapFragment* this_ptr) {
  return this_ptr->x;
}

double qt_gui_c_QPainter_PixmapFragment_y(const QPainter::PixmapFragment* this_ptr) {
  return this_ptr->y;
}

const QBrush* qt_gui_c_QPainter_background(const QPainter* this_ptr) {
  return &this_ptr->background();
}

bool qt_gui_c_QPainter_begin(QPainter* this_ptr, QPaintDevice* arg1) {
  return this_ptr->begin(arg1);
}

void qt_gui_c_QPainter_beginNativePainting(QPainter* this_ptr) {
  this_ptr->beginNativePainting();
}

void qt_gui_c_QPainter_boundingRect_to_output_QRectF_QString(QPainter* this_ptr, const QRectF* rect, const QString* text, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect(*rect, *text));
}

void qt_gui_c_QPainter_boundingRect_to_output_QRectF_QString_QTextOption(QPainter* this_ptr, const QRectF* rect, const QString* text, const QTextOption* o, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect(*rect, *text, *o));
}

void qt_gui_c_QPainter_boundingRect_to_output_QRectF_int_QString(QPainter* this_ptr, const QRectF* rect, int flags, const QString* text, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect(*rect, flags, *text));
}

void qt_gui_c_QPainter_boundingRect_to_output_QRect_int_QString(QPainter* this_ptr, const QRect* rect, int flags, const QString* text, QRect* output) {
  new(output) QRect(this_ptr->boundingRect(*rect, flags, *text));
}

void qt_gui_c_QPainter_boundingRect_to_output_int_int_int_int_int_QString(QPainter* this_ptr, int x, int y, int w, int h, int flags, const QString* text, QRect* output) {
  new(output) QRect(this_ptr->boundingRect(x, y, w, h, flags, *text));
}

const QBrush* qt_gui_c_QPainter_brush(const QPainter* this_ptr) {
  return &this_ptr->brush();
}

void qt_gui_c_QPainter_brushOrigin_to_output(const QPainter* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->brushOrigin());
}

void qt_gui_c_QPainter_clipBoundingRect_to_output(const QPainter* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->clipBoundingRect());
}

void qt_gui_c_QPainter_clipPath_to_output(const QPainter* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->clipPath());
}

QRegion* qt_gui_c_QPainter_clipRegion_as_ptr(const QPainter* this_ptr) {
  return new QRegion(this_ptr->clipRegion());
}

void qt_gui_c_QPainter_combinedMatrix_to_output(const QPainter* this_ptr, QMatrix* output) {
  new(output) QMatrix(this_ptr->combinedMatrix());
}

void qt_gui_c_QPainter_combinedTransform_to_output(const QPainter* this_ptr, QTransform* output) {
  new(output) QTransform(this_ptr->combinedTransform());
}

QPainter::CompositionMode qt_gui_c_QPainter_compositionMode(const QPainter* this_ptr) {
  return this_ptr->compositionMode();
}

void qt_gui_c_QPainter_delete(QPainter* this_ptr) {
  delete this_ptr;
}

QPaintDevice* qt_gui_c_QPainter_device(const QPainter* this_ptr) {
  return this_ptr->device();
}

const QMatrix* qt_gui_c_QPainter_deviceMatrix(const QPainter* this_ptr) {
  return &this_ptr->deviceMatrix();
}

const QTransform* qt_gui_c_QPainter_deviceTransform(const QPainter* this_ptr) {
  return &this_ptr->deviceTransform();
}

void qt_gui_c_QPainter_drawArc_arg1_a_alen(QPainter* this_ptr, const QRect* arg1, int a, int alen) {
  this_ptr->drawArc(*arg1, a, alen);
}

void qt_gui_c_QPainter_drawArc_rect_a_alen(QPainter* this_ptr, const QRectF* rect, int a, int alen) {
  this_ptr->drawArc(*rect, a, alen);
}

void qt_gui_c_QPainter_drawArc_x_y_w_h_a_alen(QPainter* this_ptr, int x, int y, int w, int h, int a, int alen) {
  this_ptr->drawArc(x, y, w, h, a, alen);
}

void qt_gui_c_QPainter_drawChord_arg1_a_alen(QPainter* this_ptr, const QRect* arg1, int a, int alen) {
  this_ptr->drawChord(*arg1, a, alen);
}

void qt_gui_c_QPainter_drawChord_rect_a_alen(QPainter* this_ptr, const QRectF* rect, int a, int alen) {
  this_ptr->drawChord(*rect, a, alen);
}

void qt_gui_c_QPainter_drawChord_x_y_w_h_a_alen(QPainter* this_ptr, int x, int y, int w, int h, int a, int alen) {
  this_ptr->drawChord(x, y, w, h, a, alen);
}

void qt_gui_c_QPainter_drawConvexPolygon_QPointF_int(QPainter* this_ptr, const QPointF* points, int pointCount) {
  this_ptr->drawConvexPolygon(points, pointCount);
}

void qt_gui_c_QPainter_drawConvexPolygon_QPoint_int(QPainter* this_ptr, const QPoint* points, int pointCount) {
  this_ptr->drawConvexPolygon(points, pointCount);
}

void qt_gui_c_QPainter_drawConvexPolygon_QPolygon(QPainter* this_ptr, const QPolygon* polygon) {
  this_ptr->drawConvexPolygon(*polygon);
}

void qt_gui_c_QPainter_drawConvexPolygon_QPolygonF(QPainter* this_ptr, const QPolygonF* polygon) {
  this_ptr->drawConvexPolygon(*polygon);
}

void qt_gui_c_QPainter_drawEllipse_QPointF_double_double(QPainter* this_ptr, const QPointF* center, double rx, double ry) {
  this_ptr->drawEllipse(*center, rx, ry);
}

void qt_gui_c_QPainter_drawEllipse_QPoint_int_int(QPainter* this_ptr, const QPoint* center, int rx, int ry) {
  this_ptr->drawEllipse(*center, rx, ry);
}

void qt_gui_c_QPainter_drawEllipse_QRect(QPainter* this_ptr, const QRect* r) {
  this_ptr->drawEllipse(*r);
}

void qt_gui_c_QPainter_drawEllipse_QRectF(QPainter* this_ptr, const QRectF* r) {
  this_ptr->drawEllipse(*r);
}

void qt_gui_c_QPainter_drawEllipse_int_int_int_int(QPainter* this_ptr, int x, int y, int w, int h) {
  this_ptr->drawEllipse(x, y, w, h);
}

void qt_gui_c_QPainter_drawGlyphRun(QPainter* this_ptr, const QPointF* position, const QGlyphRun* glyphRun) {
  this_ptr->drawGlyphRun(*position, *glyphRun);
}

void qt_gui_c_QPainter_drawImage_QPointF_QImage(QPainter* this_ptr, const QPointF* p, const QImage* image) {
  this_ptr->drawImage(*p, *image);
}

void qt_gui_c_QPainter_drawImage_QPoint_QImage(QPainter* this_ptr, const QPoint* p, const QImage* image) {
  this_ptr->drawImage(*p, *image);
}

void qt_gui_c_QPainter_drawImage_QRectF_QImage(QPainter* this_ptr, const QRectF* r, const QImage* image) {
  this_ptr->drawImage(*r, *image);
}

void qt_gui_c_QPainter_drawImage_QRect_QImage(QPainter* this_ptr, const QRect* r, const QImage* image) {
  this_ptr->drawImage(*r, *image);
}

void qt_gui_c_QPainter_drawLine_QLine(QPainter* this_ptr, const QLine* line) {
  this_ptr->drawLine(*line);
}

void qt_gui_c_QPainter_drawLine_QLineF(QPainter* this_ptr, const QLineF* line) {
  this_ptr->drawLine(*line);
}

void qt_gui_c_QPainter_drawLine_QPointF_QPointF(QPainter* this_ptr, const QPointF* p1, const QPointF* p2) {
  this_ptr->drawLine(*p1, *p2);
}

void qt_gui_c_QPainter_drawLine_QPoint_QPoint(QPainter* this_ptr, const QPoint* p1, const QPoint* p2) {
  this_ptr->drawLine(*p1, *p2);
}

void qt_gui_c_QPainter_drawLine_int_int_int_int(QPainter* this_ptr, int x1, int y1, int x2, int y2) {
  this_ptr->drawLine(x1, y1, x2, y2);
}

void qt_gui_c_QPainter_drawLines_QLineF_int(QPainter* this_ptr, const QLineF* lines, int lineCount) {
  this_ptr->drawLines(lines, lineCount);
}

void qt_gui_c_QPainter_drawLines_QLine_int(QPainter* this_ptr, const QLine* lines, int lineCount) {
  this_ptr->drawLines(lines, lineCount);
}

void qt_gui_c_QPainter_drawLines_QPointF_int(QPainter* this_ptr, const QPointF* pointPairs, int lineCount) {
  this_ptr->drawLines(pointPairs, lineCount);
}

void qt_gui_c_QPainter_drawLines_QPoint_int(QPainter* this_ptr, const QPoint* pointPairs, int lineCount) {
  this_ptr->drawLines(pointPairs, lineCount);
}

void qt_gui_c_QPainter_drawLines_QVector_QLine(QPainter* this_ptr, const QVector< QLine >* lines) {
  this_ptr->drawLines(*lines);
}

void qt_gui_c_QPainter_drawLines_QVector_QLineF(QPainter* this_ptr, const QVector< QLineF >* lines) {
  this_ptr->drawLines(*lines);
}

void qt_gui_c_QPainter_drawLines_QVector_QPoint(QPainter* this_ptr, const QVector< QPoint >* pointPairs) {
  this_ptr->drawLines(*pointPairs);
}

void qt_gui_c_QPainter_drawLines_QVector_QPointF(QPainter* this_ptr, const QVector< QPointF >* pointPairs) {
  this_ptr->drawLines(*pointPairs);
}

void qt_gui_c_QPainter_drawPath(QPainter* this_ptr, const QPainterPath* path) {
  this_ptr->drawPath(*path);
}

void qt_gui_c_QPainter_drawPicture_QPointF_QPicture(QPainter* this_ptr, const QPointF* p, const QPicture* picture) {
  this_ptr->drawPicture(*p, *picture);
}

void qt_gui_c_QPainter_drawPicture_QPoint_QPicture(QPainter* this_ptr, const QPoint* p, const QPicture* picture) {
  this_ptr->drawPicture(*p, *picture);
}

void qt_gui_c_QPainter_drawPicture_int_int_QPicture(QPainter* this_ptr, int x, int y, const QPicture* picture) {
  this_ptr->drawPicture(x, y, *picture);
}

void qt_gui_c_QPainter_drawPie_arg1_a_alen(QPainter* this_ptr, const QRect* arg1, int a, int alen) {
  this_ptr->drawPie(*arg1, a, alen);
}

void qt_gui_c_QPainter_drawPie_rect_a_alen(QPainter* this_ptr, const QRectF* rect, int a, int alen) {
  this_ptr->drawPie(*rect, a, alen);
}

void qt_gui_c_QPainter_drawPie_x_y_w_h_a_alen(QPainter* this_ptr, int x, int y, int w, int h, int a, int alen) {
  this_ptr->drawPie(x, y, w, h, a, alen);
}

void qt_gui_c_QPainter_drawPixmapFragments_fragments_fragmentCount_pixmap(QPainter* this_ptr, const QPainter::PixmapFragment* fragments, int fragmentCount, const QPixmap* pixmap) {
  this_ptr->drawPixmapFragments(fragments, fragmentCount, *pixmap);
}

void qt_gui_c_QPainter_drawPixmapFragments_fragments_fragmentCount_pixmap_hints(QPainter* this_ptr, const QPainter::PixmapFragment* fragments, int fragmentCount, const QPixmap* pixmap, unsigned int hints) {
  this_ptr->drawPixmapFragments(fragments, fragmentCount, *pixmap, QFlags< QPainter::PixmapFragmentHint >(hints));
}

void qt_gui_c_QPainter_drawPixmap_QPointF_QPixmap(QPainter* this_ptr, const QPointF* p, const QPixmap* pm) {
  this_ptr->drawPixmap(*p, *pm);
}

void qt_gui_c_QPainter_drawPixmap_QPointF_QPixmap_QRectF(QPainter* this_ptr, const QPointF* p, const QPixmap* pm, const QRectF* sr) {
  this_ptr->drawPixmap(*p, *pm, *sr);
}

void qt_gui_c_QPainter_drawPixmap_QPoint_QPixmap(QPainter* this_ptr, const QPoint* p, const QPixmap* pm) {
  this_ptr->drawPixmap(*p, *pm);
}

void qt_gui_c_QPainter_drawPixmap_QPoint_QPixmap_QRect(QPainter* this_ptr, const QPoint* p, const QPixmap* pm, const QRect* sr) {
  this_ptr->drawPixmap(*p, *pm, *sr);
}

void qt_gui_c_QPainter_drawPixmap_QRectF_QPixmap_QRectF(QPainter* this_ptr, const QRectF* targetRect, const QPixmap* pixmap, const QRectF* sourceRect) {
  this_ptr->drawPixmap(*targetRect, *pixmap, *sourceRect);
}

void qt_gui_c_QPainter_drawPixmap_QRect_QPixmap(QPainter* this_ptr, const QRect* r, const QPixmap* pm) {
  this_ptr->drawPixmap(*r, *pm);
}

void qt_gui_c_QPainter_drawPixmap_QRect_QPixmap_QRect(QPainter* this_ptr, const QRect* targetRect, const QPixmap* pixmap, const QRect* sourceRect) {
  this_ptr->drawPixmap(*targetRect, *pixmap, *sourceRect);
}

void qt_gui_c_QPainter_drawPixmap_int_int_QPixmap(QPainter* this_ptr, int x, int y, const QPixmap* pm) {
  this_ptr->drawPixmap(x, y, *pm);
}

void qt_gui_c_QPainter_drawPixmap_int_int_QPixmap_int_int_int_int(QPainter* this_ptr, int x, int y, const QPixmap* pm, int sx, int sy, int sw, int sh) {
  this_ptr->drawPixmap(x, y, *pm, sx, sy, sw, sh);
}

void qt_gui_c_QPainter_drawPixmap_int_int_int_int_QPixmap(QPainter* this_ptr, int x, int y, int w, int h, const QPixmap* pm) {
  this_ptr->drawPixmap(x, y, w, h, *pm);
}

void qt_gui_c_QPainter_drawPixmap_int_int_int_int_QPixmap_int_int_int_int(QPainter* this_ptr, int x, int y, int w, int h, const QPixmap* pm, int sx, int sy, int sw, int sh) {
  this_ptr->drawPixmap(x, y, w, h, *pm, sx, sy, sw, sh);
}

void qt_gui_c_QPainter_drawPoint_p(QPainter* this_ptr, const QPoint* p) {
  this_ptr->drawPoint(*p);
}

void qt_gui_c_QPainter_drawPoint_pt(QPainter* this_ptr, const QPointF* pt) {
  this_ptr->drawPoint(*pt);
}

void qt_gui_c_QPainter_drawPoint_x_y(QPainter* this_ptr, int x, int y) {
  this_ptr->drawPoint(x, y);
}

void qt_gui_c_QPainter_drawPoints_QPointF_int(QPainter* this_ptr, const QPointF* points, int pointCount) {
  this_ptr->drawPoints(points, pointCount);
}

void qt_gui_c_QPainter_drawPoints_QPoint_int(QPainter* this_ptr, const QPoint* points, int pointCount) {
  this_ptr->drawPoints(points, pointCount);
}

void qt_gui_c_QPainter_drawPoints_QPolygon(QPainter* this_ptr, const QPolygon* points) {
  this_ptr->drawPoints(*points);
}

void qt_gui_c_QPainter_drawPoints_QPolygonF(QPainter* this_ptr, const QPolygonF* points) {
  this_ptr->drawPoints(*points);
}

void qt_gui_c_QPainter_drawPolygon_QPointF_int(QPainter* this_ptr, const QPointF* points, int pointCount) {
  this_ptr->drawPolygon(points, pointCount);
}

void qt_gui_c_QPainter_drawPolygon_QPointF_int_Qt_FillRule(QPainter* this_ptr, const QPointF* points, int pointCount, const Qt::FillRule* fillRule) {
  this_ptr->drawPolygon(points, pointCount, *fillRule);
}

void qt_gui_c_QPainter_drawPolygon_QPoint_int(QPainter* this_ptr, const QPoint* points, int pointCount) {
  this_ptr->drawPolygon(points, pointCount);
}

void qt_gui_c_QPainter_drawPolygon_QPoint_int_Qt_FillRule(QPainter* this_ptr, const QPoint* points, int pointCount, const Qt::FillRule* fillRule) {
  this_ptr->drawPolygon(points, pointCount, *fillRule);
}

void qt_gui_c_QPainter_drawPolygon_QPolygon(QPainter* this_ptr, const QPolygon* polygon) {
  this_ptr->drawPolygon(*polygon);
}

void qt_gui_c_QPainter_drawPolygon_QPolygonF(QPainter* this_ptr, const QPolygonF* polygon) {
  this_ptr->drawPolygon(*polygon);
}

void qt_gui_c_QPainter_drawPolygon_QPolygonF_Qt_FillRule(QPainter* this_ptr, const QPolygonF* polygon, const Qt::FillRule* fillRule) {
  this_ptr->drawPolygon(*polygon, *fillRule);
}

void qt_gui_c_QPainter_drawPolygon_QPolygon_Qt_FillRule(QPainter* this_ptr, const QPolygon* polygon, const Qt::FillRule* fillRule) {
  this_ptr->drawPolygon(*polygon, *fillRule);
}

void qt_gui_c_QPainter_drawPolyline_QPointF_int(QPainter* this_ptr, const QPointF* points, int pointCount) {
  this_ptr->drawPolyline(points, pointCount);
}

void qt_gui_c_QPainter_drawPolyline_QPoint_int(QPainter* this_ptr, const QPoint* points, int pointCount) {
  this_ptr->drawPolyline(points, pointCount);
}

void qt_gui_c_QPainter_drawPolyline_QPolygon(QPainter* this_ptr, const QPolygon* polygon) {
  this_ptr->drawPolyline(*polygon);
}

void qt_gui_c_QPainter_drawPolyline_QPolygonF(QPainter* this_ptr, const QPolygonF* polyline) {
  this_ptr->drawPolyline(*polyline);
}

void qt_gui_c_QPainter_drawRect_QRect(QPainter* this_ptr, const QRect* rect) {
  this_ptr->drawRect(*rect);
}

void qt_gui_c_QPainter_drawRect_QRectF(QPainter* this_ptr, const QRectF* rect) {
  this_ptr->drawRect(*rect);
}

void qt_gui_c_QPainter_drawRect_int_int_int_int(QPainter* this_ptr, int x1, int y1, int w, int h) {
  this_ptr->drawRect(x1, y1, w, h);
}

void qt_gui_c_QPainter_drawRects_QRectF_int(QPainter* this_ptr, const QRectF* rects, int rectCount) {
  this_ptr->drawRects(rects, rectCount);
}

void qt_gui_c_QPainter_drawRects_QRect_int(QPainter* this_ptr, const QRect* rects, int rectCount) {
  this_ptr->drawRects(rects, rectCount);
}

void qt_gui_c_QPainter_drawRects_QVector_QRect(QPainter* this_ptr, const QVector< QRect >* rectangles) {
  this_ptr->drawRects(*rectangles);
}

void qt_gui_c_QPainter_drawRects_QVector_QRectF(QPainter* this_ptr, const QVector< QRectF >* rectangles) {
  this_ptr->drawRects(*rectangles);
}

void qt_gui_c_QPainter_drawRoundRect_QRect(QPainter* this_ptr, const QRect* r) {
  this_ptr->drawRoundRect(*r);
}

void qt_gui_c_QPainter_drawRoundRect_QRectF(QPainter* this_ptr, const QRectF* r) {
  this_ptr->drawRoundRect(*r);
}

void qt_gui_c_QPainter_drawRoundRect_QRectF_int(QPainter* this_ptr, const QRectF* r, int xround) {
  this_ptr->drawRoundRect(*r, xround);
}

void qt_gui_c_QPainter_drawRoundRect_QRectF_int_int(QPainter* this_ptr, const QRectF* r, int xround, int yround) {
  this_ptr->drawRoundRect(*r, xround, yround);
}

void qt_gui_c_QPainter_drawRoundRect_QRect_int(QPainter* this_ptr, const QRect* r, int xround) {
  this_ptr->drawRoundRect(*r, xround);
}

void qt_gui_c_QPainter_drawRoundRect_QRect_int_int(QPainter* this_ptr, const QRect* r, int xround, int yround) {
  this_ptr->drawRoundRect(*r, xround, yround);
}

void qt_gui_c_QPainter_drawRoundRect_int_int_int_int(QPainter* this_ptr, int x, int y, int w, int h) {
  this_ptr->drawRoundRect(x, y, w, h);
}

void qt_gui_c_QPainter_drawRoundRect_int_int_int_int_int(QPainter* this_ptr, int x, int y, int w, int h, int arg5) {
  this_ptr->drawRoundRect(x, y, w, h, arg5);
}

void qt_gui_c_QPainter_drawRoundRect_int_int_int_int_int_int(QPainter* this_ptr, int x, int y, int w, int h, int arg5, int arg6) {
  this_ptr->drawRoundRect(x, y, w, h, arg5, arg6);
}

void qt_gui_c_QPainter_drawRoundedRect_QRectF_double_double(QPainter* this_ptr, const QRectF* rect, double xRadius, double yRadius) {
  this_ptr->drawRoundedRect(*rect, xRadius, yRadius);
}

void qt_gui_c_QPainter_drawRoundedRect_QRectF_double_double_Qt_SizeMode(QPainter* this_ptr, const QRectF* rect, double xRadius, double yRadius, const Qt::SizeMode* mode) {
  this_ptr->drawRoundedRect(*rect, xRadius, yRadius, *mode);
}

void qt_gui_c_QPainter_drawRoundedRect_QRect_double_double(QPainter* this_ptr, const QRect* rect, double xRadius, double yRadius) {
  this_ptr->drawRoundedRect(*rect, xRadius, yRadius);
}

void qt_gui_c_QPainter_drawRoundedRect_QRect_double_double_Qt_SizeMode(QPainter* this_ptr, const QRect* rect, double xRadius, double yRadius, const Qt::SizeMode* mode) {
  this_ptr->drawRoundedRect(*rect, xRadius, yRadius, *mode);
}

void qt_gui_c_QPainter_drawRoundedRect_int_int_int_int_double_double(QPainter* this_ptr, int x, int y, int w, int h, double xRadius, double yRadius) {
  this_ptr->drawRoundedRect(x, y, w, h, xRadius, yRadius);
}

void qt_gui_c_QPainter_drawRoundedRect_int_int_int_int_double_double_Qt_SizeMode(QPainter* this_ptr, int x, int y, int w, int h, double xRadius, double yRadius, const Qt::SizeMode* mode) {
  this_ptr->drawRoundedRect(x, y, w, h, xRadius, yRadius, *mode);
}

void qt_gui_c_QPainter_drawStaticText_QPointF_QStaticText(QPainter* this_ptr, const QPointF* topLeftPosition, const QStaticText* staticText) {
  this_ptr->drawStaticText(*topLeftPosition, *staticText);
}

void qt_gui_c_QPainter_drawStaticText_QPoint_QStaticText(QPainter* this_ptr, const QPoint* topLeftPosition, const QStaticText* staticText) {
  this_ptr->drawStaticText(*topLeftPosition, *staticText);
}

void qt_gui_c_QPainter_drawStaticText_int_int_QStaticText(QPainter* this_ptr, int left, int top, const QStaticText* staticText) {
  this_ptr->drawStaticText(left, top, *staticText);
}

void qt_gui_c_QPainter_drawTextItem_QPointF_QTextItem(QPainter* this_ptr, const QPointF* p, const QTextItem* ti) {
  this_ptr->drawTextItem(*p, *ti);
}

void qt_gui_c_QPainter_drawTextItem_QPoint_QTextItem(QPainter* this_ptr, const QPoint* p, const QTextItem* ti) {
  this_ptr->drawTextItem(*p, *ti);
}

void qt_gui_c_QPainter_drawTextItem_int_int_QTextItem(QPainter* this_ptr, int x, int y, const QTextItem* ti) {
  this_ptr->drawTextItem(x, y, *ti);
}

void qt_gui_c_QPainter_drawText_QPointF_QString(QPainter* this_ptr, const QPointF* p, const QString* s) {
  this_ptr->drawText(*p, *s);
}

void qt_gui_c_QPainter_drawText_QPointF_QString_int_int(QPainter* this_ptr, const QPointF* p, const QString* str, int tf, int justificationPadding) {
  this_ptr->drawText(*p, *str, tf, justificationPadding);
}

void qt_gui_c_QPainter_drawText_QPoint_QString(QPainter* this_ptr, const QPoint* p, const QString* s) {
  this_ptr->drawText(*p, *s);
}

void qt_gui_c_QPainter_drawText_QRectF_QString(QPainter* this_ptr, const QRectF* r, const QString* text) {
  this_ptr->drawText(*r, *text);
}

void qt_gui_c_QPainter_drawText_QRectF_QString_QTextOption(QPainter* this_ptr, const QRectF* r, const QString* text, const QTextOption* o) {
  this_ptr->drawText(*r, *text, *o);
}

void qt_gui_c_QPainter_drawText_QRectF_int_QString(QPainter* this_ptr, const QRectF* r, int flags, const QString* text) {
  this_ptr->drawText(*r, flags, *text);
}

void qt_gui_c_QPainter_drawText_QRectF_int_QString_QRectF(QPainter* this_ptr, const QRectF* r, int flags, const QString* text, QRectF* br) {
  this_ptr->drawText(*r, flags, *text, br);
}

void qt_gui_c_QPainter_drawText_QRect_int_QString(QPainter* this_ptr, const QRect* r, int flags, const QString* text) {
  this_ptr->drawText(*r, flags, *text);
}

void qt_gui_c_QPainter_drawText_QRect_int_QString_QRect(QPainter* this_ptr, const QRect* r, int flags, const QString* text, QRect* br) {
  this_ptr->drawText(*r, flags, *text, br);
}

void qt_gui_c_QPainter_drawText_int_int_QString(QPainter* this_ptr, int x, int y, const QString* s) {
  this_ptr->drawText(x, y, *s);
}

void qt_gui_c_QPainter_drawText_int_int_int_int_int_QString(QPainter* this_ptr, int x, int y, int w, int h, int flags, const QString* text) {
  this_ptr->drawText(x, y, w, h, flags, *text);
}

void qt_gui_c_QPainter_drawText_int_int_int_int_int_QString_QRect(QPainter* this_ptr, int x, int y, int w, int h, int flags, const QString* text, QRect* br) {
  this_ptr->drawText(x, y, w, h, flags, *text, br);
}

void qt_gui_c_QPainter_drawTiledPixmap_arg1_arg2(QPainter* this_ptr, const QRect* arg1, const QPixmap* arg2) {
  this_ptr->drawTiledPixmap(*arg1, *arg2);
}

void qt_gui_c_QPainter_drawTiledPixmap_arg1_arg2_arg3(QPainter* this_ptr, const QRect* arg1, const QPixmap* arg2, const QPoint* arg3) {
  this_ptr->drawTiledPixmap(*arg1, *arg2, *arg3);
}

void qt_gui_c_QPainter_drawTiledPixmap_rect_pm(QPainter* this_ptr, const QRectF* rect, const QPixmap* pm) {
  this_ptr->drawTiledPixmap(*rect, *pm);
}

void qt_gui_c_QPainter_drawTiledPixmap_rect_pm_offset(QPainter* this_ptr, const QRectF* rect, const QPixmap* pm, const QPointF* offset) {
  this_ptr->drawTiledPixmap(*rect, *pm, *offset);
}

void qt_gui_c_QPainter_drawTiledPixmap_x_y_w_h_arg5(QPainter* this_ptr, int x, int y, int w, int h, const QPixmap* arg5) {
  this_ptr->drawTiledPixmap(x, y, w, h, *arg5);
}

void qt_gui_c_QPainter_drawTiledPixmap_x_y_w_h_arg5_sx(QPainter* this_ptr, int x, int y, int w, int h, const QPixmap* arg5, int sx) {
  this_ptr->drawTiledPixmap(x, y, w, h, *arg5, sx);
}

void qt_gui_c_QPainter_drawTiledPixmap_x_y_w_h_arg5_sx_sy(QPainter* this_ptr, int x, int y, int w, int h, const QPixmap* arg5, int sx, int sy) {
  this_ptr->drawTiledPixmap(x, y, w, h, *arg5, sx, sy);
}

bool qt_gui_c_QPainter_end(QPainter* this_ptr) {
  return this_ptr->end();
}

void qt_gui_c_QPainter_endNativePainting(QPainter* this_ptr) {
  this_ptr->endNativePainting();
}

void qt_gui_c_QPainter_eraseRect_QRect(QPainter* this_ptr, const QRect* arg1) {
  this_ptr->eraseRect(*arg1);
}

void qt_gui_c_QPainter_eraseRect_QRectF(QPainter* this_ptr, const QRectF* arg1) {
  this_ptr->eraseRect(*arg1);
}

void qt_gui_c_QPainter_eraseRect_int_int_int_int(QPainter* this_ptr, int x, int y, int w, int h) {
  this_ptr->eraseRect(x, y, w, h);
}

void qt_gui_c_QPainter_fillPath(QPainter* this_ptr, const QPainterPath* path, const QBrush* brush) {
  this_ptr->fillPath(*path, *brush);
}

void qt_gui_c_QPainter_fillRect_QRectF_QBrush(QPainter* this_ptr, const QRectF* arg1, const QBrush* arg2) {
  this_ptr->fillRect(*arg1, *arg2);
}

void qt_gui_c_QPainter_fillRect_QRectF_QColor(QPainter* this_ptr, const QRectF* arg1, const QColor* color) {
  this_ptr->fillRect(*arg1, *color);
}

void qt_gui_c_QPainter_fillRect_QRectF_Qt_BrushStyle(QPainter* this_ptr, const QRectF* r, const Qt::BrushStyle* style) {
  this_ptr->fillRect(*r, *style);
}

void qt_gui_c_QPainter_fillRect_QRectF_Qt_GlobalColor(QPainter* this_ptr, const QRectF* r, const Qt::GlobalColor* c) {
  this_ptr->fillRect(*r, *c);
}

void qt_gui_c_QPainter_fillRect_QRect_QBrush(QPainter* this_ptr, const QRect* arg1, const QBrush* arg2) {
  this_ptr->fillRect(*arg1, *arg2);
}

void qt_gui_c_QPainter_fillRect_QRect_QColor(QPainter* this_ptr, const QRect* arg1, const QColor* color) {
  this_ptr->fillRect(*arg1, *color);
}

void qt_gui_c_QPainter_fillRect_QRect_Qt_BrushStyle(QPainter* this_ptr, const QRect* r, const Qt::BrushStyle* style) {
  this_ptr->fillRect(*r, *style);
}

void qt_gui_c_QPainter_fillRect_QRect_Qt_GlobalColor(QPainter* this_ptr, const QRect* r, const Qt::GlobalColor* c) {
  this_ptr->fillRect(*r, *c);
}

void qt_gui_c_QPainter_fillRect_int_int_int_int_QBrush(QPainter* this_ptr, int x, int y, int w, int h, const QBrush* arg5) {
  this_ptr->fillRect(x, y, w, h, *arg5);
}

void qt_gui_c_QPainter_fillRect_int_int_int_int_QColor(QPainter* this_ptr, int x, int y, int w, int h, const QColor* color) {
  this_ptr->fillRect(x, y, w, h, *color);
}

void qt_gui_c_QPainter_fillRect_int_int_int_int_Qt_BrushStyle(QPainter* this_ptr, int x, int y, int w, int h, const Qt::BrushStyle* style) {
  this_ptr->fillRect(x, y, w, h, *style);
}

void qt_gui_c_QPainter_fillRect_int_int_int_int_Qt_GlobalColor(QPainter* this_ptr, int x, int y, int w, int h, const Qt::GlobalColor* c) {
  this_ptr->fillRect(x, y, w, h, *c);
}

const QFont* qt_gui_c_QPainter_font(const QPainter* this_ptr) {
  return &this_ptr->font();
}

void qt_gui_c_QPainter_fontInfo_to_output(const QPainter* this_ptr, QFontInfo* output) {
  new(output) QFontInfo(this_ptr->fontInfo());
}

void qt_gui_c_QPainter_fontMetrics_to_output(const QPainter* this_ptr, QFontMetrics* output) {
  new(output) QFontMetrics(this_ptr->fontMetrics());
}

bool qt_gui_c_QPainter_hasClipping(const QPainter* this_ptr) {
  return this_ptr->hasClipping();
}

void qt_gui_c_QPainter_initFrom(QPainter* this_ptr, const QPaintDevice* device) {
  this_ptr->initFrom(device);
}

bool qt_gui_c_QPainter_isActive(const QPainter* this_ptr) {
  return this_ptr->isActive();
}

const QMatrix* qt_gui_c_QPainter_matrix(const QPainter* this_ptr) {
  return &this_ptr->matrix();
}

bool qt_gui_c_QPainter_matrixEnabled(const QPainter* this_ptr) {
  return this_ptr->matrixEnabled();
}

QPainter* qt_gui_c_QPainter_new_arg1(QPaintDevice* arg1) {
  return new QPainter(arg1);
}

QPainter* qt_gui_c_QPainter_new_no_args() {
  return new QPainter();
}

double qt_gui_c_QPainter_opacity(const QPainter* this_ptr) {
  return this_ptr->opacity();
}

QPaintEngine* qt_gui_c_QPainter_paintEngine(const QPainter* this_ptr) {
  return this_ptr->paintEngine();
}

const QPen* qt_gui_c_QPainter_pen(const QPainter* this_ptr) {
  return &this_ptr->pen();
}

QPaintDevice* qt_gui_c_QPainter_redirected_device(const QPaintDevice* device) {
  return QPainter::redirected(device);
}

QPaintDevice* qt_gui_c_QPainter_redirected_device_offset(const QPaintDevice* device, QPoint* offset) {
  return QPainter::redirected(device, offset);
}

unsigned int qt_gui_c_QPainter_renderHints(const QPainter* this_ptr) {
  return uint(this_ptr->renderHints());
}

void qt_gui_c_QPainter_resetMatrix(QPainter* this_ptr) {
  this_ptr->resetMatrix();
}

void qt_gui_c_QPainter_resetTransform(QPainter* this_ptr) {
  this_ptr->resetTransform();
}

void qt_gui_c_QPainter_restore(QPainter* this_ptr) {
  this_ptr->restore();
}

void qt_gui_c_QPainter_restoreRedirected(const QPaintDevice* device) {
  QPainter::restoreRedirected(device);
}

void qt_gui_c_QPainter_rotate(QPainter* this_ptr, double a) {
  this_ptr->rotate(a);
}

void qt_gui_c_QPainter_save(QPainter* this_ptr) {
  this_ptr->save();
}

void qt_gui_c_QPainter_scale(QPainter* this_ptr, double sx, double sy) {
  this_ptr->scale(sx, sy);
}

void qt_gui_c_QPainter_setBackground(QPainter* this_ptr, const QBrush* bg) {
  this_ptr->setBackground(*bg);
}

void qt_gui_c_QPainter_setBackgroundMode(QPainter* this_ptr, const Qt::BGMode* mode) {
  this_ptr->setBackgroundMode(*mode);
}

void qt_gui_c_QPainter_setBrushOrigin_QPoint(QPainter* this_ptr, const QPoint* arg1) {
  this_ptr->setBrushOrigin(*arg1);
}

void qt_gui_c_QPainter_setBrushOrigin_QPointF(QPainter* this_ptr, const QPointF* arg1) {
  this_ptr->setBrushOrigin(*arg1);
}

void qt_gui_c_QPainter_setBrushOrigin_int_int(QPainter* this_ptr, int x, int y) {
  this_ptr->setBrushOrigin(x, y);
}

void qt_gui_c_QPainter_setBrush_brush(QPainter* this_ptr, const QBrush* brush) {
  this_ptr->setBrush(*brush);
}

void qt_gui_c_QPainter_setBrush_style(QPainter* this_ptr, const Qt::BrushStyle* style) {
  this_ptr->setBrush(*style);
}

void qt_gui_c_QPainter_setClipPath_path(QPainter* this_ptr, const QPainterPath* path) {
  this_ptr->setClipPath(*path);
}

void qt_gui_c_QPainter_setClipPath_path_op(QPainter* this_ptr, const QPainterPath* path, const Qt::ClipOperation* op) {
  this_ptr->setClipPath(*path, *op);
}

void qt_gui_c_QPainter_setClipRect_QRect(QPainter* this_ptr, const QRect* arg1) {
  this_ptr->setClipRect(*arg1);
}

void qt_gui_c_QPainter_setClipRect_QRectF(QPainter* this_ptr, const QRectF* arg1) {
  this_ptr->setClipRect(*arg1);
}

void qt_gui_c_QPainter_setClipRect_QRectF_Qt_ClipOperation(QPainter* this_ptr, const QRectF* arg1, const Qt::ClipOperation* op) {
  this_ptr->setClipRect(*arg1, *op);
}

void qt_gui_c_QPainter_setClipRect_QRect_Qt_ClipOperation(QPainter* this_ptr, const QRect* arg1, const Qt::ClipOperation* op) {
  this_ptr->setClipRect(*arg1, *op);
}

void qt_gui_c_QPainter_setClipRect_int_int_int_int(QPainter* this_ptr, int x, int y, int w, int h) {
  this_ptr->setClipRect(x, y, w, h);
}

void qt_gui_c_QPainter_setClipRect_int_int_int_int_Qt_ClipOperation(QPainter* this_ptr, int x, int y, int w, int h, const Qt::ClipOperation* op) {
  this_ptr->setClipRect(x, y, w, h, *op);
}

void qt_gui_c_QPainter_setClipRegion_arg1(QPainter* this_ptr, const QRegion* arg1) {
  this_ptr->setClipRegion(*arg1);
}

void qt_gui_c_QPainter_setClipRegion_arg1_op(QPainter* this_ptr, const QRegion* arg1, const Qt::ClipOperation* op) {
  this_ptr->setClipRegion(*arg1, *op);
}

void qt_gui_c_QPainter_setClipping(QPainter* this_ptr, bool enable) {
  this_ptr->setClipping(enable);
}

void qt_gui_c_QPainter_setCompositionMode(QPainter* this_ptr, QPainter::CompositionMode mode) {
  this_ptr->setCompositionMode(mode);
}

void qt_gui_c_QPainter_setFont(QPainter* this_ptr, const QFont* f) {
  this_ptr->setFont(*f);
}

void qt_gui_c_QPainter_setLayoutDirection(QPainter* this_ptr, const Qt::LayoutDirection* direction) {
  this_ptr->setLayoutDirection(*direction);
}

void qt_gui_c_QPainter_setMatrixEnabled(QPainter* this_ptr, bool enabled) {
  this_ptr->setMatrixEnabled(enabled);
}

void qt_gui_c_QPainter_setMatrix_matrix(QPainter* this_ptr, const QMatrix* matrix) {
  this_ptr->setMatrix(*matrix);
}

void qt_gui_c_QPainter_setMatrix_matrix_combine(QPainter* this_ptr, const QMatrix* matrix, bool combine) {
  this_ptr->setMatrix(*matrix, combine);
}

void qt_gui_c_QPainter_setOpacity(QPainter* this_ptr, double opacity) {
  this_ptr->setOpacity(opacity);
}

void qt_gui_c_QPainter_setPen_color(QPainter* this_ptr, const QColor* color) {
  this_ptr->setPen(*color);
}

void qt_gui_c_QPainter_setPen_pen(QPainter* this_ptr, const QPen* pen) {
  this_ptr->setPen(*pen);
}

void qt_gui_c_QPainter_setPen_style(QPainter* this_ptr, const Qt::PenStyle* style) {
  this_ptr->setPen(*style);
}

void qt_gui_c_QPainter_setRedirected_device_replacement(const QPaintDevice* device, QPaintDevice* replacement) {
  QPainter::setRedirected(device, replacement);
}

void qt_gui_c_QPainter_setRedirected_device_replacement_offset(const QPaintDevice* device, QPaintDevice* replacement, const QPoint* offset) {
  QPainter::setRedirected(device, replacement, *offset);
}

void qt_gui_c_QPainter_setRenderHint_hint(QPainter* this_ptr, QPainter::RenderHint hint) {
  this_ptr->setRenderHint(hint);
}

void qt_gui_c_QPainter_setRenderHint_hint_on(QPainter* this_ptr, QPainter::RenderHint hint, bool on) {
  this_ptr->setRenderHint(hint, on);
}

void qt_gui_c_QPainter_setRenderHints_hints(QPainter* this_ptr, unsigned int hints) {
  this_ptr->setRenderHints(QFlags< QPainter::RenderHint >(hints));
}

void qt_gui_c_QPainter_setRenderHints_hints_on(QPainter* this_ptr, unsigned int hints, bool on) {
  this_ptr->setRenderHints(QFlags< QPainter::RenderHint >(hints), on);
}

void qt_gui_c_QPainter_setTransform_transform(QPainter* this_ptr, const QTransform* transform) {
  this_ptr->setTransform(*transform);
}

void qt_gui_c_QPainter_setTransform_transform_combine(QPainter* this_ptr, const QTransform* transform, bool combine) {
  this_ptr->setTransform(*transform, combine);
}

void qt_gui_c_QPainter_setViewTransformEnabled(QPainter* this_ptr, bool enable) {
  this_ptr->setViewTransformEnabled(enable);
}

void qt_gui_c_QPainter_setViewport_viewport(QPainter* this_ptr, const QRect* viewport) {
  this_ptr->setViewport(*viewport);
}

void qt_gui_c_QPainter_setViewport_x_y_w_h(QPainter* this_ptr, int x, int y, int w, int h) {
  this_ptr->setViewport(x, y, w, h);
}

void qt_gui_c_QPainter_setWindow_window(QPainter* this_ptr, const QRect* window) {
  this_ptr->setWindow(*window);
}

void qt_gui_c_QPainter_setWindow_x_y_w_h(QPainter* this_ptr, int x, int y, int w, int h) {
  this_ptr->setWindow(x, y, w, h);
}

void qt_gui_c_QPainter_setWorldMatrixEnabled(QPainter* this_ptr, bool enabled) {
  this_ptr->setWorldMatrixEnabled(enabled);
}

void qt_gui_c_QPainter_setWorldMatrix_matrix(QPainter* this_ptr, const QMatrix* matrix) {
  this_ptr->setWorldMatrix(*matrix);
}

void qt_gui_c_QPainter_setWorldMatrix_matrix_combine(QPainter* this_ptr, const QMatrix* matrix, bool combine) {
  this_ptr->setWorldMatrix(*matrix, combine);
}

void qt_gui_c_QPainter_setWorldTransform_matrix(QPainter* this_ptr, const QTransform* matrix) {
  this_ptr->setWorldTransform(*matrix);
}

void qt_gui_c_QPainter_setWorldTransform_matrix_combine(QPainter* this_ptr, const QTransform* matrix, bool combine) {
  this_ptr->setWorldTransform(*matrix, combine);
}

void qt_gui_c_QPainter_shear(QPainter* this_ptr, double sh, double sv) {
  this_ptr->shear(sh, sv);
}

void qt_gui_c_QPainter_strokePath(QPainter* this_ptr, const QPainterPath* path, const QPen* pen) {
  this_ptr->strokePath(*path, *pen);
}

bool qt_gui_c_QPainter_testRenderHint(const QPainter* this_ptr, QPainter::RenderHint hint) {
  return this_ptr->testRenderHint(hint);
}

const QTransform* qt_gui_c_QPainter_transform(const QPainter* this_ptr) {
  return &this_ptr->transform();
}

void qt_gui_c_QPainter_translate_QPoint(QPainter* this_ptr, const QPoint* offset) {
  this_ptr->translate(*offset);
}

void qt_gui_c_QPainter_translate_QPointF(QPainter* this_ptr, const QPointF* offset) {
  this_ptr->translate(*offset);
}

void qt_gui_c_QPainter_translate_double_double(QPainter* this_ptr, double dx, double dy) {
  this_ptr->translate(dx, dy);
}

bool qt_gui_c_QPainter_viewTransformEnabled(const QPainter* this_ptr) {
  return this_ptr->viewTransformEnabled();
}

void qt_gui_c_QPainter_viewport_to_output(const QPainter* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->viewport());
}

void qt_gui_c_QPainter_window_to_output(const QPainter* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->window());
}

const QMatrix* qt_gui_c_QPainter_worldMatrix(const QPainter* this_ptr) {
  return &this_ptr->worldMatrix();
}

bool qt_gui_c_QPainter_worldMatrixEnabled(const QPainter* this_ptr) {
  return this_ptr->worldMatrixEnabled();
}

const QTransform* qt_gui_c_QPainter_worldTransform(const QPainter* this_ptr) {
  return &this_ptr->worldTransform();
}

