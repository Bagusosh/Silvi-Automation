<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Edit Inventory</name>
   <tag></tag>
   <elementGuidId>c9dbcc06-d70b-445a-974e-47d12e809255</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n\t\&quot;inventory_name\&quot; : \&quot;Garam\&quot;,\r\n    \&quot;inventory_image\&quot; : \&quot;\&quot;,\r\n\t\&quot;inventory_category\&quot; : \&quot;062499c7-9c39-42f9-b07d-353c55c3e119\&quot;,\r\n\t\&quot;unit_stock\&quot; : 15000,\r\n\t\&quot;unit_label\&quot; : \&quot;gram\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${Silvi-Staging}merchant/${userId}/inventory/${inventoryId}</restUrl>
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
      <id>40945deb-2d37-4049-9dc4-3a0f4ba06ae3</id>
      <masked>false</masked>
      <name>Silvi-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userId</defaultValue>
      <description></description>
      <id>8e20a2d8-2122-4797-913b-729877bf1dae</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.inventoryId</defaultValue>
      <description></description>
      <id>0cb4431b-4d94-4a4a-bbb1-0bc871b6f86b</id>
      <masked>false</masked>
      <name>inventoryId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
