<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Delete Product Category</name>
   <tag></tag>
   <elementGuidId>b7f4c5a6-2cb9-46d8-8f87-aa4413bb7993</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;ganteng\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>DELETE</restRequestMethod>
   <restUrl>${Silvi-V2-Staging}/product-categories/1${categoryId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Silvi-V2-Staging</defaultValue>
      <description></description>
      <id>2eaa8f73-ca79-4575-85ad-3f41c02ab65b</id>
      <masked>false</masked>
      <name>Silvi-V2-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.categoryId</defaultValue>
      <description></description>
      <id>f14eab15-54d7-49d2-be26-d9337cb7a94e</id>
      <masked>false</masked>
      <name>categoryId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
