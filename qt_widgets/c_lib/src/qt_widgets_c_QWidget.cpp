#include "qt_widgets_c_QWidget.h"

void qt_widgets_c_QWidget_G_operator_shl_to_output(const QDebug* arg1, const QWidget* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, arg2));
}

QObject* qt_widgets_c_QWidget_G_static_cast_QObject_ptr(QWidget* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QWidget_G_static_cast_QPaintDevice_ptr(QWidget* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QWidget_G_static_cast_QWidget_ptr_QObject(QObject* ptr) {
  return static_cast<QWidget*>(ptr);
}

QWidget* qt_widgets_c_QWidget_G_static_cast_QWidget_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QWidget*>(ptr);
}

bool qt_widgets_c_QWidget_acceptDrops(const QWidget* this_ptr) {
  return this_ptr->acceptDrops();
}

void qt_widgets_c_QWidget_accessibleDescription_to_output(const QWidget* this_ptr, QString* output) {
  new(output) QString(this_ptr->accessibleDescription());
}

void qt_widgets_c_QWidget_accessibleName_to_output(const QWidget* this_ptr, QString* output) {
  new(output) QString(this_ptr->accessibleName());
}

void qt_widgets_c_QWidget_actions_to_output(const QWidget* this_ptr, QList< QAction* >* output) {
  new(output) QList< QAction* >(this_ptr->actions());
}

void qt_widgets_c_QWidget_activateWindow(QWidget* this_ptr) {
  this_ptr->activateWindow();
}

void qt_widgets_c_QWidget_addAction(QWidget* this_ptr, QAction* action) {
  this_ptr->addAction(action);
}

void qt_widgets_c_QWidget_addActions(QWidget* this_ptr, const QList< QAction* >* actions) {
  this_ptr->addActions(*actions);
}

void qt_widgets_c_QWidget_adjustSize(QWidget* this_ptr) {
  this_ptr->adjustSize();
}

bool qt_widgets_c_QWidget_autoFillBackground(const QWidget* this_ptr) {
  return this_ptr->autoFillBackground();
}

QBackingStore* qt_widgets_c_QWidget_backingStore(const QWidget* this_ptr) {
  return this_ptr->backingStore();
}

void qt_widgets_c_QWidget_baseSize_to_output(const QWidget* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->baseSize());
}

QWidget* qt_widgets_c_QWidget_childAt_p(const QWidget* this_ptr, const QPoint* p) {
  return this_ptr->childAt(*p);
}

QWidget* qt_widgets_c_QWidget_childAt_x_y(const QWidget* this_ptr, int x, int y) {
  return this_ptr->childAt(x, y);
}

void qt_widgets_c_QWidget_childrenRect_to_output(const QWidget* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->childrenRect());
}

QRegion* qt_widgets_c_QWidget_childrenRegion_as_ptr(const QWidget* this_ptr) {
  return new QRegion(this_ptr->childrenRegion());
}

void qt_widgets_c_QWidget_clearFocus(QWidget* this_ptr) {
  this_ptr->clearFocus();
}

void qt_widgets_c_QWidget_clearMask(QWidget* this_ptr) {
  this_ptr->clearMask();
}

bool qt_widgets_c_QWidget_close(QWidget* this_ptr) {
  return this_ptr->close();
}

void qt_widgets_c_QWidget_contentsMargins_to_output(const QWidget* this_ptr, QMargins* output) {
  new(output) QMargins(this_ptr->contentsMargins());
}

void qt_widgets_c_QWidget_contentsRect_to_output(const QWidget* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->contentsRect());
}

void qt_widgets_c_QWidget_createWinId(QWidget* this_ptr) {
  this_ptr->createWinId();
}

QCursor* qt_widgets_c_QWidget_cursor_as_ptr(const QWidget* this_ptr) {
  return new QCursor(this_ptr->cursor());
}

void qt_widgets_c_QWidget_delete(QWidget* this_ptr) {
  delete this_ptr;
}

int qt_widgets_c_QWidget_devType(const QWidget* this_ptr) {
  return this_ptr->devType();
}

unsigned long long qt_widgets_c_QWidget_effectiveWinId(const QWidget* this_ptr) {
  return this_ptr->effectiveWinId();
}

void qt_widgets_c_QWidget_ensurePolished(const QWidget* this_ptr) {
  this_ptr->ensurePolished();
}

QWidget* qt_widgets_c_QWidget_find(unsigned long long arg1) {
  return QWidget::find(arg1);
}

QWidget* qt_widgets_c_QWidget_focusProxy(const QWidget* this_ptr) {
  return this_ptr->focusProxy();
}

QWidget* qt_widgets_c_QWidget_focusWidget(const QWidget* this_ptr) {
  return this_ptr->focusWidget();
}

const QFont* qt_widgets_c_QWidget_font(const QWidget* this_ptr) {
  return &this_ptr->font();
}

