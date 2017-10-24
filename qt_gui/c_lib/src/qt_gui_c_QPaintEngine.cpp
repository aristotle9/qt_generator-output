#include "qt_gui_c_QPaintEngine.h"

bool qt_gui_c_QPaintEngine_begin(QPaintEngine* this_ptr, QPaintDevice* pdev) {
  return this_ptr->begin(pdev);
}

void qt_gui_c_QPaintEngine_clearDirty(QPaintEngine* this_ptr, unsigned int df) {
  this_ptr->clearDirty(QFlags< QPaintEngine::DirtyFlag >(df));
}

void qt_gui_c_QPaintEngine_coordinateOffset_to_output(const QPaintEngine* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->coordinateOffset());
}

void qt_gui_c_QPaintEngine_delete(QPaintEngine* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QPaintEngine_drawEllipse_QRect(QPaintEngine* this_ptr, const QRect* r) {
  this_ptr->drawEllipse(*r);
}

void qt_gui_c_QPaintEngine_drawEllipse_QRectF(QPaintEngine* this_ptr, const QRectF* r) {
  this_ptr->drawEllipse(*r);
}

void qt_gui_c_QPaintEngine_drawLines_QLineF_int(QPaintEngine* this_ptr, const QLineF* lines, int lineCount) {
  this_ptr->drawLines(lines, lineCount);
}

void qt_gui_c_QPaintEngine_drawLines_QLine_int(QPaintEngine* this_ptr, const QLine* lines, int lineCount) {
  this_ptr->drawLines(lines, lineCount);
}

void qt_gui_c_QPaintEngine_drawPath(QPaintEngine* this_ptr, const QPainterPath* path) {
  this_ptr->drawPath(*path);
}

void qt_gui_c_QPaintEngine_drawPixmap(QPaintEngine* this_ptr, const QRectF* r, const QPixmap* pm, const QRectF* sr) {
  this_ptr->drawPixmap(*r, *pm, *sr);
}

void qt_gui_c_QPaintEngine_drawPoints_QPointF_int(QPaintEngine* this_ptr, const QPointF* points, int pointCount) {
  this_ptr->drawPoints(points, pointCount);
}

void qt_gui_c_QPaintEngine_drawPoints_QPoint_int(QPaintEngine* this_ptr, const QPoint* points, int pointCount) {
  this_ptr->drawPoints(points, pointCount);
}

void qt_gui_c_QPaintEngine_drawPolygon_QPointF_int_QPaintEngine_PolygonDrawMode(QPaintEngine* this_ptr, const QPointF* points, int pointCount, QPaintEngine::PolygonDrawMode mode) {
  this_ptr->drawPolygon(points, pointCount, mode);
}

void qt_gui_c_QPaintEngine_drawPolygon_QPoint_int_QPaintEngine_PolygonDrawMode(QPaintEngine* this_ptr, const QPoint* points, int pointCount, QPaintEngine::PolygonDrawMode mode) {
  this_ptr->drawPolygon(points, pointCount, mode);
}

void qt_gui_c_QPaintEngine_drawRects_QRectF_int(QPaintEngine* this_ptr, const QRectF* rects, int rectCount) {
  this_ptr->drawRects(rects, rectCount);
}

void qt_gui_c_QPaintEngine_drawRects_QRect_int(QPaintEngine* this_ptr, const QRect* rects, int rectCount) {
  this_ptr->drawRects(rects, rectCount);
}

void qt_gui_c_QPaintEngine_drawTextItem(QPaintEngine* this_ptr, const QPointF* p, const QTextItem* textItem) {
  this_ptr->drawTextItem(*p, *textItem);
}

void qt_gui_c_QPaintEngine_drawTiledPixmap(QPaintEngine* this_ptr, const QRectF* r, const QPixmap* pixmap, const QPointF* s) {
  this_ptr->drawTiledPixmap(*r, *pixmap, *s);
}

bool qt_gui_c_QPaintEngine_end(QPaintEngine* this_ptr) {
  return this_ptr->end();
}

void qt_gui_c_QPaintEngine_fix_neg_rect(QPaintEngine* this_ptr, int* x, int* y, int* w, int* h) {
  this_ptr->fix_neg_rect(x, y, w, h);
}

bool qt_gui_c_QPaintEngine_hasFeature(const QPaintEngine* this_ptr, unsigned int feature) {
  return this_ptr->hasFeature(QFlags< QPaintEngine::PaintEngineFeature >(feature));
}

bool qt_gui_c_QPaintEngine_isActive(const QPaintEngine* this_ptr) {
  return this_ptr->isActive();
}

bool qt_gui_c_QPaintEngine_isExtended(const QPaintEngine* this_ptr) {
  return this_ptr->isExtended();
}

QPaintDevice* qt_gui_c_QPaintEngine_paintDevice(const QPaintEngine* this_ptr) {
  return this_ptr->paintDevice();
}

QPainter* qt_gui_c_QPaintEngine_painter(const QPaintEngine* this_ptr) {
  return this_ptr->painter();
}

void qt_gui_c_QPaintEngine_setActive(QPaintEngine* this_ptr, bool newState) {
  this_ptr->setActive(newState);
}

void qt_gui_c_QPaintEngine_setDirty(QPaintEngine* this_ptr, unsigned int df) {
  this_ptr->setDirty(QFlags< QPaintEngine::DirtyFlag >(df));
}

void qt_gui_c_QPaintEngine_setPaintDevice(QPaintEngine* this_ptr, QPaintDevice* device) {
  this_ptr->setPaintDevice(device);
}

void qt_gui_c_QPaintEngine_setSystemClip(QPaintEngine* this_ptr, const QRegion* baseClip) {
  this_ptr->setSystemClip(*baseClip);
}

void qt_gui_c_QPaintEngine_setSystemRect(QPaintEngine* this_ptr, const QRect* rect) {
  this_ptr->setSystemRect(*rect);
}

void qt_gui_c_QPaintEngine_syncState(QPaintEngine* this_ptr) {
  this_ptr->syncState();
}

QRegion* qt_gui_c_QPaintEngine_systemClip_as_ptr(const QPaintEngine* this_ptr) {
  return new QRegion(this_ptr->systemClip());
}

void qt_gui_c_QPaintEngine_systemRect_to_output(const QPaintEngine* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->systemRect());
}

bool qt_gui_c_QPaintEngine_testDirty(QPaintEngine* this_ptr, unsigned int df) {
  return this_ptr->testDirty(QFlags< QPaintEngine::DirtyFlag >(df));
}

QPaintEngine::Type qt_gui_c_QPaintEngine_type(const QPaintEngine* this_ptr) {
  return this_ptr->type();
}

void qt_gui_c_QPaintEngine_updateState(QPaintEngine* this_ptr, const QPaintEngineState* state) {
  this_ptr->updateState(*state);
}

