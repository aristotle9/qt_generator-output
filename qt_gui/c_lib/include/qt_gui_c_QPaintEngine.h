#ifndef QT_GUI_C_QPAINTENGINE_H
#define QT_GUI_C_QPAINTENGINE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT bool qt_gui_c_QPaintEngine_begin(QPaintEngine* this_ptr, QPaintDevice* pdev);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_clearDirty(QPaintEngine* this_ptr, unsigned int df);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_coordinateOffset_to_output(const QPaintEngine* this_ptr, QPoint* output);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_delete(QPaintEngine* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_drawEllipse_QRect(QPaintEngine* this_ptr, const QRect* r);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_drawEllipse_QRectF(QPaintEngine* this_ptr, const QRectF* r);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_drawLines_QLineF_int(QPaintEngine* this_ptr, const QLineF* lines, int lineCount);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_drawLines_QLine_int(QPaintEngine* this_ptr, const QLine* lines, int lineCount);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_drawPath(QPaintEngine* this_ptr, const QPainterPath* path);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_drawPixmap(QPaintEngine* this_ptr, const QRectF* r, const QPixmap* pm, const QRectF* sr);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_drawPoints_QPointF_int(QPaintEngine* this_ptr, const QPointF* points, int pointCount);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_drawPoints_QPoint_int(QPaintEngine* this_ptr, const QPoint* points, int pointCount);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_drawPolygon_QPointF_int_QPaintEngine_PolygonDrawMode(QPaintEngine* this_ptr, const QPointF* points, int pointCount, QPaintEngine::PolygonDrawMode mode);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_drawPolygon_QPoint_int_QPaintEngine_PolygonDrawMode(QPaintEngine* this_ptr, const QPoint* points, int pointCount, QPaintEngine::PolygonDrawMode mode);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_drawRects_QRectF_int(QPaintEngine* this_ptr, const QRectF* rects, int rectCount);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_drawRects_QRect_int(QPaintEngine* this_ptr, const QRect* rects, int rectCount);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_drawTextItem(QPaintEngine* this_ptr, const QPointF* p, const QTextItem* textItem);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_drawTiledPixmap(QPaintEngine* this_ptr, const QRectF* r, const QPixmap* pixmap, const QPointF* s);
QT_GUI_C_EXPORT bool qt_gui_c_QPaintEngine_end(QPaintEngine* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_fix_neg_rect(QPaintEngine* this_ptr, int* x, int* y, int* w, int* h);
QT_GUI_C_EXPORT bool qt_gui_c_QPaintEngine_hasFeature(const QPaintEngine* this_ptr, unsigned int feature);
QT_GUI_C_EXPORT bool qt_gui_c_QPaintEngine_isActive(const QPaintEngine* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPaintEngine_isExtended(const QPaintEngine* this_ptr);
QT_GUI_C_EXPORT QPaintDevice* qt_gui_c_QPaintEngine_paintDevice(const QPaintEngine* this_ptr);
QT_GUI_C_EXPORT QPainter* qt_gui_c_QPaintEngine_painter(const QPaintEngine* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_setActive(QPaintEngine* this_ptr, bool newState);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_setDirty(QPaintEngine* this_ptr, unsigned int df);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_setPaintDevice(QPaintEngine* this_ptr, QPaintDevice* device);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_setSystemClip(QPaintEngine* this_ptr, const QRegion* baseClip);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_setSystemRect(QPaintEngine* this_ptr, const QRect* rect);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_syncState(QPaintEngine* this_ptr);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QPaintEngine_systemClip_as_ptr(const QPaintEngine* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_systemRect_to_output(const QPaintEngine* this_ptr, QRect* output);
QT_GUI_C_EXPORT bool qt_gui_c_QPaintEngine_testDirty(QPaintEngine* this_ptr, unsigned int df);
QT_GUI_C_EXPORT QPaintEngine::Type qt_gui_c_QPaintEngine_type(const QPaintEngine* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEngine_updateState(QPaintEngine* this_ptr, const QPaintEngineState* state);

} // extern "C"

#endif // QT_GUI_C_QPAINTENGINE_H
