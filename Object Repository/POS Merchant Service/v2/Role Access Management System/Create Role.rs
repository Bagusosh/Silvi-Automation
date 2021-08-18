<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Role</name>
   <tag></tag>
   <elementGuidId>4ab5fd69-9367-42d9-a0f6-8340105117a1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n\t\&quot;role_name\&quot; : \&quot;Kasir\&quot;,\r\n\t\&quot;role_access\&quot; : [\r\n\t\t\&quot;VIEW_ORDER\&quot;,\r\n                \&quot;CREATE_ORDER\&quot;,\r\n                \&quot;EDIT_ORDER\&quot;,\r\n                \&quot;VIEW_REPORT\&quot;,\r\n    \t        \&quot;DOWNLOAD_REPORT\&quot;\r\n\t]\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${Silvi-Staging}merchant/${userId}/roles</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Silvi-Staging</defaultValue>
      <description></description>
      <id>7210597e-94b5-4a84-b284-d8bccbadb1a7</id>
      <masked>false</masked>
      <name>Silvi-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userId</defaultValue>
      <description></description>
      <id>1ef08154-2f01-4206-955c-4fcca3d700af</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
