#include "qt_widgets_c_QGraphicsScene.h"

QGraphicsScene* qt_widgets_c_QGraphicsScene_G_static_cast_QGraphicsScene_ptr(QObject* ptr) {
  return static_cast<QGraphicsScene*>(ptr);
}

QObject* qt_widgets_c_QGraphicsScene_G_static_cast_QObject_ptr(QGraphicsScene* ptr) {
  return static_cast<QObject*>(ptr);
}

QGraphicsItem* qt_widgets_c_QGraphicsScene_activePanel(const QGraphicsScene* this_ptr) {
  return this_ptr->activePanel();
}

QGraphicsWidget* qt_widgets_c_QGraphicsScene_activeWindow(const QGraphicsScene* this_ptr) {
  return this_ptr->activeWindow();
}

QGraphicsEllipseItem* qt_widgets_c_QGraphicsScene_addEllipse_rect(QGraphicsScene* this_ptr, const QRectF* rect) {
  return this_ptr->addEllipse(*rect);
}

QGraphicsEllipseItem* qt_widgets_c_QGraphicsScene_addEllipse_rect_pen(QGraphicsScene* this_ptr, const QRectF* rect, const QPen* pen) {
  return this_ptr->addEllipse(*rect, *pen);
}

QGraphicsEllipseItem* qt_widgets_c_QGraphicsScene_addEllipse_rect_pen_brush(QGraphicsScene* this_ptr, const QRectF* rect, const QPen* pen, const QBrush* brush) {
  return this_ptr->addEllipse(*rect, *pen, *brush);
}

QGraphicsEllipseItem* qt_widgets_c_QGraphicsScene_addEllipse_x_y_w_h(QGraphicsScene* this_ptr, double x, double y, double w, double h) {
  return this_ptr->addEllipse(x, y, w, h);
}

QGraphicsEllipseItem* qt_widgets_c_QGraphicsScene_addEllipse_x_y_w_h_pen(QGraphicsScene* this_ptr, double x, double y, double w, double h, const QPen* pen) {
  return this_ptr->addEllipse(x, y, w, h, *pen);
}

QGraphicsEllipseItem* qt_widgets_c_QGraphicsScene_addEllipse_x_y_w_h_pen_brush(QGraphicsScene* this_ptr, double x, double y, double w, double h, const QPen* pen, const QBrush* brush) {
  return this_ptr->addEllipse(x, y, w, h, *pen, *brush);
}

void qt_widgets_c_QGraphicsScene_addItem(QGraphicsScene* this_ptr, QGraphicsItem* item) {
  this_ptr->addItem(item);
}

QGraphicsLineItem* qt_widgets_c_QGraphicsScene_addLine_line(QGraphicsScene* this_ptr, const QLineF* line) {
  return this_ptr->addLine(*line);
}

QGraphicsLineItem* qt_widgets_c_QGraphicsScene_addLine_line_pen(QGraphicsScene* this_ptr, const QLineF* line, const QPen* pen) {
  return this_ptr->addLine(*line, *pen);
}

QGraphicsLineItem* qt_widgets_c_QGraphicsScene_addLine_x1_y1_x2_y2(QGraphicsScene* this_ptr, double x1, double y1, double x2, double y2) {
  return this_ptr->addLine(x1, y1, x2, y2);
}

QGraphicsLineItem* qt_widgets_c_QGraphicsScene_addLine_x1_y1_x2_y2_pen(QGraphicsScene* this_ptr, double x1, double y1, double x2, double y2, const QPen* pen) {
  return this_ptr->addLine(x1, y1, x2, y2, *pen);
}

QGraphicsPathItem* qt_widgets_c_QGraphicsScene_addPath_path(QGraphicsScene* this_ptr, const QPainterPath* path) {
  return this_ptr->addPath(*path);
}

QGraphicsPathItem* qt_widgets_c_QGraphicsScene_addPath_path_pen(QGraphicsScene* this_ptr, const QPainterPath* path, const QPen* pen) {
  return this_ptr->addPath(*path, *pen);
}

