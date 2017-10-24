#include "qt_widgets_c_QGraphicsItem.h"

void qt_widgets_c_QGraphicsItem_G_operator_shl_to_output_QDebug_QGraphicsItem(const QDebug* debug, QGraphicsItem* item, QDebug* output) {
  new(output) QDebug(operator<<(*debug, item));
}

void qt_widgets_c_QGraphicsItem_G_operator_shl_to_output_QDebug_QGraphicsItem_GraphicsItemChange(const QDebug* debug, const QGraphicsItem::GraphicsItemChange* change, QDebug* output) {
  new(output) QDebug(operator<<(*debug, *change));
}

void qt_widgets_c_QGraphicsItem_G_operator_shl_to_output_QDebug_QGraphicsItem_GraphicsItemFlag(const QDebug* debug, const QGraphicsItem::GraphicsItemFlag* flag, QDebug* output) {
  new(output) QDebug(operator<<(*debug, *flag));
}

void qt_widgets_c_QGraphicsItem_G_operator_shl_to_output_QDebug_QGraphicsObject(const QDebug* debug, QGraphicsObject* item, QDebug* output) {
  new(output) QDebug(operator<<(*debug, item));
}

bool qt_widgets_c_QGraphicsItem_acceptDrops(const QGraphicsItem* this_ptr) {
  return this_ptr->acceptDrops();
}

bool qt_widgets_c_QGraphicsItem_acceptHoverEvents(const QGraphicsItem* this_ptr) {
  return this_ptr->acceptHoverEvents();
}

bool qt_widgets_c_QGraphicsItem_acceptTouchEvents(const QGraphicsItem* this_ptr) {
  return this_ptr->acceptTouchEvents();
}

void qt_widgets_c_QGraphicsItem_advance(QGraphicsItem* this_ptr, int phase) {
  this_ptr->advance(phase);
}

void qt_widgets_c_QGraphicsItem_boundingRect_to_output(const QGraphicsItem* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect());
}

double qt_widgets_c_QGraphicsItem_boundingRegionGranularity(const QGraphicsItem* this_ptr) {
  return this_ptr->boundingRegionGranularity();
}

QRegion* qt_widgets_c_QGraphicsItem_boundingRegion_as_ptr(const QGraphicsItem* this_ptr, const QTransform* itemToDeviceTransform) {
  return new QRegion(this_ptr->boundingRegion(*itemToDeviceTransform));
}

QGraphicsItem::CacheMode qt_widgets_c_QGraphicsItem_cacheMode(const QGraphicsItem* this_ptr) {
  return this_ptr->cacheMode();
}

void qt_widgets_c_QGraphicsItem_childItems_to_output(const QGraphicsItem* this_ptr, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->childItems());
}

void qt_widgets_c_QGraphicsItem_childrenBoundingRect_to_output(const QGraphicsItem* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->childrenBoundingRect());
}

void qt_widgets_c_QGraphicsItem_clearFocus(QGraphicsItem* this_ptr) {
  this_ptr->clearFocus();
}

void qt_widgets_c_QGraphicsItem_clipPath_to_output(const QGraphicsItem* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->clipPath());
}

bool qt_widgets_c_QGraphicsItem_collidesWithItem_other(const QGraphicsItem* this_ptr, const QGraphicsItem* other) {
  return this_ptr->collidesWithItem(other);
}

bool qt_widgets_c_QGraphicsItem_collidesWithItem_other_mode(const QGraphicsItem* this_ptr, const QGraphicsItem* other, const Qt::ItemSelectionMode* mode) {
  return this_ptr->collidesWithItem(other, *mode);
}

bool qt_widgets_c_QGraphicsItem_collidesWithPath_path(const QGraphicsItem* this_ptr, const QPainterPath* path) {
  return this_ptr->collidesWithPath(*path);
}

bool qt_widgets_c_QGraphicsItem_collidesWithPath_path_mode(const QGraphicsItem* this_ptr, const QPainterPath* path, const Qt::ItemSelectionMode* mode) {
  return this_ptr->collidesWithPath(*path, *mode);
}

void qt_widgets_c_QGraphicsItem_collidingItems_to_output_mode(const QGraphicsItem* this_ptr, const Qt::ItemSelectionMode* mode, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->collidingItems(*mode));
}

void qt_widgets_c_QGraphicsItem_collidingItems_to_output_no_args(const QGraphicsItem* this_ptr, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->collidingItems());
}

QGraphicsItem* qt_widgets_c_QGraphicsItem_commonAncestorItem(const QGraphicsItem* this_ptr, const QGraphicsItem* other) {
  return this_ptr->commonAncestorItem(other);
}

bool qt_widgets_c_QGraphicsItem_contains(const QGraphicsItem* this_ptr, const QPointF* point) {
  return this_ptr->contains(*point);
}

QCursor* qt_widgets_c_QGraphicsItem_cursor_as_ptr(const QGraphicsItem* this_ptr) {
  return new QCursor(this_ptr->cursor());
}

