#include "qt_core_c_QXmlStreamWriter.h"

bool qt_core_c_QXmlStreamWriter_autoFormatting(const QXmlStreamWriter* this_ptr) {
  return this_ptr->autoFormatting();
}

int qt_core_c_QXmlStreamWriter_autoFormattingIndent(const QXmlStreamWriter* this_ptr) {
  return this_ptr->autoFormattingIndent();
}

QTextCodec* qt_core_c_QXmlStreamWriter_codec(const QXmlStreamWriter* this_ptr) {
  return this_ptr->codec();
}

void qt_core_c_QXmlStreamWriter_constructor_array(QByteArray* array, QXmlStreamWriter* output) {
  new(output) QXmlStreamWriter(array);
}

void qt_core_c_QXmlStreamWriter_constructor_device(QIODevice* device, QXmlStreamWriter* output) {
  new(output) QXmlStreamWriter(device);
}

void qt_core_c_QXmlStreamWriter_constructor_no_args(QXmlStreamWriter* output) {
  new(output) QXmlStreamWriter();
}

void qt_core_c_QXmlStreamWriter_constructor_string(QString* string, QXmlStreamWriter* output) {
  new(output) QXmlStreamWriter(string);
}

void qt_core_c_QXmlStreamWriter_destructor(QXmlStreamWriter* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

QIODevice* qt_core_c_QXmlStreamWriter_device(const QXmlStreamWriter* this_ptr) {
  return this_ptr->device();
}

bool qt_core_c_QXmlStreamWriter_hasError(const QXmlStreamWriter* this_ptr) {
  return this_ptr->hasError();
}

void qt_core_c_QXmlStreamWriter_setAutoFormatting(QXmlStreamWriter* this_ptr, bool arg1) {
  this_ptr->setAutoFormatting(arg1);
}

void qt_core_c_QXmlStreamWriter_setAutoFormattingIndent(QXmlStreamWriter* this_ptr, int spacesOrTabs) {
  this_ptr->setAutoFormattingIndent(spacesOrTabs);
}

void qt_core_c_QXmlStreamWriter_setCodec_codec(QXmlStreamWriter* this_ptr, QTextCodec* codec) {
  this_ptr->setCodec(codec);
}

void qt_core_c_QXmlStreamWriter_setCodec_codecName(QXmlStreamWriter* this_ptr, const char* codecName) {
  this_ptr->setCodec(codecName);
}

void qt_core_c_QXmlStreamWriter_setDevice(QXmlStreamWriter* this_ptr, QIODevice* device) {
  this_ptr->setDevice(device);
}

void qt_core_c_QXmlStreamWriter_writeAttribute_attribute(QXmlStreamWriter* this_ptr, const QXmlStreamAttribute* attribute) {
  this_ptr->writeAttribute(*attribute);
}

void qt_core_c_QXmlStreamWriter_writeAttribute_namespaceUri_name_value(QXmlStreamWriter* this_ptr, const QString* namespaceUri, const QString* name, const QString* value) {
  this_ptr->writeAttribute(*namespaceUri, *name, *value);
}

void qt_core_c_QXmlStreamWriter_writeAttribute_qualifiedName_value(QXmlStreamWriter* this_ptr, const QString* qualifiedName, const QString* value) {
  this_ptr->writeAttribute(*qualifiedName, *value);
}

void qt_core_c_QXmlStreamWriter_writeAttributes(QXmlStreamWriter* this_ptr, const QXmlStreamAttributes* attributes) {
  this_ptr->writeAttributes(*attributes);
}

void qt_core_c_QXmlStreamWriter_writeCDATA(QXmlStreamWriter* this_ptr, const QString* text) {
  this_ptr->writeCDATA(*text);
}

void qt_core_c_QXmlStreamWriter_writeCharacters(QXmlStreamWriter* this_ptr, const QString* text) {
  this_ptr->writeCharacters(*text);
}

void qt_core_c_QXmlStreamWriter_writeComment(QXmlStreamWriter* this_ptr, const QString* text) {
  this_ptr->writeComment(*text);
}

void qt_core_c_QXmlStreamWriter_writeCurrentToken(QXmlStreamWriter* this_ptr, const QXmlStreamReader* reader) {
  this_ptr->writeCurrentToken(*reader);
}

void qt_core_c_QXmlStreamWriter_writeDTD(QXmlStreamWriter* this_ptr, const QString* dtd) {
  this_ptr->writeDTD(*dtd);
}

void qt_core_c_QXmlStreamWriter_writeDefaultNamespace(QXmlStreamWriter* this_ptr, const QString* namespaceUri) {
  this_ptr->writeDefaultNamespace(*namespaceUri);
}

void qt_core_c_QXmlStreamWriter_writeEmptyElement_namespaceUri_name(QXmlStreamWriter* this_ptr, const QString* namespaceUri, const QString* name) {
  this_ptr->writeEmptyElement(*namespaceUri, *name);
}

void qt_core_c_QXmlStreamWriter_writeEmptyElement_qualifiedName(QXmlStreamWriter* this_ptr, const QString* qualifiedName) {
  this_ptr->writeEmptyElement(*qualifiedName);
}

void qt_core_c_QXmlStreamWriter_writeEndDocument(QXmlStreamWriter* this_ptr) {
  this_ptr->writeEndDocument();
}

void qt_core_c_QXmlStreamWriter_writeEndElement(QXmlStreamWriter* this_ptr) {
  this_ptr->writeEndElement();
}

void qt_core_c_QXmlStreamWriter_writeEntityReference(QXmlStreamWriter* this_ptr, const QString* name) {
  this_ptr->writeEntityReference(*name);
}

void qt_core_c_QXmlStreamWriter_writeNamespace_namespaceUri(QXmlStreamWriter* this_ptr, const QString* namespaceUri) {
  this_ptr->writeNamespace(*namespaceUri);
}

void qt_core_c_QXmlStreamWriter_writeNamespace_namespaceUri_prefix(QXmlStreamWriter* this_ptr, const QString* namespaceUri, const QString* prefix) {
  this_ptr->writeNamespace(*namespaceUri, *prefix);
}

void qt_core_c_QXmlStreamWriter_writeProcessingInstruction_target(QXmlStreamWriter* this_ptr, const QString* target) {
  this_ptr->writeProcessingInstruction(*target);
}

void qt_core_c_QXmlStreamWriter_writeProcessingInstruction_target_data(QXmlStreamWriter* this_ptr, const QString* target, const QString* data) {
  this_ptr->writeProcessingInstruction(*target, *data);
}

void qt_core_c_QXmlStreamWriter_writeStartDocument_no_args(QXmlStreamWriter* this_ptr) {
  this_ptr->writeStartDocument();
}

void qt_core_c_QXmlStreamWriter_writeStartDocument_version(QXmlStreamWriter* this_ptr, const QString* version) {
  this_ptr->writeStartDocument(*version);
}

void qt_core_c_QXmlStreamWriter_writeStartDocument_version_standalone(QXmlStreamWriter* this_ptr, const QString* version, bool standalone) {
  this_ptr->writeStartDocument(*version, standalone);
}

void qt_core_c_QXmlStreamWriter_writeStartElement_namespaceUri_name(QXmlStreamWriter* this_ptr, const QString* namespaceUri, const QString* name) {
  this_ptr->writeStartElement(*namespaceUri, *name);
}

void qt_core_c_QXmlStreamWriter_writeStartElement_qualifiedName(QXmlStreamWriter* this_ptr, const QString* qualifiedName) {
  this_ptr->writeStartElement(*qualifiedName);
}

void qt_core_c_QXmlStreamWriter_writeTextElement_namespaceUri_name_text(QXmlStreamWriter* this_ptr, const QString* namespaceUri, const QString* name, const QString* text) {
  this_ptr->writeTextElement(*namespaceUri, *name, *text);
}

void qt_core_c_QXmlStreamWriter_writeTextElement_qualifiedName_text(QXmlStreamWriter* this_ptr, const QString* qualifiedName, const QString* text) {
  this_ptr->writeTextElement(*qualifiedName, *text);
}

