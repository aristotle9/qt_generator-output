#include "qt_widgets_c_QItemEditorCreatorBase.h"

QWidget* qt_widgets_c_QItemEditorCreatorBase_createWidget(const QItemEditorCreatorBase* this_ptr, QWidget* parent) {
  return this_ptr->createWidget(parent);
}

void qt_widgets_c_QItemEditorCreatorBase_delete(QItemEditorCreatorBase* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QItemEditorCreatorBase_valuePropertyName_to_output(const QItemEditorCreatorBase* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->valuePropertyName());
}