void qt_widgets_c_QGraphicsItem_data_to_output(const QGraphicsItem* this_ptr, int key, QVariant* output) {
  new(output) QVariant(this_ptr->data(key));
}

void qt_widgets_c_QGraphicsItem_delete(QGraphicsItem* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QGraphicsItem_deviceTransform_to_output(const QGraphicsItem* this_ptr, const QTransform* viewportTransform, QTransform* output) {
  new(output) QTransform(this_ptr->deviceTransform(*viewportTransform));
}

double qt_widgets_c_QGraphicsItem_effectiveOpacity(const QGraphicsItem* this_ptr) {
  return this_ptr->effectiveOpacity();
}

void qt_widgets_c_QGraphicsItem_ensureVisible_no_args(QGraphicsItem* this_ptr) {
  this_ptr->ensureVisible();
}

void qt_widgets_c_QGraphicsItem_ensureVisible_rect(QGraphicsItem* this_ptr, const QRectF* rect) {
  this_ptr->ensureVisible(*rect);
}

void qt_widgets_c_QGraphicsItem_ensureVisible_rect_xmargin(QGraphicsItem* this_ptr, const QRectF* rect, int xmargin) {
  this_ptr->ensureVisible(*rect, xmargin);
}

void qt_widgets_c_QGraphicsItem_ensureVisible_rect_xmargin_ymargin(QGraphicsItem* this_ptr, const QRectF* rect, int xmargin, int ymargin) {
  this_ptr->ensureVisible(*rect, xmargin, ymargin);
}

void qt_widgets_c_QGraphicsItem_ensureVisible_x_y_w_h(QGraphicsItem* this_ptr, double x, double y, double w, double h) {
  this_ptr->ensureVisible(x, y, w, h);
}

void qt_widgets_c_QGraphicsItem_ensureVisible_x_y_w_h_xmargin(QGraphicsItem* this_ptr, double x, double y, double w, double h, int xmargin) {
  this_ptr->ensureVisible(x, y, w, h, xmargin);
}

void qt_widgets_c_QGraphicsItem_ensureVisible_x_y_w_h_xmargin_ymargin(QGraphicsItem* this_ptr, double x, double y, double w, double h, int xmargin, int ymargin) {
  this_ptr->ensureVisible(x, y, w, h, xmargin, ymargin);
}

bool qt_widgets_c_QGraphicsItem_filtersChildEvents(const QGraphicsItem* this_ptr) {
  return this_ptr->filtersChildEvents();
}

unsigned int qt_widgets_c_QGraphicsItem_flags(const QGraphicsItem* this_ptr) {
  return uint(this_ptr->flags());
}

QGraphicsItem* qt_widgets_c_QGraphicsItem_focusItem(const QGraphicsItem* this_ptr) {
  return this_ptr->focusItem();
}

QGraphicsItem* qt_widgets_c_QGraphicsItem_focusProxy(const QGraphicsItem* this_ptr) {
  return this_ptr->focusProxy();
}

QGraphicsItem* qt_widgets_c_QGraphicsItem_focusScopeItem(const QGraphicsItem* this_ptr) {
  return this_ptr->focusScopeItem();
}

void qt_widgets_c_QGraphicsItem_grabKeyboard(QGraphicsItem* this_ptr) {
  this_ptr->grabKeyboard();
}

void qt_widgets_c_QGraphicsItem_grabMouse(QGraphicsItem* this_ptr) {
  this_ptr->grabMouse();
}

QGraphicsEffect* qt_widgets_c_QGraphicsItem_graphicsEffect(const QGraphicsItem* this_ptr) {
  return this_ptr->graphicsEffect();
}

QGraphicsItemGroup* qt_widgets_c_QGraphicsItem_group(const QGraphicsItem* this_ptr) {
  return this_ptr->group();
}

bool qt_widgets_c_QGraphicsItem_handlesChildEvents(const QGraphicsItem* this_ptr) {
  return this_ptr->handlesChildEvents();
}

bool qt_widgets_c_QGraphicsItem_hasCursor(const QGraphicsItem* this_ptr) {
  return this_ptr->hasCursor();
}

bool qt_widgets_c_QGraphicsItem_hasFocus(const QGraphicsItem* this_ptr) {
  return this_ptr->hasFocus();
}

void qt_widgets_c_QGraphicsItem_hide(QGraphicsItem* this_ptr) {
  this_ptr->hide();
}

void qt_widgets_c_QGraphicsItem_installSceneEventFilter(QGraphicsItem* this_ptr, QGraphicsItem* filterItem) {
  this_ptr->installSceneEventFilter(filterItem);
}

bool qt_widgets_c_QGraphicsItem_isActive(const QGraphicsItem* this_ptr) {
  return this_ptr->isActive();
}

bool qt_widgets_c_QGraphicsItem_isAncestorOf(const QGraphicsItem* this_ptr, const QGraphicsItem* child) {
  return this_ptr->isAncestorOf(child);
}

bool qt_widgets_c_QGraphicsItem_isBlockedByModalPanel_blockingPanel(const QGraphicsItem* this_ptr, QGraphicsItem** blockingPanel) {
  return this_ptr->isBlockedByModalPanel(blockingPanel);
}

bool qt_widgets_c_QGraphicsItem_isBlockedByModalPanel_no_args(const QGraphicsItem* this_ptr) {
  return this_ptr->isBlockedByModalPanel();
}

bool qt_widgets_c_QGraphicsItem_isClipped(const QGraphicsItem* this_ptr) {
  return this_ptr->isClipped();
}

bool qt_widgets_c_QGraphicsItem_isEnabled(const QGraphicsItem* this_ptr) {
  return this_ptr->isEnabled();
}

bool qt_widgets_c_QGraphicsItem_isObscuredBy(const QGraphicsItem* this_ptr, const QGraphicsItem* item) {
  return this_ptr->isObscuredBy(item);
}

bool qt_widgets_c_QGraphicsItem_isObscured_no_args(const QGraphicsItem* this_ptr) {
  return this_ptr->isObscured();
}

bool qt_widgets_c_QGraphicsItem_isObscured_rect(const QGraphicsItem* this_ptr, const QRectF* rect) {
  return this_ptr->isObscured(*rect);
}

bool qt_widgets_c_QGraphicsItem_isObscured_x_y_w_h(const QGraphicsItem* this_ptr, double x, double y, double w, double h) {
  return this_ptr->isObscured(x, y, w, h);
}

bool qt_widgets_c_QGraphicsItem_isPanel(const QGraphicsItem* this_ptr) {
  return this_ptr->isPanel();
}

bool qt_widgets_c_QGraphicsItem_isSelected(const QGraphicsItem* this_ptr) {
  return this_ptr->isSelected();
}

bool qt_widgets_c_QGraphicsItem_isUnderMouse(const QGraphicsItem* this_ptr) {
  return this_ptr->isUnderMouse();
}

bool qt_widgets_c_QGraphicsItem_isVisible(const QGraphicsItem* this_ptr) {
  return this_ptr->isVisible();
}

bool qt_widgets_c_QGraphicsItem_isVisibleTo(const QGraphicsItem* this_ptr, const QGraphicsItem* parent) {
  return this_ptr->isVisibleTo(parent);
}

bool qt_widgets_c_QGraphicsItem_isWidget(const QGraphicsItem* this_ptr) {
  return this_ptr->isWidget();
}

bool qt_widgets_c_QGraphicsItem_isWindow(const QGraphicsItem* this_ptr) {
  return this_ptr->isWindow();
}

void qt_widgets_c_QGraphicsItem_itemTransform_to_output_other(const QGraphicsItem* this_ptr, const QGraphicsItem* other, QTransform* output) {
  new(output) QTransform(this_ptr->itemTransform(other));
}

void qt_widgets_c_QGraphicsItem_itemTransform_to_output_other_ok(const QGraphicsItem* this_ptr, const QGraphicsItem* other, bool* ok, QTransform* output) {
  new(output) QTransform(this_ptr->itemTransform(other, ok));
}

void qt_widgets_c_QGraphicsItem_mapFromItem_to_output_item_path(const QGraphicsItem* this_ptr, const QGraphicsItem* item, const QPainterPath* path, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->mapFromItem(item, *path));
}

