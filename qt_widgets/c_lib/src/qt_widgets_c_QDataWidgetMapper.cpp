#include "qt_widgets_c_QDataWidgetMapper.h"

QDataWidgetMapper* qt_widgets_c_QDataWidgetMapper_G_static_cast_QDataWidgetMapper_ptr(QObject* ptr) {
  return static_cast<QDataWidgetMapper*>(ptr);
}

QObject* qt_widgets_c_QDataWidgetMapper_G_static_cast_QObject_ptr(QDataWidgetMapper* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_widgets_c_QDataWidgetMapper_addMapping_widget_section(QDataWidgetMapper* this_ptr, QWidget* widget, int section) {
  this_ptr->addMapping(widget, section);
}

void qt_widgets_c_QDataWidgetMapper_addMapping_widget_section_propertyName(QDataWidgetMapper* this_ptr, QWidget* widget, int section, const QByteArray* propertyName) {
  this_ptr->addMapping(widget, section, *propertyName);
}

void qt_widgets_c_QDataWidgetMapper_clearMapping(QDataWidgetMapper* this_ptr) {
  this_ptr->clearMapping();
}

int qt_widgets_c_QDataWidgetMapper_currentIndex(const QDataWidgetMapper* this_ptr) {
  return this_ptr->currentIndex();
}

void qt_widgets_c_QDataWidgetMapper_delete(QDataWidgetMapper* this_ptr) {
  delete this_ptr;
}

QAbstractItemDelegate* qt_widgets_c_QDataWidgetMapper_itemDelegate(const QDataWidgetMapper* this_ptr) {
  return this_ptr->itemDelegate();
}

void qt_widgets_c_QDataWidgetMapper_mappedPropertyName_to_output(const QDataWidgetMapper* this_ptr, QWidget* widget, QByteArray* output) {
  new(output) QByteArray(this_ptr->mappedPropertyName(widget));
}

int qt_widgets_c_QDataWidgetMapper_mappedSection(const QDataWidgetMapper* this_ptr, QWidget* widget) {
  return this_ptr->mappedSection(widget);
}

QWidget* qt_widgets_c_QDataWidgetMapper_mappedWidgetAt(const QDataWidgetMapper* this_ptr, int section) {
  return this_ptr->mappedWidgetAt(section);
}

const QMetaObject* qt_widgets_c_QDataWidgetMapper_metaObject(const QDataWidgetMapper* this_ptr) {
  return this_ptr->metaObject();
}

QAbstractItemModel* qt_widgets_c_QDataWidgetMapper_model(const QDataWidgetMapper* this_ptr) {
  return this_ptr->model();
}

QDataWidgetMapper* qt_widgets_c_QDataWidgetMapper_new_no_args() {
  return new QDataWidgetMapper();
}

QDataWidgetMapper* qt_widgets_c_QDataWidgetMapper_new_parent(QObject* parent) {
  return new QDataWidgetMapper(parent);
}

int qt_widgets_c_QDataWidgetMapper_qt_metacall(QDataWidgetMapper* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QDataWidgetMapper_qt_metacast(QDataWidgetMapper* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QDataWidgetMapper_removeMapping(QDataWidgetMapper* this_ptr, QWidget* widget) {
  this_ptr->removeMapping(widget);
}

void qt_widgets_c_QDataWidgetMapper_revert(QDataWidgetMapper* this_ptr) {
  this_ptr->revert();
}

void qt_widgets_c_QDataWidgetMapper_rootIndex_to_output(const QDataWidgetMapper* this_ptr, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->rootIndex());
}

void qt_widgets_c_QDataWidgetMapper_setCurrentIndex(QDataWidgetMapper* this_ptr, int index) {
  this_ptr->setCurrentIndex(index);
}

void qt_widgets_c_QDataWidgetMapper_setCurrentModelIndex(QDataWidgetMapper* this_ptr, const QModelIndex* index) {
  this_ptr->setCurrentModelIndex(*index);
}

void qt_widgets_c_QDataWidgetMapper_setItemDelegate(QDataWidgetMapper* this_ptr, QAbstractItemDelegate* delegate) {
  this_ptr->setItemDelegate(delegate);
}

void qt_widgets_c_QDataWidgetMapper_setModel(QDataWidgetMapper* this_ptr, QAbstractItemModel* model) {
  this_ptr->setModel(model);
}

void qt_widgets_c_QDataWidgetMapper_setOrientation(QDataWidgetMapper* this_ptr, const Qt::Orientation* aOrientation) {
  this_ptr->setOrientation(*aOrientation);
}

void qt_widgets_c_QDataWidgetMapper_setRootIndex(QDataWidgetMapper* this_ptr, const QModelIndex* index) {
  this_ptr->setRootIndex(*index);
}

void qt_widgets_c_QDataWidgetMapper_setSubmitPolicy(QDataWidgetMapper* this_ptr, QDataWidgetMapper::SubmitPolicy policy) {
  this_ptr->setSubmitPolicy(policy);
}

bool qt_widgets_c_QDataWidgetMapper_submit(QDataWidgetMapper* this_ptr) {
  return this_ptr->submit();
}

QDataWidgetMapper::SubmitPolicy qt_widgets_c_QDataWidgetMapper_submitPolicy(const QDataWidgetMapper* this_ptr) {
  return this_ptr->submitPolicy();
}

void qt_widgets_c_QDataWidgetMapper_toFirst(QDataWidgetMapper* this_ptr) {
  this_ptr->toFirst();
}

void qt_widgets_c_QDataWidgetMapper_toLast(QDataWidgetMapper* this_ptr) {
  this_ptr->toLast();
}

void qt_widgets_c_QDataWidgetMapper_toNext(QDataWidgetMapper* this_ptr) {
  this_ptr->toNext();
}

void qt_widgets_c_QDataWidgetMapper_toPrevious(QDataWidgetMapper* this_ptr) {
  this_ptr->toPrevious();
}

void qt_widgets_c_QDataWidgetMapper_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDataWidgetMapper::trUtf8(s, c, n));
}

void qt_widgets_c_QDataWidgetMapper_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDataWidgetMapper::tr(s, c, n));
}

