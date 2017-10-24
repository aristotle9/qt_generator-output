#include "qt_widgets_c_QGraphicsView.h"

QGraphicsView* qt_widgets_c_QGraphicsView_G_dynamic_cast_QGraphicsView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return dynamic_cast<QGraphicsView*>(ptr);
}

QGraphicsView* qt_widgets_c_QGraphicsView_G_dynamic_cast_QGraphicsView_ptr_QFrame(QFrame* ptr) {
  return dynamic_cast<QGraphicsView*>(ptr);
}

QGraphicsView* qt_widgets_c_QGraphicsView_G_dynamic_cast_QGraphicsView_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QGraphicsView*>(ptr);
}

QAbstractScrollArea* qt_widgets_c_QGraphicsView_G_static_cast_QAbstractScrollArea_ptr(QGraphicsView* ptr) {
  return static_cast<QAbstractScrollArea*>(ptr);
}

QFrame* qt_widgets_c_QGraphicsView_G_static_cast_QFrame_ptr(QGraphicsView* ptr) {
  return static_cast<QFrame*>(ptr);
}

QGraphicsView* qt_widgets_c_QGraphicsView_G_static_cast_QGraphicsView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return static_cast<QGraphicsView*>(ptr);
}

QGraphicsView* qt_widgets_c_QGraphicsView_G_static_cast_QGraphicsView_ptr_QFrame(QFrame* ptr) {
  return static_cast<QGraphicsView*>(ptr);
}

QGraphicsView* qt_widgets_c_QGraphicsView_G_static_cast_QGraphicsView_ptr_QObject(QObject* ptr) {
  return static_cast<QGraphicsView*>(ptr);
}

QGraphicsView* qt_widgets_c_QGraphicsView_G_static_cast_QGraphicsView_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QGraphicsView*>(ptr);
}

QGraphicsView* qt_widgets_c_QGraphicsView_G_static_cast_QGraphicsView_ptr_QWidget(QWidget* ptr) {
  return static_cast<QGraphicsView*>(ptr);
}