QGraphicsPathItem* qt_widgets_c_QGraphicsScene_addPath_path_pen_brush(QGraphicsScene* this_ptr, const QPainterPath* path, const QPen* pen, const QBrush* brush) {
  return this_ptr->addPath(*path, *pen, *brush);
}

QGraphicsPixmapItem* qt_widgets_c_QGraphicsScene_addPixmap(QGraphicsScene* this_ptr, const QPixmap* pixmap) {
  return this_ptr->addPixmap(*pixmap);
}

QGraphicsPolygonItem* qt_widgets_c_QGraphicsScene_addPolygon_polygon(QGraphicsScene* this_ptr, const QPolygonF* polygon) {
  return this_ptr->addPolygon(*polygon);
}

QGraphicsPolygonItem* qt_widgets_c_QGraphicsScene_addPolygon_polygon_pen(QGraphicsScene* this_ptr, const QPolygonF* polygon, const QPen* pen) {
  return this_ptr->addPolygon(*polygon, *pen);
}

QGraphicsPolygonItem* qt_widgets_c_QGraphicsScene_addPolygon_polygon_pen_brush(QGraphicsScene* this_ptr, const QPolygonF* polygon, const QPen* pen, const QBrush* brush) {
  return this_ptr->addPolygon(*polygon, *pen, *brush);
}

QGraphicsRectItem* qt_widgets_c_QGraphicsScene_addRect_rect(QGraphicsScene* this_ptr, const QRectF* rect) {
  return this_ptr->addRect(*rect);
}

QGraphicsRectItem* qt_widgets_c_QGraphicsScene_addRect_rect_pen(QGraphicsScene* this_ptr, const QRectF* rect, const QPen* pen) {
  return this_ptr->addRect(*rect, *pen);
}

QGraphicsRectItem* qt_widgets_c_QGraphicsScene_addRect_rect_pen_brush(QGraphicsScene* this_ptr, const QRectF* rect, const QPen* pen, const QBrush* brush) {
  return this_ptr->addRect(*rect, *pen, *brush);
}

QGraphicsRectItem* qt_widgets_c_QGraphicsScene_addRect_x_y_w_h(QGraphicsScene* this_ptr, double x, double y, double w, double h) {
  return this_ptr->addRect(x, y, w, h);
}

QGraphicsRectItem* qt_widgets_c_QGraphicsScene_addRect_x_y_w_h_pen(QGraphicsScene* this_ptr, double x, double y, double w, double h, const QPen* pen) {
  return this_ptr->addRect(x, y, w, h, *pen);
}

QGraphicsRectItem* qt_widgets_c_QGraphicsScene_addRect_x_y_w_h_pen_brush(QGraphicsScene* this_ptr, double x, double y, double w, double h, const QPen* pen, const QBrush* brush) {
  return this_ptr->addRect(x, y, w, h, *pen, *brush);
}

QGraphicsSimpleTextItem* qt_widgets_c_QGraphicsScene_addSimpleText_text(QGraphicsScene* this_ptr, const QString* text) {
  return this_ptr->addSimpleText(*text);
}

QGraphicsSimpleTextItem* qt_widgets_c_QGraphicsScene_addSimpleText_text_font(QGraphicsScene* this_ptr, const QString* text, const QFont* font) {
  return this_ptr->addSimpleText(*text, *font);
}

QGraphicsTextItem* qt_widgets_c_QGraphicsScene_addText_text(QGraphicsScene* this_ptr, const QString* text) {
  return this_ptr->addText(*text);
}

QGraphicsTextItem* qt_widgets_c_QGraphicsScene_addText_text_font(QGraphicsScene* this_ptr, const QString* text, const QFont* font) {
  return this_ptr->addText(*text, *font);
}

void qt_widgets_c_QGraphicsScene_advance(QGraphicsScene* this_ptr) {
  this_ptr->advance();
}

void qt_widgets_c_QGraphicsScene_backgroundBrush_to_output(const QGraphicsScene* this_ptr, QBrush* output) {
  new(output) QBrush(this_ptr->backgroundBrush());
}

