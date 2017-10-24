#include "qt_widgets_c_QGraphicsWidget.h"

QGraphicsWidget* qt_widgets_c_QGraphicsWidget_G_dynamic_cast_QGraphicsWidget_ptr_QGraphicsItem(QGraphicsItem* ptr) {
  return dynamic_cast<QGraphicsWidget*>(ptr);
}

QGraphicsWidget* qt_widgets_c_QGraphicsWidget_G_dynamic_cast_QGraphicsWidget_ptr_QGraphicsLayoutItem(QGraphicsLayoutItem* ptr) {
  return dynamic_cast<QGraphicsWidget*>(ptr);
}

QGraphicsWidget* qt_widgets_c_QGraphicsWidget_G_dynamic_cast_QGraphicsWidget_ptr_QGraphicsObject(QGraphicsObject* ptr) {
  return dynamic_cast<QGraphicsWidget*>(ptr);
}

QGraphicsItem* qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsItem_ptr(QGraphicsWidget* ptr) {
  return static_cast<QGraphicsItem*>(ptr);
}

QGraphicsLayoutItem* qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsLayoutItem_ptr(QGraphicsWidget* ptr) {
  return static_cast<QGraphicsLayoutItem*>(ptr);
}

QGraphicsObject* qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsObject_ptr(QGraphicsWidget* ptr) {
  return static_cast<QGraphicsObject*>(ptr);
}

QGraphicsWidget* qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsWidget_ptr_QGraphicsItem(QGraphicsItem* ptr) {
  return static_cast<QGraphicsWidget*>(ptr);
}

QGraphicsWidget* qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsWidget_ptr_QGraphicsLayoutItem(QGraphicsLayoutItem* ptr) {
  return static_cast<QGraphicsWidget*>(ptr);
}

QGraphicsWidget* qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsWidget_ptr_QGraphicsObject(QGraphicsObject* ptr) {
  return static_cast<QGraphicsWidget*>(ptr);
}

QGraphicsWidget* qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsWidget_ptr_QObject(QObject* ptr) {
  return static_cast<QGraphicsWidget*>(ptr);
}

QObject* qt_widgets_c_QGraphicsWidget_G_static_cast_QObject_ptr(QGraphicsWidget* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_widgets_c_QGraphicsWidget_actions_to_output(const QGraphicsWidget* this_ptr, QList< QAction* >* output) {
  new(output) QList< QAction* >(this_ptr->actions());
}

void qt_widgets_c_QGraphicsWidget_addAction(QGraphicsWidget* this_ptr, QAction* action) {
  this_ptr->addAction(action);
}

void qt_widgets_c_QGraphicsWidget_addActions(QGraphicsWidget* this_ptr, const QList< QAction* >* actions) {
  this_ptr->addActions(*actions);
}

void qt_widgets_c_QGraphicsWidget_adjustSize(QGraphicsWidget* this_ptr) {
  this_ptr->adjustSize();
}

bool qt_widgets_c_QGraphicsWidget_autoFillBackground(const QGraphicsWidget* this_ptr) {
  return this_ptr->autoFillBackground();
}

void qt_widgets_c_QGraphicsWidget_boundingRect_to_output(const QGraphicsWidget* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect());
}

bool qt_widgets_c_QGraphicsWidget_close(QGraphicsWidget* this_ptr) {
  return this_ptr->close();
}

void qt_widgets_c_QGraphicsWidget_delete(QGraphicsWidget* this_ptr) {
  delete this_ptr;
}

QGraphicsWidget* qt_widgets_c_QGraphicsWidget_focusWidget(const QGraphicsWidget* this_ptr) {
  return this_ptr->focusWidget();
}

void qt_widgets_c_QGraphicsWidget_font_to_output(const QGraphicsWidget* this_ptr, QFont* output) {
  new(output) QFont(this_ptr->font());
}

void qt_widgets_c_QGraphicsWidget_getContentsMargins(const QGraphicsWidget* this_ptr, double* left, double* top, double* right, double* bottom) {
  this_ptr->getContentsMargins(left, top, right, bottom);
}

void qt_widgets_c_QGraphicsWidget_getWindowFrameMargins(const QGraphicsWidget* this_ptr, double* left, double* top, double* right, double* bottom) {
  this_ptr->getWindowFrameMargins(left, top, right, bottom);
}

