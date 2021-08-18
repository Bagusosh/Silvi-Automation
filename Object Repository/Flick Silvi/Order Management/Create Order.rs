<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Order</name>
   <tag></tag>
   <elementGuidId>e218b06f-a42c-4994-83d0-6b9efd291fee</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;order_list\&quot; : [\r\n        {\r\n            \&quot;product_id\&quot; : \&quot;6759c468-a075-4828-97f7-f0dbea37e1ab\&quot;,\r\n            \&quot;order_notes\&quot; : \&quot;Earbuds nya pink ya\&quot;,\r\n            \&quot;order_quantity\&quot; : 1\r\n        }\r\n    ],\r\n    \&quot;customer_id\&quot; : \&quot;78dcfc94-2b61-41e6-af3a-0831c18a4a92\&quot;,\r\n    \&quot;customer_name\&quot; : \&quot;Putu Prema\&quot;,\r\n    \&quot;payment_method\&quot; : \&quot;b5f3a8c7-7dc7-4cb9-99a1-2221e6481c8d\&quot;,\r\n    \&quot;merchant_table\&quot; : \&quot;03979e03-3618-4b39-a1dc-68ef939191d0\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${Flick-Silvi-Staging}/users/${userId}/order</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Flick-Silvi-Staging</defaultValue>
      <description></description>
      <id>c8f0560c-1897-4a4b-ad4c-b79c68ddbf93</id>
      <masked>false</masked>
      <name>Flick-Silvi-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userId</defaultValue>
      <description></description>
      <id>50099b11-1c87-46b7-8c9c-80b9d25a3922</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