int qt_widgets_c_QGraphicsScene_bspTreeDepth(const QGraphicsScene* this_ptr) {
  return this_ptr->bspTreeDepth();
}

void qt_widgets_c_QGraphicsScene_clear(QGraphicsScene* this_ptr) {
  this_ptr->clear();
}

void qt_widgets_c_QGraphicsScene_clearFocus(QGraphicsScene* this_ptr) {
  this_ptr->clearFocus();
}

void qt_widgets_c_QGraphicsScene_clearSelection(QGraphicsScene* this_ptr) {
  this_ptr->clearSelection();
}

void qt_widgets_c_QGraphicsScene_collidingItems_to_output_item(const QGraphicsScene* this_ptr, const QGraphicsItem* item, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->collidingItems(item));
}

void qt_widgets_c_QGraphicsScene_collidingItems_to_output_item_mode(const QGraphicsScene* this_ptr, const QGraphicsItem* item, const Qt::ItemSelectionMode* mode, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->collidingItems(item, *mode));
}

QGraphicsItemGroup* qt_widgets_c_QGraphicsScene_createItemGroup(QGraphicsScene* this_ptr, const QList< QGraphicsItem* >* items) {
  return this_ptr->createItemGroup(*items);
}

void qt_widgets_c_QGraphicsScene_delete(QGraphicsScene* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QGraphicsScene_destroyItemGroup(QGraphicsScene* this_ptr, QGraphicsItemGroup* group) {
  this_ptr->destroyItemGroup(group);
}

QGraphicsItem* qt_widgets_c_QGraphicsScene_focusItem(const QGraphicsScene* this_ptr) {
  return this_ptr->focusItem();
}

void qt_widgets_c_QGraphicsScene_font_to_output(const QGraphicsScene* this_ptr, QFont* output) {
  new(output) QFont(this_ptr->font());
}

void qt_widgets_c_QGraphicsScene_foregroundBrush_to_output(const QGraphicsScene* this_ptr, QBrush* output) {
  new(output) QBrush(this_ptr->foregroundBrush());
}

bool qt_widgets_c_QGraphicsScene_hasFocus(const QGraphicsScene* this_ptr) {
  return this_ptr->hasFocus();
}

double qt_widgets_c_QGraphicsScene_height(const QGraphicsScene* this_ptr) {
  return this_ptr->height();
}

void qt_widgets_c_QGraphicsScene_inputMethodQuery_to_output(const QGraphicsScene* this_ptr, const Qt::InputMethodQuery* query, QVariant* output) {
  new(output) QVariant(this_ptr->inputMethodQuery(*query));
}

void qt_widgets_c_QGraphicsScene_invalidate_no_args(QGraphicsScene* this_ptr) {
  this_ptr->invalidate();
}

void qt_widgets_c_QGraphicsScene_invalidate_rect(QGraphicsScene* this_ptr, const QRectF* rect) {
  this_ptr->invalidate(*rect);
}

void qt_widgets_c_QGraphicsScene_invalidate_rect_layers(QGraphicsScene* this_ptr, const QRectF* rect, unsigned int layers) {
  this_ptr->invalidate(*rect, QFlags< QGraphicsScene::SceneLayer >(layers));
}

void qt_widgets_c_QGraphicsScene_invalidate_x_y_w_h(QGraphicsScene* this_ptr, double x, double y, double w, double h) {
  this_ptr->invalidate(x, y, w, h);
}

void qt_widgets_c_QGraphicsScene_invalidate_x_y_w_h_layers(QGraphicsScene* this_ptr, double x, double y, double w, double h, unsigned int layers) {
  this_ptr->invalidate(x, y, w, h, QFlags< QGraphicsScene::SceneLayer >(layers));
}

bool qt_widgets_c_QGraphicsScene_isActive(const QGraphicsScene* this_ptr) {
  return this_ptr->isActive();
}

bool qt_widgets_c_QGraphicsScene_isSortCacheEnabled(const QGraphicsScene* this_ptr) {
  return this_ptr->isSortCacheEnabled();
}

QGraphicsItem* qt_widgets_c_QGraphicsScene_itemAt_pos_deviceTransform(const QGraphicsScene* this_ptr, const QPointF* pos, const QTransform* deviceTransform) {
  return this_ptr->itemAt(*pos, *deviceTransform);
}

QGraphicsItem* qt_widgets_c_QGraphicsScene_itemAt_x_y_deviceTransform(const QGraphicsScene* this_ptr, double x, double y, const QTransform* deviceTransform) {
  return this_ptr->itemAt(x, y, *deviceTransform);
}

QGraphicsScene::ItemIndexMethod qt_widgets_c_QGraphicsScene_itemIndexMethod(const QGraphicsScene* this_ptr) {
  return this_ptr->itemIndexMethod();
}

void qt_widgets_c_QGraphicsScene_itemsBoundingRect_to_output(const QGraphicsScene* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->itemsBoundingRect());
}

