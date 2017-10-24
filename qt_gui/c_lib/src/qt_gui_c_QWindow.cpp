#include "qt_gui_c_QWindow.h"

QWindow* qt_gui_c_QWindow_G_dynamic_cast_QWindow_ptr(QSurface* ptr) {
  return dynamic_cast<QWindow*>(ptr);
}

void qt_gui_c_QWindow_G_operator_shl_to_output(const QDebug* arg1, const QWindow* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, arg2));
}

QObject* qt_gui_c_QWindow_G_static_cast_QObject_ptr(QWindow* ptr) {
  return static_cast<QObject*>(ptr);
}

QSurface* qt_gui_c_QWindow_G_static_cast_QSurface_ptr(QWindow* ptr) {
  return static_cast<QSurface*>(ptr);
}

QWindow* qt_gui_c_QWindow_G_static_cast_QWindow_ptr_QObject(QObject* ptr) {
  return static_cast<QWindow*>(ptr);
}

QWindow* qt_gui_c_QWindow_G_static_cast_QWindow_ptr_QSurface(QSurface* ptr) {
  return static_cast<QWindow*>(ptr);
}

QAccessibleInterface* qt_gui_c_QWindow_accessibleRoot(const QWindow* this_ptr) {
  return this_ptr->accessibleRoot();
}

void qt_gui_c_QWindow_alert(QWindow* this_ptr, int msec) {
  this_ptr->alert(msec);
}

void qt_gui_c_QWindow_baseSize_to_output(const QWindow* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->baseSize());
}

bool qt_gui_c_QWindow_close(QWindow* this_ptr) {
  return this_ptr->close();
}

void qt_gui_c_QWindow_create(QWindow* this_ptr) {
  this_ptr->create();
}

QCursor* qt_gui_c_QWindow_cursor_as_ptr(const QWindow* this_ptr) {
  return new QCursor(this_ptr->cursor());
}

void qt_gui_c_QWindow_delete(QWindow* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QWindow_destroy(QWindow* this_ptr) {
  this_ptr->destroy();
}

double qt_gui_c_QWindow_devicePixelRatio(const QWindow* this_ptr) {
  return this_ptr->devicePixelRatio();
}

void qt_gui_c_QWindow_filePath_to_output(const QWindow* this_ptr, QString* output) {
  new(output) QString(this_ptr->filePath());
}

QObject* qt_gui_c_QWindow_focusObject(const QWindow* this_ptr) {
  return this_ptr->focusObject();
}

void qt_gui_c_QWindow_format_to_output(const QWindow* this_ptr, QSurfaceFormat* output) {
  new(output) QSurfaceFormat(this_ptr->format());
}

void qt_gui_c_QWindow_frameGeometry_to_output(const QWindow* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->frameGeometry());
}

void qt_gui_c_QWindow_frameMargins_to_output(const QWindow* this_ptr, QMargins* output) {
  new(output) QMargins(this_ptr->frameMargins());
}

void qt_gui_c_QWindow_framePosition_to_output(const QWindow* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->framePosition());
}

QWindow* qt_gui_c_QWindow_fromWinId(unsigned long long id) {
  return QWindow::fromWinId(id);
}

void qt_gui_c_QWindow_geometry_to_output(const QWindow* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->geometry());
}

int qt_gui_c_QWindow_height(const QWindow* this_ptr) {
  return this_ptr->height();
}

void qt_gui_c_QWindow_hide(QWindow* this_ptr) {
  this_ptr->hide();
}

void qt_gui_c_QWindow_icon_to_output(const QWindow* this_ptr, QIcon* output) {
  new(output) QIcon(this_ptr->icon());
}

bool qt_gui_c_QWindow_isActive(const QWindow* this_ptr) {
  return this_ptr->isActive();
}

bool qt_gui_c_QWindow_isAncestorOf_child(const QWindow* this_ptr, const QWindow* child) {
  return this_ptr->isAncestorOf(child);
}