void qt_widgets_c_QWidget_fontInfo_to_output(const QWidget* this_ptr, QFontInfo* output) {
  new(output) QFontInfo(this_ptr->fontInfo());
}

void qt_widgets_c_QWidget_fontMetrics_to_output(const QWidget* this_ptr, QFontMetrics* output) {
  new(output) QFontMetrics(this_ptr->fontMetrics());
}

void qt_widgets_c_QWidget_frameGeometry_to_output(const QWidget* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->frameGeometry());
}

void qt_widgets_c_QWidget_frameSize_to_output(const QWidget* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->frameSize());
}

const QRect* qt_widgets_c_QWidget_geometry(const QWidget* this_ptr) {
  return &this_ptr->geometry();
}

void qt_widgets_c_QWidget_getContentsMargins(const QWidget* this_ptr, int* left, int* top, int* right, int* bottom) {
  this_ptr->getContentsMargins(left, top, right, bottom);
}

void qt_widgets_c_QWidget_grabKeyboard(QWidget* this_ptr) {
  this_ptr->grabKeyboard();
}

void qt_widgets_c_QWidget_grabMouse_arg1(QWidget* this_ptr, const QCursor* arg1) {
  this_ptr->grabMouse(*arg1);
}

void qt_widgets_c_QWidget_grabMouse_no_args(QWidget* this_ptr) {
  this_ptr->grabMouse();
}

int qt_widgets_c_QWidget_grabShortcut_key(QWidget* this_ptr, const QKeySequence* key) {
  return this_ptr->grabShortcut(*key);
}

int qt_widgets_c_QWidget_grabShortcut_key_context(QWidget* this_ptr, const QKeySequence* key, const Qt::ShortcutContext* context) {
  return this_ptr->grabShortcut(*key, *context);
}

QPixmap* qt_widgets_c_QWidget_grab_as_ptr_no_args(QWidget* this_ptr) {
  return new QPixmap(this_ptr->grab());
}

QPixmap* qt_widgets_c_QWidget_grab_as_ptr_rectangle(QWidget* this_ptr, const QRect* rectangle) {
  return new QPixmap(this_ptr->grab(*rectangle));
}

QGraphicsEffect* qt_widgets_c_QWidget_graphicsEffect(const QWidget* this_ptr) {
  return this_ptr->graphicsEffect();
}

QGraphicsProxyWidget* qt_widgets_c_QWidget_graphicsProxyWidget(const QWidget* this_ptr) {
  return this_ptr->graphicsProxyWidget();
}

bool qt_widgets_c_QWidget_hasFocus(const QWidget* this_ptr) {
  return this_ptr->hasFocus();
}

bool qt_widgets_c_QWidget_hasHeightForWidth(const QWidget* this_ptr) {
  return this_ptr->hasHeightForWidth();
}

bool qt_widgets_c_QWidget_hasMouseTracking(const QWidget* this_ptr) {
  return this_ptr->hasMouseTracking();
}

bool qt_widgets_c_QWidget_hasTabletTracking(const QWidget* this_ptr) {
  return this_ptr->hasTabletTracking();
}

int qt_widgets_c_QWidget_height(const QWidget* this_ptr) {
  return this_ptr->height();
}

int qt_widgets_c_QWidget_heightForWidth(const QWidget* this_ptr, int arg1) {
  return this_ptr->heightForWidth(arg1);
}

void qt_widgets_c_QWidget_hide(QWidget* this_ptr) {
  this_ptr->hide();
}

void qt_widgets_c_QWidget_inputMethodQuery_to_output(const QWidget* this_ptr, const Qt::InputMethodQuery* arg1, QVariant* output) {
  new(output) QVariant(this_ptr->inputMethodQuery(*arg1));
}

void qt_widgets_c_QWidget_insertAction(QWidget* this_ptr, QAction* before, QAction* action) {
  this_ptr->insertAction(before, action);
}

void qt_widgets_c_QWidget_insertActions(QWidget* this_ptr, QAction* before, const QList< QAction* >* actions) {
  this_ptr->insertActions(before, *actions);
}

unsigned long long qt_widgets_c_QWidget_internalWinId(const QWidget* this_ptr) {
  return this_ptr->internalWinId();
}

bool qt_widgets_c_QWidget_isActiveWindow(const QWidget* this_ptr) {
  return this_ptr->isActiveWindow();
}

bool qt_widgets_c_QWidget_isAncestorOf(const QWidget* this_ptr, const QWidget* child) {
  return this_ptr->isAncestorOf(child);
}

bool qt_widgets_c_QWidget_isEnabled(const QWidget* this_ptr) {
  return this_ptr->isEnabled();
}

bool qt_widgets_c_QWidget_isEnabledTo(const QWidget* this_ptr, const QWidget* arg1) {
  return this_ptr->isEnabledTo(arg1);
}

bool qt_widgets_c_QWidget_isEnabledToTLW(const QWidget* this_ptr) {
  return this_ptr->isEnabledToTLW();
}

