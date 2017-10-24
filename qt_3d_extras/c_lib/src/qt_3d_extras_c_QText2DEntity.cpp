#include "qt_3d_extras_c_QText2DEntity.h"

QObject* qt_3d_extras_c_QText2DEntity_G_static_cast_QObject_ptr(Qt3DExtras::QText2DEntity* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QEntity* qt_3d_extras_c_QText2DEntity_G_static_cast_Qt3DCore_QEntity_ptr(Qt3DExtras::QText2DEntity* ptr) {
  return static_cast<Qt3DCore::QEntity*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QText2DEntity_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QText2DEntity* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QText2DEntity* qt_3d_extras_c_QText2DEntity_G_static_cast_Qt3DExtras_QText2DEntity_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QText2DEntity*>(ptr);
}

Qt3DExtras::QText2DEntity* qt_3d_extras_c_QText2DEntity_G_static_cast_Qt3DExtras_QText2DEntity_ptr_Qt3DCore_QEntity(Qt3DCore::QEntity* ptr) {
  return static_cast<Qt3DExtras::QText2DEntity*>(ptr);
}

Qt3DExtras::QText2DEntity* qt_3d_extras_c_QText2DEntity_G_static_cast_Qt3DExtras_QText2DEntity_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QText2DEntity*>(ptr);
}

void qt_3d_extras_c_Qt3DExtras_QText2DEntity_color_to_output(const Qt3DExtras::QText2DEntity* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->color());
}

void qt_3d_extras_c_Qt3DExtras_QText2DEntity_delete(Qt3DExtras::QText2DEntity* this_ptr) {
  delete this_ptr;
}

void qt_3d_extras_c_Qt3DExtras_QText2DEntity_font_to_output(const Qt3DExtras::QText2DEntity* this_ptr, QFont* output) {
  new(output) QFont(this_ptr->font());
}

float qt_3d_extras_c_Qt3DExtras_QText2DEntity_height(const Qt3DExtras::QText2DEntity* this_ptr) {
  return this_ptr->height();
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QText2DEntity_metaObject(const Qt3DExtras::QText2DEntity* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DExtras::QText2DEntity* qt_3d_extras_c_Qt3DExtras_QText2DEntity_new_no_args() {
  return new Qt3DExtras::QText2DEntity();
}

Qt3DExtras::QText2DEntity* qt_3d_extras_c_Qt3DExtras_QText2DEntity_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QText2DEntity(parent);
}

int qt_3d_extras_c_Qt3DExtras_QText2DEntity_qt_metacall(Qt3DExtras::QText2DEntity* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QText2DEntity_qt_metacast(Qt3DExtras::QText2DEntity* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_extras_c_Qt3DExtras_QText2DEntity_setColor(Qt3DExtras::QText2DEntity* this_ptr, const QColor* color) {
  this_ptr->setColor(*color);
}

void qt_3d_extras_c_Qt3DExtras_QText2DEntity_setFont(Qt3DExtras::QText2DEntity* this_ptr, const QFont* font) {
  this_ptr->setFont(*font);
}

void qt_3d_extras_c_Qt3DExtras_QText2DEntity_setHeight(Qt3DExtras::QText2DEntity* this_ptr, float height) {
  this_ptr->setHeight(height);
}

void qt_3d_extras_c_Qt3DExtras_QText2DEntity_setText(Qt3DExtras::QText2DEntity* this_ptr, const QString* text) {
  this_ptr->setText(*text);
}

void qt_3d_extras_c_Qt3DExtras_QText2DEntity_setWidth(Qt3DExtras::QText2DEntity* this_ptr, float width) {
  this_ptr->setWidth(width);
}

void qt_3d_extras_c_Qt3DExtras_QText2DEntity_text_to_output(const Qt3DExtras::QText2DEntity* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

void qt_3d_extras_c_Qt3DExtras_QText2DEntity_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QText2DEntity::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QText2DEntity_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QText2DEntity::tr(s, c, n));
}

float qt_3d_extras_c_Qt3DExtras_QText2DEntity_width(const Qt3DExtras::QText2DEntity* this_ptr) {
  return this_ptr->width();
}