QObject* qt_widgets_c_QGraphicsView_G_static_cast_QObject_ptr(QGraphicsView* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QGraphicsView_G_static_cast_QPaintDevice_ptr(QGraphicsView* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QGraphicsView_G_static_cast_QWidget_ptr(QGraphicsView* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QGraphicsView_backgroundBrush_to_output(const QGraphicsView* this_ptr, QBrush* output) {
  new(output) QBrush(this_ptr->backgroundBrush());
}

unsigned int qt_widgets_c_QGraphicsView_cacheMode(const QGraphicsView* this_ptr) {
  return uint(this_ptr->cacheMode());
}

void qt_widgets_c_QGraphicsView_centerOn_item(QGraphicsView* this_ptr, const QGraphicsItem* item) {
  this_ptr->centerOn(item);
}

void qt_widgets_c_QGraphicsView_centerOn_pos(QGraphicsView* this_ptr, const QPointF* pos) {
  this_ptr->centerOn(*pos);
}

void qt_widgets_c_QGraphicsView_centerOn_x_y(QGraphicsView* this_ptr, double x, double y) {
  this_ptr->centerOn(x, y);
}

void qt_widgets_c_QGraphicsView_delete(QGraphicsView* this_ptr) {
  delete this_ptr;
}

QGraphicsView::DragMode qt_widgets_c_QGraphicsView_dragMode(const QGraphicsView* this_ptr) {
  return this_ptr->dragMode();
}

void qt_widgets_c_QGraphicsView_ensureVisible_item(QGraphicsView* this_ptr, const QGraphicsItem* item) {
  this_ptr->ensureVisible(item);
}

void qt_widgets_c_QGraphicsView_ensureVisible_item_xmargin(QGraphicsView* this_ptr, const QGraphicsItem* item, int xmargin) {
  this_ptr->ensureVisible(item, xmargin);
}

void qt_widgets_c_QGraphicsView_ensureVisible_item_xmargin_ymargin(QGraphicsView* this_ptr, const QGraphicsItem* item, int xmargin, int ymargin) {
  this_ptr->ensureVisible(item, xmargin, ymargin);
}

void qt_widgets_c_QGraphicsView_ensureVisible_rect(QGraphicsView* this_ptr, const QRectF* rect) {
  this_ptr->ensureVisible(*rect);
}

void qt_widgets_c_QGraphicsView_ensureVisible_rect_xmargin(QGraphicsView* this_ptr, const QRectF* rect, int xmargin) {
  this_ptr->ensureVisible(*rect, xmargin);
}

void qt_widgets_c_QGraphicsView_ensureVisible_rect_xmargin_ymargin(QGraphicsView* this_ptr, const QRectF* rect, int xmargin, int ymargin) {
  this_ptr->ensureVisible(*rect, xmargin, ymargin);
}

void qt_widgets_c_QGraphicsView_ensureVisible_x_y_w_h(QGraphicsView* this_ptr, double x, double y, double w, double h) {
  this_ptr->ensureVisible(x, y, w, h);
}

void qt_widgets_c_QGraphicsView_ensureVisible_x_y_w_h_xmargin(QGraphicsView* this_ptr, double x, double y, double w, double h, int xmargin) {
  this_ptr->ensureVisible(x, y, w, h, xmargin);
}

void qt_widgets_c_QGraphicsView_ensureVisible_x_y_w_h_xmargin_ymargin(QGraphicsView* this_ptr, double x, double y, double w, double h, int xmargin, int ymargin) {
  this_ptr->ensureVisible(x, y, w, h, xmargin, ymargin);
}

void qt_widgets_c_QGraphicsView_fitInView_item(QGraphicsView* this_ptr, const QGraphicsItem* item) {
  this_ptr->fitInView(item);
}

void qt_widgets_c_QGraphicsView_fitInView_item_aspectRadioMode(QGraphicsView* this_ptr, const QGraphicsItem* item, const Qt::AspectRatioMode* aspectRadioMode) {
  this_ptr->fitInView(item, *aspectRadioMode);
}

void qt_widgets_c_QGraphicsView_fitInView_rect(QGraphicsView* this_ptr, const QRectF* rect) {
  this_ptr->fitInView(*rect);
}

void qt_widgets_c_QGraphicsView_fitInView_rect_aspectRadioMode(QGraphicsView* this_ptr, const QRectF* rect, const Qt::AspectRatioMode* aspectRadioMode) {
  this_ptr->fitInView(*rect, *aspectRadioMode);
}

void qt_widgets_c_QGraphicsView_fitInView_x_y_w_h(QGraphicsView* this_ptr, double x, double y, double w, double h) {
  this_ptr->fitInView(x, y, w, h);
}

void qt_widgets_c_QGraphicsView_fitInView_x_y_w_h_aspectRadioMode(QGraphicsView* this_ptr, double x, double y, double w, double h, const Qt::AspectRatioMode* aspectRadioMode) {
  this_ptr->fitInView(x, y, w, h, *aspectRadioMode);
}

void qt_widgets_c_QGraphicsView_foregroundBrush_to_output(const QGraphicsView* this_ptr, QBrush* output) {
  new(output) QBrush(this_ptr->foregroundBrush());
}

void qt_widgets_c_QGraphicsView_inputMethodQuery_to_output(const QGraphicsView* this_ptr, const Qt::InputMethodQuery* query, QVariant* output) {
  new(output) QVariant(this_ptr->inputMethodQuery(*query));
}

bool qt_widgets_c_QGraphicsView_isInteractive(const QGraphicsView* this_ptr) {
  return this_ptr->isInteractive();
}

bool qt_widgets_c_QGraphicsView_isTransformed(const QGraphicsView* this_ptr) {
  return this_ptr->isTransformed();
}

QGraphicsItem* qt_widgets_c_QGraphicsView_itemAt_pos(const QGraphicsView* this_ptr, const QPoint* pos) {
  return this_ptr->itemAt(*pos);
}

QGraphicsItem* qt_widgets_c_QGraphicsView_itemAt_x_y(const QGraphicsView* this_ptr, int x, int y) {
  return this_ptr->itemAt(x, y);
}

void qt_widgets_c_QGraphicsView_items_to_output_no_args(const QGraphicsView* this_ptr, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items());
}

void qt_widgets_c_QGraphicsView_items_to_output_path(const QGraphicsView* this_ptr, const QPainterPath* path, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*path));
}

void qt_widgets_c_QGraphicsView_items_to_output_path_mode(const QGraphicsView* this_ptr, const QPainterPath* path, const Qt::ItemSelectionMode* mode, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*path, *mode));
}

void qt_widgets_c_QGraphicsView_items_to_output_polygon(const QGraphicsView* this_ptr, const QPolygon* polygon, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*polygon));
}

