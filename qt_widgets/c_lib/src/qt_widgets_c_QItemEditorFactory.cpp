#include "qt_widgets_c_QItemEditorFactory.h"

QWidget* qt_widgets_c_QItemEditorFactory_createEditor(const QItemEditorFactory* this_ptr, int userType, QWidget* parent) {
  return this_ptr->createEditor(userType, parent);
}

const QItemEditorFactory* qt_widgets_c_QItemEditorFactory_defaultFactory() {
  return QItemEditorFactory::defaultFactory();
}

void qt_widgets_c_QItemEditorFactory_delete(QItemEditorFactory* this_ptr) {
  delete this_ptr;
}

QItemEditorFactory* qt_widgets_c_QItemEditorFactory_new() {
  return new QItemEditorFactory();
}

void qt_widgets_c_QItemEditorFactory_registerEditor(QItemEditorFactory* this_ptr, int userType, QItemEditorCreatorBase* creator) {
  this_ptr->registerEditor(userType, creator);
}

void qt_widgets_c_QItemEditorFactory_setDefaultFactory(QItemEditorFactory* factory) {
  QItemEditorFactory::setDefaultFactory(factory);
}

void qt_widgets_c_QItemEditorFactory_valuePropertyName_to_output(const QItemEditorFactory* this_ptr, int userType, QByteArray* output) {
  new(output) QByteArray(this_ptr->valuePropertyName(userType));
}