bool qt_widgets_c_QWidget_isFullScreen(const QWidget* this_ptr) {
  return this_ptr->isFullScreen();
}

bool qt_widgets_c_QWidget_isHidden(const QWidget* this_ptr) {
  return this_ptr->isHidden();
}

bool qt_widgets_c_QWidget_isLeftToRight(const QWidget* this_ptr) {
  return this_ptr->isLeftToRight();
}

bool qt_widgets_c_QWidget_isMaximized(const QWidget* this_ptr) {
  return this_ptr->isMaximized();
}

bool qt_widgets_c_QWidget_isMinimized(const QWidget* this_ptr) {
  return this_ptr->isMinimized();
}

bool qt_widgets_c_QWidget_isModal(const QWidget* this_ptr) {
  return this_ptr->isModal();
}

bool qt_widgets_c_QWidget_isRightToLeft(const QWidget* this_ptr) {
  return this_ptr->isRightToLeft();
}

bool qt_widgets_c_QWidget_isTopLevel(const QWidget* this_ptr) {
  return this_ptr->isTopLevel();
}

bool qt_widgets_c_QWidget_isVisible(const QWidget* this_ptr) {
  return this_ptr->isVisible();
}

bool qt_widgets_c_QWidget_isVisibleTo(const QWidget* this_ptr, const QWidget* arg1) {
  return this_ptr->isVisibleTo(arg1);
}

bool qt_widgets_c_QWidget_isWindow(const QWidget* this_ptr) {
  return this_ptr->isWindow();
}

bool qt_widgets_c_QWidget_isWindowModified(const QWidget* this_ptr) {
  return this_ptr->isWindowModified();
}

QWidget* qt_widgets_c_QWidget_keyboardGrabber() {
  return QWidget::keyboardGrabber();
}

QLayout* qt_widgets_c_QWidget_layout(const QWidget* this_ptr) {
  return this_ptr->layout();
}

void qt_widgets_c_QWidget_locale_to_output(const QWidget* this_ptr, QLocale* output) {
  new(output) QLocale(this_ptr->locale());
}

void qt_widgets_c_QWidget_lower(QWidget* this_ptr) {
  this_ptr->lower();
}

void qt_widgets_c_QWidget_mapFromGlobal_to_output(const QWidget* this_ptr, const QPoint* arg1, QPoint* output) {
  new(output) QPoint(this_ptr->mapFromGlobal(*arg1));
}

void qt_widgets_c_QWidget_mapFromParent_to_output(const QWidget* this_ptr, const QPoint* arg1, QPoint* output) {
  new(output) QPoint(this_ptr->mapFromParent(*arg1));
}

void qt_widgets_c_QWidget_mapFrom_to_output(const QWidget* this_ptr, const QWidget* arg1, const QPoint* arg2, QPoint* output) {
  new(output) QPoint(this_ptr->mapFrom(arg1, *arg2));
}

void qt_widgets_c_QWidget_mapToGlobal_to_output(const QWidget* this_ptr, const QPoint* arg1, QPoint* output) {
  new(output) QPoint(this_ptr->mapToGlobal(*arg1));
}

void qt_widgets_c_QWidget_mapToParent_to_output(const QWidget* this_ptr, const QPoint* arg1, QPoint* output) {
  new(output) QPoint(this_ptr->mapToParent(*arg1));
}

void qt_widgets_c_QWidget_mapTo_to_output(const QWidget* this_ptr, const QWidget* arg1, const QPoint* arg2, QPoint* output) {
  new(output) QPoint(this_ptr->mapTo(arg1, *arg2));
}

QRegion* qt_widgets_c_QWidget_mask_as_ptr(const QWidget* this_ptr) {
  return new QRegion(this_ptr->mask());
}

int qt_widgets_c_QWidget_maximumHeight(const QWidget* this_ptr) {
  return this_ptr->maximumHeight();
}

void qt_widgets_c_QWidget_maximumSize_to_output(const QWidget* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->maximumSize());
}

int qt_widgets_c_QWidget_maximumWidth(const QWidget* this_ptr) {
  return this_ptr->maximumWidth();
}

const QMetaObject* qt_widgets_c_QWidget_metaObject(const QWidget* this_ptr) {
  return this_ptr->metaObject();
}

int qt_widgets_c_QWidget_minimumHeight(const QWidget* this_ptr) {
  return this_ptr->minimumHeight();
}

void qt_widgets_c_QWidget_minimumSizeHint_to_output(const QWidget* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSizeHint());
}

void qt_widgets_c_QWidget_minimumSize_to_output(const QWidget* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSize());
}

int qt_widgets_c_QWidget_minimumWidth(const QWidget* this_ptr) {
  return this_ptr->minimumWidth();
}

QWidget* qt_widgets_c_QWidget_mouseGrabber() {
  return QWidget::mouseGrabber();
}