void qt_widgets_c_QGraphicsView_items_to_output_polygon_mode(const QGraphicsView* this_ptr, const QPolygon* polygon, const Qt::ItemSelectionMode* mode, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*polygon, *mode));
}

void qt_widgets_c_QGraphicsView_items_to_output_pos(const QGraphicsView* this_ptr, const QPoint* pos, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*pos));
}

void qt_widgets_c_QGraphicsView_items_to_output_rect(const QGraphicsView* this_ptr, const QRect* rect, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*rect));
}

void qt_widgets_c_QGraphicsView_items_to_output_rect_mode(const QGraphicsView* this_ptr, const QRect* rect, const Qt::ItemSelectionMode* mode, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*rect, *mode));
}

void qt_widgets_c_QGraphicsView_items_to_output_x_y(const QGraphicsView* this_ptr, int x, int y, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(x, y));
}

void qt_widgets_c_QGraphicsView_items_to_output_x_y_w_h(const QGraphicsView* this_ptr, int x, int y, int w, int h, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(x, y, w, h));
}

void qt_widgets_c_QGraphicsView_items_to_output_x_y_w_h_mode(const QGraphicsView* this_ptr, int x, int y, int w, int h, const Qt::ItemSelectionMode* mode, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(x, y, w, h, *mode));
}

void qt_widgets_c_QGraphicsView_mapFromScene_to_output_path(const QGraphicsView* this_ptr, const QPainterPath* path, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->mapFromScene(*path));
}

void qt_widgets_c_QGraphicsView_mapFromScene_to_output_point(const QGraphicsView* this_ptr, const QPointF* point, QPoint* output) {
  new(output) QPoint(this_ptr->mapFromScene(*point));
}

void qt_widgets_c_QGraphicsView_mapFromScene_to_output_polygon(const QGraphicsView* this_ptr, const QPolygonF* polygon, QPolygon* output) {
  new(output) QPolygon(this_ptr->mapFromScene(*polygon));
}

void qt_widgets_c_QGraphicsView_mapFromScene_to_output_rect(const QGraphicsView* this_ptr, const QRectF* rect, QPolygon* output) {
  new(output) QPolygon(this_ptr->mapFromScene(*rect));
}

void qt_widgets_c_QGraphicsView_mapFromScene_to_output_x_y(const QGraphicsView* this_ptr, double x, double y, QPoint* output) {
  new(output) QPoint(this_ptr->mapFromScene(x, y));
}

void qt_widgets_c_QGraphicsView_mapFromScene_to_output_x_y_w_h(const QGraphicsView* this_ptr, double x, double y, double w, double h, QPolygon* output) {
  new(output) QPolygon(this_ptr->mapFromScene(x, y, w, h));
}

void qt_widgets_c_QGraphicsView_mapToScene_to_output_path(const QGraphicsView* this_ptr, const QPainterPath* path, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->mapToScene(*path));
}

void qt_widgets_c_QGraphicsView_mapToScene_to_output_point(const QGraphicsView* this_ptr, const QPoint* point, QPointF* output) {
  new(output) QPointF(this_ptr->mapToScene(*point));
}

void qt_widgets_c_QGraphicsView_mapToScene_to_output_polygon(const QGraphicsView* this_ptr, const QPolygon* polygon, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->mapToScene(*polygon));
}

void qt_widgets_c_QGraphicsView_mapToScene_to_output_rect(const QGraphicsView* this_ptr, const QRect* rect, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->mapToScene(*rect));
}

void qt_widgets_c_QGraphicsView_mapToScene_to_output_x_y(const QGraphicsView* this_ptr, int x, int y, QPointF* output) {
  new(output) QPointF(this_ptr->mapToScene(x, y));
}

void qt_widgets_c_QGraphicsView_mapToScene_to_output_x_y_w_h(const QGraphicsView* this_ptr, int x, int y, int w, int h, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->mapToScene(x, y, w, h));
}

void qt_widgets_c_QGraphicsView_matrix_to_output(const QGraphicsView* this_ptr, QMatrix* output) {
  new(output) QMatrix(this_ptr->matrix());
}

const QMetaObject* qt_widgets_c_QGraphicsView_metaObject(const QGraphicsView* this_ptr) {
  return this_ptr->metaObject();
}

QGraphicsView* qt_widgets_c_QGraphicsView_new_no_args() {
  return new QGraphicsView();
}

QGraphicsView* qt_widgets_c_QGraphicsView_new_parent(QWidget* parent) {
  return new QGraphicsView(parent);
}

