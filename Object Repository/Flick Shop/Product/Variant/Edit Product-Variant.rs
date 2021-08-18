<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Edit Product-Variant</name>
   <tag></tag>
   <elementGuidId>43c1151d-76ec-4306-8027-d2e0239a471a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n  \&quot;type\&quot;: \&quot;string\&quot;,\r\n  \&quot;items\&quot;: [\r\n    {\r\n      \&quot;name\&quot;: \&quot;string\&quot;,\r\n      \&quot;quantity\&quot;: 0,\r\n      \&quot;price\&quot;: 0,\r\n      \&quot;childVariant\&quot;: {\r\n        \&quot;type\&quot;: \&quot;string\&quot;,\r\n        \&quot;childItems\&quot;: [\r\n          {\r\n            \&quot;name\&quot;: \&quot;string\&quot;,\r\n            \&quot;quantity\&quot;: 0,\r\n            \&quot;price\&quot;: 0\r\n          }\r\n        ]\r\n      },\r\n      \&quot;image\&quot;: \&quot;string\&quot;\r\n    }\r\n  ]\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${FlickShop-Staging}/v1/product-variant/${productId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.FlickShop-Staging</defaultValue>
      <description></description>
      <id>6c3792a3-145a-48c5-8530-40f6aa59be88</id>
      <masked>false</masked>
      <name>FlickShop-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.productId</defaultValue>
      <description></description>
      <id>5106d7b2-f0a8-450b-9dbc-bd6479909fb9</id>
      <masked>false</masked>
      <name>productId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