void qt_widgets_c_QWidget_move_arg1(QWidget* this_ptr, const QPoint* arg1) {
  this_ptr->move(*arg1);
}

void qt_widgets_c_QWidget_move_x_y(QWidget* this_ptr, int x, int y) {
  this_ptr->move(x, y);
}

QWidget* qt_widgets_c_QWidget_nativeParentWidget(const QWidget* this_ptr) {
  return this_ptr->nativeParentWidget();
}

QWidget* qt_widgets_c_QWidget_nextInFocusChain(const QWidget* this_ptr) {
  return this_ptr->nextInFocusChain();
}

void qt_widgets_c_QWidget_normalGeometry_to_output(const QWidget* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->normalGeometry());
}

QPaintEngine* qt_widgets_c_QWidget_paintEngine(const QWidget* this_ptr) {
  return this_ptr->paintEngine();
}

const QPalette* qt_widgets_c_QWidget_palette(const QWidget* this_ptr) {
  return &this_ptr->palette();
}

QWidget* qt_widgets_c_QWidget_parentWidget(const QWidget* this_ptr) {
  return this_ptr->parentWidget();
}

void qt_widgets_c_QWidget_pos_to_output(const QWidget* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->pos());
}

QWidget* qt_widgets_c_QWidget_previousInFocusChain(const QWidget* this_ptr) {
  return this_ptr->previousInFocusChain();
}

int qt_widgets_c_QWidget_qt_metacall(QWidget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QWidget_qt_metacast(QWidget* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QWidget_raise(QWidget* this_ptr) {
  this_ptr->raise();
}

void qt_widgets_c_QWidget_rect_to_output(const QWidget* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->rect());
}

void qt_widgets_c_QWidget_releaseKeyboard(QWidget* this_ptr) {
  this_ptr->releaseKeyboard();
}

void qt_widgets_c_QWidget_releaseMouse(QWidget* this_ptr) {
  this_ptr->releaseMouse();
}

void qt_widgets_c_QWidget_releaseShortcut(QWidget* this_ptr, int id) {
  this_ptr->releaseShortcut(id);
}

void qt_widgets_c_QWidget_removeAction(QWidget* this_ptr, QAction* action) {
  this_ptr->removeAction(action);
}

void qt_widgets_c_QWidget_render_painter(QWidget* this_ptr, QPainter* painter) {
  this_ptr->render(painter);
}

void qt_widgets_c_QWidget_render_painter_targetOffset(QWidget* this_ptr, QPainter* painter, const QPoint* targetOffset) {
  this_ptr->render(painter, *targetOffset);
}

void qt_widgets_c_QWidget_render_painter_targetOffset_sourceRegion(QWidget* this_ptr, QPainter* painter, const QPoint* targetOffset, const QRegion* sourceRegion) {
  this_ptr->render(painter, *targetOffset, *sourceRegion);
}

void qt_widgets_c_QWidget_render_painter_targetOffset_sourceRegion_renderFlags(QWidget* this_ptr, QPainter* painter, const QPoint* targetOffset, const QRegion* sourceRegion, unsigned int renderFlags) {
  this_ptr->render(painter, *targetOffset, *sourceRegion, QFlags< QWidget::RenderFlag >(renderFlags));
}

void qt_widgets_c_QWidget_render_target(QWidget* this_ptr, QPaintDevice* target) {
  this_ptr->render(target);
}

void qt_widgets_c_QWidget_render_target_targetOffset(QWidget* this_ptr, QPaintDevice* target, const QPoint* targetOffset) {
  this_ptr->render(target, *targetOffset);
}

void qt_widgets_c_QWidget_render_target_targetOffset_sourceRegion(QWidget* this_ptr, QPaintDevice* target, const QPoint* targetOffset, const QRegion* sourceRegion) {
  this_ptr->render(target, *targetOffset, *sourceRegion);
}

void qt_widgets_c_QWidget_render_target_targetOffset_sourceRegion_renderFlags(QWidget* this_ptr, QPaintDevice* target, const QPoint* targetOffset, const QRegion* sourceRegion, unsigned int renderFlags) {
  this_ptr->render(target, *targetOffset, *sourceRegion, QFlags< QWidget::RenderFlag >(renderFlags));
}

void qt_widgets_c_QWidget_repaint_QRect(QWidget* this_ptr, const QRect* arg1) {
  this_ptr->repaint(*arg1);
}

void qt_widgets_c_QWidget_repaint_QRegion(QWidget* this_ptr, const QRegion* arg1) {
  this_ptr->repaint(*arg1);
}

void qt_widgets_c_QWidget_repaint_int_int_int_int(QWidget* this_ptr, int x, int y, int w, int h) {
  this_ptr->repaint(x, y, w, h);
}

void qt_widgets_c_QWidget_repaint_no_args(QWidget* this_ptr) {
  this_ptr->repaint();
}

void qt_widgets_c_QWidget_resize_arg1(QWidget* this_ptr, const QSize* arg1) {
  this_ptr->resize(*arg1);
}

void qt_widgets_c_QWidget_resize_w_h(QWidget* this_ptr, int w, int h) {
  this_ptr->resize(w, h);
}

bool qt_widgets_c_QWidget_restoreGeometry(QWidget* this_ptr, const QByteArray* geometry) {
  return this_ptr->restoreGeometry(*geometry);
}

void qt_widgets_c_QWidget_saveGeometry_to_output(const QWidget* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->saveGeometry());
}