void qt_widgets_c_QGraphicsScene_items_to_output_no_args(const QGraphicsScene* this_ptr, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items());
}

void qt_widgets_c_QGraphicsScene_items_to_output_order(const QGraphicsScene* this_ptr, const Qt::SortOrder* order, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*order));
}

void qt_widgets_c_QGraphicsScene_items_to_output_path(const QGraphicsScene* this_ptr, const QPainterPath* path, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*path));
}

void qt_widgets_c_QGraphicsScene_items_to_output_path_mode(const QGraphicsScene* this_ptr, const QPainterPath* path, const Qt::ItemSelectionMode* mode, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*path, *mode));
}

void qt_widgets_c_QGraphicsScene_items_to_output_path_mode_order(const QGraphicsScene* this_ptr, const QPainterPath* path, const Qt::ItemSelectionMode* mode, const Qt::SortOrder* order, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*path, *mode, *order));
}

void qt_widgets_c_QGraphicsScene_items_to_output_path_mode_order_deviceTransform(const QGraphicsScene* this_ptr, const QPainterPath* path, const Qt::ItemSelectionMode* mode, const Qt::SortOrder* order, const QTransform* deviceTransform, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*path, *mode, *order, *deviceTransform));
}

void qt_widgets_c_QGraphicsScene_items_to_output_polygon(const QGraphicsScene* this_ptr, const QPolygonF* polygon, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*polygon));
}

void qt_widgets_c_QGraphicsScene_items_to_output_polygon_mode(const QGraphicsScene* this_ptr, const QPolygonF* polygon, const Qt::ItemSelectionMode* mode, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*polygon, *mode));
}

void qt_widgets_c_QGraphicsScene_items_to_output_polygon_mode_order(const QGraphicsScene* this_ptr, const QPolygonF* polygon, const Qt::ItemSelectionMode* mode, const Qt::SortOrder* order, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*polygon, *mode, *order));
}

void qt_widgets_c_QGraphicsScene_items_to_output_polygon_mode_order_deviceTransform(const QGraphicsScene* this_ptr, const QPolygonF* polygon, const Qt::ItemSelectionMode* mode, const Qt::SortOrder* order, const QTransform* deviceTransform, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*polygon, *mode, *order, *deviceTransform));
}

void qt_widgets_c_QGraphicsScene_items_to_output_pos(const QGraphicsScene* this_ptr, const QPointF* pos, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*pos));
}

void qt_widgets_c_QGraphicsScene_items_to_output_pos_mode(const QGraphicsScene* this_ptr, const QPointF* pos, const Qt::ItemSelectionMode* mode, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*pos, *mode));
}

void qt_widgets_c_QGraphicsScene_items_to_output_pos_mode_order(const QGraphicsScene* this_ptr, const QPointF* pos, const Qt::ItemSelectionMode* mode, const Qt::SortOrder* order, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*pos, *mode, *order));
}

void qt_widgets_c_QGraphicsScene_items_to_output_pos_mode_order_deviceTransform(const QGraphicsScene* this_ptr, const QPointF* pos, const Qt::ItemSelectionMode* mode, const Qt::SortOrder* order, const QTransform* deviceTransform, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*pos, *mode, *order, *deviceTransform));
}