void qt_widgets_c_QGraphicsItem_mapFromItem_to_output_item_point(const QGraphicsItem* this_ptr, const QGraphicsItem* item, const QPointF* point, QPointF* output) {
  new(output) QPointF(this_ptr->mapFromItem(item, *point));
}

void qt_widgets_c_QGraphicsItem_mapFromItem_to_output_item_polygon(const QGraphicsItem* this_ptr, const QGraphicsItem* item, const QPolygonF* polygon, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->mapFromItem(item, *polygon));
}

void qt_widgets_c_QGraphicsItem_mapFromItem_to_output_item_rect(const QGraphicsItem* this_ptr, const QGraphicsItem* item, const QRectF* rect, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->mapFromItem(item, *rect));
}

void qt_widgets_c_QGraphicsItem_mapFromItem_to_output_item_x_y(const QGraphicsItem* this_ptr, const QGraphicsItem* item, double x, double y, QPointF* output) {
  new(output) QPointF(this_ptr->mapFromItem(item, x, y));
}

void qt_widgets_c_QGraphicsItem_mapFromItem_to_output_item_x_y_w_h(const QGraphicsItem* this_ptr, const QGraphicsItem* item, double x, double y, double w, double h, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->mapFromItem(item, x, y, w, h));
}

void qt_widgets_c_QGraphicsItem_mapFromParent_to_output_path(const QGraphicsItem* this_ptr, const QPainterPath* path, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->mapFromParent(*path));
}

