#ifndef QT_WIDGETS_C_QGRAPHICSVIEW_H
#define QT_WIDGETS_C_QGRAPHICSVIEW_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGraphicsView* qt_widgets_c_QGraphicsView_G_dynamic_cast_QGraphicsView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QGraphicsView* qt_widgets_c_QGraphicsView_G_dynamic_cast_QGraphicsView_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QGraphicsView* qt_widgets_c_QGraphicsView_G_dynamic_cast_QGraphicsView_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractScrollArea* qt_widgets_c_QGraphicsView_G_static_cast_QAbstractScrollArea_ptr(QGraphicsView* ptr);
QT_WIDGETS_C_EXPORT QFrame* qt_widgets_c_QGraphicsView_G_static_cast_QFrame_ptr(QGraphicsView* ptr);
QT_WIDGETS_C_EXPORT QGraphicsView* qt_widgets_c_QGraphicsView_G_static_cast_QGraphicsView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QGraphicsView* qt_widgets_c_QGraphicsView_G_static_cast_QGraphicsView_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QGraphicsView* qt_widgets_c_QGraphicsView_G_static_cast_QGraphicsView_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QGraphicsView* qt_widgets_c_QGraphicsView_G_static_cast_QGraphicsView_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QGraphicsView* qt_widgets_c_QGraphicsView_G_static_cast_QGraphicsView_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QGraphicsView_G_static_cast_QObject_ptr(QGraphicsView* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QGraphicsView_G_static_cast_QPaintDevice_ptr(QGraphicsView* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QGraphicsView_G_static_cast_QWidget_ptr(QGraphicsView* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_backgroundBrush_to_output(const QGraphicsView* this_ptr, QBrush* output);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QGraphicsView_cacheMode(const QGraphicsView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_centerOn_item(QGraphicsView* this_ptr, const QGraphicsItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_centerOn_pos(QGraphicsView* this_ptr, const QPointF* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_centerOn_x_y(QGraphicsView* this_ptr, double x, double y);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_delete(QGraphicsView* this_ptr);
QT_WIDGETS_C_EXPORT QGraphicsView::DragMode qt_widgets_c_QGraphicsView_dragMode(const QGraphicsView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_ensureVisible_item(QGraphicsView* this_ptr, const QGraphicsItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_ensureVisible_item_xmargin(QGraphicsView* this_ptr, const QGraphicsItem* item, int xmargin);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_ensureVisible_item_xmargin_ymargin(QGraphicsView* this_ptr, const QGraphicsItem* item, int xmargin, int ymargin);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_ensureVisible_rect(QGraphicsView* this_ptr, const QRectF* rect);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_ensureVisible_rect_xmargin(QGraphicsView* this_ptr, const QRectF* rect, int xmargin);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_ensureVisible_rect_xmargin_ymargin(QGraphicsView* this_ptr, const QRectF* rect, int xmargin, int ymargin);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_ensureVisible_x_y_w_h(QGraphicsView* this_ptr, double x, double y, double w, double h);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_ensureVisible_x_y_w_h_xmargin(QGraphicsView* this_ptr, double x, double y, double w, double h, int xmargin);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_ensureVisible_x_y_w_h_xmargin_ymargin(QGraphicsView* this_ptr, double x, double y, double w, double h, int xmargin, int ymargin);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_fitInView_item(QGraphicsView* this_ptr, const QGraphicsItem* item);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_fitInView_item_aspectRadioMode(QGraphicsView* this_ptr, const QGraphicsItem* item, const Qt::AspectRatioMode* aspectRadioMode);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_fitInView_rect(QGraphicsView* this_ptr, const QRectF* rect);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_fitInView_rect_aspectRadioMode(QGraphicsView* this_ptr, const QRectF* rect, const Qt::AspectRatioMode* aspectRadioMode);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_fitInView_x_y_w_h(QGraphicsView* this_ptr, double x, double y, double w, double h);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_fitInView_x_y_w_h_aspectRadioMode(QGraphicsView* this_ptr, double x, double y, double w, double h, const Qt::AspectRatioMode* aspectRadioMode);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_foregroundBrush_to_output(const QGraphicsView* this_ptr, QBrush* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_inputMethodQuery_to_output(const QGraphicsView* this_ptr, const Qt::InputMethodQuery* query, QVariant* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsView_isInteractive(const QGraphicsView* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsView_isTransformed(const QGraphicsView* this_ptr);
QT_WIDGETS_C_EXPORT QGraphicsItem* qt_widgets_c_QGraphicsView_itemAt_pos(const QGraphicsView* this_ptr, const QPoint* pos);
QT_WIDGETS_C_EXPORT QGraphicsItem* qt_widgets_c_QGraphicsView_itemAt_x_y(const QGraphicsView* this_ptr, int x, int y);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_items_to_output_no_args(const QGraphicsView* this_ptr, QList< QGraphicsItem* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_items_to_output_path(const QGraphicsView* this_ptr, const QPainterPath* path, QList< QGraphicsItem* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_items_to_output_path_mode(const QGraphicsView* this_ptr, const QPainterPath* path, const Qt::ItemSelectionMode* mode, QList< QGraphicsItem* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_items_to_output_polygon(const QGraphicsView* this_ptr, const QPolygon* polygon, QList< QGraphicsItem* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_items_to_output_polygon_mode(const QGraphicsView* this_ptr, const QPolygon* polygon, const Qt::ItemSelectionMode* mode, QList< QGraphicsItem* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_items_to_output_pos(const QGraphicsView* this_ptr, const QPoint* pos, QList< QGraphicsItem* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_items_to_output_rect(const QGraphicsView* this_ptr, const QRect* rect, QList< QGraphicsItem* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_items_to_output_rect_mode(const QGraphicsView* this_ptr, const QRect* rect, const Qt::ItemSelectionMode* mode, QList< QGraphicsItem* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_items_to_output_x_y(const QGraphicsView* this_ptr, int x, int y, QList< QGraphicsItem* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_items_to_output_x_y_w_h(const QGraphicsView* this_ptr, int x, int y, int w, int h, QList< QGraphicsItem* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_items_to_output_x_y_w_h_mode(const QGraphicsView* this_ptr, int x, int y, int w, int h, const Qt::ItemSelectionMode* mode, QList< QGraphicsItem* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_mapFromScene_to_output_path(const QGraphicsView* this_ptr, const QPainterPath* path, QPainterPath* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_mapFromScene_to_output_point(const QGraphicsView* this_ptr, const QPointF* point, QPoint* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_mapFromScene_to_output_polygon(const QGraphicsView* this_ptr, const QPolygonF* polygon, QPolygon* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_mapFromScene_to_output_rect(const QGraphicsView* this_ptr, const QRectF* rect, QPolygon* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_mapFromScene_to_output_x_y(const QGraphicsView* this_ptr, double x, double y, QPoint* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_mapFromScene_to_output_x_y_w_h(const QGraphicsView* this_ptr, double x, double y, double w, double h, QPolygon* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_mapToScene_to_output_path(const QGraphicsView* this_ptr, const QPainterPath* path, QPainterPath* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_mapToScene_to_output_point(const QGraphicsView* this_ptr, const QPoint* point, QPointF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_mapToScene_to_output_polygon(const QGraphicsView* this_ptr, const QPolygon* polygon, QPolygonF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_mapToScene_to_output_rect(const QGraphicsView* this_ptr, const QRect* rect, QPolygonF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_mapToScene_to_output_x_y(const QGraphicsView* this_ptr, int x, int y, QPointF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_mapToScene_to_output_x_y_w_h(const QGraphicsView* this_ptr, int x, int y, int w, int h, QPolygonF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_matrix_to_output(const QGraphicsView* this_ptr, QMatrix* output);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QGraphicsView_metaObject(const QGraphicsView* this_ptr);
QT_WIDGETS_C_EXPORT QGraphicsView* qt_widgets_c_QGraphicsView_new_no_args();
QT_WIDGETS_C_EXPORT QGraphicsView* qt_widgets_c_QGraphicsView_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT QGraphicsView* qt_widgets_c_QGraphicsView_new_scene(QGraphicsScene* scene);
QT_WIDGETS_C_EXPORT QGraphicsView* qt_widgets_c_QGraphicsView_new_scene_parent(QGraphicsScene* scene, QWidget* parent);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QGraphicsView_optimizationFlags(const QGraphicsView* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsView_qt_metacall(QGraphicsView* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QGraphicsView_qt_metacast(QGraphicsView* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_render_painter(QGraphicsView* this_ptr, QPainter* painter);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_render_painter_target(QGraphicsView* this_ptr, QPainter* painter, const QRectF* target);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_render_painter_target_source(QGraphicsView* this_ptr, QPainter* painter, const QRectF* target, const QRect* source);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_render_painter_target_source_aspectRatioMode(QGraphicsView* this_ptr, QPainter* painter, const QRectF* target, const QRect* source, const Qt::AspectRatioMode* aspectRatioMode);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_resetCachedContent(QGraphicsView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_resetMatrix(QGraphicsView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_resetTransform(QGraphicsView* this_ptr);
QT_WIDGETS_C_EXPORT QGraphicsView::ViewportAnchor qt_widgets_c_QGraphicsView_resizeAnchor(const QGraphicsView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_rotate(QGraphicsView* this_ptr, double angle);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_rubberBandRect_to_output(const QGraphicsView* this_ptr, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_scale(QGraphicsView* this_ptr, double sx, double sy);
QT_WIDGETS_C_EXPORT QGraphicsScene* qt_widgets_c_QGraphicsView_scene(const QGraphicsView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_sceneRect_to_output(const QGraphicsView* this_ptr, QRectF* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_setBackgroundBrush(QGraphicsView* this_ptr, const QBrush* brush);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_setCacheMode(QGraphicsView* this_ptr, unsigned int mode);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_setDragMode(QGraphicsView* this_ptr, QGraphicsView::DragMode mode);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_setForegroundBrush(QGraphicsView* this_ptr, const QBrush* brush);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_setInteractive(QGraphicsView* this_ptr, bool allowed);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_setMatrix_matrix(QGraphicsView* this_ptr, const QMatrix* matrix);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_setMatrix_matrix_combine(QGraphicsView* this_ptr, const QMatrix* matrix, bool combine);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_setOptimizationFlag_flag(QGraphicsView* this_ptr, QGraphicsView::OptimizationFlag flag);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_setOptimizationFlag_flag_enabled(QGraphicsView* this_ptr, QGraphicsView::OptimizationFlag flag, bool enabled);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_setOptimizationFlags(QGraphicsView* this_ptr, unsigned int flags);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_setRenderHint_hint(QGraphicsView* this_ptr, const QPainter::RenderHint* hint);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_setRenderHint_hint_enabled(QGraphicsView* this_ptr, const QPainter::RenderHint* hint, bool enabled);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_setResizeAnchor(QGraphicsView* this_ptr, QGraphicsView::ViewportAnchor anchor);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_setRubberBandSelectionMode(QGraphicsView* this_ptr, const Qt::ItemSelectionMode* mode);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_setScene(QGraphicsView* this_ptr, QGraphicsScene* scene);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_setSceneRect_rect(QGraphicsView* this_ptr, const QRectF* rect);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_setSceneRect_x_y_w_h(QGraphicsView* this_ptr, double x, double y, double w, double h);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_setTransform_matrix(QGraphicsView* this_ptr, const QTransform* matrix);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_setTransform_matrix_combine(QGraphicsView* this_ptr, const QTransform* matrix, bool combine);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_setTransformationAnchor(QGraphicsView* this_ptr, QGraphicsView::ViewportAnchor anchor);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_setViewportUpdateMode(QGraphicsView* this_ptr, QGraphicsView::ViewportUpdateMode mode);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_shear(QGraphicsView* this_ptr, double sh, double sv);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_sizeHint_to_output(const QGraphicsView* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_transform_to_output(const QGraphicsView* this_ptr, QTransform* output);
QT_WIDGETS_C_EXPORT QGraphicsView::ViewportAnchor qt_widgets_c_QGraphicsView_transformationAnchor(const QGraphicsView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_translate(QGraphicsView* this_ptr, double dx, double dy);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_updateScene(QGraphicsView* this_ptr, const QList< QRectF >* rects);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_updateSceneRect(QGraphicsView* this_ptr, const QRectF* rect);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsView_viewportTransform_to_output(const QGraphicsView* this_ptr, QTransform* output);
QT_WIDGETS_C_EXPORT QGraphicsView::ViewportUpdateMode qt_widgets_c_QGraphicsView_viewportUpdateMode(const QGraphicsView* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSVIEW_H
