<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Edit Address</name>
   <tag></tag>
   <elementGuidId>7a445a09-0e6a-4a1c-a8b6-525e3f15cb59</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;address1\&quot; : \&quot;\&quot;,\r\n    \&quot;kodePos\&quot; : \&quot;\&quot;,\r\n    \&quot;kecamatan\&quot; : \&quot;\&quot;,\r\n    \&quot;kota\&quot; : \&quot;\&quot;,\r\n    \&quot;provinsi\&quot; : \&quot;\&quot;,\r\n    \&quot;latitude\&quot; : \&quot;\&quot;,\r\n    \&quot;longitude\&quot; : \&quot;\&quot;,\r\n    \&quot;default\&quot; : \&quot;\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${BaseFlick-Staging}/v2/users/${userId}/alamat/${alamatId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.BaseFlick-Staging</defaultValue>
      <description></description>
      <id>fcca8ee0-e974-426a-a3fa-12e17617c803</id>
      <masked>false</masked>
      <name>BaseFlick-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userId</defaultValue>
      <description></description>
      <id>924eaf1e-2d18-479a-b3ba-da03ba0b4d1e</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alamatId</defaultValue>
      <description></description>
      <id>7ad664e1-75e9-4b6b-bbf5-67f42810c6f0</id>
      <masked>false</masked>
      <name>alamatId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
