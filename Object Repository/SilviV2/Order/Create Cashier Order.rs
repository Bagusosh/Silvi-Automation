<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Cashier Order</name>
   <tag></tag>
   <elementGuidId>d9261c5c-e0ca-456c-ae61-26037e58c00d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;merchant_id\&quot;: \&quot;c010c7ed-527f-4514-8ff3-0aeb7acf37c0\&quot;,\n    \&quot;payment_method_id\&quot;: \&quot;df89cbeb-5983-4f52-b460-3c60cafca621\&quot;,\n    \&quot;order_note\&quot;: \&quot;Jangan kekecilan\&quot;,\n    \&quot;money_received\&quot;: 130000,\n    \&quot;customer_name\&quot;: \&quot;Anonim\&quot;,\n    \&quot;order_items\&quot;: [\n        {\n            \&quot;product_id\&quot;: \&quot;68be6b22-8021-4b7c-b27e-83e9b44a146d\&quot;,\n            \&quot;quantity\&quot;: 1,\n            \&quot;order_item_note\&quot;: \&quot;kecil kecilan\&quot;\n        }\n    ],\n    \&quot;custom_order_items\&quot;: [\n        {\n            \&quot;product_name\&quot;: \&quot;Apapun itu minumnya teh botol sosro\&quot;,\n            \&quot;quantity\&quot;: 1, \n            \&quot;base_price\&quot;: 30000\n        }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${Silvi-V2-Local}/order-summaries/cashier</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Silvi-V2-Local</defaultValue>
      <description></description>
      <id>b0345ca9-6922-46c0-8bb0-9e7c272f186a</id>
      <masked>false</masked>
      <name>Silvi-V2-Local</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