QGraphicsView* qt_widgets_c_QGraphicsView_new_scene(QGraphicsScene* scene) {
  return new QGraphicsView(scene);
}

QGraphicsView* qt_widgets_c_QGraphicsView_new_scene_parent(QGraphicsScene* scene, QWidget* parent) {
  return new QGraphicsView(scene, parent);
}

unsigned int qt_widgets_c_QGraphicsView_optimizationFlags(const QGraphicsView* this_ptr) {
  return uint(this_ptr->optimizationFlags());
}

int qt_widgets_c_QGraphicsView_qt_metacall(QGraphicsView* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QGraphicsView_qt_metacast(QGraphicsView* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QGraphicsView_render_painter(QGraphicsView* this_ptr, QPainter* painter) {
  this_ptr->render(painter);
}

void qt_widgets_c_QGraphicsView_render_painter_target(QGraphicsView* this_ptr, QPainter* painter, const QRectF* target) {
  this_ptr->render(painter, *target);
}

void qt_widgets_c_QGraphicsView_render_painter_target_source(QGraphicsView* this_ptr, QPainter* painter, const QRectF* target, const QRect* source) {
  this_ptr->render(painter, *target, *source);
}

void qt_widgets_c_QGraphicsView_render_painter_target_source_aspectRatioMode(QGraphicsView* this_ptr, QPainter* painter, const QRectF* target, const QRect* source, const Qt::AspectRatioMode* aspectRatioMode) {
  this_ptr->render(painter, *target, *source, *aspectRatioMode);
}

void qt_widgets_c_QGraphicsView_resetCachedContent(QGraphicsView* this_ptr) {
  this_ptr->resetCachedContent();
}

void qt_widgets_c_QGraphicsView_resetMatrix(QGraphicsView* this_ptr) {
  this_ptr->resetMatrix();
}

void qt_widgets_c_QGraphicsView_resetTransform(QGraphicsView* this_ptr) {
  this_ptr->resetTransform();
}

QGraphicsView::ViewportAnchor qt_widgets_c_QGraphicsView_resizeAnchor(const QGraphicsView* this_ptr) {
  return this_ptr->resizeAnchor();
}

void qt_widgets_c_QGraphicsView_rotate(QGraphicsView* this_ptr, double angle) {
  this_ptr->rotate(angle);
}

void qt_widgets_c_QGraphicsView_rubberBandRect_to_output(const QGraphicsView* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->rubberBandRect());
}

void qt_widgets_c_QGraphicsView_scale(QGraphicsView* this_ptr, double sx, double sy) {
  this_ptr->scale(sx, sy);
}

QGraphicsScene* qt_widgets_c_QGraphicsView_scene(const QGraphicsView* this_ptr) {
  return this_ptr->scene();
}

void qt_widgets_c_QGraphicsView_sceneRect_to_output(const QGraphicsView* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->sceneRect());
}

void qt_widgets_c_QGraphicsView_setBackgroundBrush(QGraphicsView* this_ptr, const QBrush* brush) {
  this_ptr->setBackgroundBrush(*brush);
}

void qt_widgets_c_QGraphicsView_setCacheMode(QGraphicsView* this_ptr, unsigned int mode) {
  this_ptr->setCacheMode(QFlags< QGraphicsView::CacheModeFlag >(mode));
}

void qt_widgets_c_QGraphicsView_setDragMode(QGraphicsView* this_ptr, QGraphicsView::DragMode mode) {
  this_ptr->setDragMode(mode);
}

void qt_widgets_c_QGraphicsView_setForegroundBrush(QGraphicsView* this_ptr, const QBrush* brush) {
  this_ptr->setForegroundBrush(*brush);
}

void qt_widgets_c_QGraphicsView_setInteractive(QGraphicsView* this_ptr, bool allowed) {
  this_ptr->setInteractive(allowed);
}

void qt_widgets_c_QGraphicsView_setMatrix_matrix(QGraphicsView* this_ptr, const QMatrix* matrix) {
  this_ptr->setMatrix(*matrix);
}

void qt_widgets_c_QGraphicsView_setMatrix_matrix_combine(QGraphicsView* this_ptr, const QMatrix* matrix, bool combine) {
  this_ptr->setMatrix(*matrix, combine);
}

void qt_widgets_c_QGraphicsView_setOptimizationFlag_flag(QGraphicsView* this_ptr, QGraphicsView::OptimizationFlag flag) {
  this_ptr->setOptimizationFlag(flag);
}