bool qt_gui_c_QWindow_isAncestorOf_child_mode(const QWindow* this_ptr, const QWindow* child, QWindow::AncestorMode mode) {
  return this_ptr->isAncestorOf(child, mode);
}

bool qt_gui_c_QWindow_isExposed(const QWindow* this_ptr) {
  return this_ptr->isExposed();
}

bool qt_gui_c_QWindow_isModal(const QWindow* this_ptr) {
  return this_ptr->isModal();
}

bool qt_gui_c_QWindow_isTopLevel(const QWindow* this_ptr) {
  return this_ptr->isTopLevel();
}

bool qt_gui_c_QWindow_isVisible(const QWindow* this_ptr) {
  return this_ptr->isVisible();
}

void qt_gui_c_QWindow_lower(QWindow* this_ptr) {
  this_ptr->lower();
}

void qt_gui_c_QWindow_mapFromGlobal_to_output(const QWindow* this_ptr, const QPoint* pos, QPoint* output) {
  new(output) QPoint(this_ptr->mapFromGlobal(*pos));
}

void qt_gui_c_QWindow_mapToGlobal_to_output(const QWindow* this_ptr, const QPoint* pos, QPoint* output) {
  new(output) QPoint(this_ptr->mapToGlobal(*pos));
}

QRegion* qt_gui_c_QWindow_mask_as_ptr(const QWindow* this_ptr) {
  return new QRegion(this_ptr->mask());
}

int qt_gui_c_QWindow_maximumHeight(const QWindow* this_ptr) {
  return this_ptr->maximumHeight();
}

void qt_gui_c_QWindow_maximumSize_to_output(const QWindow* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->maximumSize());
}

int qt_gui_c_QWindow_maximumWidth(const QWindow* this_ptr) {
  return this_ptr->maximumWidth();
}

const QMetaObject* qt_gui_c_QWindow_metaObject(const QWindow* this_ptr) {
  return this_ptr->metaObject();
}

int qt_gui_c_QWindow_minimumHeight(const QWindow* this_ptr) {
  return this_ptr->minimumHeight();
}

void qt_gui_c_QWindow_minimumSize_to_output(const QWindow* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSize());
}

int qt_gui_c_QWindow_minimumWidth(const QWindow* this_ptr) {
  return this_ptr->minimumWidth();
}

QWindow* qt_gui_c_QWindow_new_no_args() {
  return new QWindow();
}

QWindow* qt_gui_c_QWindow_new_parent(QWindow* parent) {
  return new QWindow(parent);
}

QWindow* qt_gui_c_QWindow_new_screen(QScreen* screen) {
  return new QWindow(screen);
}

double qt_gui_c_QWindow_opacity(const QWindow* this_ptr) {
  return this_ptr->opacity();
}

QWindow* qt_gui_c_QWindow_parent_mode(const QWindow* this_ptr, QWindow::AncestorMode mode) {
  return this_ptr->parent(mode);
}

QWindow* qt_gui_c_QWindow_parent_no_args(const QWindow* this_ptr) {
  return this_ptr->parent();
}

void qt_gui_c_QWindow_position_to_output(const QWindow* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->position());
}

