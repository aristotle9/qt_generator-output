#include "qt_widgets_c_QTabBar.h"

QTabBar* qt_widgets_c_QTabBar_G_dynamic_cast_QTabBar_ptr(QWidget* ptr) {
  return dynamic_cast<QTabBar*>(ptr);
}

QObject* qt_widgets_c_QTabBar_G_static_cast_QObject_ptr(QTabBar* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QTabBar_G_static_cast_QPaintDevice_ptr(QTabBar* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QTabBar* qt_widgets_c_QTabBar_G_static_cast_QTabBar_ptr_QObject(QObject* ptr) {
  return static_cast<QTabBar*>(ptr);
}

QTabBar* qt_widgets_c_QTabBar_G_static_cast_QTabBar_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QTabBar*>(ptr);
}

QTabBar* qt_widgets_c_QTabBar_G_static_cast_QTabBar_ptr_QWidget(QWidget* ptr) {
  return static_cast<QTabBar*>(ptr);
}

QWidget* qt_widgets_c_QTabBar_G_static_cast_QWidget_ptr(QTabBar* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QTabBar_accessibleTabName_to_output(const QTabBar* this_ptr, int index, QString* output) {
  new(output) QString(this_ptr->accessibleTabName(index));
}

int qt_widgets_c_QTabBar_addTab_icon_text(QTabBar* this_ptr, const QIcon* icon, const QString* text) {
  return this_ptr->addTab(*icon, *text);
}

int qt_widgets_c_QTabBar_addTab_text(QTabBar* this_ptr, const QString* text) {
  return this_ptr->addTab(*text);
}

bool qt_widgets_c_QTabBar_autoHide(const QTabBar* this_ptr) {
  return this_ptr->autoHide();
}

bool qt_widgets_c_QTabBar_changeCurrentOnDrag(const QTabBar* this_ptr) {
  return this_ptr->changeCurrentOnDrag();
}

int qt_widgets_c_QTabBar_count(const QTabBar* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QTabBar_currentIndex(const QTabBar* this_ptr) {
  return this_ptr->currentIndex();
}

void qt_widgets_c_QTabBar_delete(QTabBar* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QTabBar_documentMode(const QTabBar* this_ptr) {
  return this_ptr->documentMode();
}

bool qt_widgets_c_QTabBar_drawBase(const QTabBar* this_ptr) {
  return this_ptr->drawBase();
}

bool qt_widgets_c_QTabBar_expanding(const QTabBar* this_ptr) {
  return this_ptr->expanding();
}

void qt_widgets_c_QTabBar_iconSize_to_output(const QTabBar* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->iconSize());
}

int qt_widgets_c_QTabBar_insertTab_index_icon_text(QTabBar* this_ptr, int index, const QIcon* icon, const QString* text) {
  return this_ptr->insertTab(index, *icon, *text);
}

int qt_widgets_c_QTabBar_insertTab_index_text(QTabBar* this_ptr, int index, const QString* text) {
  return this_ptr->insertTab(index, *text);
}

bool qt_widgets_c_QTabBar_isMovable(const QTabBar* this_ptr) {
  return this_ptr->isMovable();
}

bool qt_widgets_c_QTabBar_isTabEnabled(const QTabBar* this_ptr, int index) {
  return this_ptr->isTabEnabled(index);
}

const QMetaObject* qt_widgets_c_QTabBar_metaObject(const QTabBar* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QTabBar_minimumSizeHint_to_output(const QTabBar* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSizeHint());
}

void qt_widgets_c_QTabBar_moveTab(QTabBar* this_ptr, int from, int to) {
  this_ptr->moveTab(from, to);
}

QTabBar* qt_widgets_c_QTabBar_new_no_args() {
  return new QTabBar();
}

QTabBar* qt_widgets_c_QTabBar_new_parent(QWidget* parent) {
  return new QTabBar(parent);
}

int qt_widgets_c_QTabBar_qt_metacall(QTabBar* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QTabBar_qt_metacast(QTabBar* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QTabBar_removeTab(QTabBar* this_ptr, int index) {
  this_ptr->removeTab(index);
}

QTabBar::SelectionBehavior qt_widgets_c_QTabBar_selectionBehaviorOnRemove(const QTabBar* this_ptr) {
  return this_ptr->selectionBehaviorOnRemove();
}

void qt_widgets_c_QTabBar_setAccessibleTabName(QTabBar* this_ptr, int index, const QString* name) {
  this_ptr->setAccessibleTabName(index, *name);
}

void qt_widgets_c_QTabBar_setAutoHide(QTabBar* this_ptr, bool hide) {
  this_ptr->setAutoHide(hide);
}

void qt_widgets_c_QTabBar_setChangeCurrentOnDrag(QTabBar* this_ptr, bool change) {
  this_ptr->setChangeCurrentOnDrag(change);
}

void qt_widgets_c_QTabBar_setCurrentIndex(QTabBar* this_ptr, int index) {
  this_ptr->setCurrentIndex(index);
}

void qt_widgets_c_QTabBar_setDocumentMode(QTabBar* this_ptr, bool set) {
  this_ptr->setDocumentMode(set);
}

void qt_widgets_c_QTabBar_setDrawBase(QTabBar* this_ptr, bool drawTheBase) {
  this_ptr->setDrawBase(drawTheBase);
}

void qt_widgets_c_QTabBar_setElideMode(QTabBar* this_ptr, const Qt::TextElideMode* arg1) {
  this_ptr->setElideMode(*arg1);
}

void qt_widgets_c_QTabBar_setExpanding(QTabBar* this_ptr, bool enabled) {
  this_ptr->setExpanding(enabled);
}

void qt_widgets_c_QTabBar_setIconSize(QTabBar* this_ptr, const QSize* size) {
  this_ptr->setIconSize(*size);
}

void qt_widgets_c_QTabBar_setMovable(QTabBar* this_ptr, bool movable) {
  this_ptr->setMovable(movable);
}

void qt_widgets_c_QTabBar_setSelectionBehaviorOnRemove(QTabBar* this_ptr, QTabBar::SelectionBehavior behavior) {
  this_ptr->setSelectionBehaviorOnRemove(behavior);
}

void qt_widgets_c_QTabBar_setShape(QTabBar* this_ptr, QTabBar::Shape shape) {
  this_ptr->setShape(shape);
}

void qt_widgets_c_QTabBar_setTabButton(QTabBar* this_ptr, int index, QTabBar::ButtonPosition position, QWidget* widget) {
  this_ptr->setTabButton(index, position, widget);
}

void qt_widgets_c_QTabBar_setTabData(QTabBar* this_ptr, int index, const QVariant* data) {
  this_ptr->setTabData(index, *data);
}

void qt_widgets_c_QTabBar_setTabEnabled(QTabBar* this_ptr, int index, bool arg2) {
  this_ptr->setTabEnabled(index, arg2);
}

void qt_widgets_c_QTabBar_setTabIcon(QTabBar* this_ptr, int index, const QIcon* icon) {
  this_ptr->setTabIcon(index, *icon);
}

void qt_widgets_c_QTabBar_setTabText(QTabBar* this_ptr, int index, const QString* text) {
  this_ptr->setTabText(index, *text);
}

void qt_widgets_c_QTabBar_setTabTextColor(QTabBar* this_ptr, int index, const QColor* color) {
  this_ptr->setTabTextColor(index, *color);
}

void qt_widgets_c_QTabBar_setTabToolTip(QTabBar* this_ptr, int index, const QString* tip) {
  this_ptr->setTabToolTip(index, *tip);
}

void qt_widgets_c_QTabBar_setTabWhatsThis(QTabBar* this_ptr, int index, const QString* text) {
  this_ptr->setTabWhatsThis(index, *text);
}

void qt_widgets_c_QTabBar_setTabsClosable(QTabBar* this_ptr, bool closable) {
  this_ptr->setTabsClosable(closable);
}

void qt_widgets_c_QTabBar_setUsesScrollButtons(QTabBar* this_ptr, bool useButtons) {
  this_ptr->setUsesScrollButtons(useButtons);
}

QTabBar::Shape qt_widgets_c_QTabBar_shape(const QTabBar* this_ptr) {
  return this_ptr->shape();
}

void qt_widgets_c_QTabBar_sizeHint_to_output(const QTabBar* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

int qt_widgets_c_QTabBar_tabAt(const QTabBar* this_ptr, const QPoint* pos) {
  return this_ptr->tabAt(*pos);
}

QWidget* qt_widgets_c_QTabBar_tabButton(const QTabBar* this_ptr, int index, QTabBar::ButtonPosition position) {
  return this_ptr->tabButton(index, position);
}

void qt_widgets_c_QTabBar_tabData_to_output(const QTabBar* this_ptr, int index, QVariant* output) {
  new(output) QVariant(this_ptr->tabData(index));
}

void qt_widgets_c_QTabBar_tabIcon_to_output(const QTabBar* this_ptr, int index, QIcon* output) {
  new(output) QIcon(this_ptr->tabIcon(index));
}

void qt_widgets_c_QTabBar_tabRect_to_output(const QTabBar* this_ptr, int index, QRect* output) {
  new(output) QRect(this_ptr->tabRect(index));
}

void qt_widgets_c_QTabBar_tabTextColor_to_output(const QTabBar* this_ptr, int index, QColor* output) {
  new(output) QColor(this_ptr->tabTextColor(index));
}

void qt_widgets_c_QTabBar_tabText_to_output(const QTabBar* this_ptr, int index, QString* output) {
  new(output) QString(this_ptr->tabText(index));
}

void qt_widgets_c_QTabBar_tabToolTip_to_output(const QTabBar* this_ptr, int index, QString* output) {
  new(output) QString(this_ptr->tabToolTip(index));
}

void qt_widgets_c_QTabBar_tabWhatsThis_to_output(const QTabBar* this_ptr, int index, QString* output) {
  new(output) QString(this_ptr->tabWhatsThis(index));
}

bool qt_widgets_c_QTabBar_tabsClosable(const QTabBar* this_ptr) {
  return this_ptr->tabsClosable();
}

void qt_widgets_c_QTabBar_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTabBar::trUtf8(s, c, n));
}

void qt_widgets_c_QTabBar_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTabBar::tr(s, c, n));
}

bool qt_widgets_c_QTabBar_usesScrollButtons(const QTabBar* this_ptr) {
  return this_ptr->usesScrollButtons();
}