void qt_widgets_c_QGraphicsScene_items_to_output_rect(const QGraphicsScene* this_ptr, const QRectF* rect, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*rect));
}

void qt_widgets_c_QGraphicsScene_items_to_output_rect_mode(const QGraphicsScene* this_ptr, const QRectF* rect, const Qt::ItemSelectionMode* mode, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*rect, *mode));
}

void qt_widgets_c_QGraphicsScene_items_to_output_rect_mode_order(const QGraphicsScene* this_ptr, const QRectF* rect, const Qt::ItemSelectionMode* mode, const Qt::SortOrder* order, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*rect, *mode, *order));
}

void qt_widgets_c_QGraphicsScene_items_to_output_rect_mode_order_deviceTransform(const QGraphicsScene* this_ptr, const QRectF* rect, const Qt::ItemSelectionMode* mode, const Qt::SortOrder* order, const QTransform* deviceTransform, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(*rect, *mode, *order, *deviceTransform));
}

void qt_widgets_c_QGraphicsScene_items_to_output_x_y_w_h_mode_order(const QGraphicsScene* this_ptr, double x, double y, double w, double h, const Qt::ItemSelectionMode* mode, const Qt::SortOrder* order, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(x, y, w, h, *mode, *order));
}

void qt_widgets_c_QGraphicsScene_items_to_output_x_y_w_h_mode_order_deviceTransform(const QGraphicsScene* this_ptr, double x, double y, double w, double h, const Qt::ItemSelectionMode* mode, const Qt::SortOrder* order, const QTransform* deviceTransform, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->items(x, y, w, h, *mode, *order, *deviceTransform));
}

const QMetaObject* qt_widgets_c_QGraphicsScene_metaObject(const QGraphicsScene* this_ptr) {
  return this_ptr->metaObject();
}

double qt_widgets_c_QGraphicsScene_minimumRenderSize(const QGraphicsScene* this_ptr) {
  return this_ptr->minimumRenderSize();
}

QGraphicsItem* qt_widgets_c_QGraphicsScene_mouseGrabberItem(const QGraphicsScene* this_ptr) {
  return this_ptr->mouseGrabberItem();
}

QGraphicsScene* qt_widgets_c_QGraphicsScene_new_no_args() {
  return new QGraphicsScene();
}

QGraphicsScene* qt_widgets_c_QGraphicsScene_new_parent(QObject* parent) {
  return new QGraphicsScene(parent);
}

QGraphicsScene* qt_widgets_c_QGraphicsScene_new_sceneRect(const QRectF* sceneRect) {
  return new QGraphicsScene(*sceneRect);
}

QGraphicsScene* qt_widgets_c_QGraphicsScene_new_sceneRect_parent(const QRectF* sceneRect, QObject* parent) {
  return new QGraphicsScene(*sceneRect, parent);
}

QGraphicsScene* qt_widgets_c_QGraphicsScene_new_x_y_width_height(double x, double y, double width, double height) {
  return new QGraphicsScene(x, y, width, height);
}

QGraphicsScene* qt_widgets_c_QGraphicsScene_new_x_y_width_height_parent(double x, double y, double width, double height, QObject* parent) {
  return new QGraphicsScene(x, y, width, height, parent);
}

void qt_widgets_c_QGraphicsScene_palette_to_output(const QGraphicsScene* this_ptr, QPalette* output) {
  new(output) QPalette(this_ptr->palette());
}