void qt_widgets_c_QWidget_scroll_dx_dy(QWidget* this_ptr, int dx, int dy) {
  this_ptr->scroll(dx, dy);
}

void qt_widgets_c_QWidget_scroll_dx_dy_arg3(QWidget* this_ptr, int dx, int dy, const QRect* arg3) {
  this_ptr->scroll(dx, dy, *arg3);
}

void qt_widgets_c_QWidget_setAcceptDrops(QWidget* this_ptr, bool on) {
  this_ptr->setAcceptDrops(on);
}

void qt_widgets_c_QWidget_setAccessibleDescription(QWidget* this_ptr, const QString* description) {
  this_ptr->setAccessibleDescription(*description);
}

void qt_widgets_c_QWidget_setAccessibleName(QWidget* this_ptr, const QString* name) {
  this_ptr->setAccessibleName(*name);
}

void qt_widgets_c_QWidget_setAttribute_arg1(QWidget* this_ptr, const Qt::WidgetAttribute* arg1) {
  this_ptr->setAttribute(*arg1);
}

void qt_widgets_c_QWidget_setAttribute_arg1_on(QWidget* this_ptr, const Qt::WidgetAttribute* arg1, bool on) {
  this_ptr->setAttribute(*arg1, on);
}

void qt_widgets_c_QWidget_setAutoFillBackground(QWidget* this_ptr, bool enabled) {
  this_ptr->setAutoFillBackground(enabled);
}

void qt_widgets_c_QWidget_setBackgroundRole(QWidget* this_ptr, const QPalette::ColorRole* arg1) {
  this_ptr->setBackgroundRole(*arg1);
}

void qt_widgets_c_QWidget_setBaseSize_arg1(QWidget* this_ptr, const QSize* arg1) {
  this_ptr->setBaseSize(*arg1);
}

void qt_widgets_c_QWidget_setBaseSize_basew_baseh(QWidget* this_ptr, int basew, int baseh) {
  this_ptr->setBaseSize(basew, baseh);
}

void qt_widgets_c_QWidget_setContentsMargins_left_top_right_bottom(QWidget* this_ptr, int left, int top, int right, int bottom) {
  this_ptr->setContentsMargins(left, top, right, bottom);
}

void qt_widgets_c_QWidget_setContentsMargins_margins(QWidget* this_ptr, const QMargins* margins) {
  this_ptr->setContentsMargins(*margins);
}

void qt_widgets_c_QWidget_setContextMenuPolicy(QWidget* this_ptr, const Qt::ContextMenuPolicy* policy) {
  this_ptr->setContextMenuPolicy(*policy);
}

void qt_widgets_c_QWidget_setCursor(QWidget* this_ptr, const QCursor* arg1) {
  this_ptr->setCursor(*arg1);
}

void qt_widgets_c_QWidget_setDisabled(QWidget* this_ptr, bool arg1) {
  this_ptr->setDisabled(arg1);
}

void qt_widgets_c_QWidget_setEnabled(QWidget* this_ptr, bool arg1) {
  this_ptr->setEnabled(arg1);
}

void qt_widgets_c_QWidget_setFixedHeight(QWidget* this_ptr, int h) {
  this_ptr->setFixedHeight(h);
}

void qt_widgets_c_QWidget_setFixedSize_arg1(QWidget* this_ptr, const QSize* arg1) {
  this_ptr->setFixedSize(*arg1);
}

void qt_widgets_c_QWidget_setFixedSize_w_h(QWidget* this_ptr, int w, int h) {
  this_ptr->setFixedSize(w, h);
}

void qt_widgets_c_QWidget_setFixedWidth(QWidget* this_ptr, int w) {
  this_ptr->setFixedWidth(w);
}

void qt_widgets_c_QWidget_setFocusPolicy(QWidget* this_ptr, const Qt::FocusPolicy* policy) {
  this_ptr->setFocusPolicy(*policy);
}

void qt_widgets_c_QWidget_setFocusProxy(QWidget* this_ptr, QWidget* arg1) {
  this_ptr->setFocusProxy(arg1);
}

void qt_widgets_c_QWidget_setFocus_no_args(QWidget* this_ptr) {
  this_ptr->setFocus();
}

void qt_widgets_c_QWidget_setFocus_reason(QWidget* this_ptr, const Qt::FocusReason* reason) {
  this_ptr->setFocus(*reason);
}

