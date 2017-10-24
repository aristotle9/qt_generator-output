#include "qt_3d_core_c_QNode.h"

void qt_3d_core_c_QNode_G_Qt3DCore_qIdForNode_to_output(Qt3DCore::QNode* node, Qt3DCore::QNodeId* output) {
  new(output) Qt3DCore::QNodeId(Qt3DCore::qIdForNode(node));
}

QObject* qt_3d_core_c_QNode_G_static_cast_QObject_ptr(Qt3DCore::QNode* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_core_c_QNode_G_static_cast_Qt3DCore_QNode_ptr(QObject* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

void qt_3d_core_c_Qt3DCore_QNodeIdTypePair_constructor__id__type(const Qt3DCore::QNodeId* _id, const QMetaObject* _type, Qt3DCore::QNodeIdTypePair* output) {
  new(output) Qt3DCore::QNodeIdTypePair(*_id, _type);
}

void qt_3d_core_c_Qt3DCore_QNodeIdTypePair_constructor_no_args(Qt3DCore::QNodeIdTypePair* output) {
  new(output) Qt3DCore::QNodeIdTypePair();
}

void qt_3d_core_c_Qt3DCore_QNodeIdTypePair_destructor(Qt3DCore::QNodeIdTypePair* this_ptr) {
  qt_3d_core_c_call_destructor(this_ptr);
}

const Qt3DCore::QNodeId* qt_3d_core_c_Qt3DCore_QNodeIdTypePair_id(const Qt3DCore::QNodeIdTypePair* this_ptr) {
  return &this_ptr->id;
}

Qt3DCore::QNodeId* qt_3d_core_c_Qt3DCore_QNodeIdTypePair_id_mut(Qt3DCore::QNodeIdTypePair* this_ptr) {
  return &this_ptr->id;
}

void qt_3d_core_c_Qt3DCore_QNodeIdTypePair_set_id(Qt3DCore::QNodeIdTypePair* this_ptr, const Qt3DCore::QNodeId* value) {
  this_ptr->id = *value;
}

void qt_3d_core_c_Qt3DCore_QNodeIdTypePair_set_type(Qt3DCore::QNodeIdTypePair* this_ptr, const QMetaObject* value) {
  this_ptr->type = value;
}

const QMetaObject* qt_3d_core_c_Qt3DCore_QNodeIdTypePair_type(const Qt3DCore::QNodeIdTypePair* this_ptr) {
  return this_ptr->type;
}

bool qt_3d_core_c_Qt3DCore_QNode_blockNotifications(Qt3DCore::QNode* this_ptr, bool block) {
  return this_ptr->blockNotifications(block);
}

void qt_3d_core_c_Qt3DCore_QNode_childNodes_to_output(const Qt3DCore::QNode* this_ptr, QVector< Qt3DCore::QNode* >* output) {
  new(output) QVector< Qt3DCore::QNode* >(this_ptr->childNodes());
}

void qt_3d_core_c_Qt3DCore_QNode_clearPropertyTracking(Qt3DCore::QNode* this_ptr, const QString* propertyName) {
  this_ptr->clearPropertyTracking(*propertyName);
}

void qt_3d_core_c_Qt3DCore_QNode_clearPropertyTrackings(Qt3DCore::QNode* this_ptr) {
  this_ptr->clearPropertyTrackings();
}

Qt3DCore::QNode::PropertyTrackingMode qt_3d_core_c_Qt3DCore_QNode_defaultPropertyTrackingMode(const Qt3DCore::QNode* this_ptr) {
  return this_ptr->defaultPropertyTrackingMode();
}

void qt_3d_core_c_Qt3DCore_QNode_delete(Qt3DCore::QNode* this_ptr) {
  delete this_ptr;
}

void qt_3d_core_c_Qt3DCore_QNode_id_to_output(const Qt3DCore::QNode* this_ptr, Qt3DCore::QNodeId* output) {
  new(output) Qt3DCore::QNodeId(this_ptr->id());
}

bool qt_3d_core_c_Qt3DCore_QNode_isEnabled(const Qt3DCore::QNode* this_ptr) {
  return this_ptr->isEnabled();
}

const QMetaObject* qt_3d_core_c_Qt3DCore_QNode_metaObject(const Qt3DCore::QNode* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DCore::QNode* qt_3d_core_c_Qt3DCore_QNode_new_no_args() {
  return new Qt3DCore::QNode();
}

Qt3DCore::QNode* qt_3d_core_c_Qt3DCore_QNode_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DCore::QNode(parent);
}

bool qt_3d_core_c_Qt3DCore_QNode_notificationsBlocked(const Qt3DCore::QNode* this_ptr) {
  return this_ptr->notificationsBlocked();
}

Qt3DCore::QNode* qt_3d_core_c_Qt3DCore_QNode_parentNode(const Qt3DCore::QNode* this_ptr) {
  return this_ptr->parentNode();
}

Qt3DCore::QNode::PropertyTrackingMode qt_3d_core_c_Qt3DCore_QNode_propertyTracking(const Qt3DCore::QNode* this_ptr, const QString* propertyName) {
  return this_ptr->propertyTracking(*propertyName);
}

int qt_3d_core_c_Qt3DCore_QNode_qt_metacall(Qt3DCore::QNode* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_core_c_Qt3DCore_QNode_qt_metacast(Qt3DCore::QNode* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_core_c_Qt3DCore_QNode_setDefaultPropertyTrackingMode(Qt3DCore::QNode* this_ptr, Qt3DCore::QNode::PropertyTrackingMode mode) {
  this_ptr->setDefaultPropertyTrackingMode(mode);
}

void qt_3d_core_c_Qt3DCore_QNode_setEnabled(Qt3DCore::QNode* this_ptr, bool isEnabled) {
  this_ptr->setEnabled(isEnabled);
}

void qt_3d_core_c_Qt3DCore_QNode_setParent(Qt3DCore::QNode* this_ptr, Qt3DCore::QNode* parent) {
  this_ptr->setParent(parent);
}

void qt_3d_core_c_Qt3DCore_QNode_setPropertyTracking(Qt3DCore::QNode* this_ptr, const QString* propertyName, Qt3DCore::QNode::PropertyTrackingMode trackMode) {
  this_ptr->setPropertyTracking(*propertyName, trackMode);
}

void qt_3d_core_c_Qt3DCore_QNode_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DCore::QNode::trUtf8(s, c, n));
}

void qt_3d_core_c_Qt3DCore_QNode_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DCore::QNode::tr(s, c, n));
}