void qt_widgets_c_QGraphicsItem_mapFromParent_to_output_point(const QGraphicsItem* this_ptr, const QPointF* point, QPointF* output) {
  new(output) QPointF(this_ptr->mapFromParent(*point));
}

void qt_widgets_c_QGraphicsItem_mapFromParent_to_output_polygon(const QGraphicsItem* this_ptr, const QPolygonF* polygon, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->mapFromParent(*polygon));
}

void qt_widgets_c_QGraphicsItem_mapFromParent_to_output_rect(const QGraphicsItem* this_ptr, const QRectF* rect, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->mapFromParent(*rect));
}

void qt_widgets_c_QGraphicsItem_mapFromParent_to_output_x_y(const QGraphicsItem* this_ptr, double x, double y, QPointF* output) {
  new(output) QPointF(this_ptr->mapFromParent(x, y));
}

void qt_widgets_c_QGraphicsItem_mapFromParent_to_output_x_y_w_h(const QGraphicsItem* this_ptr, double x, double y, double w, double h, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->mapFromParent(x, y, w, h));
}

void qt_widgets_c_QGraphicsItem_mapFromScene_to_output_path(const QGraphicsItem* this_ptr, const QPainterPath* path, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->mapFromScene(*path));
}

void qt_widgets_c_QGraphicsItem_mapFromScene_to_output_point(const QGraphicsItem* this_ptr, const QPointF* point, QPointF* output) {
  new(output) QPointF(this_ptr->mapFromScene(*point));
}

void qt_widgets_c_QGraphicsItem_mapFromScene_to_output_polygon(const QGraphicsItem* this_ptr, const QPolygonF* polygon, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->mapFromScene(*polygon));
}

void qt_widgets_c_QGraphicsItem_mapFromScene_to_output_rect(const QGraphicsItem* this_ptr, const QRectF* rect, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->mapFromScene(*rect));
}

void qt_widgets_c_QGraphicsItem_mapFromScene_to_output_x_y(const QGraphicsItem* this_ptr, double x, double y, QPointF* output) {
  new(output) QPointF(this_ptr->mapFromScene(x, y));
}

void qt_widgets_c_QGraphicsItem_mapFromScene_to_output_x_y_w_h(const QGraphicsItem* this_ptr, double x, double y, double w, double h, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->mapFromScene(x, y, w, h));
}

void qt_widgets_c_QGraphicsItem_mapRectFromItem_to_output_item_rect(const QGraphicsItem* this_ptr, const QGraphicsItem* item, const QRectF* rect, QRectF* output) {
  new(output) QRectF(this_ptr->mapRectFromItem(item, *rect));
}

void qt_widgets_c_QGraphicsItem_mapRectFromItem_to_output_item_x_y_w_h(const QGraphicsItem* this_ptr, const QGraphicsItem* item, double x, double y, double w, double h, QRectF* output) {
  new(output) QRectF(this_ptr->mapRectFromItem(item, x, y, w, h));
}

void qt_widgets_c_QGraphicsItem_mapRectFromParent_to_output_rect(const QGraphicsItem* this_ptr, const QRectF* rect, QRectF* output) {
  new(output) QRectF(this_ptr->mapRectFromParent(*rect));
}

void qt_widgets_c_QGraphicsItem_mapRectFromParent_to_output_x_y_w_h(const QGraphicsItem* this_ptr, double x, double y, double w, double h, QRectF* output) {
  new(output) QRectF(this_ptr->mapRectFromParent(x, y, w, h));
}

void qt_widgets_c_QGraphicsItem_mapRectFromScene_to_output_rect(const QGraphicsItem* this_ptr, const QRectF* rect, QRectF* output) {
  new(output) QRectF(this_ptr->mapRectFromScene(*rect));
}

void qt_widgets_c_QGraphicsItem_mapRectFromScene_to_output_x_y_w_h(const QGraphicsItem* this_ptr, double x, double y, double w, double h, QRectF* output) {
  new(output) QRectF(this_ptr->mapRectFromScene(x, y, w, h));
}

void qt_widgets_c_QGraphicsItem_mapRectToItem_to_output_item_rect(const QGraphicsItem* this_ptr, const QGraphicsItem* item, const QRectF* rect, QRectF* output) {
  new(output) QRectF(this_ptr->mapRectToItem(item, *rect));
}

void qt_widgets_c_QGraphicsItem_mapRectToItem_to_output_item_x_y_w_h(const QGraphicsItem* this_ptr, const QGraphicsItem* item, double x, double y, double w, double h, QRectF* output) {
  new(output) QRectF(this_ptr->mapRectToItem(item, x, y, w, h));
}

void qt_widgets_c_QGraphicsItem_mapRectToParent_to_output_rect(const QGraphicsItem* this_ptr, const QRectF* rect, QRectF* output) {
  new(output) QRectF(this_ptr->mapRectToParent(*rect));
}

