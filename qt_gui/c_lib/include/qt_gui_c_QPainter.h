#ifndef QT_GUI_C_QPAINTER_H
#define QT_GUI_C_QPAINTER_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QPainter::PixmapFragment* qt_gui_c_QPainter_PixmapFragment_create_as_ptr_pos_sourceRect(const QPointF* pos, const QRectF* sourceRect);
QT_GUI_C_EXPORT QPainter::PixmapFragment* qt_gui_c_QPainter_PixmapFragment_create_as_ptr_pos_sourceRect_scaleX(const QPointF* pos, const QRectF* sourceRect, double scaleX);
QT_GUI_C_EXPORT QPainter::PixmapFragment* qt_gui_c_QPainter_PixmapFragment_create_as_ptr_pos_sourceRect_scaleX_scaleY(const QPointF* pos, const QRectF* sourceRect, double scaleX, double scaleY);
QT_GUI_C_EXPORT QPainter::PixmapFragment* qt_gui_c_QPainter_PixmapFragment_create_as_ptr_pos_sourceRect_scaleX_scaleY_rotation(const QPointF* pos, const QRectF* sourceRect, double scaleX, double scaleY, double rotation);
QT_GUI_C_EXPORT QPainter::PixmapFragment* qt_gui_c_QPainter_PixmapFragment_create_as_ptr_pos_sourceRect_scaleX_scaleY_rotation_opacity(const QPointF* pos, const QRectF* sourceRect, double scaleX, double scaleY, double rotation, double opacity);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_PixmapFragment_delete(QPainter::PixmapFragment* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QPainter_PixmapFragment_height(const QPainter::PixmapFragment* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QPainter_PixmapFragment_opacity(const QPainter::PixmapFragment* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QPainter_PixmapFragment_rotation(const QPainter::PixmapFragment* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QPainter_PixmapFragment_scaleX(const QPainter::PixmapFragment* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QPainter_PixmapFragment_scaleY(const QPainter::PixmapFragment* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_PixmapFragment_set_height(QPainter::PixmapFragment* this_ptr, double value);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_PixmapFragment_set_opacity(QPainter::PixmapFragment* this_ptr, double value);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_PixmapFragment_set_rotation(QPainter::PixmapFragment* this_ptr, double value);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_PixmapFragment_set_scaleX(QPainter::PixmapFragment* this_ptr, double value);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_PixmapFragment_set_scaleY(QPainter::PixmapFragment* this_ptr, double value);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_PixmapFragment_set_sourceLeft(QPainter::PixmapFragment* this_ptr, double value);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_PixmapFragment_set_sourceTop(QPainter::PixmapFragment* this_ptr, double value);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_PixmapFragment_set_width(QPainter::PixmapFragment* this_ptr, double value);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_PixmapFragment_set_x(QPainter::PixmapFragment* this_ptr, double value);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_PixmapFragment_set_y(QPainter::PixmapFragment* this_ptr, double value);
QT_GUI_C_EXPORT double qt_gui_c_QPainter_PixmapFragment_sourceLeft(const QPainter::PixmapFragment* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QPainter_PixmapFragment_sourceTop(const QPainter::PixmapFragment* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QPainter_PixmapFragment_width(const QPainter::PixmapFragment* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QPainter_PixmapFragment_x(const QPainter::PixmapFragment* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QPainter_PixmapFragment_y(const QPainter::PixmapFragment* this_ptr);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPainter_background(const QPainter* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPainter_begin(QPainter* this_ptr, QPaintDevice* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_beginNativePainting(QPainter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_boundingRect_to_output_QRectF_QString(QPainter* this_ptr, const QRectF* rect, const QString* text, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_boundingRect_to_output_QRectF_QString_QTextOption(QPainter* this_ptr, const QRectF* rect, const QString* text, const QTextOption* o, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_boundingRect_to_output_QRectF_int_QString(QPainter* this_ptr, const QRectF* rect, int flags, const QString* text, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_boundingRect_to_output_QRect_int_QString(QPainter* this_ptr, const QRect* rect, int flags, const QString* text, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_boundingRect_to_output_int_int_int_int_int_QString(QPainter* this_ptr, int x, int y, int w, int h, int flags, const QString* text, QRect* output);
QT_GUI_C_EXPORT const QBrush* qt_gui_c_QPainter_brush(const QPainter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_brushOrigin_to_output(const QPainter* this_ptr, QPoint* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_clipBoundingRect_to_output(const QPainter* this_ptr, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_clipPath_to_output(const QPainter* this_ptr, QPainterPath* output);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QPainter_clipRegion_as_ptr(const QPainter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_combinedMatrix_to_output(const QPainter* this_ptr, QMatrix* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_combinedTransform_to_output(const QPainter* this_ptr, QTransform* output);
QT_GUI_C_EXPORT QPainter::CompositionMode qt_gui_c_QPainter_compositionMode(const QPainter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_delete(QPainter* this_ptr);
QT_GUI_C_EXPORT QPaintDevice* qt_gui_c_QPainter_device(const QPainter* this_ptr);
QT_GUI_C_EXPORT const QMatrix* qt_gui_c_QPainter_deviceMatrix(const QPainter* this_ptr);
QT_GUI_C_EXPORT const QTransform* qt_gui_c_QPainter_deviceTransform(const QPainter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawArc_arg1_a_alen(QPainter* this_ptr, const QRect* arg1, int a, int alen);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawArc_rect_a_alen(QPainter* this_ptr, const QRectF* rect, int a, int alen);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawArc_x_y_w_h_a_alen(QPainter* this_ptr, int x, int y, int w, int h, int a, int alen);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawChord_arg1_a_alen(QPainter* this_ptr, const QRect* arg1, int a, int alen);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawChord_rect_a_alen(QPainter* this_ptr, const QRectF* rect, int a, int alen);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawChord_x_y_w_h_a_alen(QPainter* this_ptr, int x, int y, int w, int h, int a, int alen);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawConvexPolygon_QPointF_int(QPainter* this_ptr, const QPointF* points, int pointCount);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawConvexPolygon_QPoint_int(QPainter* this_ptr, const QPoint* points, int pointCount);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawConvexPolygon_QPolygon(QPainter* this_ptr, const QPolygon* polygon);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawConvexPolygon_QPolygonF(QPainter* this_ptr, const QPolygonF* polygon);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawEllipse_QPointF_double_double(QPainter* this_ptr, const QPointF* center, double rx, double ry);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawEllipse_QPoint_int_int(QPainter* this_ptr, const QPoint* center, int rx, int ry);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawEllipse_QRect(QPainter* this_ptr, const QRect* r);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawEllipse_QRectF(QPainter* this_ptr, const QRectF* r);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawEllipse_int_int_int_int(QPainter* this_ptr, int x, int y, int w, int h);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawGlyphRun(QPainter* this_ptr, const QPointF* position, const QGlyphRun* glyphRun);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawImage_QPointF_QImage(QPainter* this_ptr, const QPointF* p, const QImage* image);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawImage_QPoint_QImage(QPainter* this_ptr, const QPoint* p, const QImage* image);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawImage_QRectF_QImage(QPainter* this_ptr, const QRectF* r, const QImage* image);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawImage_QRect_QImage(QPainter* this_ptr, const QRect* r, const QImage* image);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawLine_QLine(QPainter* this_ptr, const QLine* line);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawLine_QLineF(QPainter* this_ptr, const QLineF* line);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawLine_QPointF_QPointF(QPainter* this_ptr, const QPointF* p1, const QPointF* p2);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawLine_QPoint_QPoint(QPainter* this_ptr, const QPoint* p1, const QPoint* p2);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawLine_int_int_int_int(QPainter* this_ptr, int x1, int y1, int x2, int y2);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawLines_QLineF_int(QPainter* this_ptr, const QLineF* lines, int lineCount);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawLines_QLine_int(QPainter* this_ptr, const QLine* lines, int lineCount);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawLines_QPointF_int(QPainter* this_ptr, const QPointF* pointPairs, int lineCount);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawLines_QPoint_int(QPainter* this_ptr, const QPoint* pointPairs, int lineCount);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawLines_QVector_QLine(QPainter* this_ptr, const QVector< QLine >* lines);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawLines_QVector_QLineF(QPainter* this_ptr, const QVector< QLineF >* lines);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawLines_QVector_QPoint(QPainter* this_ptr, const QVector< QPoint >* pointPairs);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawLines_QVector_QPointF(QPainter* this_ptr, const QVector< QPointF >* pointPairs);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPath(QPainter* this_ptr, const QPainterPath* path);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPicture_QPointF_QPicture(QPainter* this_ptr, const QPointF* p, const QPicture* picture);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPicture_QPoint_QPicture(QPainter* this_ptr, const QPoint* p, const QPicture* picture);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPicture_int_int_QPicture(QPainter* this_ptr, int x, int y, const QPicture* picture);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPie_arg1_a_alen(QPainter* this_ptr, const QRect* arg1, int a, int alen);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPie_rect_a_alen(QPainter* this_ptr, const QRectF* rect, int a, int alen);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPie_x_y_w_h_a_alen(QPainter* this_ptr, int x, int y, int w, int h, int a, int alen);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPixmapFragments_fragments_fragmentCount_pixmap(QPainter* this_ptr, const QPainter::PixmapFragment* fragments, int fragmentCount, const QPixmap* pixmap);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPixmapFragments_fragments_fragmentCount_pixmap_hints(QPainter* this_ptr, const QPainter::PixmapFragment* fragments, int fragmentCount, const QPixmap* pixmap, unsigned int hints);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPixmap_QPointF_QPixmap(QPainter* this_ptr, const QPointF* p, const QPixmap* pm);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPixmap_QPointF_QPixmap_QRectF(QPainter* this_ptr, const QPointF* p, const QPixmap* pm, const QRectF* sr);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPixmap_QPoint_QPixmap(QPainter* this_ptr, const QPoint* p, const QPixmap* pm);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPixmap_QPoint_QPixmap_QRect(QPainter* this_ptr, const QPoint* p, const QPixmap* pm, const QRect* sr);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPixmap_QRectF_QPixmap_QRectF(QPainter* this_ptr, const QRectF* targetRect, const QPixmap* pixmap, const QRectF* sourceRect);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPixmap_QRect_QPixmap(QPainter* this_ptr, const QRect* r, const QPixmap* pm);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPixmap_QRect_QPixmap_QRect(QPainter* this_ptr, const QRect* targetRect, const QPixmap* pixmap, const QRect* sourceRect);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPixmap_int_int_QPixmap(QPainter* this_ptr, int x, int y, const QPixmap* pm);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPixmap_int_int_QPixmap_int_int_int_int(QPainter* this_ptr, int x, int y, const QPixmap* pm, int sx, int sy, int sw, int sh);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPixmap_int_int_int_int_QPixmap(QPainter* this_ptr, int x, int y, int w, int h, const QPixmap* pm);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPixmap_int_int_int_int_QPixmap_int_int_int_int(QPainter* this_ptr, int x, int y, int w, int h, const QPixmap* pm, int sx, int sy, int sw, int sh);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPoint_p(QPainter* this_ptr, const QPoint* p);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPoint_pt(QPainter* this_ptr, const QPointF* pt);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPoint_x_y(QPainter* this_ptr, int x, int y);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPoints_QPointF_int(QPainter* this_ptr, const QPointF* points, int pointCount);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPoints_QPoint_int(QPainter* this_ptr, const QPoint* points, int pointCount);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPoints_QPolygon(QPainter* this_ptr, const QPolygon* points);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPoints_QPolygonF(QPainter* this_ptr, const QPolygonF* points);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPolygon_QPointF_int(QPainter* this_ptr, const QPointF* points, int pointCount);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPolygon_QPointF_int_Qt_FillRule(QPainter* this_ptr, const QPointF* points, int pointCount, const Qt::FillRule* fillRule);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPolygon_QPoint_int(QPainter* this_ptr, const QPoint* points, int pointCount);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPolygon_QPoint_int_Qt_FillRule(QPainter* this_ptr, const QPoint* points, int pointCount, const Qt::FillRule* fillRule);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPolygon_QPolygon(QPainter* this_ptr, const QPolygon* polygon);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPolygon_QPolygonF(QPainter* this_ptr, const QPolygonF* polygon);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPolygon_QPolygonF_Qt_FillRule(QPainter* this_ptr, const QPolygonF* polygon, const Qt::FillRule* fillRule);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPolygon_QPolygon_Qt_FillRule(QPainter* this_ptr, const QPolygon* polygon, const Qt::FillRule* fillRule);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPolyline_QPointF_int(QPainter* this_ptr, const QPointF* points, int pointCount);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPolyline_QPoint_int(QPainter* this_ptr, const QPoint* points, int pointCount);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPolyline_QPolygon(QPainter* this_ptr, const QPolygon* polygon);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawPolyline_QPolygonF(QPainter* this_ptr, const QPolygonF* polyline);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawRect_QRect(QPainter* this_ptr, const QRect* rect);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawRect_QRectF(QPainter* this_ptr, const QRectF* rect);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawRect_int_int_int_int(QPainter* this_ptr, int x1, int y1, int w, int h);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawRects_QRectF_int(QPainter* this_ptr, const QRectF* rects, int rectCount);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawRects_QRect_int(QPainter* this_ptr, const QRect* rects, int rectCount);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawRects_QVector_QRect(QPainter* this_ptr, const QVector< QRect >* rectangles);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawRects_QVector_QRectF(QPainter* this_ptr, const QVector< QRectF >* rectangles);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawRoundRect_QRect(QPainter* this_ptr, const QRect* r);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawRoundRect_QRectF(QPainter* this_ptr, const QRectF* r);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawRoundRect_QRectF_int(QPainter* this_ptr, const QRectF* r, int xround);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawRoundRect_QRectF_int_int(QPainter* this_ptr, const QRectF* r, int xround, int yround);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawRoundRect_QRect_int(QPainter* this_ptr, const QRect* r, int xround);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawRoundRect_QRect_int_int(QPainter* this_ptr, const QRect* r, int xround, int yround);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawRoundRect_int_int_int_int(QPainter* this_ptr, int x, int y, int w, int h);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawRoundRect_int_int_int_int_int(QPainter* this_ptr, int x, int y, int w, int h, int arg5);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawRoundRect_int_int_int_int_int_int(QPainter* this_ptr, int x, int y, int w, int h, int arg5, int arg6);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawRoundedRect_QRectF_double_double(QPainter* this_ptr, const QRectF* rect, double xRadius, double yRadius);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawRoundedRect_QRectF_double_double_Qt_SizeMode(QPainter* this_ptr, const QRectF* rect, double xRadius, double yRadius, const Qt::SizeMode* mode);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawRoundedRect_QRect_double_double(QPainter* this_ptr, const QRect* rect, double xRadius, double yRadius);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawRoundedRect_QRect_double_double_Qt_SizeMode(QPainter* this_ptr, const QRect* rect, double xRadius, double yRadius, const Qt::SizeMode* mode);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawRoundedRect_int_int_int_int_double_double(QPainter* this_ptr, int x, int y, int w, int h, double xRadius, double yRadius);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawRoundedRect_int_int_int_int_double_double_Qt_SizeMode(QPainter* this_ptr, int x, int y, int w, int h, double xRadius, double yRadius, const Qt::SizeMode* mode);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawStaticText_QPointF_QStaticText(QPainter* this_ptr, const QPointF* topLeftPosition, const QStaticText* staticText);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawStaticText_QPoint_QStaticText(QPainter* this_ptr, const QPoint* topLeftPosition, const QStaticText* staticText);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawStaticText_int_int_QStaticText(QPainter* this_ptr, int left, int top, const QStaticText* staticText);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawTextItem_QPointF_QTextItem(QPainter* this_ptr, const QPointF* p, const QTextItem* ti);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawTextItem_QPoint_QTextItem(QPainter* this_ptr, const QPoint* p, const QTextItem* ti);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawTextItem_int_int_QTextItem(QPainter* this_ptr, int x, int y, const QTextItem* ti);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawText_QPointF_QString(QPainter* this_ptr, const QPointF* p, const QString* s);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawText_QPointF_QString_int_int(QPainter* this_ptr, const QPointF* p, const QString* str, int tf, int justificationPadding);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawText_QPoint_QString(QPainter* this_ptr, const QPoint* p, const QString* s);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawText_QRectF_QString(QPainter* this_ptr, const QRectF* r, const QString* text);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawText_QRectF_QString_QTextOption(QPainter* this_ptr, const QRectF* r, const QString* text, const QTextOption* o);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawText_QRectF_int_QString(QPainter* this_ptr, const QRectF* r, int flags, const QString* text);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawText_QRectF_int_QString_QRectF(QPainter* this_ptr, const QRectF* r, int flags, const QString* text, QRectF* br);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawText_QRect_int_QString(QPainter* this_ptr, const QRect* r, int flags, const QString* text);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawText_QRect_int_QString_QRect(QPainter* this_ptr, const QRect* r, int flags, const QString* text, QRect* br);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawText_int_int_QString(QPainter* this_ptr, int x, int y, const QString* s);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawText_int_int_int_int_int_QString(QPainter* this_ptr, int x, int y, int w, int h, int flags, const QString* text);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawText_int_int_int_int_int_QString_QRect(QPainter* this_ptr, int x, int y, int w, int h, int flags, const QString* text, QRect* br);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawTiledPixmap_arg1_arg2(QPainter* this_ptr, const QRect* arg1, const QPixmap* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawTiledPixmap_arg1_arg2_arg3(QPainter* this_ptr, const QRect* arg1, const QPixmap* arg2, const QPoint* arg3);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawTiledPixmap_rect_pm(QPainter* this_ptr, const QRectF* rect, const QPixmap* pm);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawTiledPixmap_rect_pm_offset(QPainter* this_ptr, const QRectF* rect, const QPixmap* pm, const QPointF* offset);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawTiledPixmap_x_y_w_h_arg5(QPainter* this_ptr, int x, int y, int w, int h, const QPixmap* arg5);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawTiledPixmap_x_y_w_h_arg5_sx(QPainter* this_ptr, int x, int y, int w, int h, const QPixmap* arg5, int sx);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_drawTiledPixmap_x_y_w_h_arg5_sx_sy(QPainter* this_ptr, int x, int y, int w, int h, const QPixmap* arg5, int sx, int sy);
QT_GUI_C_EXPORT bool qt_gui_c_QPainter_end(QPainter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_endNativePainting(QPainter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_eraseRect_QRect(QPainter* this_ptr, const QRect* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_eraseRect_QRectF(QPainter* this_ptr, const QRectF* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_eraseRect_int_int_int_int(QPainter* this_ptr, int x, int y, int w, int h);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_fillPath(QPainter* this_ptr, const QPainterPath* path, const QBrush* brush);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_fillRect_QRectF_QBrush(QPainter* this_ptr, const QRectF* arg1, const QBrush* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_fillRect_QRectF_QColor(QPainter* this_ptr, const QRectF* arg1, const QColor* color);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_fillRect_QRectF_Qt_BrushStyle(QPainter* this_ptr, const QRectF* r, const Qt::BrushStyle* style);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_fillRect_QRectF_Qt_GlobalColor(QPainter* this_ptr, const QRectF* r, const Qt::GlobalColor* c);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_fillRect_QRect_QBrush(QPainter* this_ptr, const QRect* arg1, const QBrush* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_fillRect_QRect_QColor(QPainter* this_ptr, const QRect* arg1, const QColor* color);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_fillRect_QRect_Qt_BrushStyle(QPainter* this_ptr, const QRect* r, const Qt::BrushStyle* style);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_fillRect_QRect_Qt_GlobalColor(QPainter* this_ptr, const QRect* r, const Qt::GlobalColor* c);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_fillRect_int_int_int_int_QBrush(QPainter* this_ptr, int x, int y, int w, int h, const QBrush* arg5);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_fillRect_int_int_int_int_QColor(QPainter* this_ptr, int x, int y, int w, int h, const QColor* color);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_fillRect_int_int_int_int_Qt_BrushStyle(QPainter* this_ptr, int x, int y, int w, int h, const Qt::BrushStyle* style);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_fillRect_int_int_int_int_Qt_GlobalColor(QPainter* this_ptr, int x, int y, int w, int h, const Qt::GlobalColor* c);
QT_GUI_C_EXPORT const QFont* qt_gui_c_QPainter_font(const QPainter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_fontInfo_to_output(const QPainter* this_ptr, QFontInfo* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_fontMetrics_to_output(const QPainter* this_ptr, QFontMetrics* output);
QT_GUI_C_EXPORT bool qt_gui_c_QPainter_hasClipping(const QPainter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_initFrom(QPainter* this_ptr, const QPaintDevice* device);
QT_GUI_C_EXPORT bool qt_gui_c_QPainter_isActive(const QPainter* this_ptr);
QT_GUI_C_EXPORT const QMatrix* qt_gui_c_QPainter_matrix(const QPainter* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPainter_matrixEnabled(const QPainter* this_ptr);
QT_GUI_C_EXPORT QPainter* qt_gui_c_QPainter_new_arg1(QPaintDevice* arg1);
QT_GUI_C_EXPORT QPainter* qt_gui_c_QPainter_new_no_args();
QT_GUI_C_EXPORT double qt_gui_c_QPainter_opacity(const QPainter* this_ptr);
QT_GUI_C_EXPORT QPaintEngine* qt_gui_c_QPainter_paintEngine(const QPainter* this_ptr);
QT_GUI_C_EXPORT const QPen* qt_gui_c_QPainter_pen(const QPainter* this_ptr);
QT_GUI_C_EXPORT QPaintDevice* qt_gui_c_QPainter_redirected_device(const QPaintDevice* device);
QT_GUI_C_EXPORT QPaintDevice* qt_gui_c_QPainter_redirected_device_offset(const QPaintDevice* device, QPoint* offset);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QPainter_renderHints(const QPainter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_resetMatrix(QPainter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_resetTransform(QPainter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_restore(QPainter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_restoreRedirected(const QPaintDevice* device);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_rotate(QPainter* this_ptr, double a);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_save(QPainter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_scale(QPainter* this_ptr, double sx, double sy);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setBackground(QPainter* this_ptr, const QBrush* bg);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setBackgroundMode(QPainter* this_ptr, const Qt::BGMode* mode);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setBrushOrigin_QPoint(QPainter* this_ptr, const QPoint* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setBrushOrigin_QPointF(QPainter* this_ptr, const QPointF* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setBrushOrigin_int_int(QPainter* this_ptr, int x, int y);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setBrush_brush(QPainter* this_ptr, const QBrush* brush);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setBrush_style(QPainter* this_ptr, const Qt::BrushStyle* style);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setClipPath_path(QPainter* this_ptr, const QPainterPath* path);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setClipPath_path_op(QPainter* this_ptr, const QPainterPath* path, const Qt::ClipOperation* op);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setClipRect_QRect(QPainter* this_ptr, const QRect* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setClipRect_QRectF(QPainter* this_ptr, const QRectF* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setClipRect_QRectF_Qt_ClipOperation(QPainter* this_ptr, const QRectF* arg1, const Qt::ClipOperation* op);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setClipRect_QRect_Qt_ClipOperation(QPainter* this_ptr, const QRect* arg1, const Qt::ClipOperation* op);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setClipRect_int_int_int_int(QPainter* this_ptr, int x, int y, int w, int h);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setClipRect_int_int_int_int_Qt_ClipOperation(QPainter* this_ptr, int x, int y, int w, int h, const Qt::ClipOperation* op);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setClipRegion_arg1(QPainter* this_ptr, const QRegion* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setClipRegion_arg1_op(QPainter* this_ptr, const QRegion* arg1, const Qt::ClipOperation* op);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setClipping(QPainter* this_ptr, bool enable);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setCompositionMode(QPainter* this_ptr, QPainter::CompositionMode mode);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setFont(QPainter* this_ptr, const QFont* f);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setLayoutDirection(QPainter* this_ptr, const Qt::LayoutDirection* direction);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setMatrixEnabled(QPainter* this_ptr, bool enabled);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setMatrix_matrix(QPainter* this_ptr, const QMatrix* matrix);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setMatrix_matrix_combine(QPainter* this_ptr, const QMatrix* matrix, bool combine);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setOpacity(QPainter* this_ptr, double opacity);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setPen_color(QPainter* this_ptr, const QColor* color);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setPen_pen(QPainter* this_ptr, const QPen* pen);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setPen_style(QPainter* this_ptr, const Qt::PenStyle* style);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setRedirected_device_replacement(const QPaintDevice* device, QPaintDevice* replacement);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setRedirected_device_replacement_offset(const QPaintDevice* device, QPaintDevice* replacement, const QPoint* offset);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setRenderHint_hint(QPainter* this_ptr, QPainter::RenderHint hint);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setRenderHint_hint_on(QPainter* this_ptr, QPainter::RenderHint hint, bool on);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setRenderHints_hints(QPainter* this_ptr, unsigned int hints);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setRenderHints_hints_on(QPainter* this_ptr, unsigned int hints, bool on);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setTransform_transform(QPainter* this_ptr, const QTransform* transform);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setTransform_transform_combine(QPainter* this_ptr, const QTransform* transform, bool combine);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setViewTransformEnabled(QPainter* this_ptr, bool enable);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setViewport_viewport(QPainter* this_ptr, const QRect* viewport);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setViewport_x_y_w_h(QPainter* this_ptr, int x, int y, int w, int h);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setWindow_window(QPainter* this_ptr, const QRect* window);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setWindow_x_y_w_h(QPainter* this_ptr, int x, int y, int w, int h);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setWorldMatrixEnabled(QPainter* this_ptr, bool enabled);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setWorldMatrix_matrix(QPainter* this_ptr, const QMatrix* matrix);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setWorldMatrix_matrix_combine(QPainter* this_ptr, const QMatrix* matrix, bool combine);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setWorldTransform_matrix(QPainter* this_ptr, const QTransform* matrix);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_setWorldTransform_matrix_combine(QPainter* this_ptr, const QTransform* matrix, bool combine);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_shear(QPainter* this_ptr, double sh, double sv);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_strokePath(QPainter* this_ptr, const QPainterPath* path, const QPen* pen);
QT_GUI_C_EXPORT bool qt_gui_c_QPainter_testRenderHint(const QPainter* this_ptr, QPainter::RenderHint hint);
QT_GUI_C_EXPORT const QTransform* qt_gui_c_QPainter_transform(const QPainter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_translate_QPoint(QPainter* this_ptr, const QPoint* offset);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_translate_QPointF(QPainter* this_ptr, const QPointF* offset);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_translate_double_double(QPainter* this_ptr, double dx, double dy);
QT_GUI_C_EXPORT bool qt_gui_c_QPainter_viewTransformEnabled(const QPainter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_viewport_to_output(const QPainter* this_ptr, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QPainter_window_to_output(const QPainter* this_ptr, QRect* output);
QT_GUI_C_EXPORT const QMatrix* qt_gui_c_QPainter_worldMatrix(const QPainter* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPainter_worldMatrixEnabled(const QPainter* this_ptr);
QT_GUI_C_EXPORT const QTransform* qt_gui_c_QPainter_worldTransform(const QPainter* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QPAINTER_H