int qt_gui_c_QWindow_qt_metacall(QWindow* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QWindow_qt_metacast(QWindow* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QWindow_raise(QWindow* this_ptr) {
  this_ptr->raise();
}

void qt_gui_c_QWindow_reportContentOrientationChange(QWindow* this_ptr, const Qt::ScreenOrientation* orientation) {
  this_ptr->reportContentOrientationChange(*orientation);
}

void qt_gui_c_QWindow_requestActivate(QWindow* this_ptr) {
  this_ptr->requestActivate();
}

void qt_gui_c_QWindow_requestUpdate(QWindow* this_ptr) {
  this_ptr->requestUpdate();
}

void qt_gui_c_QWindow_requestedFormat_to_output(const QWindow* this_ptr, QSurfaceFormat* output) {
  new(output) QSurfaceFormat(this_ptr->requestedFormat());
}

void qt_gui_c_QWindow_resize_newSize(QWindow* this_ptr, const QSize* newSize) {
  this_ptr->resize(*newSize);
}

void qt_gui_c_QWindow_resize_w_h(QWindow* this_ptr, int w, int h) {
  this_ptr->resize(w, h);
}

QScreen* qt_gui_c_QWindow_screen(const QWindow* this_ptr) {
  return this_ptr->screen();
}

void qt_gui_c_QWindow_setBaseSize(QWindow* this_ptr, const QSize* size) {
  this_ptr->setBaseSize(*size);
}

void qt_gui_c_QWindow_setCursor(QWindow* this_ptr, const QCursor* arg1) {
  this_ptr->setCursor(*arg1);
}

void qt_gui_c_QWindow_setFilePath(QWindow* this_ptr, const QString* filePath) {
  this_ptr->setFilePath(*filePath);
}

void qt_gui_c_QWindow_setFlag_arg1(QWindow* this_ptr, const Qt::WindowType* arg1) {
  this_ptr->setFlag(*arg1);
}

void qt_gui_c_QWindow_setFlag_arg1_on(QWindow* this_ptr, const Qt::WindowType* arg1, bool on) {
  this_ptr->setFlag(*arg1, on);
}

void qt_gui_c_QWindow_setFormat(QWindow* this_ptr, const QSurfaceFormat* format) {
  this_ptr->setFormat(*format);
}

void qt_gui_c_QWindow_setFramePosition(QWindow* this_ptr, const QPoint* point) {
  this_ptr->setFramePosition(*point);
}

void qt_gui_c_QWindow_setGeometry_posx_posy_w_h(QWindow* this_ptr, int posx, int posy, int w, int h) {
  this_ptr->setGeometry(posx, posy, w, h);
}

void qt_gui_c_QWindow_setGeometry_rect(QWindow* this_ptr, const QRect* rect) {
  this_ptr->setGeometry(*rect);
}

void qt_gui_c_QWindow_setHeight(QWindow* this_ptr, int arg) {
  this_ptr->setHeight(arg);
}

void qt_gui_c_QWindow_setIcon(QWindow* this_ptr, const QIcon* icon) {
  this_ptr->setIcon(*icon);
}

bool qt_gui_c_QWindow_setKeyboardGrabEnabled(QWindow* this_ptr, bool grab) {
  return this_ptr->setKeyboardGrabEnabled(grab);
}

void qt_gui_c_QWindow_setMask(QWindow* this_ptr, const QRegion* region) {
  this_ptr->setMask(*region);
}

void qt_gui_c_QWindow_setMaximumHeight(QWindow* this_ptr, int h) {
  this_ptr->setMaximumHeight(h);
}

void qt_gui_c_QWindow_setMaximumSize(QWindow* this_ptr, const QSize* size) {
  this_ptr->setMaximumSize(*size);
}

void qt_gui_c_QWindow_setMaximumWidth(QWindow* this_ptr, int w) {
  this_ptr->setMaximumWidth(w);
}

void qt_gui_c_QWindow_setMinimumHeight(QWindow* this_ptr, int h) {
  this_ptr->setMinimumHeight(h);
}

void qt_gui_c_QWindow_setMinimumSize(QWindow* this_ptr, const QSize* size) {
  this_ptr->setMinimumSize(*size);
}

void qt_gui_c_QWindow_setMinimumWidth(QWindow* this_ptr, int w) {
  this_ptr->setMinimumWidth(w);
}

void qt_gui_c_QWindow_setModality(QWindow* this_ptr, const Qt::WindowModality* modality) {
  this_ptr->setModality(*modality);
}

bool qt_gui_c_QWindow_setMouseGrabEnabled(QWindow* this_ptr, bool grab) {
  return this_ptr->setMouseGrabEnabled(grab);
}

void qt_gui_c_QWindow_setOpacity(QWindow* this_ptr, double level) {
  this_ptr->setOpacity(level);
}

void qt_gui_c_QWindow_setParent(QWindow* this_ptr, QWindow* parent) {
  this_ptr->setParent(parent);
}

void qt_gui_c_QWindow_setPosition_posx_posy(QWindow* this_ptr, int posx, int posy) {
  this_ptr->setPosition(posx, posy);
}

void qt_gui_c_QWindow_setPosition_pt(QWindow* this_ptr, const QPoint* pt) {
  this_ptr->setPosition(*pt);
}

void qt_gui_c_QWindow_setScreen(QWindow* this_ptr, QScreen* screen) {
  this_ptr->setScreen(screen);
}

void qt_gui_c_QWindow_setSizeIncrement(QWindow* this_ptr, const QSize* size) {
  this_ptr->setSizeIncrement(*size);
}

void qt_gui_c_QWindow_setSurfaceType(QWindow* this_ptr, QSurface::SurfaceType surfaceType) {
  this_ptr->setSurfaceType(surfaceType);
}

void qt_gui_c_QWindow_setTitle(QWindow* this_ptr, const QString* arg1) {
  this_ptr->setTitle(*arg1);
}

void qt_gui_c_QWindow_setTransientParent(QWindow* this_ptr, QWindow* parent) {
  this_ptr->setTransientParent(parent);
}

void qt_gui_c_QWindow_setVisibility(QWindow* this_ptr, QWindow::Visibility v) {
  this_ptr->setVisibility(v);
}

void qt_gui_c_QWindow_setVisible(QWindow* this_ptr, bool visible) {
  this_ptr->setVisible(visible);
}

void qt_gui_c_QWindow_setWidth(QWindow* this_ptr, int arg) {
  this_ptr->setWidth(arg);
}

void qt_gui_c_QWindow_setWindowState(QWindow* this_ptr, const Qt::WindowState* state) {
  this_ptr->setWindowState(*state);
}

void qt_gui_c_QWindow_setX(QWindow* this_ptr, int arg) {
  this_ptr->setX(arg);
}

void qt_gui_c_QWindow_setY(QWindow* this_ptr, int arg) {
  this_ptr->setY(arg);
}

void qt_gui_c_QWindow_show(QWindow* this_ptr) {
  this_ptr->show();
}

void qt_gui_c_QWindow_showFullScreen(QWindow* this_ptr) {
  this_ptr->showFullScreen();
}

void qt_gui_c_QWindow_showMaximized(QWindow* this_ptr) {
  this_ptr->showMaximized();
}

void qt_gui_c_QWindow_showMinimized(QWindow* this_ptr) {
  this_ptr->showMinimized();
}

void qt_gui_c_QWindow_showNormal(QWindow* this_ptr) {
  this_ptr->showNormal();
}

void qt_gui_c_QWindow_sizeIncrement_to_output(const QWindow* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeIncrement());
}

void qt_gui_c_QWindow_size_to_output(const QWindow* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->size());
}