void qt_widgets_c_QGraphicsItem_mapRectToParent_to_output_x_y_w_h(const QGraphicsItem* this_ptr, double x, double y, double w, double h, QRectF* output) {
  new(output) QRectF(this_ptr->mapRectToParent(x, y, w, h));
}

void qt_widgets_c_QGraphicsItem_mapRectToScene_to_output_rect(const QGraphicsItem* this_ptr, const QRectF* rect, QRectF* output) {
  new(output) QRectF(this_ptr->mapRectToScene(*rect));
}

void qt_widgets_c_QGraphicsItem_mapRectToScene_to_output_x_y_w_h(const QGraphicsItem* this_ptr, double x, double y, double w, double h, QRectF* output) {
  new(output) QRectF(this_ptr->mapRectToScene(x, y, w, h));
}

void qt_widgets_c_QGraphicsItem_mapToItem_to_output_item_path(const QGraphicsItem* this_ptr, const QGraphicsItem* item, const QPainterPath* path, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->mapToItem(item, *path));
}

void qt_widgets_c_QGraphicsItem_mapToItem_to_output_item_point(const QGraphicsItem* this_ptr, const QGraphicsItem* item, const QPointF* point, QPointF* output) {
  new(output) QPointF(this_ptr->mapToItem(item, *point));
}

void qt_widgets_c_QGraphicsItem_mapToItem_to_output_item_polygon(const QGraphicsItem* this_ptr, const QGraphicsItem* item, const QPolygonF* polygon, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->mapToItem(item, *polygon));
}

void qt_widgets_c_QGraphicsItem_mapToItem_to_output_item_rect(const QGraphicsItem* this_ptr, const QGraphicsItem* item, const QRectF* rect, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->mapToItem(item, *rect));
}

void qt_widgets_c_QGraphicsItem_mapToItem_to_output_item_x_y(const QGraphicsItem* this_ptr, const QGraphicsItem* item, double x, double y, QPointF* output) {
  new(output) QPointF(this_ptr->mapToItem(item, x, y));
}

void qt_widgets_c_QGraphicsItem_mapToItem_to_output_item_x_y_w_h(const QGraphicsItem* this_ptr, const QGraphicsItem* item, double x, double y, double w, double h, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->mapToItem(item, x, y, w, h));
}

void qt_widgets_c_QGraphicsItem_mapToParent_to_output_path(const QGraphicsItem* this_ptr, const QPainterPath* path, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->mapToParent(*path));
}

void qt_widgets_c_QGraphicsItem_mapToParent_to_output_point(const QGraphicsItem* this_ptr, const QPointF* point, QPointF* output) {
  new(output) QPointF(this_ptr->mapToParent(*point));
}

void qt_widgets_c_QGraphicsItem_mapToParent_to_output_polygon(const QGraphicsItem* this_ptr, const QPolygonF* polygon, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->mapToParent(*polygon));
}

void qt_widgets_c_QGraphicsItem_mapToParent_to_output_rect(const QGraphicsItem* this_ptr, const QRectF* rect, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->mapToParent(*rect));
}

void qt_widgets_c_QGraphicsItem_mapToParent_to_output_x_y(const QGraphicsItem* this_ptr, double x, double y, QPointF* output) {
  new(output) QPointF(this_ptr->mapToParent(x, y));
}

void qt_widgets_c_QGraphicsItem_mapToParent_to_output_x_y_w_h(const QGraphicsItem* this_ptr, double x, double y, double w, double h, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->mapToParent(x, y, w, h));
}

void qt_widgets_c_QGraphicsItem_mapToScene_to_output_path(const QGraphicsItem* this_ptr, const QPainterPath* path, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->mapToScene(*path));
}

void qt_widgets_c_QGraphicsItem_mapToScene_to_output_point(const QGraphicsItem* this_ptr, const QPointF* point, QPointF* output) {
  new(output) QPointF(this_ptr->mapToScene(*point));
}

void qt_widgets_c_QGraphicsItem_mapToScene_to_output_polygon(const QGraphicsItem* this_ptr, const QPolygonF* polygon, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->mapToScene(*polygon));
}

void qt_widgets_c_QGraphicsItem_mapToScene_to_output_rect(const QGraphicsItem* this_ptr, const QRectF* rect, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->mapToScene(*rect));
}

void qt_widgets_c_QGraphicsItem_mapToScene_to_output_x_y(const QGraphicsItem* this_ptr, double x, double y, QPointF* output) {
  new(output) QPointF(this_ptr->mapToScene(x, y));
}

void qt_widgets_c_QGraphicsItem_mapToScene_to_output_x_y_w_h(const QGraphicsItem* this_ptr, double x, double y, double w, double h, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->mapToScene(x, y, w, h));
}

void qt_widgets_c_QGraphicsItem_matrix_to_output(const QGraphicsItem* this_ptr, QMatrix* output) {
  new(output) QMatrix(this_ptr->matrix());
}

void qt_widgets_c_QGraphicsItem_moveBy(QGraphicsItem* this_ptr, double dx, double dy) {
  this_ptr->moveBy(dx, dy);
}