void qt_widgets_c_QWidget_setFont(QWidget* this_ptr, const QFont* arg1) {
  this_ptr->setFont(*arg1);
}

void qt_widgets_c_QWidget_setForegroundRole(QWidget* this_ptr, const QPalette::ColorRole* arg1) {
  this_ptr->setForegroundRole(*arg1);
}

void qt_widgets_c_QWidget_setGeometry_arg1(QWidget* this_ptr, const QRect* arg1) {
  this_ptr->setGeometry(*arg1);
}

void qt_widgets_c_QWidget_setGeometry_x_y_w_h(QWidget* this_ptr, int x, int y, int w, int h) {
  this_ptr->setGeometry(x, y, w, h);
}

void qt_widgets_c_QWidget_setGraphicsEffect(QWidget* this_ptr, QGraphicsEffect* effect) {
  this_ptr->setGraphicsEffect(effect);
}

void qt_widgets_c_QWidget_setHidden(QWidget* this_ptr, bool hidden) {
  this_ptr->setHidden(hidden);
}

void qt_widgets_c_QWidget_setLayout(QWidget* this_ptr, QLayout* arg1) {
  this_ptr->setLayout(arg1);
}

void qt_widgets_c_QWidget_setLayoutDirection(QWidget* this_ptr, const Qt::LayoutDirection* direction) {
  this_ptr->setLayoutDirection(*direction);
}

void qt_widgets_c_QWidget_setLocale(QWidget* this_ptr, const QLocale* locale) {
  this_ptr->setLocale(*locale);
}

void qt_widgets_c_QWidget_setMask_QBitmap(QWidget* this_ptr, const QBitmap* arg1) {
  this_ptr->setMask(*arg1);
}

void qt_widgets_c_QWidget_setMask_QRegion(QWidget* this_ptr, const QRegion* arg1) {
  this_ptr->setMask(*arg1);
}

void qt_widgets_c_QWidget_setMaximumHeight(QWidget* this_ptr, int maxh) {
  this_ptr->setMaximumHeight(maxh);
}

void qt_widgets_c_QWidget_setMaximumSize_arg1(QWidget* this_ptr, const QSize* arg1) {
  this_ptr->setMaximumSize(*arg1);
}

void qt_widgets_c_QWidget_setMaximumSize_maxw_maxh(QWidget* this_ptr, int maxw, int maxh) {
  this_ptr->setMaximumSize(maxw, maxh);
}

void qt_widgets_c_QWidget_setMaximumWidth(QWidget* this_ptr, int maxw) {
  this_ptr->setMaximumWidth(maxw);
}

void qt_widgets_c_QWidget_setMinimumHeight(QWidget* this_ptr, int minh) {
  this_ptr->setMinimumHeight(minh);
}

void qt_widgets_c_QWidget_setMinimumSize_arg1(QWidget* this_ptr, const QSize* arg1) {
  this_ptr->setMinimumSize(*arg1);
}

void qt_widgets_c_QWidget_setMinimumSize_minw_minh(QWidget* this_ptr, int minw, int minh) {
  this_ptr->setMinimumSize(minw, minh);
}

void qt_widgets_c_QWidget_setMinimumWidth(QWidget* this_ptr, int minw) {
  this_ptr->setMinimumWidth(minw);
}

void qt_widgets_c_QWidget_setMouseTracking(QWidget* this_ptr, bool enable) {
  this_ptr->setMouseTracking(enable);
}

void qt_widgets_c_QWidget_setPalette(QWidget* this_ptr, const QPalette* arg1) {
  this_ptr->setPalette(*arg1);
}

void qt_widgets_c_QWidget_setParent(QWidget* this_ptr, QWidget* parent) {
  this_ptr->setParent(parent);
}

void qt_widgets_c_QWidget_setShortcutAutoRepeat_id(QWidget* this_ptr, int id) {
  this_ptr->setShortcutAutoRepeat(id);
}

void qt_widgets_c_QWidget_setShortcutAutoRepeat_id_enable(QWidget* this_ptr, int id, bool enable) {
  this_ptr->setShortcutAutoRepeat(id, enable);
}

void qt_widgets_c_QWidget_setShortcutEnabled_id(QWidget* this_ptr, int id) {
  this_ptr->setShortcutEnabled(id);
}

void qt_widgets_c_QWidget_setShortcutEnabled_id_enable(QWidget* this_ptr, int id, bool enable) {
  this_ptr->setShortcutEnabled(id, enable);
}

void qt_widgets_c_QWidget_setSizeIncrement_arg1(QWidget* this_ptr, const QSize* arg1) {
  this_ptr->setSizeIncrement(*arg1);
}

void qt_widgets_c_QWidget_setSizeIncrement_w_h(QWidget* this_ptr, int w, int h) {
  this_ptr->setSizeIncrement(w, h);
}

void qt_widgets_c_QWidget_setSizePolicy_arg1(QWidget* this_ptr, const QSizePolicy* arg1) {
  this_ptr->setSizePolicy(*arg1);
}

