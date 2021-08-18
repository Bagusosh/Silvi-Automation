<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Change Order List</name>
   <tag></tag>
   <elementGuidId>149a7f06-7e52-4a68-82c7-b44dff77eb0f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n\t\&quot;order_list\&quot; : [\r\n\t\t{\&quot;product_id\&quot; : \&quot;d7f77cb6-1622-4f90-ba81-c55affc1ed62\&quot;, \&quot;order_quantity\&quot; : 4}\r\n\t\t\r\n\t]\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${Silvi-Staging}/merchant/${userId}/order/${orderId}/change?billing_type=INVOICE</restUrl>
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
      <id>4b4ac46a-07a4-4df7-80ff-0fb898b3e946</id>
      <masked>false</masked>
      <name>Silvi-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userId</defaultValue>
      <description></description>
      <id>9051c98b-e1db-4ae5-8eea-a5eb23acdcbf</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.orderId</defaultValue>
      <description></description>
      <id>fab93931-88c1-4ea2-b26b-ff70ed0635e0</id>
      <masked>false</masked>
      <name>orderId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
