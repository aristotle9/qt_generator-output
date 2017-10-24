#ifndef QT_CORE_C_QXMLSTREAMWRITER_H
#define QT_CORE_C_QXMLSTREAMWRITER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamWriter_autoFormatting(const QXmlStreamWriter* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QXmlStreamWriter_autoFormattingIndent(const QXmlStreamWriter* this_ptr);
QT_CORE_C_EXPORT QTextCodec* qt_core_c_QXmlStreamWriter_codec(const QXmlStreamWriter* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_constructor_array(QByteArray* array, QXmlStreamWriter* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_constructor_device(QIODevice* device, QXmlStreamWriter* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_constructor_no_args(QXmlStreamWriter* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_constructor_string(QString* string, QXmlStreamWriter* output);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_destructor(QXmlStreamWriter* this_ptr);
QT_CORE_C_EXPORT QIODevice* qt_core_c_QXmlStreamWriter_device(const QXmlStreamWriter* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QXmlStreamWriter_hasError(const QXmlStreamWriter* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_setAutoFormatting(QXmlStreamWriter* this_ptr, bool arg1);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_setAutoFormattingIndent(QXmlStreamWriter* this_ptr, int spacesOrTabs);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_setCodec_codec(QXmlStreamWriter* this_ptr, QTextCodec* codec);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_setCodec_codecName(QXmlStreamWriter* this_ptr, const char* codecName);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_setDevice(QXmlStreamWriter* this_ptr, QIODevice* device);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeAttribute_attribute(QXmlStreamWriter* this_ptr, const QXmlStreamAttribute* attribute);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeAttribute_namespaceUri_name_value(QXmlStreamWriter* this_ptr, const QString* namespaceUri, const QString* name, const QString* value);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeAttribute_qualifiedName_value(QXmlStreamWriter* this_ptr, const QString* qualifiedName, const QString* value);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeAttributes(QXmlStreamWriter* this_ptr, const QXmlStreamAttributes* attributes);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeCDATA(QXmlStreamWriter* this_ptr, const QString* text);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeCharacters(QXmlStreamWriter* this_ptr, const QString* text);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeComment(QXmlStreamWriter* this_ptr, const QString* text);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeCurrentToken(QXmlStreamWriter* this_ptr, const QXmlStreamReader* reader);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeDTD(QXmlStreamWriter* this_ptr, const QString* dtd);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeDefaultNamespace(QXmlStreamWriter* this_ptr, const QString* namespaceUri);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeEmptyElement_namespaceUri_name(QXmlStreamWriter* this_ptr, const QString* namespaceUri, const QString* name);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeEmptyElement_qualifiedName(QXmlStreamWriter* this_ptr, const QString* qualifiedName);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeEndDocument(QXmlStreamWriter* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeEndElement(QXmlStreamWriter* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeEntityReference(QXmlStreamWriter* this_ptr, const QString* name);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeNamespace_namespaceUri(QXmlStreamWriter* this_ptr, const QString* namespaceUri);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeNamespace_namespaceUri_prefix(QXmlStreamWriter* this_ptr, const QString* namespaceUri, const QString* prefix);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeProcessingInstruction_target(QXmlStreamWriter* this_ptr, const QString* target);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeProcessingInstruction_target_data(QXmlStreamWriter* this_ptr, const QString* target, const QString* data);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeStartDocument_no_args(QXmlStreamWriter* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeStartDocument_version(QXmlStreamWriter* this_ptr, const QString* version);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeStartDocument_version_standalone(QXmlStreamWriter* this_ptr, const QString* version, bool standalone);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeStartElement_namespaceUri_name(QXmlStreamWriter* this_ptr, const QString* namespaceUri, const QString* name);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeStartElement_qualifiedName(QXmlStreamWriter* this_ptr, const QString* qualifiedName);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeTextElement_namespaceUri_name_text(QXmlStreamWriter* this_ptr, const QString* namespaceUri, const QString* name, const QString* text);
QT_CORE_C_EXPORT void qt_core_c_QXmlStreamWriter_writeTextElement_qualifiedName_text(QXmlStreamWriter* this_ptr, const QString* qualifiedName, const QString* text);

} // extern "C"

#endif // QT_CORE_C_QXMLSTREAMWRITER_H