void qt_widgets_c_QWidget_setSizePolicy_horizontal_vertical(QWidget* this_ptr, const QSizePolicy::Policy* horizontal, const QSizePolicy::Policy* vertical) {
  this_ptr->setSizePolicy(*horizontal, *vertical);
}

void qt_widgets_c_QWidget_setStatusTip(QWidget* this_ptr, const QString* arg1) {
  this_ptr->setStatusTip(*arg1);
}

void qt_widgets_c_QWidget_setStyle(QWidget* this_ptr, QStyle* arg1) {
  this_ptr->setStyle(arg1);
}

void qt_widgets_c_QWidget_setStyleSheet(QWidget* this_ptr, const QString* styleSheet) {
  this_ptr->setStyleSheet(*styleSheet);
}

void qt_widgets_c_QWidget_setTabOrder(QWidget* arg1, QWidget* arg2) {
  QWidget::setTabOrder(arg1, arg2);
}

void qt_widgets_c_QWidget_setTabletTracking(QWidget* this_ptr, bool enable) {
  this_ptr->setTabletTracking(enable);
}

void qt_widgets_c_QWidget_setToolTip(QWidget* this_ptr, const QString* arg1) {
  this_ptr->setToolTip(*arg1);
}

void qt_widgets_c_QWidget_setToolTipDuration(QWidget* this_ptr, int msec) {
  this_ptr->setToolTipDuration(msec);
}

void qt_widgets_c_QWidget_setUpdatesEnabled(QWidget* this_ptr, bool enable) {
  this_ptr->setUpdatesEnabled(enable);
}

void qt_widgets_c_QWidget_setVisible(QWidget* this_ptr, bool visible) {
  this_ptr->setVisible(visible);
}

void qt_widgets_c_QWidget_setWhatsThis(QWidget* this_ptr, const QString* arg1) {
  this_ptr->setWhatsThis(*arg1);
}

void qt_widgets_c_QWidget_setWindowFilePath(QWidget* this_ptr, const QString* filePath) {
  this_ptr->setWindowFilePath(*filePath);
}

void qt_widgets_c_QWidget_setWindowFlag_arg1(QWidget* this_ptr, const Qt::WindowType* arg1) {
  this_ptr->setWindowFlag(*arg1);
}

void qt_widgets_c_QWidget_setWindowFlag_arg1_on(QWidget* this_ptr, const Qt::WindowType* arg1, bool on) {
  this_ptr->setWindowFlag(*arg1, on);
}

void qt_widgets_c_QWidget_setWindowIcon(QWidget* this_ptr, const QIcon* icon) {
  this_ptr->setWindowIcon(*icon);
}

void qt_widgets_c_QWidget_setWindowIconText(QWidget* this_ptr, const QString* arg1) {
  this_ptr->setWindowIconText(*arg1);
}

void qt_widgets_c_QWidget_setWindowModality(QWidget* this_ptr, const Qt::WindowModality* windowModality) {
  this_ptr->setWindowModality(*windowModality);
}

void qt_widgets_c_QWidget_setWindowModified(QWidget* this_ptr, bool arg1) {
  this_ptr->setWindowModified(arg1);
}

void qt_widgets_c_QWidget_setWindowOpacity(QWidget* this_ptr, double level) {
  this_ptr->setWindowOpacity(level);
}

void qt_widgets_c_QWidget_setWindowRole(QWidget* this_ptr, const QString* arg1) {
  this_ptr->setWindowRole(*arg1);
}

void qt_widgets_c_QWidget_setWindowTitle(QWidget* this_ptr, const QString* arg1) {
  this_ptr->setWindowTitle(*arg1);
}

void qt_widgets_c_QWidget_show(QWidget* this_ptr) {
  this_ptr->show();
}

void qt_widgets_c_QWidget_showFullScreen(QWidget* this_ptr) {
  this_ptr->showFullScreen();
}

void qt_widgets_c_QWidget_showMaximized(QWidget* this_ptr) {
  this_ptr->showMaximized();
}

void qt_widgets_c_QWidget_showMinimized(QWidget* this_ptr) {
  this_ptr->showMinimized();
}

void qt_widgets_c_QWidget_showNormal(QWidget* this_ptr) {
  this_ptr->showNormal();
}

void qt_widgets_c_QWidget_sizeHint_to_output(const QWidget* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QWidget_sizeIncrement_to_output(const QWidget* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeIncrement());
}

void qt_widgets_c_QWidget_sizePolicy_to_output(const QWidget* this_ptr, QSizePolicy* output) {
  new(output) QSizePolicy(this_ptr->sizePolicy());
}

void qt_widgets_c_QWidget_size_to_output(const QWidget* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->size());
}

void qt_widgets_c_QWidget_stackUnder(QWidget* this_ptr, QWidget* arg1) {
  this_ptr->stackUnder(arg1);
}