QSurface::SurfaceType qt_gui_c_QWindow_surfaceType(const QWindow* this_ptr) {
  return this_ptr->surfaceType();
}

void qt_gui_c_QWindow_title_to_output(const QWindow* this_ptr, QString* output) {
  new(output) QString(this_ptr->title());
}

void qt_gui_c_QWindow_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QWindow::trUtf8(s, c, n));
}

void qt_gui_c_QWindow_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QWindow::tr(s, c, n));
}

QWindow* qt_gui_c_QWindow_transientParent(const QWindow* this_ptr) {
  return this_ptr->transientParent();
}

void qt_gui_c_QWindow_unsetCursor(QWindow* this_ptr) {
  this_ptr->unsetCursor();
}

QWindow::Visibility qt_gui_c_QWindow_visibility(const QWindow* this_ptr) {
  return this_ptr->visibility();
}

int qt_gui_c_QWindow_width(const QWindow* this_ptr) {
  return this_ptr->width();
}

unsigned long long qt_gui_c_QWindow_winId(const QWindow* this_ptr) {
  return this_ptr->winId();
}

int qt_gui_c_QWindow_x(const QWindow* this_ptr) {
  return this_ptr->x();
}

int qt_gui_c_QWindow_y(const QWindow* this_ptr) {
  return this_ptr->y();
}