double qt_widgets_c_QGraphicsItem_opacity(const QGraphicsItem* this_ptr) {
  return this_ptr->opacity();
}

void qt_widgets_c_QGraphicsItem_opaqueArea_to_output(const QGraphicsItem* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->opaqueArea());
}

void qt_widgets_c_QGraphicsItem_paint_painter_option(QGraphicsItem* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option) {
  this_ptr->paint(painter, option);
}

void qt_widgets_c_QGraphicsItem_paint_painter_option_widget(QGraphicsItem* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget) {
  this_ptr->paint(painter, option, widget);
}

QGraphicsItem* qt_widgets_c_QGraphicsItem_panel(const QGraphicsItem* this_ptr) {
  return this_ptr->panel();
}

QGraphicsItem::PanelModality qt_widgets_c_QGraphicsItem_panelModality(const QGraphicsItem* this_ptr) {
  return this_ptr->panelModality();
}

QGraphicsItem* qt_widgets_c_QGraphicsItem_parentItem(const QGraphicsItem* this_ptr) {
  return this_ptr->parentItem();
}

QGraphicsObject* qt_widgets_c_QGraphicsItem_parentObject(const QGraphicsItem* this_ptr) {
  return this_ptr->parentObject();
}

QGraphicsWidget* qt_widgets_c_QGraphicsItem_parentWidget(const QGraphicsItem* this_ptr) {
  return this_ptr->parentWidget();
}

void qt_widgets_c_QGraphicsItem_pos_to_output(const QGraphicsItem* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->pos());
}

void qt_widgets_c_QGraphicsItem_removeSceneEventFilter(QGraphicsItem* this_ptr, QGraphicsItem* filterItem) {
  this_ptr->removeSceneEventFilter(filterItem);
}

void qt_widgets_c_QGraphicsItem_resetMatrix(QGraphicsItem* this_ptr) {
  this_ptr->resetMatrix();
}

void qt_widgets_c_QGraphicsItem_resetTransform(QGraphicsItem* this_ptr) {
  this_ptr->resetTransform();
}

double qt_widgets_c_QGraphicsItem_rotation(const QGraphicsItem* this_ptr) {
  return this_ptr->rotation();
}

double qt_widgets_c_QGraphicsItem_scale(const QGraphicsItem* this_ptr) {
  return this_ptr->scale();
}

QGraphicsScene* qt_widgets_c_QGraphicsItem_scene(const QGraphicsItem* this_ptr) {
  return this_ptr->scene();
}

void qt_widgets_c_QGraphicsItem_sceneBoundingRect_to_output(const QGraphicsItem* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->sceneBoundingRect());
}

void qt_widgets_c_QGraphicsItem_sceneMatrix_to_output(const QGraphicsItem* this_ptr, QMatrix* output) {
  new(output) QMatrix(this_ptr->sceneMatrix());
}

void qt_widgets_c_QGraphicsItem_scenePos_to_output(const QGraphicsItem* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->scenePos());
}

void qt_widgets_c_QGraphicsItem_sceneTransform_to_output(const QGraphicsItem* this_ptr, QTransform* output) {
  new(output) QTransform(this_ptr->sceneTransform());
}

void qt_widgets_c_QGraphicsItem_scroll_dx_dy(QGraphicsItem* this_ptr, double dx, double dy) {
  this_ptr->scroll(dx, dy);
}

void qt_widgets_c_QGraphicsItem_scroll_dx_dy_rect(QGraphicsItem* this_ptr, double dx, double dy, const QRectF* rect) {
  this_ptr->scroll(dx, dy, *rect);
}

void qt_widgets_c_QGraphicsItem_setAcceptDrops(QGraphicsItem* this_ptr, bool on) {
  this_ptr->setAcceptDrops(on);
}

void qt_widgets_c_QGraphicsItem_setAcceptHoverEvents(QGraphicsItem* this_ptr, bool enabled) {
  this_ptr->setAcceptHoverEvents(enabled);
}

void qt_widgets_c_QGraphicsItem_setAcceptTouchEvents(QGraphicsItem* this_ptr, bool enabled) {
  this_ptr->setAcceptTouchEvents(enabled);
}

void qt_widgets_c_QGraphicsItem_setActive(QGraphicsItem* this_ptr, bool active) {
  this_ptr->setActive(active);
}

void qt_widgets_c_QGraphicsItem_setBoundingRegionGranularity(QGraphicsItem* this_ptr, double granularity) {
  this_ptr->setBoundingRegionGranularity(granularity);
}

void qt_widgets_c_QGraphicsItem_setCacheMode_mode(QGraphicsItem* this_ptr, QGraphicsItem::CacheMode mode) {
  this_ptr->setCacheMode(mode);
}

void qt_widgets_c_QGraphicsItem_setCacheMode_mode_cacheSize(QGraphicsItem* this_ptr, QGraphicsItem::CacheMode mode, const QSize* cacheSize) {
  this_ptr->setCacheMode(mode, *cacheSize);
}