int qt_widgets_c_QGraphicsScene_qt_metacall(QGraphicsScene* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QGraphicsScene_qt_metacast(QGraphicsScene* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QGraphicsScene_removeItem(QGraphicsScene* this_ptr, QGraphicsItem* item) {
  this_ptr->removeItem(item);
}

void qt_widgets_c_QGraphicsScene_render_painter(QGraphicsScene* this_ptr, QPainter* painter) {
  this_ptr->render(painter);
}

void qt_widgets_c_QGraphicsScene_render_painter_target(QGraphicsScene* this_ptr, QPainter* painter, const QRectF* target) {
  this_ptr->render(painter, *target);
}

void qt_widgets_c_QGraphicsScene_render_painter_target_source(QGraphicsScene* this_ptr, QPainter* painter, const QRectF* target, const QRectF* source) {
  this_ptr->render(painter, *target, *source);
}

void qt_widgets_c_QGraphicsScene_render_painter_target_source_aspectRatioMode(QGraphicsScene* this_ptr, QPainter* painter, const QRectF* target, const QRectF* source, const Qt::AspectRatioMode* aspectRatioMode) {
  this_ptr->render(painter, *target, *source, *aspectRatioMode);
}

void qt_widgets_c_QGraphicsScene_sceneRect_to_output(const QGraphicsScene* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->sceneRect());
}

void qt_widgets_c_QGraphicsScene_selectedItems_to_output(const QGraphicsScene* this_ptr, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->selectedItems());
}

void qt_widgets_c_QGraphicsScene_selectionArea_to_output(const QGraphicsScene* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->selectionArea());
}

bool qt_widgets_c_QGraphicsScene_sendEvent(QGraphicsScene* this_ptr, QGraphicsItem* item, QEvent* event) {
  return this_ptr->sendEvent(item, event);
}

void qt_widgets_c_QGraphicsScene_setActivePanel(QGraphicsScene* this_ptr, QGraphicsItem* item) {
  this_ptr->setActivePanel(item);
}

void qt_widgets_c_QGraphicsScene_setActiveWindow(QGraphicsScene* this_ptr, QGraphicsWidget* widget) {
  this_ptr->setActiveWindow(widget);
}

void qt_widgets_c_QGraphicsScene_setBackgroundBrush(QGraphicsScene* this_ptr, const QBrush* brush) {
  this_ptr->setBackgroundBrush(*brush);
}

void qt_widgets_c_QGraphicsScene_setBspTreeDepth(QGraphicsScene* this_ptr, int depth) {
  this_ptr->setBspTreeDepth(depth);
}

void qt_widgets_c_QGraphicsScene_setFocusItem_item(QGraphicsScene* this_ptr, QGraphicsItem* item) {
  this_ptr->setFocusItem(item);
}

void qt_widgets_c_QGraphicsScene_setFocusItem_item_focusReason(QGraphicsScene* this_ptr, QGraphicsItem* item, const Qt::FocusReason* focusReason) {
  this_ptr->setFocusItem(item, *focusReason);
}

void qt_widgets_c_QGraphicsScene_setFocus_focusReason(QGraphicsScene* this_ptr, const Qt::FocusReason* focusReason) {
  this_ptr->setFocus(*focusReason);
}

void qt_widgets_c_QGraphicsScene_setFocus_no_args(QGraphicsScene* this_ptr) {
  this_ptr->setFocus();
}

void qt_widgets_c_QGraphicsScene_setFont(QGraphicsScene* this_ptr, const QFont* font) {
  this_ptr->setFont(*font);
}

void qt_widgets_c_QGraphicsScene_setForegroundBrush(QGraphicsScene* this_ptr, const QBrush* brush) {
  this_ptr->setForegroundBrush(*brush);
}

void qt_widgets_c_QGraphicsScene_setItemIndexMethod(QGraphicsScene* this_ptr, QGraphicsScene::ItemIndexMethod method) {
  this_ptr->setItemIndexMethod(method);
}

void qt_widgets_c_QGraphicsScene_setMinimumRenderSize(QGraphicsScene* this_ptr, double minSize) {
  this_ptr->setMinimumRenderSize(minSize);
}

void qt_widgets_c_QGraphicsScene_setPalette(QGraphicsScene* this_ptr, const QPalette* palette) {
  this_ptr->setPalette(*palette);
}

void qt_widgets_c_QGraphicsScene_setSceneRect_rect(QGraphicsScene* this_ptr, const QRectF* rect) {
  this_ptr->setSceneRect(*rect);
}

void qt_widgets_c_QGraphicsScene_setSceneRect_x_y_w_h(QGraphicsScene* this_ptr, double x, double y, double w, double h) {
  this_ptr->setSceneRect(x, y, w, h);
}

