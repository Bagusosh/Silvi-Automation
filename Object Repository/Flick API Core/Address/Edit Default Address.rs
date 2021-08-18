<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Edit Default Address</name>
   <tag></tag>
   <elementGuidId>db1968f1-9bdb-461a-b0dd-82886ad8dac7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;address1\&quot; : \&quot;\&quot;,\r\n    \&quot;kodePos\&quot; : \&quot;20124\&quot;,\r\n    \&quot;kecamatan\&quot; : \&quot;\&quot;,\r\n    \&quot;kota\&quot; : \&quot;\&quot;,\r\n    \&quot;provinsi\&quot; : \&quot;\&quot;,\r\n    \&quot;latitude\&quot; : \&quot;\&quot;,\r\n    \&quot;longitude\&quot; : \&quot;\&quot;,\r\n    \&quot;default\&quot; : \&quot;\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${BaseFlick-Staging}/v2/users/${userId}/alamat/default</restUrl>
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
      <id>8a7aa470-2ffa-4ac1-a33a-14129dc5b64c</id>
      <masked>false</masked>
      <name>BaseFlick-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userId</defaultValue>
      <description></description>
      <id>78e75566-80e7-40f5-ab13-30cf17977bad</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