void qt_widgets_c_QGraphicsItem_setCursor(QGraphicsItem* this_ptr, const QCursor* cursor) {
  this_ptr->setCursor(*cursor);
}

void qt_widgets_c_QGraphicsItem_setData(QGraphicsItem* this_ptr, int key, const QVariant* value) {
  this_ptr->setData(key, *value);
}

void qt_widgets_c_QGraphicsItem_setEnabled(QGraphicsItem* this_ptr, bool enabled) {
  this_ptr->setEnabled(enabled);
}

void qt_widgets_c_QGraphicsItem_setFiltersChildEvents(QGraphicsItem* this_ptr, bool enabled) {
  this_ptr->setFiltersChildEvents(enabled);
}

void qt_widgets_c_QGraphicsItem_setFlag_flag(QGraphicsItem* this_ptr, QGraphicsItem::GraphicsItemFlag flag) {
  this_ptr->setFlag(flag);
}

void qt_widgets_c_QGraphicsItem_setFlag_flag_enabled(QGraphicsItem* this_ptr, QGraphicsItem::GraphicsItemFlag flag, bool enabled) {
  this_ptr->setFlag(flag, enabled);
}

void qt_widgets_c_QGraphicsItem_setFlags(QGraphicsItem* this_ptr, unsigned int flags) {
  this_ptr->setFlags(QFlags< QGraphicsItem::GraphicsItemFlag >(flags));
}

void qt_widgets_c_QGraphicsItem_setFocusProxy(QGraphicsItem* this_ptr, QGraphicsItem* item) {
  this_ptr->setFocusProxy(item);
}

void qt_widgets_c_QGraphicsItem_setFocus_focusReason(QGraphicsItem* this_ptr, const Qt::FocusReason* focusReason) {
  this_ptr->setFocus(*focusReason);
}

void qt_widgets_c_QGraphicsItem_setFocus_no_args(QGraphicsItem* this_ptr) {
  this_ptr->setFocus();
}

void qt_widgets_c_QGraphicsItem_setGraphicsEffect(QGraphicsItem* this_ptr, QGraphicsEffect* effect) {
  this_ptr->setGraphicsEffect(effect);
}

void qt_widgets_c_QGraphicsItem_setGroup(QGraphicsItem* this_ptr, QGraphicsItemGroup* group) {
  this_ptr->setGroup(group);
}

void qt_widgets_c_QGraphicsItem_setHandlesChildEvents(QGraphicsItem* this_ptr, bool enabled) {
  this_ptr->setHandlesChildEvents(enabled);
}

void qt_widgets_c_QGraphicsItem_setMatrix_matrix(QGraphicsItem* this_ptr, const QMatrix* matrix) {
  this_ptr->setMatrix(*matrix);
}

void qt_widgets_c_QGraphicsItem_setMatrix_matrix_combine(QGraphicsItem* this_ptr, const QMatrix* matrix, bool combine) {
  this_ptr->setMatrix(*matrix, combine);
}

void qt_widgets_c_QGraphicsItem_setOpacity(QGraphicsItem* this_ptr, double opacity) {
  this_ptr->setOpacity(opacity);
}

void qt_widgets_c_QGraphicsItem_setPanelModality(QGraphicsItem* this_ptr, QGraphicsItem::PanelModality panelModality) {
  this_ptr->setPanelModality(panelModality);
}

void qt_widgets_c_QGraphicsItem_setParentItem(QGraphicsItem* this_ptr, QGraphicsItem* parent) {
  this_ptr->setParentItem(parent);
}

void qt_widgets_c_QGraphicsItem_setPos_pos(QGraphicsItem* this_ptr, const QPointF* pos) {
  this_ptr->setPos(*pos);
}

void qt_widgets_c_QGraphicsItem_setPos_x_y(QGraphicsItem* this_ptr, double x, double y) {
  this_ptr->setPos(x, y);
}

void qt_widgets_c_QGraphicsItem_setRotation(QGraphicsItem* this_ptr, double angle) {
  this_ptr->setRotation(angle);
}

void qt_widgets_c_QGraphicsItem_setScale(QGraphicsItem* this_ptr, double scale) {
  this_ptr->setScale(scale);
}

void qt_widgets_c_QGraphicsItem_setSelected(QGraphicsItem* this_ptr, bool selected) {
  this_ptr->setSelected(selected);
}

void qt_widgets_c_QGraphicsItem_setToolTip(QGraphicsItem* this_ptr, const QString* toolTip) {
  this_ptr->setToolTip(*toolTip);
}

void qt_widgets_c_QGraphicsItem_setTransformOriginPoint_ax_ay(QGraphicsItem* this_ptr, double ax, double ay) {
  this_ptr->setTransformOriginPoint(ax, ay);
}