void qt_widgets_c_QGraphicsView_setOptimizationFlag_flag_enabled(QGraphicsView* this_ptr, QGraphicsView::OptimizationFlag flag, bool enabled) {
  this_ptr->setOptimizationFlag(flag, enabled);
}

void qt_widgets_c_QGraphicsView_setOptimizationFlags(QGraphicsView* this_ptr, unsigned int flags) {
  this_ptr->setOptimizationFlags(QFlags< QGraphicsView::OptimizationFlag >(flags));
}

void qt_widgets_c_QGraphicsView_setRenderHint_hint(QGraphicsView* this_ptr, const QPainter::RenderHint* hint) {
  this_ptr->setRenderHint(*hint);
}

void qt_widgets_c_QGraphicsView_setRenderHint_hint_enabled(QGraphicsView* this_ptr, const QPainter::RenderHint* hint, bool enabled) {
  this_ptr->setRenderHint(*hint, enabled);
}

void qt_widgets_c_QGraphicsView_setResizeAnchor(QGraphicsView* this_ptr, QGraphicsView::ViewportAnchor anchor) {
  this_ptr->setResizeAnchor(anchor);
}

void qt_widgets_c_QGraphicsView_setRubberBandSelectionMode(QGraphicsView* this_ptr, const Qt::ItemSelectionMode* mode) {
  this_ptr->setRubberBandSelectionMode(*mode);
}

void qt_widgets_c_QGraphicsView_setScene(QGraphicsView* this_ptr, QGraphicsScene* scene) {
  this_ptr->setScene(scene);
}

void qt_widgets_c_QGraphicsView_setSceneRect_rect(QGraphicsView* this_ptr, const QRectF* rect) {
  this_ptr->setSceneRect(*rect);
}

void qt_widgets_c_QGraphicsView_setSceneRect_x_y_w_h(QGraphicsView* this_ptr, double x, double y, double w, double h) {
  this_ptr->setSceneRect(x, y, w, h);
}

void qt_widgets_c_QGraphicsView_setTransform_matrix(QGraphicsView* this_ptr, const QTransform* matrix) {
  this_ptr->setTransform(*matrix);
}

void qt_widgets_c_QGraphicsView_setTransform_matrix_combine(QGraphicsView* this_ptr, const QTransform* matrix, bool combine) {
  this_ptr->setTransform(*matrix, combine);
}

void qt_widgets_c_QGraphicsView_setTransformationAnchor(QGraphicsView* this_ptr, QGraphicsView::ViewportAnchor anchor) {
  this_ptr->setTransformationAnchor(anchor);
}

void qt_widgets_c_QGraphicsView_setViewportUpdateMode(QGraphicsView* this_ptr, QGraphicsView::ViewportUpdateMode mode) {
  this_ptr->setViewportUpdateMode(mode);
}

void qt_widgets_c_QGraphicsView_shear(QGraphicsView* this_ptr, double sh, double sv) {
  this_ptr->shear(sh, sv);
}

void qt_widgets_c_QGraphicsView_sizeHint_to_output(const QGraphicsView* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QGraphicsView_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsView::trUtf8(s, c, n));
}

void qt_widgets_c_QGraphicsView_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsView::tr(s, c, n));
}

void qt_widgets_c_QGraphicsView_transform_to_output(const QGraphicsView* this_ptr, QTransform* output) {
  new(output) QTransform(this_ptr->transform());
}

QGraphicsView::ViewportAnchor qt_widgets_c_QGraphicsView_transformationAnchor(const QGraphicsView* this_ptr) {
  return this_ptr->transformationAnchor();
}

void qt_widgets_c_QGraphicsView_translate(QGraphicsView* this_ptr, double dx, double dy) {
  this_ptr->translate(dx, dy);
}

void qt_widgets_c_QGraphicsView_updateScene(QGraphicsView* this_ptr, const QList< QRectF >* rects) {
  this_ptr->updateScene(*rects);
}

void qt_widgets_c_QGraphicsView_updateSceneRect(QGraphicsView* this_ptr, const QRectF* rect) {
  this_ptr->updateSceneRect(*rect);
}

void qt_widgets_c_QGraphicsView_viewportTransform_to_output(const QGraphicsView* this_ptr, QTransform* output) {
  new(output) QTransform(this_ptr->viewportTransform());
}

QGraphicsView::ViewportUpdateMode qt_widgets_c_QGraphicsView_viewportUpdateMode(const QGraphicsView* this_ptr) {
  return this_ptr->viewportUpdateMode();
}

