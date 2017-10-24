#include "qt_widgets_c_QToolBox.h"

QToolBox* qt_widgets_c_QToolBox_G_dynamic_cast_QToolBox_ptr_QFrame(QFrame* ptr) {
  return dynamic_cast<QToolBox*>(ptr);
}

QToolBox* qt_widgets_c_QToolBox_G_dynamic_cast_QToolBox_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QToolBox*>(ptr);
}

QFrame* qt_widgets_c_QToolBox_G_static_cast_QFrame_ptr(QToolBox* ptr) {
  return static_cast<QFrame*>(ptr);
}

QObject* qt_widgets_c_QToolBox_G_static_cast_QObject_ptr(QToolBox* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QToolBox_G_static_cast_QPaintDevice_ptr(QToolBox* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QToolBox* qt_widgets_c_QToolBox_G_static_cast_QToolBox_ptr_QFrame(QFrame* ptr) {
  return static_cast<QToolBox*>(ptr);
}

QToolBox* qt_widgets_c_QToolBox_G_static_cast_QToolBox_ptr_QObject(QObject* ptr) {
  return static_cast<QToolBox*>(ptr);
}

QToolBox* qt_widgets_c_QToolBox_G_static_cast_QToolBox_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QToolBox*>(ptr);
}

QToolBox* qt_widgets_c_QToolBox_G_static_cast_QToolBox_ptr_QWidget(QWidget* ptr) {
  return static_cast<QToolBox*>(ptr);
}

QWidget* qt_widgets_c_QToolBox_G_static_cast_QWidget_ptr(QToolBox* ptr) {
  return static_cast<QWidget*>(ptr);
}

int qt_widgets_c_QToolBox_addItem_widget_icon_text(QToolBox* this_ptr, QWidget* widget, const QIcon* icon, const QString* text) {
  return this_ptr->addItem(widget, *icon, *text);
}

int qt_widgets_c_QToolBox_addItem_widget_text(QToolBox* this_ptr, QWidget* widget, const QString* text) {
  return this_ptr->addItem(widget, *text);
}

int qt_widgets_c_QToolBox_count(const QToolBox* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QToolBox_currentIndex(const QToolBox* this_ptr) {
  return this_ptr->currentIndex();
}

QWidget* qt_widgets_c_QToolBox_currentWidget(const QToolBox* this_ptr) {
  return this_ptr->currentWidget();
}

void qt_widgets_c_QToolBox_delete(QToolBox* this_ptr) {
  delete this_ptr;
}

int qt_widgets_c_QToolBox_indexOf(const QToolBox* this_ptr, QWidget* widget) {
  return this_ptr->indexOf(widget);
}

int qt_widgets_c_QToolBox_insertItem_index_widget_icon_text(QToolBox* this_ptr, int index, QWidget* widget, const QIcon* icon, const QString* text) {
  return this_ptr->insertItem(index, widget, *icon, *text);
}

int qt_widgets_c_QToolBox_insertItem_index_widget_text(QToolBox* this_ptr, int index, QWidget* widget, const QString* text) {
  return this_ptr->insertItem(index, widget, *text);
}

bool qt_widgets_c_QToolBox_isItemEnabled(const QToolBox* this_ptr, int index) {
  return this_ptr->isItemEnabled(index);
}

void qt_widgets_c_QToolBox_itemIcon_to_output(const QToolBox* this_ptr, int index, QIcon* output) {
  new(output) QIcon(this_ptr->itemIcon(index));
}

void qt_widgets_c_QToolBox_itemText_to_output(const QToolBox* this_ptr, int index, QString* output) {
  new(output) QString(this_ptr->itemText(index));
}

void qt_widgets_c_QToolBox_itemToolTip_to_output(const QToolBox* this_ptr, int index, QString* output) {
  new(output) QString(this_ptr->itemToolTip(index));
}

const QMetaObject* qt_widgets_c_QToolBox_metaObject(const QToolBox* this_ptr) {
  return this_ptr->metaObject();
}

int qt_widgets_c_QToolBox_qt_metacall(QToolBox* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QToolBox_qt_metacast(QToolBox* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QToolBox_removeItem(QToolBox* this_ptr, int index) {
  this_ptr->removeItem(index);
}

void qt_widgets_c_QToolBox_setCurrentIndex(QToolBox* this_ptr, int index) {
  this_ptr->setCurrentIndex(index);
}

void qt_widgets_c_QToolBox_setCurrentWidget(QToolBox* this_ptr, QWidget* widget) {
  this_ptr->setCurrentWidget(widget);
}

void qt_widgets_c_QToolBox_setItemEnabled(QToolBox* this_ptr, int index, bool enabled) {
  this_ptr->setItemEnabled(index, enabled);
}

void qt_widgets_c_QToolBox_setItemIcon(QToolBox* this_ptr, int index, const QIcon* icon) {
  this_ptr->setItemIcon(index, *icon);
}

void qt_widgets_c_QToolBox_setItemText(QToolBox* this_ptr, int index, const QString* text) {
  this_ptr->setItemText(index, *text);
}

void qt_widgets_c_QToolBox_setItemToolTip(QToolBox* this_ptr, int index, const QString* toolTip) {
  this_ptr->setItemToolTip(index, *toolTip);
}

void qt_widgets_c_QToolBox_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QToolBox::trUtf8(s, c, n));
}

void qt_widgets_c_QToolBox_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QToolBox::tr(s, c, n));
}

QWidget* qt_widgets_c_QToolBox_widget(const QToolBox* this_ptr, int index) {
  return this_ptr->widget(index);
}