void qt_widgets_c_QGraphicsScene_setSelectionArea_path(QGraphicsScene* this_ptr, const QPainterPath* path) {
  this_ptr->setSelectionArea(*path);
}

void qt_widgets_c_QGraphicsScene_setSelectionArea_path_deviceTransform(QGraphicsScene* this_ptr, const QPainterPath* path, const QTransform* deviceTransform) {
  this_ptr->setSelectionArea(*path, *deviceTransform);
}

void qt_widgets_c_QGraphicsScene_setSelectionArea_path_mode(QGraphicsScene* this_ptr, const QPainterPath* path, const Qt::ItemSelectionMode* mode) {
  this_ptr->setSelectionArea(*path, *mode);
}

void qt_widgets_c_QGraphicsScene_setSelectionArea_path_mode_deviceTransform(QGraphicsScene* this_ptr, const QPainterPath* path, const Qt::ItemSelectionMode* mode, const QTransform* deviceTransform) {
  this_ptr->setSelectionArea(*path, *mode, *deviceTransform);
}

void qt_widgets_c_QGraphicsScene_setSelectionArea_path_selectionOperation(QGraphicsScene* this_ptr, const QPainterPath* path, const Qt::ItemSelectionOperation* selectionOperation) {
  this_ptr->setSelectionArea(*path, *selectionOperation);
}

void qt_widgets_c_QGraphicsScene_setSelectionArea_path_selectionOperation_mode(QGraphicsScene* this_ptr, const QPainterPath* path, const Qt::ItemSelectionOperation* selectionOperation, const Qt::ItemSelectionMode* mode) {
  this_ptr->setSelectionArea(*path, *selectionOperation, *mode);
}

void qt_widgets_c_QGraphicsScene_setSelectionArea_path_selectionOperation_mode_deviceTransform(QGraphicsScene* this_ptr, const QPainterPath* path, const Qt::ItemSelectionOperation* selectionOperation, const Qt::ItemSelectionMode* mode, const QTransform* deviceTransform) {
  this_ptr->setSelectionArea(*path, *selectionOperation, *mode, *deviceTransform);
}

void qt_widgets_c_QGraphicsScene_setSortCacheEnabled(QGraphicsScene* this_ptr, bool enabled) {
  this_ptr->setSortCacheEnabled(enabled);
}

void qt_widgets_c_QGraphicsScene_setStickyFocus(QGraphicsScene* this_ptr, bool enabled) {
  this_ptr->setStickyFocus(enabled);
}

void qt_widgets_c_QGraphicsScene_setStyle(QGraphicsScene* this_ptr, QStyle* style) {
  this_ptr->setStyle(style);
}

bool qt_widgets_c_QGraphicsScene_stickyFocus(const QGraphicsScene* this_ptr) {
  return this_ptr->stickyFocus();
}

QStyle* qt_widgets_c_QGraphicsScene_style(const QGraphicsScene* this_ptr) {
  return this_ptr->style();
}

void qt_widgets_c_QGraphicsScene_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsScene::trUtf8(s, c, n));
}

void qt_widgets_c_QGraphicsScene_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsScene::tr(s, c, n));
}

void qt_widgets_c_QGraphicsScene_update_no_args(QGraphicsScene* this_ptr) {
  this_ptr->update();
}

void qt_widgets_c_QGraphicsScene_update_rect(QGraphicsScene* this_ptr, const QRectF* rect) {
  this_ptr->update(*rect);
}

void qt_widgets_c_QGraphicsScene_update_x_y_w_h(QGraphicsScene* this_ptr, double x, double y, double w, double h) {
  this_ptr->update(x, y, w, h);
}

void qt_widgets_c_QGraphicsScene_views_to_output(const QGraphicsScene* this_ptr, QList< QGraphicsView* >* output) {
  new(output) QList< QGraphicsView* >(this_ptr->views());
}

double qt_widgets_c_QGraphicsScene_width(const QGraphicsScene* this_ptr) {
  return this_ptr->width();
}

