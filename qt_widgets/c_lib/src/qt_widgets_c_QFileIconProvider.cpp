#include "qt_widgets_c_QFileIconProvider.h"

void qt_widgets_c_QFileIconProvider_delete(QFileIconProvider* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QFileIconProvider_icon_to_output_info(const QFileIconProvider* this_ptr, const QFileInfo* info, QIcon* output) {
  new(output) QIcon(this_ptr->icon(*info));
}

void qt_widgets_c_QFileIconProvider_icon_to_output_type(const QFileIconProvider* this_ptr, QFileIconProvider::IconType type, QIcon* output) {
  new(output) QIcon(this_ptr->icon(type));
}

QFileIconProvider* qt_widgets_c_QFileIconProvider_new() {
  return new QFileIconProvider();
}

unsigned int qt_widgets_c_QFileIconProvider_options(const QFileIconProvider* this_ptr) {
  return uint(this_ptr->options());
}

void qt_widgets_c_QFileIconProvider_setOptions(QFileIconProvider* this_ptr, unsigned int options) {
  this_ptr->setOptions(QFlags< QFileIconProvider::Option >(options));
}

void qt_widgets_c_QFileIconProvider_type_to_output(const QFileIconProvider* this_ptr, const QFileInfo* info, QString* output) {
  new(output) QString(this_ptr->type(*info));
}

