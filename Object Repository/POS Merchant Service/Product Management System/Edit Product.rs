<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Edit Product</name>
   <tag></tag>
   <elementGuidId>f0bb852f-d9f2-4100-ac6c-3db3a78c822f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;product_name\&quot;: \&quot;Makaroni Panggang Special\&quot;,\r\n    \&quot;product_image\&quot;: \&quot;\&quot;,\r\n    \&quot;product_category\&quot;: \&quot;8f3cc212-5103-46ba-b097-f9bc42d23353\&quot;,\r\n    \&quot;product_description\&quot; : \&quot;Makaroni di panggang\&quot;,\r\n    \&quot;product_selling_price\&quot;: 75000,\r\n    \&quot;product_purchase_price\&quot;: 30000,\r\n    \&quot;product_stock\&quot;: 20,\r\n    \&quot;product_min_stock\&quot;: 1\r\n}\r\n&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${Silvi-Staging}merchant/${userId}/product/${productId}</restUrl>
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
      <id>9fcf0fb0-142a-41cf-a8ab-e281814d93ea</id>
      <masked>false</masked>
      <name>Silvi-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userId</defaultValue>
      <description></description>
      <id>f563d94b-b485-4a1a-81cd-550524bbd8de</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.productId</defaultValue>
      <description></description>
      <id>13a5f977-d1d8-4a95-a106-4381b85ea172</id>
      <masked>false</masked>
      <name>productId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