void qt_widgets_c_QGraphicsItem_setTransformOriginPoint_origin(QGraphicsItem* this_ptr, const QPointF* origin) {
  this_ptr->setTransformOriginPoint(*origin);
}

void qt_widgets_c_QGraphicsItem_setTransform_matrix(QGraphicsItem* this_ptr, const QTransform* matrix) {
  this_ptr->setTransform(*matrix);
}

void qt_widgets_c_QGraphicsItem_setTransform_matrix_combine(QGraphicsItem* this_ptr, const QTransform* matrix, bool combine) {
  this_ptr->setTransform(*matrix, combine);
}

void qt_widgets_c_QGraphicsItem_setTransformations(QGraphicsItem* this_ptr, const QList< QGraphicsTransform* >* transformations) {
  this_ptr->setTransformations(*transformations);
}

void qt_widgets_c_QGraphicsItem_setVisible(QGraphicsItem* this_ptr, bool visible) {
  this_ptr->setVisible(visible);
}

void qt_widgets_c_QGraphicsItem_setX(QGraphicsItem* this_ptr, double x) {
  this_ptr->setX(x);
}

void qt_widgets_c_QGraphicsItem_setY(QGraphicsItem* this_ptr, double y) {
  this_ptr->setY(y);
}

void qt_widgets_c_QGraphicsItem_setZValue(QGraphicsItem* this_ptr, double z) {
  this_ptr->setZValue(z);
}

void qt_widgets_c_QGraphicsItem_shape_to_output(const QGraphicsItem* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->shape());
}

void qt_widgets_c_QGraphicsItem_show(QGraphicsItem* this_ptr) {
  this_ptr->show();
}

void qt_widgets_c_QGraphicsItem_stackBefore(QGraphicsItem* this_ptr, const QGraphicsItem* sibling) {
  this_ptr->stackBefore(sibling);
}

QGraphicsObject* qt_widgets_c_QGraphicsItem_toGraphicsObject(QGraphicsItem* this_ptr) {
  return this_ptr->toGraphicsObject();
}

const QGraphicsObject* qt_widgets_c_QGraphicsItem_toGraphicsObject_const(const QGraphicsItem* this_ptr) {
  return this_ptr->toGraphicsObject();
}

void qt_widgets_c_QGraphicsItem_toolTip_to_output(const QGraphicsItem* this_ptr, QString* output) {
  new(output) QString(this_ptr->toolTip());
}

QGraphicsItem* qt_widgets_c_QGraphicsItem_topLevelItem(const QGraphicsItem* this_ptr) {
  return this_ptr->topLevelItem();
}

QGraphicsWidget* qt_widgets_c_QGraphicsItem_topLevelWidget(const QGraphicsItem* this_ptr) {
  return this_ptr->topLevelWidget();
}

void qt_widgets_c_QGraphicsItem_transformOriginPoint_to_output(const QGraphicsItem* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->transformOriginPoint());
}

void qt_widgets_c_QGraphicsItem_transform_to_output(const QGraphicsItem* this_ptr, QTransform* output) {
  new(output) QTransform(this_ptr->transform());
}

void qt_widgets_c_QGraphicsItem_transformations_to_output(const QGraphicsItem* this_ptr, QList< QGraphicsTransform* >* output) {
  new(output) QList< QGraphicsTransform* >(this_ptr->transformations());
}

int qt_widgets_c_QGraphicsItem_type(const QGraphicsItem* this_ptr) {
  return this_ptr->type();
}

void qt_widgets_c_QGraphicsItem_ungrabKeyboard(QGraphicsItem* this_ptr) {
  this_ptr->ungrabKeyboard();
}

void qt_widgets_c_QGraphicsItem_ungrabMouse(QGraphicsItem* this_ptr) {
  this_ptr->ungrabMouse();
}

void qt_widgets_c_QGraphicsItem_unsetCursor(QGraphicsItem* this_ptr) {
  this_ptr->unsetCursor();
}

void qt_widgets_c_QGraphicsItem_update_no_args(QGraphicsItem* this_ptr) {
  this_ptr->update();
}

void qt_widgets_c_QGraphicsItem_update_rect(QGraphicsItem* this_ptr, const QRectF* rect) {
  this_ptr->update(*rect);
}

void qt_widgets_c_QGraphicsItem_update_x_y_width_height(QGraphicsItem* this_ptr, double x, double y, double width, double height) {
  this_ptr->update(x, y, width, height);
}

QGraphicsWidget* qt_widgets_c_QGraphicsItem_window(const QGraphicsItem* this_ptr) {
  return this_ptr->window();
}

double qt_widgets_c_QGraphicsItem_x(const QGraphicsItem* this_ptr) {
  return this_ptr->x();
}

double qt_widgets_c_QGraphicsItem_y(const QGraphicsItem* this_ptr) {
  return this_ptr->y();
}

double qt_widgets_c_QGraphicsItem_zValue(const QGraphicsItem* this_ptr) {
  return this_ptr->zValue();
}