void qt_widgets_c_QWidget_statusTip_to_output(const QWidget* this_ptr, QString* output) {
  new(output) QString(this_ptr->statusTip());
}

QStyle* qt_widgets_c_QWidget_style(const QWidget* this_ptr) {
  return this_ptr->style();
}

void qt_widgets_c_QWidget_styleSheet_to_output(const QWidget* this_ptr, QString* output) {
  new(output) QString(this_ptr->styleSheet());
}

bool qt_widgets_c_QWidget_testAttribute(const QWidget* this_ptr, const Qt::WidgetAttribute* arg1) {
  return this_ptr->testAttribute(*arg1);
}

int qt_widgets_c_QWidget_toolTipDuration(const QWidget* this_ptr) {
  return this_ptr->toolTipDuration();
}

void qt_widgets_c_QWidget_toolTip_to_output(const QWidget* this_ptr, QString* output) {
  new(output) QString(this_ptr->toolTip());
}

QWidget* qt_widgets_c_QWidget_topLevelWidget(const QWidget* this_ptr) {
  return this_ptr->topLevelWidget();
}

void qt_widgets_c_QWidget_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QWidget::trUtf8(s, c, n));
}

void qt_widgets_c_QWidget_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QWidget::tr(s, c, n));
}

bool qt_widgets_c_QWidget_underMouse(const QWidget* this_ptr) {
  return this_ptr->underMouse();
}

void qt_widgets_c_QWidget_ungrabGesture(QWidget* this_ptr, const Qt::GestureType* type) {
  this_ptr->ungrabGesture(*type);
}

void qt_widgets_c_QWidget_unsetCursor(QWidget* this_ptr) {
  this_ptr->unsetCursor();
}

void qt_widgets_c_QWidget_unsetLayoutDirection(QWidget* this_ptr) {
  this_ptr->unsetLayoutDirection();
}

void qt_widgets_c_QWidget_unsetLocale(QWidget* this_ptr) {
  this_ptr->unsetLocale();
}

void qt_widgets_c_QWidget_updateGeometry(QWidget* this_ptr) {
  this_ptr->updateGeometry();
}

void qt_widgets_c_QWidget_update_QRect(QWidget* this_ptr, const QRect* arg1) {
  this_ptr->update(*arg1);
}

void qt_widgets_c_QWidget_update_QRegion(QWidget* this_ptr, const QRegion* arg1) {
  this_ptr->update(*arg1);
}

void qt_widgets_c_QWidget_update_int_int_int_int(QWidget* this_ptr, int x, int y, int w, int h) {
  this_ptr->update(x, y, w, h);
}

void qt_widgets_c_QWidget_update_no_args(QWidget* this_ptr) {
  this_ptr->update();
}

bool qt_widgets_c_QWidget_updatesEnabled(const QWidget* this_ptr) {
  return this_ptr->updatesEnabled();
}

QRegion* qt_widgets_c_QWidget_visibleRegion_as_ptr(const QWidget* this_ptr) {
  return new QRegion(this_ptr->visibleRegion());
}

void qt_widgets_c_QWidget_whatsThis_to_output(const QWidget* this_ptr, QString* output) {
  new(output) QString(this_ptr->whatsThis());
}

int qt_widgets_c_QWidget_width(const QWidget* this_ptr) {
  return this_ptr->width();
}

unsigned long long qt_widgets_c_QWidget_winId(const QWidget* this_ptr) {
  return this_ptr->winId();
}

QWidget* qt_widgets_c_QWidget_window(const QWidget* this_ptr) {
  return this_ptr->window();
}

void qt_widgets_c_QWidget_windowFilePath_to_output(const QWidget* this_ptr, QString* output) {
  new(output) QString(this_ptr->windowFilePath());
}

QWindow* qt_widgets_c_QWidget_windowHandle(const QWidget* this_ptr) {
  return this_ptr->windowHandle();
}

void qt_widgets_c_QWidget_windowIconText_to_output(const QWidget* this_ptr, QString* output) {
  new(output) QString(this_ptr->windowIconText());
}

void qt_widgets_c_QWidget_windowIcon_to_output(const QWidget* this_ptr, QIcon* output) {
  new(output) QIcon(this_ptr->windowIcon());
}

double qt_widgets_c_QWidget_windowOpacity(const QWidget* this_ptr) {
  return this_ptr->windowOpacity();
}

void qt_widgets_c_QWidget_windowRole_to_output(const QWidget* this_ptr, QString* output) {
  new(output) QString(this_ptr->windowRole());
}

void qt_widgets_c_QWidget_windowTitle_to_output(const QWidget* this_ptr, QString* output) {
  new(output) QString(this_ptr->windowTitle());
}

int qt_widgets_c_QWidget_x(const QWidget* this_ptr) {
  return this_ptr->x();
}

int qt_widgets_c_QWidget_y(const QWidget* this_ptr) {
  return this_ptr->y();
}

