<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Order</name>
   <tag></tag>
   <elementGuidId>c9de53cb-be66-4773-ab60-cf417d40d639</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;merchant_id\&quot;: \&quot;c010c7ed-527f-4514-8ff3-0aeb7acf37c0\&quot;,\n    \&quot;table_id\&quot;: \&quot;c713d8e3-dd79-4257-8438-d229e8aa6af4\&quot;,\n    \&quot;customer_id\&quot;: \&quot;78dcfc94-2b61-41e6-af3a-0831c18a4a92\&quot;,\n    \&quot;payment_method_id\&quot;: \&quot;e9b3e6f2-fd6f-4e3f-9ec5-19cf6cc05d24\&quot;,\n    \&quot;order_note\&quot;: \&quot;Jangan kekecilan\&quot;,\n    \&quot;order_items\&quot;: [\n        {\n            \&quot;product_id\&quot;: \&quot;8dcec4c7-ca15-4700-af56-9af1a2a3ae0d\&quot;,\n            \&quot;quantity\&quot;: 1,\n            \&quot;order_item_note\&quot;: \&quot;kecil kecilan\&quot;\n        }\n    ],\n    \&quot;discount_id\&quot;: \&quot;37b82dd7-bc2f-4524-9ba9-5fa290e35507\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${Silvi-V2-Local}/order-summaries</restUrl>
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
      <id>8fa2fff6-fb10-45ce-af03-c8dca09c2a92</id>
      <masked>false</masked>
      <name>Silvi-V2-Local</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
