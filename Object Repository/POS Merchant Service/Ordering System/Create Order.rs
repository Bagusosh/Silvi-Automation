<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Order</name>
   <tag></tag>
   <elementGuidId>cd65dd4f-fd8c-4a23-89b1-1a7956758900</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n\&quot;order_list\&quot; : [\r\n\t\t{\&quot;product_id\&quot; : \&quot;d7f77cb6-1622-4f90-ba81-c55affc1ed62\&quot;, \&quot;order_quantity\&quot; : 4}\r\n\t],\r\n        \&quot;order_custom_list\&quot; : [\r\n                {\&quot;product_name\&quot; : \&quot;Tissue\&quot;, \&quot;product_purchase_price\&quot; : 1000, \&quot;product_selling_price\&quot; : 3000, \&quot;order_quantity\&quot; : 1}\r\n        ],\r\n\t\&quot;customer_name\&quot; : \&quot;Aga\&quot;,\r\n\t\&quot;order_notes\&quot; : \&quot;Gak pedes\&quot;,\r\n        \&quot;payment_method\&quot; : \&quot;52b1ee2a-06ef-4c60-b422-39c6df52d96c\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${Silvi-Staging}/merchant/${userId}/order?billing_type=INVOICE</restUrl>
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
      <id>2f9c9792-12f4-4d05-b95b-9bcd373382f3</id>
      <masked>false</masked>
      <name>Silvi-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userId</defaultValue>
      <description></description>
      <id>fb74a460-f8ce-4bb8-bb58-837420fb3be2</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