int qt_widgets_c_QGraphicsWidget_grabShortcut_sequence(QGraphicsWidget* this_ptr, const QKeySequence* sequence) {
  return this_ptr->grabShortcut(*sequence);
}

int qt_widgets_c_QGraphicsWidget_grabShortcut_sequence_context(QGraphicsWidget* this_ptr, const QKeySequence* sequence, const Qt::ShortcutContext* context) {
  return this_ptr->grabShortcut(*sequence, *context);
}

void qt_widgets_c_QGraphicsWidget_insertAction(QGraphicsWidget* this_ptr, QAction* before, QAction* action) {
  this_ptr->insertAction(before, action);
}

void qt_widgets_c_QGraphicsWidget_insertActions(QGraphicsWidget* this_ptr, QAction* before, const QList< QAction* >* actions) {
  this_ptr->insertActions(before, *actions);
}

bool qt_widgets_c_QGraphicsWidget_isActiveWindow(const QGraphicsWidget* this_ptr) {
  return this_ptr->isActiveWindow();
}

QGraphicsLayout* qt_widgets_c_QGraphicsWidget_layout(const QGraphicsWidget* this_ptr) {
  return this_ptr->layout();
}

const QMetaObject* qt_widgets_c_QGraphicsWidget_metaObject(const QGraphicsWidget* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QGraphicsWidget_paintWindowFrame_painter_option(QGraphicsWidget* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option) {
  this_ptr->paintWindowFrame(painter, option);
}

void qt_widgets_c_QGraphicsWidget_paintWindowFrame_painter_option_widget(QGraphicsWidget* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget) {
  this_ptr->paintWindowFrame(painter, option, widget);
}

void qt_widgets_c_QGraphicsWidget_paint_painter_option(QGraphicsWidget* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option) {
  this_ptr->paint(painter, option);
}

void qt_widgets_c_QGraphicsWidget_paint_painter_option_widget(QGraphicsWidget* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget) {
  this_ptr->paint(painter, option, widget);
}

void qt_widgets_c_QGraphicsWidget_palette_to_output(const QGraphicsWidget* this_ptr, QPalette* output) {
  new(output) QPalette(this_ptr->palette());
}

int qt_widgets_c_QGraphicsWidget_qt_metacall(QGraphicsWidget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QGraphicsWidget_qt_metacast(QGraphicsWidget* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QGraphicsWidget_rect_to_output(const QGraphicsWidget* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->rect());
}

void qt_widgets_c_QGraphicsWidget_releaseShortcut(QGraphicsWidget* this_ptr, int id) {
  this_ptr->releaseShortcut(id);
}

void qt_widgets_c_QGraphicsWidget_removeAction(QGraphicsWidget* this_ptr, QAction* action) {
  this_ptr->removeAction(action);
}

void qt_widgets_c_QGraphicsWidget_resize_size(QGraphicsWidget* this_ptr, const QSizeF* size) {
  this_ptr->resize(*size);
}

void qt_widgets_c_QGraphicsWidget_resize_w_h(QGraphicsWidget* this_ptr, double w, double h) {
  this_ptr->resize(w, h);
}

void qt_widgets_c_QGraphicsWidget_setAttribute_attribute(QGraphicsWidget* this_ptr, const Qt::WidgetAttribute* attribute) {
  this_ptr->setAttribute(*attribute);
}

void qt_widgets_c_QGraphicsWidget_setAttribute_attribute_on(QGraphicsWidget* this_ptr, const Qt::WidgetAttribute* attribute, bool on) {
  this_ptr->setAttribute(*attribute, on);
}

void qt_widgets_c_QGraphicsWidget_setAutoFillBackground(QGraphicsWidget* this_ptr, bool enabled) {
  this_ptr->setAutoFillBackground(enabled);
}

void qt_widgets_c_QGraphicsWidget_setContentsMargins(QGraphicsWidget* this_ptr, double left, double top, double right, double bottom) {
  this_ptr->setContentsMargins(left, top, right, bottom);
}

void qt_widgets_c_QGraphicsWidget_setFocusPolicy(QGraphicsWidget* this_ptr, const Qt::FocusPolicy* policy) {
  this_ptr->setFocusPolicy(*policy);
}

void qt_widgets_c_QGraphicsWidget_setFont(QGraphicsWidget* this_ptr, const QFont* font) {
  this_ptr->setFont(*font);
}

void qt_widgets_c_QGraphicsWidget_setGeometry_rect(QGraphicsWidget* this_ptr, const QRectF* rect) {
  this_ptr->setGeometry(*rect);
}

void qt_widgets_c_QGraphicsWidget_setGeometry_x_y_w_h(QGraphicsWidget* this_ptr, double x, double y, double w, double h) {
  this_ptr->setGeometry(x, y, w, h);
}

void qt_widgets_c_QGraphicsWidget_setLayout(QGraphicsWidget* this_ptr, QGraphicsLayout* layout) {
  this_ptr->setLayout(layout);
}

void qt_widgets_c_QGraphicsWidget_setLayoutDirection(QGraphicsWidget* this_ptr, const Qt::LayoutDirection* direction) {
  this_ptr->setLayoutDirection(*direction);
}

void qt_widgets_c_QGraphicsWidget_setPalette(QGraphicsWidget* this_ptr, const QPalette* palette) {
  this_ptr->setPalette(*palette);
}

void qt_widgets_c_QGraphicsWidget_setShortcutAutoRepeat_id(QGraphicsWidget* this_ptr, int id) {
  this_ptr->setShortcutAutoRepeat(id);
}

void qt_widgets_c_QGraphicsWidget_setShortcutAutoRepeat_id_enabled(QGraphicsWidget* this_ptr, int id, bool enabled) {
  this_ptr->setShortcutAutoRepeat(id, enabled);
}

void qt_widgets_c_QGraphicsWidget_setShortcutEnabled_id(QGraphicsWidget* this_ptr, int id) {
  this_ptr->setShortcutEnabled(id);
}

void qt_widgets_c_QGraphicsWidget_setShortcutEnabled_id_enabled(QGraphicsWidget* this_ptr, int id, bool enabled) {
  this_ptr->setShortcutEnabled(id, enabled);
}

void qt_widgets_c_QGraphicsWidget_setStyle(QGraphicsWidget* this_ptr, QStyle* style) {
  this_ptr->setStyle(style);
}

void qt_widgets_c_QGraphicsWidget_setTabOrder(QGraphicsWidget* first, QGraphicsWidget* second) {
  QGraphicsWidget::setTabOrder(first, second);
}

void qt_widgets_c_QGraphicsWidget_setWindowFrameMargins(QGraphicsWidget* this_ptr, double left, double top, double right, double bottom) {
  this_ptr->setWindowFrameMargins(left, top, right, bottom);
}

void qt_widgets_c_QGraphicsWidget_setWindowTitle(QGraphicsWidget* this_ptr, const QString* title) {
  this_ptr->setWindowTitle(*title);
}

void qt_widgets_c_QGraphicsWidget_shape_to_output(const QGraphicsWidget* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->shape());
}

void qt_widgets_c_QGraphicsWidget_size_to_output(const QGraphicsWidget* this_ptr, QSizeF* output) {
  new(output) QSizeF(this_ptr->size());
}

QStyle* qt_widgets_c_QGraphicsWidget_style(const QGraphicsWidget* this_ptr) {
  return this_ptr->style();
}

bool qt_widgets_c_QGraphicsWidget_testAttribute(const QGraphicsWidget* this_ptr, const Qt::WidgetAttribute* attribute) {
  return this_ptr->testAttribute(*attribute);
}

void qt_widgets_c_QGraphicsWidget_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsWidget::trUtf8(s, c, n));
}

void qt_widgets_c_QGraphicsWidget_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsWidget::tr(s, c, n));
}

int qt_widgets_c_QGraphicsWidget_type(const QGraphicsWidget* this_ptr) {
  return this_ptr->type();
}

void qt_widgets_c_QGraphicsWidget_unsetLayoutDirection(QGraphicsWidget* this_ptr) {
  this_ptr->unsetLayoutDirection();
}

void qt_widgets_c_QGraphicsWidget_unsetWindowFrameMargins(QGraphicsWidget* this_ptr) {
  this_ptr->unsetWindowFrameMargins();
}

void qt_widgets_c_QGraphicsWidget_windowFrameGeometry_to_output(const QGraphicsWidget* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->windowFrameGeometry());
}

void qt_widgets_c_QGraphicsWidget_windowFrameRect_to_output(const QGraphicsWidget* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->windowFrameRect());
}

void qt_widgets_c_QGraphicsWidget_windowTitle_to_output(const QGraphicsWidget* this_ptr, QString* output) {
  new(output) QString(this_ptr->windowTitle());
}

